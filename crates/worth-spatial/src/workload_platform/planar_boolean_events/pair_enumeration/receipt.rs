use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

use super::counters::PlanarBooleanSegmentPairEnumerationCounters;
use super::product::{
    PlanarBooleanCandidateIndexFallbackPosture, PlanarBooleanCandidateIndexLifecycleOutcome,
    PlanarBooleanCandidateIndexStrategy, PlanarBooleanSegmentCandidateIndexProduct,
    PlanarBooleanSegmentCandidateRowReceipt,
};
use super::work_item::PlanarBooleanSegmentPairWorkItem;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSegmentPairEnumerationReceipt {
    segment_pair_enumeration_identity: String,
    candidate_index_product: PlanarBooleanSegmentCandidateIndexProduct,
    work_items: Vec<PlanarBooleanSegmentPairWorkItem>,
}

impl PlanarBooleanSegmentPairEnumerationReceipt {
    pub(crate) fn new(
        segment_pair_enumeration_identity: impl Into<String>,
        product: PlanarBooleanSegmentCandidateIndexProduct,
    ) -> Self {
        let work_items = product.work_items();
        Self {
            segment_pair_enumeration_identity: segment_pair_enumeration_identity.into(),
            candidate_index_product: product,
            work_items,
        }
    }

    pub fn segment_pair_enumeration_identity(&self) -> &str {
        &self.segment_pair_enumeration_identity
    }

    pub fn canonical_segment_set_identity(&self) -> &str {
        self.candidate_index_product
            .canonical_segment_set_identity()
    }

    pub fn query_index_identity(&self) -> &str {
        self.candidate_index_product.product_identity()
    }

    pub fn query_index_declaration_digest(&self) -> &str {
        self.candidate_index_product.declaration_digest()
    }

    pub fn query_index_plan_digest(&self) -> &str {
        self.candidate_index_product.plan_digest()
    }

    pub fn query_index_envelope_digest(&self) -> &str {
        self.candidate_index_product.envelope_digest()
    }

    pub fn candidate_index_product_identity(&self) -> &str {
        self.candidate_index_product.product_identity()
    }

    pub fn candidate_index_product(&self) -> &PlanarBooleanSegmentCandidateIndexProduct {
        &self.candidate_index_product
    }

    pub fn candidate_index_strategy(&self) -> PlanarBooleanCandidateIndexStrategy {
        self.candidate_index_product.strategy()
    }

    pub fn fallback_posture(&self) -> PlanarBooleanCandidateIndexFallbackPosture {
        self.candidate_index_product.fallback_posture()
    }

    pub fn candidate_index_lifecycle_outcome(&self) -> PlanarBooleanCandidateIndexLifecycleOutcome {
        self.candidate_index_product.lifecycle_outcome()
    }

    pub fn counters(&self) -> PlanarBooleanSegmentPairEnumerationCounters {
        self.candidate_index_product.counters()
    }

    pub fn candidate_rows(&self) -> &[PlanarBooleanSegmentCandidateRowReceipt] {
        self.candidate_index_product.rows()
    }

    pub fn work_items(&self) -> &[PlanarBooleanSegmentPairWorkItem] {
        &self.work_items
    }
}

impl BooleanEvidenceReceipt for PlanarBooleanSegmentPairEnumerationReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::SegmentPairEnumeration
    }

    fn evidence_identity(&self) -> &str {
        self.segment_pair_enumeration_identity()
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_segment_pair_enumeration(self.counters())
    }
}

impl BooleanEvidenceRowAuthority for PlanarBooleanSegmentPairEnumerationReceipt {}
