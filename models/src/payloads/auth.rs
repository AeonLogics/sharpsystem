use serde::{Deserialize, Serialize};

const RESERVED_HANDLES: &[&str] = &[
    "admin",
    "api",
    "login",
    "register",
    "dashboard",
    "static",
    "pkg",
    "internal",
    "system",
    "support",
    "root",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

impl LoginPayload {
    pub fn validate(&self) -> Result<(), crate::auth::AuthError> {
        SignupPayload::validate_email(&self.email)?;
        if self.password.is_empty() {
            return Err(crate::auth::AuthError::InvalidInput(
                "Password is required.".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupPayload {
    pub user_name: String,
    pub system_name: String,
    pub workspace_handle: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

impl SignupPayload {
    pub fn validate(&self) -> Result<(), crate::auth::AuthError> {
        Self::validate_user_name(&self.user_name)?;
        Self::validate_system_name(&self.system_name)?;
        Self::validate_workspace_handle(&self.workspace_handle)?;
        Self::validate_email(&self.email)?;
        Self::validate_password(&self.password)?;
        Self::validate_confirm_password(&self.password, &self.confirm_password)?;
        Ok(())
    }

    pub fn validate_user_name(name: &str) -> Result<(), crate::auth::AuthError> {
        if name.trim().is_empty() {
            return Err(crate::auth::AuthError::InvalidInput(
                "User name is required.".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_system_name(name: &str) -> Result<(), crate::auth::AuthError> {
        if name.trim().is_empty() {
            return Err(crate::auth::AuthError::InvalidInput(
                "System name is required.".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_workspace_handle(handle: &str) -> Result<(), crate::auth::AuthError> {
        let handle = handle.trim();
        if handle.len() < 3 {
            return Err(crate::auth::AuthError::InvalidInput(
                "Workspace handle must be at least 3 characters.".to_string(),
            ));
        }

        if RESERVED_HANDLES.contains(&handle) {
            return Err(crate::auth::AuthError::InvalidInput(format!(
                "Handle '{}' is reserved.",
                handle
            )));
        }

        if !handle
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(crate::auth::AuthError::InvalidInput(
                "Workspace handle can only contain lowercase letters, numbers, and hyphens."
                    .to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_email(email: &str) -> Result<(), crate::auth::AuthError> {
        if !email.contains('@') {
            return Err(crate::auth::AuthError::InvalidInput(
                "Please provide a valid email address.".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_password(password: &str) -> Result<(), crate::auth::AuthError> {
        if password.len() < 8 {
            return Err(crate::auth::AuthError::InvalidInput(
                "Password must be at least 8 characters long.".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_confirm_password(
        password: &str,
        confirm_password: &str,
    ) -> Result<(), crate::auth::AuthError> {
        if password != confirm_password {
            return Err(crate::auth::AuthError::InvalidInput(
                "Passwords do not match.".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_workspace_handle() {
        assert!(SignupPayload::validate_workspace_handle("valid-handle-123").is_ok());
        assert!(SignupPayload::validate_workspace_handle("sh").is_err()); // Too short
        assert!(SignupPayload::validate_workspace_handle("Invalid_Handle").is_err()); // Uppercase and underscore
        assert!(SignupPayload::validate_workspace_handle("handle!").is_err()); // Special char
        assert!(SignupPayload::validate_workspace_handle("admin").is_err()); // Reserved
        assert!(SignupPayload::validate_workspace_handle("api").is_err()); // Reserved
    }
}
