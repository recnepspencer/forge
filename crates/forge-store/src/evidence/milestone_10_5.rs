use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone10_5ComplexityPathStatus {
    verified: bool,
    detail: String,
}

impl Milestone10_5ComplexityPathStatus {
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

    pub fn is_verified(&self) -> bool {
        self.verified
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone10_5ComplexitySurface {
    pub declaration_lowering: Milestone10_5ComplexityPathStatus,
    pub batch_admission: Milestone10_5ComplexityPathStatus,
    pub maintenance_resume: Milestone10_5ComplexityPathStatus,
    pub durable_status_lookup: Milestone10_5ComplexityPathStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone10_5CounterContract {
    pub maintenance_declaration_count: u64,
    pub maintenance_admission_count: u64,
    pub maintenance_rejection_count: u64,
    pub maintenance_resume_count: u64,
    pub maintenance_checkpoint_count: u64,
    pub maintenance_completion_count: u64,
    pub maintenance_failure_count: u64,
    pub maintenance_debt_link_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone10_5MaintenanceReport {
    pub declared_batch_count: u64,
    pub persisted_declaration_count: u64,
    pub active_declaration_count: u64,
    pub completed_declaration_count: u64,
    pub failed_declaration_count: u64,
    pub checkpoint_count: u64,
}
