use crate::helpers::TestApplication;

#[tokio::test]
async fn verify_token() {
    let app = TestApplication::new().await;
    let response = app.get_verify_token().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
