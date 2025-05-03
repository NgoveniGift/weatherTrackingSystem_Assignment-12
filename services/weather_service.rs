// services/weather_service.rs

use crate::repository_interface::{WeatherReport, WeatherReportRepository};

pub struct WeatherReportService<'a> {
    pub repository: &'a mut dyn WeatherReportRepository,
}

impl<'a> WeatherReportService<'a> {
    pub fn new(repository: &'a mut dyn WeatherReportRepository) -> Self {
        Self { repository }
    }

    pub fn create_report(&mut self, report: WeatherReport) {
        self.repository.save(report.report_id.clone(), report);
    }

    pub fn get_report_by_id(&self, report_id: &str) -> Option<&WeatherReport> {
        self.repository.find_by_id(&report_id.to_string())
    }

    pub fn get_all_reports(&self) -> Vec<&WeatherReport> {
        self.repository.find_all()
    }

    pub fn delete_report(&mut self, report_id: &str) {
        self.repository.delete(&report_id.to_string());
    }
}

