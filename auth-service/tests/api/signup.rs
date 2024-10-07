use crate::helpers::TestApp;
use reqwest::StatusCode;

#[tokio::test]
pub async fn should_return_200() {
    let app = TestApp::new().await;

    let test_case = serde_json::json!(
        {
            "email": TestApp::get_random_email(),
            "password":"password123!",
            "requires2FA":true
        }
    );

    let result = app.post_signup(&test_case).await;
    assert_eq!(result.status(), StatusCode::CREATED);
    // then how do we go about deleting that sign in user?
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = TestApp::get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "password123",
            "requires2FA": true
        }),
        serde_json::json!({
            "email": &random_email,
            "password": "password123",
        }),
        serde_json::json!({
            "email": &random_email,
            "requires2FA": true
        }),
    ];

    for test in test_cases {
        let response = app.post_signup(&test).await;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
