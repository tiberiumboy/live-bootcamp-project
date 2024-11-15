use crate::helpers::TestApp;
use reqwest::StatusCode;
use secrecy::ExposeSecret;
use test_helpers::api_test;

#[api_test]
pub async fn should_return_201_if_valid_input() {
    let test_case = serde_json::json!(
        {
            "email": TestApp::get_random_email().expose_secret(),
            "password":"password123!",
            "requires2FA":true
        }
    );

    let result = app.post_signup(&test_case).await;
    assert_eq!(result.status(), StatusCode::CREATED);
}

#[api_test]
pub async fn should_return_400_if_invalid_input() {
    // the input is considered invalid if :
    // - the email is empty or does not contain '@'
    // - the password is less than 8 characters
    let random_email = TestApp::get_random_email();

    let test_cases = [
        serde_json::json!({
            // password is too short
            "email": &random_email.expose_secret(),
            "password": "passwor",
            "requires2FA": true
        }),
        serde_json::json!({
            // email do not have '@' sign which is required
            "email": &random_email.expose_secret().replace("@", "."),
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            // email is empty
            "email": "",
            "password": "password123",
            "requires2FA": true
        }),
    ];

    for test in test_cases {
        let response = app.post_signup(&test).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

#[api_test]
pub async fn should_return_409_if_email_already_exists() {
    let random_email = TestApp::get_random_email();
    let random_password = TestApp::get_random_email();

    let user = serde_json::json!({
        "email": random_email.expose_secret(),
        "password": random_password.expose_secret(),
        "requires2FA": true
    });

    // first we insert the user successfully
    let response = app.post_signup(&user).await;
    assert_eq!(response.status(), StatusCode::CREATED);

    // then if we try to insert the user again we should get 409 error code
    let response = app.post_signup(&user).await;
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

#[api_test]
async fn should_return_422_if_malformed_input() {
    let random_email = TestApp::get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": &random_email.expose_secret(),
            "password": "password123",
        }),
        serde_json::json!({
            "email": &random_email.expose_secret(),
            "requires2FA": true
        }),
    ];

    for test in test_cases {
        let response = app.post_signup(&test).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

// not sure how we can check for error code 500 since that's a server side issue not a software issue...?
