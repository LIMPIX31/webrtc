use std::collections::HashMap;

use util::sync::Mutex;

use super::StatsReportType;

#[derive(Debug, Default)]
pub struct StatsCollector {
    pub reports: Mutex<HashMap<String, StatsReportType>>,
}

impl StatsCollector {
    pub fn new() -> Self {
        StatsCollector {
            ..Default::default()
        }
    }

    pub fn insert(&self, id: String, stats: StatsReportType) {
        let mut reports = self.reports.lock();
        reports.insert(id, stats);
    }

    pub fn merge(&self, stats: HashMap<String, StatsReportType>) {
        let mut reports = self.reports.lock();
        reports.extend(stats)
    }

    pub fn into_reports(self) -> HashMap<String, StatsReportType> {
        self.reports.into_inner()
    }
}
