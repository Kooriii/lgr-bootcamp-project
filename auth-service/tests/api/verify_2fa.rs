use crate::helpers::TestApplication;

#[tokio::test]
async fn verify_2fa() {
    let app = TestApplication::new().await;
    let response = app.get_verify_2fa().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
