// api/weather_routes.rs

use axum::{routing::{get, post, delete}, Json, Router};
use axum::extract::Path;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use crate::repository_interface::WeatherReport;
use crate::services::weather_service::WeatherReportService;

#[derive(Clone)]
pub struct WeatherAppState {
    pub service: Arc<Mutex<WeatherReportService<'static>>>,
}

pub fn weather_routes(state: WeatherAppState) -> Router {
    Router::new()
        .route("/api/weather", get(get_all_reports).post(create_report))
        .route("/api/weather/:id", get(get_report_by_id).delete(delete_report))
        .with_state(state)
}

async fn get_all_reports(state: axum::extract::State<WeatherAppState>) -> Json<Vec<WeatherReport>> {
    let service = state.service.lock().unwrap();
    Json(service.get_all_reports().into_iter().cloned().collect())
}

async fn create_report(state: axum::extract::State<WeatherAppState>, Json(payload): Json<WeatherReport>) {
    let mut service = state.service.lock().unwrap();
    service.create_report(payload);
}

async fn get_report_by_id(Path(id): Path<String>, state: axum::extract::State<WeatherAppState>) -> Option<Json<WeatherReport>> {
    let service = state.service.lock().unwrap();
    service.get_report_by_id(&id).cloned().map(Json)
}

async fn delete_report(Path(id): Path<String>, state: axum::extract::State<WeatherAppState>) {
    let mut service = state.service.lock().unwrap();
    service.delete_report(&id);
}

