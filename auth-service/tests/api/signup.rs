use crate::helpers::TestApplication;

#[tokio::test]
async fn signup() {
    let app = TestApplication::new().await;
    let response = app.get_signup().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
