use crate::helpers::TestApp;
use auth_service::{
    domain::email::Email, routes::TwoFactorAuthResponse, utils::constants::JWT_COOKIE_NAME,
};
use reqwest::StatusCode;
use secrecy::{ExposeSecret, Secret};
use test_helpers::api_test;

#[api_test]
pub async fn should_return_200_if_valid_cred_no_2fa() {
    let email = TestApp::get_random_email();

    let signup = serde_json::json!({
        "email":email.expose_secret(),
        "password":"Password123!",
        "requires2FA": false
    });

    let response = app.post_signup(&signup).await;
    assert_eq!(response.status(), StatusCode::CREATED);

    let login = serde_json::json!({
        "email": email.expose_secret(),
        "password":"Password123!"
    });

    let response = app.post_login(&login).await;
    assert_eq!(response.status(), StatusCode::OK);

    let auth_cookie = response
        .cookies()
        .find(|cookie| cookie.name() == JWT_COOKIE_NAME)
        .expect("No auth cookie found!");

    assert!(!auth_cookie.value().is_empty());
}

#[api_test]
pub async fn should_return_206_if_valid_cred_with_2fa() {
    let input = "test@test.com".to_owned();
    let secret = Secret::new(input.clone());
    let email = Email::parse(secret).expect("Unable to parse dummy email account");
    // first, create a test account.
    let body = serde_json::json!({
        "email":input.clone(),
        "password":"Password123!",
        "requires2FA": true
    });
    let new_account = app.post_signup(&body).await;
    assert_eq!(new_account.status(), StatusCode::CREATED);

    // then, log into test account
    let test = serde_json::json!({
        "email":input,
        "password":"Password123!"
    });

    // check for result
    let result = app.post_login(&test).await;
    assert_eq!(result.status(), StatusCode::PARTIAL_CONTENT);
    let body = result
        .json::<TwoFactorAuthResponse>()
        .await
        .expect("Should receive a Login response!");
    {
        let two_fa_code_store = &app.two_fa_code_store.read().await;
        let code = two_fa_code_store
            .get_code(&email)
            .await
            .expect("Could not find entry in twoFACodeStore db!");

        assert_eq!(body.login_attempt_id, code.id.as_ref());
    }
}

#[api_test]
async fn malformed_input_should_return_422() {
    let test_case = [
        serde_json::json!({
            "email":"test@test.com"
        }),
        serde_json::json!({
            "password":"test@test.com"
        }),
        // test against irregular values - we must validate input sanitization
        serde_json::json!({
            "email":true,
            "password":false
        }),
        serde_json::json!({
            "email":true,
            "password":"test@test.com"
        }),
        serde_json::json!({
            "email":"test@test.com",
            "password":true
        }),
        serde_json::json!({
            "email":"test@test.com",
            "password":456
        }),
        serde_json::json!({
            "email":123,
            "password":"test@test.com"
        }),
    ];

    for test in test_case {
        let response = &app.post_login(&test).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

#[api_test]
async fn invalid_input_should_return_400() {
    let test_case = [
        serde_json::json!({
            "email":"test.test.com",
            "password":"Password123!"
        }),
        serde_json::json!({
            "email":"test@test.com",
            "password":"password123"
        }),
        serde_json::json!({
            "email":"test@test.com",
            "password":"password"
        }),
    ];

    for test in test_case {
        let response = &app.post_login(&test).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

#[api_test]
async fn non_existing_user_should_return_401() {
    let email = "test@test.com";
    let password = "Password123!";

    let invalid_user = serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": true,
    });

    let response = app.post_login(&invalid_user).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[api_test]
async fn unauthorize_user_should_return_401() {
    let email = "test@test.com";
    let password = "Password123!";
    let wrong_password = "password123!";

    let user = serde_json::json!({
        "email":email,
        "password":password,
        "requires2FA":true,
    });

    // we don't care. It shouldn't be possible to collide with another existing user?
    let _ = app.post_signup(&user).await;

    let invalid_user = serde_json::json!({
        "email": email,
        "password": wrong_password,
        "requires2FA": true,
    });

    let response = app.post_login(&invalid_user).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
