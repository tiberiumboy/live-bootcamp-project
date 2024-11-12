use crate::helpers::TestApp;
use auth_service::domain::{
    email::Email, login_attempt_id::LoginAttemptId, two_fa_code::TwoFACode,
};
use reqwest::StatusCode;
use secrecy::{ExposeSecret, Secret};
use test_helpers::api_test;

/*
    400: Invalid Input
    401: Authentication failed
    422: Unprocessable content
    500: Unexpected error (Should never happen)
*/

#[api_test]
async fn verify_2fa_should_pass() {
    // we need to provide a invalid data input somehow?
    let email = Email::parse(TestApp::get_random_email())
        .expect("Unable to parse dummy email for unit test!");
    let id = LoginAttemptId::default();
    let code = TwoFACode::default();

    {
        let mut store = app.two_fa_code_store.write().await;
        let _ = store
            .add_code(email.clone(), id.clone(), code.clone())
            .await;
    }

    let context = serde_json::json!({
        "email": email.as_ref().expose_secret(),
        "loginAttemptId": id.as_ref(),
        "2FACode": code.as_ref().expose_secret()
    });

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::OK);
}

#[api_test]
async fn malform_field_input_should_return_400() {
    /* JSON Requirement, all field must be present, otherwise throw 422: Unprocessable entity
       {
           email: "<String>",
           loginAttemptId: "<String>",
           2FACode: "<String>"
       }
    */
    let email = Email::parse(TestApp::get_random_email()).unwrap();
    let code = TwoFACode::default();
    let id = LoginAttemptId::default();

    let test_cases = [
        serde_json::json!({
            "email": &email.as_ref().expose_secret(),
            "loginAttemptId": "abc this should not be possible?",
            "2FACode": &code.as_ref().expose_secret(),
        }),
        serde_json::json!({
            "email": "",
            "loginAttemptId": &id.as_ref(),
            "2FACode": &code.as_ref().expose_secret(),
        }),
        serde_json::json!({
            "email": &email.as_ref().expose_secret(),
            "loginAttemptId": &id.as_ref(),
            "2FACode": "123A",
        }),
        serde_json::json!({
            "email": &email.as_ref().expose_secret(),
            "loginAttemptId": "c7585553-84d5-4fbc-bab5-62bad43e42@",
            "2FACode": &code.as_ref().expose_secret(),
        }),
        serde_json::json!({
            "email": "test_test_com",
            "loginAttemptId": &id.as_ref(),
            "2FACode": &code.as_ref().expose_secret(),
        }),
        serde_json::json!({
            "email": &email.as_ref().expose_secret(),
            "loginAttemptId": &id.as_ref(),
            "2FACode": "",
        }),
    ];

    for test in test_cases {
        let response = &app.post_verify_2fa(&test).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}

#[api_test]
async fn malformed_input_should_return_422() {
    /* JSON Requirement, all field must be present, otherwise throw 422: Unprocessable entity
       {
           email: "<String>",
           loginAttemptId: "<String>",
           2FACode: "<String>"
       }
    */

    let input = "test@test.com".to_owned();
    let secret = Secret::new(input);
    let email = Email::parse(secret).unwrap();
    let code = TwoFACode::default();
    let id = LoginAttemptId::default();

    let test_cases = [
        serde_json::json!({
                "email": &email.as_ref().expose_secret(),
                "2FACode": &code.as_ref().expose_secret(),
        }),
        serde_json::json!({
                "loginAttemptId": &id.as_ref(),
                "2FACode": &code.as_ref().expose_secret(),
        }),
        serde_json::json!({
                "email": &email.as_ref().expose_secret(),
                "loginAttemptId": &id.as_ref(),
        }),
        serde_json::json!({
                "email": &email.as_ref().expose_secret(),
        }),
        serde_json::json!({
                "loginAttemptId": &id.as_ref(),
        }),
        serde_json::json!({
                "2FACode": &code.as_ref().expose_secret(),
        }),
    ];

    for test in test_cases {
        let response = &app.post_verify_2fa(&test).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}

#[api_test]
async fn non_existing_email_should_return_401() {
    let email = Email::parse(TestApp::get_random_email())
        .expect("Unable to parse dummy email for unit test!");
    let fake_user = Email::parse(TestApp::get_random_email())
        .expect("Unable to parse dummy email for unit test!");

    let id = LoginAttemptId::default();
    let code = TwoFACode::default();

    {
        let mut store = app.two_fa_code_store.write().await;
        let _ = store
            .add_code(email.clone(), id.clone(), code.clone())
            .await;
    }

    let context = serde_json::json!({
        "email": fake_user.as_ref().expose_secret(),
        "loginAttemptId": id.as_ref(),
        "2FACode": code.as_ref().expose_secret()
    });

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[api_test]
async fn invalid_id_should_return_401() {
    let email = Email::parse(TestApp::get_random_email())
        .expect("Unable to parse dummy email for unit test!");

    let id = LoginAttemptId::default();
    let fake_id = LoginAttemptId::default();
    let code = TwoFACode::default();

    {
        let mut store = app.two_fa_code_store.write().await;
        let _ = store
            .add_code(email.clone(), id.clone(), code.clone())
            .await;
    }

    let context = serde_json::json!({
        "email": email.as_ref().expose_secret(),
        "loginAttemptId": fake_id.as_ref(),
        "2FACode": code.as_ref().expose_secret()
    });

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[api_test]
async fn invalid_code_should_return_401() {
    let email = Email::parse(TestApp::get_random_email())
        .expect("Unable to parse dummy email for unit test!");

    let id = LoginAttemptId::default();
    let code = TwoFACode::default();
    let fake_code = TwoFACode::default();

    {
        let mut store = app.two_fa_code_store.write().await;
        let _ = store
            .add_code(email.clone(), id.clone(), code.clone())
            .await;
    }

    let context = serde_json::json!({
        "email": email.as_ref().expose_secret(),
        "loginAttemptId": id.as_ref(),
        "2FACode": fake_code.as_ref().expose_secret()
    });

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[api_test]
async fn using_same_2fa_code_twice_should_return_401() {
    // we need to provide a invalid data input somehow?
    let email = Email::parse(TestApp::get_random_email())
        .expect("Unable to parse dummy email for unit test!");
    let id = LoginAttemptId::default();
    let code = TwoFACode::default();

    {
        let mut store = app.two_fa_code_store.write().await;
        let _ = store
            .add_code(email.clone(), id.clone(), code.clone())
            .await;
    }

    let context = serde_json::json!({
        "email": email.as_ref().expose_secret(),
        "loginAttemptId": id.as_ref(),
        "2FACode": code.as_ref().expose_secret()
    });

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::OK);

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
