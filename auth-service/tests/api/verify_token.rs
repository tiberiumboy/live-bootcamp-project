use crate::helpers::TestApp;
use auth_service::domain::email::Email;
use auth_service::routes::jwt::JWToken;
use auth_service::utils::auth::generate_auth_token;
use reqwest::StatusCode;
use test_helpers::api_test;

#[api_test]
async fn valid_token_should_return_200() {
    let random_email = TestApp::get_random_email();
    let email = Email::parse(random_email).expect("Unable to parse email");
    let jwt = generate_auth_token(&email)
        .expect("dummy token is not valid! Please provide a valid token!");
    let body = JWToken { token: jwt };

    let result = app.post_verify_token(&body).await;
    assert_eq!(result.status(), StatusCode::OK);
}

#[api_test]
async fn malformed_input_should_return_422() {
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
}

#[api_test]
async fn invalid_token_should_return_401() {
    let body = serde_json::json!({
        "token": "invalid",
    });
    let result = app.post_verify_token(&body).await;
    assert_eq!(result.status(), StatusCode::UNAUTHORIZED);
}
