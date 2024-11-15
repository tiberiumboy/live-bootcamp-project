use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode};
use axum_extra::extract::{cookie::Cookie, CookieJar};

use crate::app_state::AppState;
use crate::{
    domain::error::AuthAPIError,
    utils::{auth::validate_token, constants::JWT_COOKIE_NAME},
};

#[tracing::instrument(name = "Logout route", skip_all)]
pub async fn logout(
    // TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    State(state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    // if the cookie is missing return 400

    let jar_clone = jar.clone();
    let cookie = match &jar.get(JWT_COOKIE_NAME) {
        Some(cookie) => cookie.value(),
        None => return (jar_clone, Err(AuthAPIError::MissingToken)),
    };

    // remove JWT cookie and add to ban list
    let jar_clone = jar_clone.remove(Cookie::from(JWT_COOKIE_NAME));
    let mut ban_list = state.banned_token_store.write().await;
    let _ = ban_list.add_token(cookie).await;

    // if the cookie contains invalid JWT return 401
    // else if succeed - return 200
    match validate_token(cookie).await {
        Ok(_) => (jar_clone, Ok(StatusCode::OK.into_response())),
        Err(_) => (jar_clone, Err(AuthAPIError::InvalidToken)),
    }
}
