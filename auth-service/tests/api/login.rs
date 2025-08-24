use crate::helpers::TestApplication;

#[tokio::test]
async fn login() {
    let app = TestApplication::new().await;
    let response = app.get_login().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
