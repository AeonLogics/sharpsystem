use leptos::prelude::*;

#[server(VerifyUser, "ssr")]
pub async fn verify_user() -> Result<bool, ServerFnError> {
    Ok(true)
}
