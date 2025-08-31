use std::collections::HashMap;

use crate::domain::user::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
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

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".to_owned(),
            password: "password".to_owned(),
            requires_2fa: false,
        };

        let result = user_store.add_user(user.clone());
        assert!(result.is_ok());

        let result = user_store.add_user(user);
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".to_owned(),
            password: "password".to_owned(),
            requires_2fa: false,
        };

        let result = user_store.add_user(user.clone());
        assert!(result.is_ok());

        let result = user_store.get_user("test@example.com");
        assert!(result.is_ok());
        assert!(result.unwrap().email == "test@example.com");

        let result = user_store.get_user("fake@example.com");
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User {
            email: "test@example.com".to_owned(),
            password: "password".to_owned(),
            requires_2fa: false,
        };

        let result = user_store.add_user(user.clone());
        assert!(result.is_ok());

        let result = user_store.validate_user("test@example.com", "password");
        assert!(result.is_ok());

        let result = user_store.validate_user("test@example.com", "fakepassword");
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        let result = user_store.validate_user("fake@example.com", "password");
        assert_eq!(result, Err(UserStoreError::UserNotFound));
    }
}
