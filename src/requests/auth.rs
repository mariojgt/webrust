use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct ResetPasswordWithTokenRequest {
    #[validate(email(message = "Please provide a valid email address"))]
    pub email: String,

    #[validate(length(min = 1, message = "Reset token is required"))]
    pub reset_token: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub password_confirmation: String,
}

#[derive(Deserialize, Validate, Debug)]
pub struct LoginRequest {
    #[validate(email(message = "Please provide a valid email address"))]
    pub email: String,

    #[validate(length(min = 1, message = "Password is required"))]
    pub password: String,
}
