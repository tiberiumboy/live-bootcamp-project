use crate::helpers::TestApp;
use reqwest::StatusCode;

#[tokio::test]
async fn verify_2fa_should_pass() {
    // we need to provide a invalid data input somehow?
    let app = TestApp::new().await;
    // let email = "test@test.com".to_owned();
    // let code = "1234".to_owned();
    // let token = VerifyToken::new(email, code);

    let context = serde_json::json!({
        "email":"test@test.com",
        "loginAttemptId":"",
        "2FACode": "0000"
    });

    let response = app.post_verify_2fa(&context).await;
    assert_eq!(response.status(), StatusCode::OK);
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

    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({
                "email": "user@example.com",
                "2FACode": "string"
        }),
        serde_json::json!({
                "loginAttemptId": "string",
                "2FACode": "string"
        }),
        serde_json::json!({
                "email": "user@example.com",
                "loginAttemptId": "string"
        }),
        serde_json::json!({
                "email": "user@example.com"
        }),
        serde_json::json!({
                "loginAttemptId": "string"

        }),
        serde_json::json!({
                "2FACode": "string"
        }),
    ];

    for test in test_cases {
        let response = &app.post_verify_2fa(&test).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
