use crate::helpers::TestApp;
use reqwest::StatusCode;

#[tokio::test]
pub async fn should_return_206() {
    let app = TestApp::new().await;

    let test = serde_json::json!({
        "email":"test@test.com",
        "password":"password123!"
    });

    let result = app.post_login(&test).await;
    assert_eq!(result.status(), StatusCode::PARTIAL_CONTENT);
}

#[tokio::test]
async fn malformed_input_should_return_422() {
    let app = TestApp::new().await;

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
