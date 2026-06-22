use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceStage, WorkloadEvidenceStageLinkSet,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapOperatorReceipt {
    pub(crate) operator_digest: String,
    pub(crate) consumed_evidence_identities: Vec<String>,
    pub(crate) consumed_stage_links: WorkloadEvidenceStageLinkSet,
    pub(crate) overlap_extraction_identities: Vec<String>,
    pub(crate) operator_input_count: usize,
    pub(crate) operator_receipt_count: usize,
    pub(crate) overlap_extraction_receipt_count: usize,
    pub(crate) overlap_candidate_pair_breadth: usize,
    pub(crate) overlap_segment_contacts_certified: usize,
    pub(crate) overlap_shared_intervals: usize,
    pub(crate) overlap_islands: usize,
    pub(crate) overlap_containment_relations: usize,
    pub(crate) overlap_policy_required_exits: usize,
    pub(crate) overlap_ambiguous_contacts: usize,
}

impl CoplanarOverlapOperatorReceipt {
    pub fn operator_digest(&self) -> &str {
        &self.operator_digest
    }

    pub fn consumed_evidence_identities(&self) -> &[String] {
        &self.consumed_evidence_identities
    }

    pub fn consumed_stage_links(&self) -> &WorkloadEvidenceStageLinkSet {
        &self.consumed_stage_links
    }

    pub fn overlap_extraction_identities(&self) -> &[String] {
        &self.overlap_extraction_identities
    }

    pub fn operator_input_count(&self) -> usize {
        self.operator_input_count
    }

    pub fn operator_receipt_count(&self) -> usize {
        self.operator_receipt_count
    }

    pub fn overlap_extraction_receipt_count(&self) -> usize {
        self.overlap_extraction_receipt_count
    }

    pub fn overlap_candidate_pair_breadth(&self) -> usize {
        self.overlap_candidate_pair_breadth
    }

    pub fn overlap_segment_contacts_certified(&self) -> usize {
        self.overlap_segment_contacts_certified
    }

    pub fn overlap_shared_intervals(&self) -> usize {
        self.overlap_shared_intervals
    }

    pub fn overlap_islands(&self) -> usize {
        self.overlap_islands
    }

    pub fn overlap_containment_relations(&self) -> usize {
        self.overlap_containment_relations
    }

    pub fn overlap_policy_required_exits(&self) -> usize {
        self.overlap_policy_required_exits
    }

    pub fn overlap_ambiguous_contacts(&self) -> usize {
        self.overlap_ambiguous_contacts
    }

    pub fn links_to_stage(&self, stage: WorkloadEvidenceStage) -> bool {
        self.consumed_stage_links.links_to(stage)
    }
}
