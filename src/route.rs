use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};

use crate::{handler::get_places_search_near_by, AppState};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/place/nearbysearch", get(get_places_search_near_by))
        .with_state(app_state)
}
