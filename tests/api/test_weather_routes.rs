// tests/api/test_weather_routes.rs

#[cfg(test)]
mod tests {
    use axum::{Router, body::Body, http::{Request, StatusCode}, routing::{get, post}};
    use axum::response::Response;
    use tower::ServiceExt;
    use std::sync::{Arc, Mutex};
    use crate::services::weather_service::WeatherReportService;
    use crate::repository_interface::{WeatherReport, WeatherReportRepository, Repository};
    use crate::api::weather_routes::{weather_routes, WeatherAppState};
    use std::collections::HashMap;

    struct MockRepo {
        pub store: HashMap<String, WeatherReport>,
    }

    impl MockRepo {
        fn new() -> Self {
            Self { store: HashMap::new() }
        }
    }

    impl Repository<String, WeatherReport> for MockRepo {
        fn save(&mut self, id: String, entity: WeatherReport) {
            self.store.insert(id, entity);
        }

        fn find_by_id(&self, id: &String) -> Option<&WeatherReport> {
            self.store.get(id)
        }

        fn find_all(&self) -> Vec<&WeatherReport> {
            self.store.values().collect()
        }

        fn delete(&mut self, id: &String) {
            self.store.remove(id);
        }
    }

    impl WeatherReportRepository for MockRepo {}

    #[tokio::test]
    async fn test_get_all_weather_reports() {
        let mut repo = MockRepo::new();
        repo.save("w1".into(), WeatherReport {
            report_id: "w1".into(),
            temperature: 21.5,
            humidity: 60.0,
            wind_speed: 9.5,
            timestamp: "2025-05-01T08:00:00Z".into(),
        });

        let service = WeatherReportService::new(&mut repo);
        let app = weather_routes(WeatherAppState {
            service: Arc::new(Mutex::new(service)),
        });

        let response = app
            .oneshot(Request::builder().uri("/api/weather").method("GET").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_weather_report() {
        let mut repo = MockRepo::new();
        let service = WeatherReportService::new(&mut repo);
        let app = weather_routes(WeatherAppState {
            service: Arc::new(Mutex::new(service)),
        });

        let json = r#"{
            \"report_id\": \"w2\",
            \"temperature\": 18.0,
            \"humidity\": 70.0,
            \"wind_speed\": 8.2,
            \"timestamp\": \"2025-05-01T09:00:00Z\"
        }"#;

        let response = app
            .oneshot(Request::builder()
                .uri("/api/weather")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(json))
                .unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}

