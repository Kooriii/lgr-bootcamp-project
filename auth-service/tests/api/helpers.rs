use auth_service::Application;
use uuid::Uuid;

pub struct TestApplication {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApplication {
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build application");

        let address = format!("http://{}", app.address);

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::new();

        Self {
            address,
            http_client,
        }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to signup")
    }

    pub fn get_random_email() -> String {
        format!("{}@example.com", Uuid::new_v4())
    }

    pub async fn get_login(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .send()
            .await
            .expect("Failed to login")
    }

    pub async fn get_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to logout")
    }

    pub async fn get_verify_2fa(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify_2fa", &self.address))
            .send()
            .await
            .expect("Failed to verify 2fa")
    }

    pub async fn get_verify_token(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify_token", &self.address))
            .send()
            .await
            .expect("Failed to verify token")
    }
}
