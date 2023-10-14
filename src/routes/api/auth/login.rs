use crate::errors::AppError;
use crate::ServerState;
use axum::extract::{Host, Query};
use axum::response::Redirect;
use axum::Extension;
use base64::engine::general_purpose;
use base64::Engine;
use hyper::Uri;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Arc;

use crate::routes::api::auth::determine_callback_url;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequestInfo {
    pub redirect_url: String,
}

pub async fn handle_login_request(
    Extension(state): Extension<Arc<ServerState>>,
    Host(host): Host,
    Query(redirect_url): Query<LoginRequestInfo>,
) -> Result<Redirect, AppError> {
    let Ok(uri) = Uri::from_str(&redirect_url.redirect_url) else {
        return AppError::invalid_redirect_url(redirect_url.redirect_url).into();
    };
    if !state.config.permitted_redirect_urls.contains(&uri) {
        return AppError::invalid_redirect_url(redirect_url.redirect_url).into();
    }

    let redirection_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth\
    ?client_id={}\
    &redirect_uri={}\
    &response_type={}\
    &state={}\
    &scope=openid+email+profile",
        &state.config.google_client_id,
        determine_callback_url(host),
        "code",
        general_purpose::STANDARD_NO_PAD.encode(serde_json::to_string(&LoginRequestInfo { redirect_url: uri.to_string() }).unwrap())
    );

    Ok(Redirect::to(&*redirection_url))
}
