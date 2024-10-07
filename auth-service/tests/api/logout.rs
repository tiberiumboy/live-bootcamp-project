use reqwest::StatusCode;

use crate::helpers::TestApp;

#[tokio::test]
async fn logoff_should_pass() {
    let app = TestApp::new().await;
    let result = app.post_logout().await;
    assert_eq!(result.status(), StatusCode::OK);
}
