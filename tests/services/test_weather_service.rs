// tests/services/test_weather_service.rs

use crate::services::weather_service::WeatherReportService;
use crate::repository_interface::{WeatherReport, Repository, WeatherReportRepository};
use std::collections::HashMap;

struct MockWeatherReportRepository {
    store: HashMap<String, WeatherReport>,
}

impl MockWeatherReportRepository {
    fn new() -> Self {
        Self { store: HashMap::new() }
    }
}

impl Repository<String, WeatherReport> for MockWeatherReportRepository {
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

impl WeatherReportRepository for MockWeatherReportRepository {}

#[test]
fn test_weather_service_crud() {
    let mut mock_repo = MockWeatherReportRepository::new();
    let mut service = WeatherReportService::new(&mut mock_repo);

    let report = WeatherReport {
        report_id: "r1".into(),
        temperature: 25.0,
        humidity: 50.0,
        wind_speed: 12.0,
        timestamp: "2025-05-01T12:00:00Z".into(),
    };

    service.create_report(report.clone());
    assert_eq!(service.get_report_by_id("r1"), Some(&report));

    let all_reports = service.get_all_reports();
    assert_eq!(all_reports.len(), 1);

    service.delete_report("r1");
    assert!(service.get_report_by_id("r1").is_none());
}

