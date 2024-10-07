use crate::helpers::TestApp;
use auth_service::routes::jwt::JWToken;

#[tokio::test]
async fn verify_token_should_pass() {
    let app = TestApp::new().await;
    let email = "test@test.com".to_owned();
    let code = "0000";
    let id = "";
    let jwt = JWToken::validate(email, id, code)
        .expect("dummy token is not valid! Please provide a valid token!");
    let result = app.post_verify_token(&jwt).await;
    assert_eq!(result.status().as_u16(), 200);
}
