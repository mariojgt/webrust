use axum::Router;

use crate::controllers::{
    home::index as home_index,
    users::index as users_index,
    contact::submit as contact_submit,
    auth::reset_password,
};
use crate::route;
use crate::framework::AppState;

/// Web routes - for your main HTML pages
pub fn web(state: AppState) -> Router {
    route::web()
        .get("/", home_index)
        .get("/users", users_index)
        .post("/contact", contact_submit)
        .post("/reset-password", reset_password)
        .build()
        .with_state(state)
}
