use crate::helpers::TestApp;
use auth_service::domain::email::Email;
use auth_service::routes::jwt::JWToken;
use auth_service::utils::auth::generate_auth_token;
use reqwest::StatusCode;

#[tokio::test]
async fn valid_token_should_return_200() {
    let mut app = TestApp::new().await;
    let email = Email::parse("test@test.com").expect("Unable to parse email");
    let jwt = generate_auth_token(&email)
        .expect("dummy token is not valid! Please provide a valid token!");
    let body = JWToken { token: jwt };
    let result = app.post_verify_token(&body).await;
    assert_eq!(result.status().as_u16(), 200);

    app.clean_up().await;
}

#[tokio::test]
async fn malformed_input_should_return_422() {
    let mut app = TestApp::new().await;

    // an error 422 returns unprocessable content. Fill in invalid token type.
    let test_case = [
        serde_json::json!({
            "token":true,
        }),
        serde_json::json!({
            "token":null,       // wrong typecast
        }),
        serde_json::json!({
            "token":123
        }),
        serde_json::json!({
            "token1":"token"    // misspelled field name
        }),
    ];

    for test in test_case {
        let response = &app.post_verify_token(&test).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    app.clean_up().await;
}

#[tokio::test]
async fn invalid_token_should_return_401() {
    let mut app = TestApp::new().await;
    let body = serde_json::json!({
        "token": "invalid",
    });
    let result = app.post_verify_token(&body).await;
    assert_eq!(result.status(), StatusCode::UNAUTHORIZED);

    app.clean_up().await;
}
