use serde::Serialize;

use super::admission::Milestone12AdmissionReport;
use super::counter_names::{
    MILESTONE_12_ADMISSION_REPORT_COUNTER_FIELD_NAMES, MILESTONE_12_COUNTER_NAMES,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CounterContract {
    pub counter_names: Vec<&'static str>,
}

impl Milestone12CounterContract {
    pub fn phase_1() -> Self {
        Self {
            counter_names: MILESTONE_12_COUNTER_NAMES.to_vec(),
        }
    }

    pub fn validate_report(
        &self,
        _report: &Milestone12AdmissionReport,
    ) -> Result<(), Milestone12CounterContractViolation> {
        for counter in MILESTONE_12_ADMISSION_REPORT_COUNTER_FIELD_NAMES {
            if !self.counter_names.contains(counter) {
                return Err(Milestone12CounterContractViolation::MissingReportCounter);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Milestone12CounterContractViolation {
    MissingReportCounter,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12ComplexityPathStatus {
    pub verified: bool,
    pub detail: String,
}

impl Milestone12ComplexityPathStatus {
    pub fn verified(detail: impl Into<String>) -> Self {
        Self {
            verified: true,
            detail: detail.into(),
        }
    }

    pub fn debt(detail: impl Into<String>) -> Self {
        Self {
            verified: false,
            detail: detail.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12ComplexitySurface {
    pub relation_recheck: Milestone12ComplexityPathStatus,
    pub index_lookup: Milestone12ComplexityPathStatus,
    pub adapter_cost: Milestone12ComplexityPathStatus,
    pub restore_scan: Milestone12ComplexityPathStatus,
}
