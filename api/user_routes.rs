// api/user_routes.rs

use axum::{routing::{get, post, delete}, Json, Router};
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::repository_interface::User;
use crate::services::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<Mutex<UserService<'static>>>,
}

pub fn user_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/users", get(get_all_users).post(create_user))
        .route("/api/users/:id", get(get_user_by_id).delete(delete_user))
        .with_state(state)
}

async fn get_all_users(state: axum::extract::State<AppState>) -> Json<Vec<User>> {
    let service = state.service.lock().unwrap();
    Json(service.get_all_users().into_iter().cloned().collect())
}

async fn create_user(state: axum::extract::State<AppState>, Json(payload): Json<User>) {
    let mut service = state.service.lock().unwrap();
    service.create_user(payload);
}

async fn get_user_by_id(Path(id): Path<String>, state: axum::extract::State<AppState>) -> Option<Json<User>> {
    let service = state.service.lock().unwrap();
    service.get_user_by_id(&id).cloned().map(Json)
}

async fn delete_user(Path(id): Path<String>, state: axum::extract::State<AppState>) {
    let mut service = state.service.lock().unwrap();
    service.delete_user(&id);
}

