use serde::{Deserialize, Serialize};

use crate::{
    MaintenanceBatchClass, MaintenanceDebtSummary, MaintenanceLaneKey, MaintenanceLocalitySummary,
    MaintenanceQueueSummary, MaintenanceReservationSummary, MaintenanceResourceBudgetSummary,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceBatchRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub batch_class: MaintenanceBatchClass,
    pub declaration_ids: Vec<String>,
    pub declaration_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceCheckpointRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub declaration_id: String,
    pub completed_phase: String,
    pub checkpoint_order: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceQueueSummaryRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub summary: MaintenanceQueueSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceLocalitySummaryRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub summary: MaintenanceLocalitySummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceReservationSummaryRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub summary: MaintenanceReservationSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceResourceBudgetSummaryRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub summary: MaintenanceResourceBudgetSummary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintenanceDebtSummaryRecord {
    pub artifact_id: String,
    pub family_version: u32,
    pub lane_key: MaintenanceLaneKey,
    pub summary: MaintenanceDebtSummary,
}
