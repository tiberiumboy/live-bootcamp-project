use crate::helpers::TestApp;
use auth_service::{
    domain::email::Email,
    utils::{auth::generate_auth_cookie, constants::JWT_COOKIE_NAME},
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use reqwest::{cookie::CookieStore, StatusCode, Url};

// this function helps reduce multiple repeated code patterns.
async fn build_test_app<'a>(cookie: &Cookie<'a>) -> TestApp {
    let app = TestApp::new().await;
    let url = Url::parse("http://127.0.0.1").expect("Unable to parse the url for testing purposes");
    let _ = &app.cookie_jar.add_cookie_str(&cookie.to_string(), &url);
    app
}

fn get_fake_email() -> Email {
    Email::parse("test@test.com").expect("Unable to parse test email account!")
}

fn get_token() -> String {
    let email = get_fake_email();
    let cookie = generate_auth_cookie(&email).expect("Unable to generate dummy token to test!");
    cookie.value().to_owned()
}

#[tokio::test]
async fn valid_jwt_should_return_200() {
    let token = get_token();

    let cookie = Cookie::build((JWT_COOKIE_NAME, &token))
        .path("/")
        .same_site(SameSite::Lax)
        // .http_only(true)
        // .secure(true)
        .build();

    let app = build_test_app(&cookie).await;
    let result = &app.post_logout().await;
    assert_eq!(result.status(), StatusCode::OK);

    // verify that the banned token have a new entry in the banned list.
    let store = app.banned_store.clone();
    let result = store.read().await.check_token(&token).await;
    assert_eq!(result, true);
}

#[tokio::test]
async fn missing_jwt_should_return_400() {
    let app = TestApp::new().await;
    let result = app.post_logout().await;
    assert_eq!(result.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn logout_twice_should_return_400() {
    // log in and verify that first.
    // then log out again.
    let token = get_token();
    let cookie = Cookie::build((JWT_COOKIE_NAME, token))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .secure(true)
        .build();
    let app = build_test_app(&cookie).await;
    let result = app.post_logout().await;
    assert_eq!(result.status(), StatusCode::OK);

    let result = app.post_logout().await;
    assert_eq!(result.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn invalid_jwt_should_return_401() {
    let cookie = Cookie::build((JWT_COOKIE_NAME, "invalid"))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .secure(true)
        .build();
    let app = build_test_app(&cookie).await;
    let result = app.post_logout().await;
    assert_eq!(result.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn banned_token_should_return_401() {
    let token = get_token();
    let cookie = Cookie::build((JWT_COOKIE_NAME, &token))
        .path("/")
        .same_site(SameSite::Lax)
        .secure(true)
        .http_only(true)
        .build();
    let app = build_test_app(&cookie).await;

    let store = app.banned_store.clone();
    let mut ban_list = store.write().await;
    let _ = ban_list.add_token(&token).await;

    let result = app.post_logout().await;
    assert_eq!(result.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn ensure_cookie_is_clear_after_success_logout() {
    let email = get_fake_email();
    let token = generate_auth_cookie(&email).expect("Unable to generate dummy token to test");
    let cookie = Cookie::build((JWT_COOKIE_NAME, token.value()))
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .secure(true)
        .build();
    let app = build_test_app(&cookie).await;
    let result = app.post_logout().await;
    assert_eq!(result.status(), StatusCode::OK);

    // one we get a successful logout prompt - we need to check the cookiejar and ensure that jwt is cleared
    let url = Url::parse("http://127.0.0.1").expect("Unable to parse app's address!");
    let result = &app.cookie_jar.cookies(&url);
    dbg!(&result);
    assert_eq!(result.is_none(), true);
}
