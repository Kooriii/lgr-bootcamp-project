use crate::helpers::TestApplication;

#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApplication::new().await;

    let response = app.get_root().await;

    assert_eq!(response.status(), reqwest::StatusCode::OK);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}
