use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupPayload {
    pub username: String,
    pub company_name: String,
    pub email: String,
    pub password: String,
}

impl SignupPayload {
    pub fn validate(&self) -> Result<(), crate::auth::AuthError> {
        Self::validate_username(&self.username)?;
        Self::validate_company_name(&self.company_name)?;
        Self::validate_email(&self.email)?;
        Self::validate_password(&self.password)?;
        Ok(())
    }

    pub fn validate_username(username: &str) -> Result<(), crate::auth::AuthError> {
        if username.trim().len() < 3 {
            return Err(crate::auth::AuthError::InvalidInput(
                "Username must be at least 3 characters.".to_string(),
            ));
        }
        Ok(())
    }

    pub fn validate_company_name(name: &str) -> Result<(), crate::auth::AuthError> {
        if name.trim().is_empty() {
            return Err(crate::auth::AuthError::InvalidInput(
                "Company name is required.".to_string(),
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
}
