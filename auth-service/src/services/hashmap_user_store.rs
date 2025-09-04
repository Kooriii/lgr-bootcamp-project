use std::collections::HashMap;

use crate::domain::{user::User, Email, Password, UserStore, UserStoreError};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(&email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: Email::parse("test@example.com".to_owned()).unwrap(),
            password: Password::parse("password".to_owned()).unwrap(),
            requires_2fa: false,
        };

        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        let result = user_store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let user = User {
            email: email.clone(),
            password: Password::parse("password".to_owned()).unwrap(),
            requires_2fa: false,
        };

        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        let result = user_store.get_user(&email).await;
        assert!(result.is_ok());
        assert!(result.unwrap().email == email);

        let result = user_store
            .get_user(&Email::parse("fake@example.com".to_owned()).unwrap())
            .await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let email = Email::parse("test@example.com".to_owned()).unwrap();
        let password = Password::parse("password".to_owned()).unwrap();
        let user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: false,
        };

        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        let result = user_store.validate_user(&email, &password).await;
        assert!(result.is_ok());

        let result = user_store
            .validate_user(&email, &Password::parse("fakepassword".to_owned()).unwrap())
            .await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        let result = user_store
            .validate_user(
                &Email::parse("fake@example.com".to_owned()).unwrap(),
                &password,
            )
            .await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
