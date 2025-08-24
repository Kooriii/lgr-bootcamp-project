use crate::helpers::TestApplication;

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApplication::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup() {
    let app = TestApplication::new().await;
    let response = app.get_signup().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn login() {
    let app = TestApplication::new().await;
    let response = app.get_login().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn logout() {
    let app = TestApplication::new().await;
    let response = app.get_logout().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn verify_2fa() {
    let app = TestApplication::new().await;
    let response = app.get_verify_2fa().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}

#[tokio::test]
async fn verify_token() {
    let app = TestApplication::new().await;
    let response = app.get_verify_token().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
