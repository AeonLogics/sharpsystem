use leptos::prelude::*;
use models::SignupPayload;
use tracing::instrument;

#[instrument(ret, err)]
#[server(VerifyUser)]
pub async fn verify_user() -> Result<bool, ServerFnError> {
    Ok(true)
}

#[instrument(ret, err)]
#[server(Signup)]
pub async fn signup(payload: SignupPayload) -> Result<bool, ServerFnError> {
    // 1. Validate the payload
    payload
        .validate()
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // 2. Hash password (TODO)
    // 3. Create company in database (TODO: use payload.company_name)
    // 4. Create user linked to company (TODO: use payload.username, payload.email)

    tracing::info!(
        "Registration initialized for user {} at company {}",
        payload.username,
        payload.company_name
    );

    Ok(true)
}
