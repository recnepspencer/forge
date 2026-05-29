use crate::topology_operators::TopologyEditFamily;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MilestoneThreeOperatorFamilyClosureRow {
    pub(crate) family: TopologyEditFamily,
    pub(crate) admitted_lane_labels: Vec<String>,
    pub(crate) legal_evidence_labels: Vec<String>,
    pub(crate) hostile_evidence_labels: Vec<String>,
    pub(crate) replay_evidence_labels: Vec<String>,
    pub(crate) rejection_evidence_labels: Vec<String>,
    pub(crate) direct_hostile_scenario_labels: Vec<String>,
    pub(crate) legal_execution_count: usize,
    pub(crate) hostile_workload_count: usize,
    pub(crate) replay_evidence_count: usize,
    pub(crate) rejection_evidence_count: usize,
    pub(crate) localized_rejection_evidence_count: usize,
    pub(crate) branch_local_evidence_count: usize,
    pub(crate) primitive_family_evidence_count: usize,
    pub(crate) scale_pressure_evidence_count: usize,
    pub(crate) derived_breadth_evidence_count: usize,
    pub(crate) row_digest: String,
}




