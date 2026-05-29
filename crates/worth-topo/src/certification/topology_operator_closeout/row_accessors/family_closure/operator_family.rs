use super::super::super::operator_family_proof::MilestoneThreeOperatorFamilyClosureRow;
use crate::topology_operators::TopologyEditFamily;

impl MilestoneThreeOperatorFamilyClosureRow {
    pub fn family(&self) -> TopologyEditFamily {
        self.family
    }

    pub fn admitted_lane_labels(&self) -> &[String] {
        self.admitted_lane_labels.as_slice()
    }

    pub fn legal_evidence_labels(&self) -> &[String] {
        self.legal_evidence_labels.as_slice()
    }

    pub fn hostile_evidence_labels(&self) -> &[String] {
        self.hostile_evidence_labels.as_slice()
    }

    pub fn replay_evidence_labels(&self) -> &[String] {
        self.replay_evidence_labels.as_slice()
    }

    pub fn rejection_evidence_labels(&self) -> &[String] {
        self.rejection_evidence_labels.as_slice()
    }

    pub fn direct_hostile_scenario_labels(&self) -> &[String] {
        self.direct_hostile_scenario_labels.as_slice()
    }

    pub fn legal_execution_count(&self) -> usize {
        self.legal_execution_count
    }

    pub fn hostile_workload_count(&self) -> usize {
        self.hostile_workload_count
    }

    pub fn replay_evidence_count(&self) -> usize {
        self.replay_evidence_count
    }

    pub fn rejection_evidence_count(&self) -> usize {
        self.rejection_evidence_count
    }

    pub fn localized_rejection_evidence_count(&self) -> usize {
        self.localized_rejection_evidence_count
    }

    pub fn branch_local_evidence_count(&self) -> usize {
        self.branch_local_evidence_count
    }

    pub fn primitive_family_evidence_count(&self) -> usize {
        self.primitive_family_evidence_count
    }

    pub fn scale_pressure_evidence_count(&self) -> usize {
        self.scale_pressure_evidence_count
    }

    pub fn derived_breadth_evidence_count(&self) -> usize {
        self.derived_breadth_evidence_count
    }

    pub fn row_digest(&self) -> &str {
        self.row_digest.as_str()
    }
}
