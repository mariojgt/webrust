use crate::prelude::*;
use crate::requests::auth::ResetPasswordWithTokenRequest;
use axum::response::IntoResponse;

pub async fn reset_password(
    FormRequest(req): FormRequest<ResetPasswordWithTokenRequest>
) -> impl IntoResponse {
    // If we get here, validation has passed!
    // 'req' is the ResetPasswordWithTokenRequest struct.

    dd!(req); // Dump the validated request data to prove it works

    Html("Password reset logic would go here".to_string())
}
