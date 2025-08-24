use crate::helpers::TestApplication;

#[tokio::test]
async fn logout() {
    let app = TestApplication::new().await;
    let response = app.get_logout().await;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
}
