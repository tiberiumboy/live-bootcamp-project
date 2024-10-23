use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::app_state::AppState;
use crate::{
    domain::error::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

pub async fn logout(
    // TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // if the cookie is missing return 400
    let jar_clone = jar.clone();
    let content = jar_clone.get(JWT_COOKIE_NAME);
    let cookie = match &content {
        Some(cookie) => cookie,
        None => return (jar, Err(AuthAPIError::MissingToken)),
    };

    let token = cookie.value();
    let jar = jar.remove(Cookie::from(JWT_COOKIE_NAME));
    let mut ban_list = state.banned_token_store.write().await;
    let _ = ban_list.add_token(token).await;

    // if the cookie contains invalid JWT return 401
    // else if succeed - return 200
    match validate_token(token).await {
        Ok(_) => (jar, Ok(StatusCode::OK.into_response())),
        Err(_) => (jar, Err(AuthAPIError::InvalidToken)),
    }
}
