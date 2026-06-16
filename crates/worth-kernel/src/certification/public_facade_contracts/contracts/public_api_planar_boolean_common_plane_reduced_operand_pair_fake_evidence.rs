use worth_kernel::workload_composition::PlanarBooleanCommonPlaneReducedOperandPairRequest;
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceStageKind, WorkloadEvidenceStageCounters,
    WorkloadEvidenceSupport,
};

pub(crate) struct CounterlessReducedOperandPairEvidence {
    digest: String,
}

impl CounterlessReducedOperandPairEvidence {
    pub(crate) fn new(reduced_pair: &PlanarBooleanCommonPlaneReducedOperandPairRequest) -> Self {
        Self {
            digest: reduced_pair
                .reduced_operand_pair_request_identity()
                .to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for CounterlessReducedOperandPairEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::ReducedOperandPair
    }

    fn evidence_identity(&self) -> &str {
        &self.digest
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::default()
    }
}

pub(crate) struct SupportMismatchedReducedOperandPairEvidence {
    digest: String,
}

impl SupportMismatchedReducedOperandPairEvidence {
    pub(crate) fn new(reduced_pair: &PlanarBooleanCommonPlaneReducedOperandPairRequest) -> Self {
        Self {
            digest: reduced_pair
                .reduced_operand_pair_request_identity()
                .to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for SupportMismatchedReducedOperandPairEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::ReducedOperandPair
    }

    fn evidence_identity(&self) -> &str {
        &self.digest
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Unsupported
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_reduced_operand_pair()
    }
}

pub(crate) struct WrongCounterFamilyReducedOperandPairEvidence {
    digest: String,
}

impl WrongCounterFamilyReducedOperandPairEvidence {
    pub(crate) fn new(reduced_pair: &PlanarBooleanCommonPlaneReducedOperandPairRequest) -> Self {
        Self {
            digest: reduced_pair
                .reduced_operand_pair_request_identity()
                .to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for WrongCounterFamilyReducedOperandPairEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::ReducedOperandPair
    }

    fn evidence_identity(&self) -> &str {
        &self.digest
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_operand_a_projection_consumption()
    }
}
