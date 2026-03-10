#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use models::errors::SystemError;

#[cfg(feature = "ssr")]
pub fn set_session_token(token: &str) -> Result<(), SystemError> {
    use axum::http::HeaderValue;
    use axum_extra::extract::cookie::{Cookie, SameSite};
    use leptos_axum::ResponseOptions;

    let response_options = use_context::<ResponseOptions>()
        .ok_or_else(|| SystemError::general("ResponseOptions context not found"))?;

    let cookie = Cookie::build(("session_token", token.to_string()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(false) // Keep false for localhost dev; production should be true
        .max_age(time::Duration::days(7))
        .build();

    let header_value = HeaderValue::from_str(&cookie.to_string())
        .map_err(|e| SystemError::general(e.to_string()))?;

    response_options.insert_header(axum::http::header::SET_COOKIE, header_value);
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn get_session_token() -> Option<String> {
    use axum_extra::extract::cookie::CookieJar;
    use leptos_axum::extract;
    let jar: CookieJar = extract().await.ok()?;
    jar.get("session_token").map(|c| c.value().to_string())
}

#[cfg(feature = "ssr")]
pub fn remove_session_token() -> Result<(), SystemError> {
    use axum::http::HeaderValue;
    use axum_extra::extract::cookie::Cookie;
    use leptos_axum::ResponseOptions;

    let response_options = use_context::<ResponseOptions>()
        .ok_or_else(|| SystemError::general("ResponseOptions context not found"))?;

    let mut cookie = Cookie::build("session_token").path("/").build();

    // This tells the browser to delete the cookie immediately
    cookie.make_removal();

    let header_value = HeaderValue::from_str(&cookie.to_string())
        .map_err(|e| SystemError::general(e.to_string()))?;

    response_options.insert_header(axum::http::header::SET_COOKIE, header_value);
    Ok(())
}
