use crate::helpers::TestApp;
use auth_service::domain::{
    email::Email, login_attempt_id::LoginAttemptId, two_fa_code::TwoFACode,
};
use reqwest::StatusCode;

/*
    400: Invalid Input
    401: Authentication failed
    422: Unprocessable content
    500: Unexpected error (Should never happen)
*/

#[tokio::test]
async fn verify_2fa_should_pass() {
    // we need to provide a invalid data input somehow?
    let mut app = TestApp::new().await;
    let email = Email::parse(&TestApp::get_random_email())
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
        "email": email.as_ref(),
        "loginAttemptId": id.as_ref(),
        "2FACode": code.as_ref()
    });

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::OK);
    app.clean_up().await;
}

#[tokio::test]
async fn malform_field_input_should_return_400() {
    /* JSON Requirement, all field must be present, otherwise throw 422: Unprocessable entity
       {
           email: "<String>",
           loginAttemptId: "<String>",
           2FACode: "<String>"
       }
    */

    let mut app = TestApp::new().await;
    let email = Email::parse(&TestApp::get_random_email()).unwrap();
    let code = TwoFACode::default();
    let id = LoginAttemptId::default();

    let test_cases = [
        serde_json::json!({
            "email": &email.as_ref(),
            "loginAttemptId": "abc this should not be possible?",
            "2FACode": &code.as_ref(),
        }),
        serde_json::json!({
            "email": "",
            "loginAttemptId": &id.as_ref(),
            "2FACode": &code.as_ref(),
        }),
        serde_json::json!({
            "email": &email.as_ref(),
            "loginAttemptId": &id.as_ref(),
            "2FACode": "123A",
        }),
        serde_json::json!({
            "email": &email.as_ref(),
            "loginAttemptId": "c7585553-84d5-4fbc-bab5-62bad43e42@",
            "2FACode": &code.as_ref(),
        }),
        serde_json::json!({
            "email": "test_test_com",
            "loginAttemptId": &id.as_ref(),
            "2FACode": &code.as_ref(),
        }),
        serde_json::json!({
            "email": &email.as_ref(),
            "loginAttemptId": &id.as_ref(),
            "2FACode": "",
        }),
    ];

    for test in test_cases {
        let response = &app.post_verify_2fa(&test).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
    app.clean_up().await;
}

#[tokio::test]
async fn malformed_input_should_return_422() {
    /* JSON Requirement, all field must be present, otherwise throw 422: Unprocessable entity
       {
           email: "<String>",
           loginAttemptId: "<String>",
           2FACode: "<String>"
       }
    */

    let mut app = TestApp::new().await;
    let email = Email::parse("test@test.com").unwrap();
    let code = TwoFACode::default();
    let id = LoginAttemptId::default();

    let test_cases = [
        serde_json::json!({
                "email": &email.as_ref(),
                "2FACode": &code.as_ref(),
        }),
        serde_json::json!({
                "loginAttemptId": &id.as_ref(),
                "2FACode": &code.as_ref(),
        }),
        serde_json::json!({
                "email": &email.as_ref(),
                "loginAttemptId": &id.as_ref(),
        }),
        serde_json::json!({
                "email": &email.as_ref(),
        }),
        serde_json::json!({
                "loginAttemptId": &id.as_ref(),
        }),
        serde_json::json!({
                "2FACode": &code.as_ref(),
        }),
    ];

    for test in test_cases {
        let response = &app.post_verify_2fa(&test).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
    app.clean_up().await;
}

#[tokio::test]
async fn non_existing_email_should_return_401() {
    let mut app = TestApp::new().await;
    let email = Email::parse(&TestApp::get_random_email())
        .expect("Unable to parse dummy email for unit test!");
    let fake_user = Email::parse(&TestApp::get_random_email())
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
        "email": fake_user.as_ref(),
        "loginAttemptId": id.as_ref(),
        "2FACode": code.as_ref()
    });

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    app.clean_up().await;
}

#[tokio::test]
async fn invalid_id_should_return_401() {
    let mut app = TestApp::new().await;
    let email = Email::parse(&TestApp::get_random_email())
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
        "email": email.as_ref(),
        "loginAttemptId": fake_id.as_ref(),
        "2FACode": code.as_ref()
    });

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    app.clean_up().await;
}

#[tokio::test]
async fn invalid_code_should_return_401() {
    let mut app = TestApp::new().await;
    let email = Email::parse(&TestApp::get_random_email())
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
        "email": email.as_ref(),
        "loginAttemptId": id.as_ref(),
        "2FACode": fake_code.as_ref()
    });

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    app.clean_up().await;
}

#[tokio::test]
async fn using_same_2fa_code_twice_should_return_401() {
    // we need to provide a invalid data input somehow?
    let mut app = TestApp::new().await;
    let email = Email::parse(&TestApp::get_random_email())
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
        "email": email.as_ref(),
        "loginAttemptId": id.as_ref(),
        "2FACode": code.as_ref()
    });

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::OK);

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    app.clean_up().await;
}
