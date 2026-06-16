use worth_spatial::facade::planar_boolean_events::PlanarBooleanSegmentPairEnumerationReceipt;
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceStageKind, WorkloadEvidenceStageCounters,
    WorkloadEvidenceSupport,
};

pub(crate) struct CounterlessSegmentPairEnumerationEvidence {
    digest: String,
}

impl CounterlessSegmentPairEnumerationEvidence {
    pub(crate) fn new(receipt: &PlanarBooleanSegmentPairEnumerationReceipt) -> Self {
        Self {
            digest: receipt.segment_pair_enumeration_identity().to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for CounterlessSegmentPairEnumerationEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::SegmentPairEnumeration
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

pub(crate) struct SupportMismatchedSegmentPairEnumerationEvidence {
    digest: String,
    counters: WorkloadEvidenceStageCounters,
}

impl SupportMismatchedSegmentPairEnumerationEvidence {
    pub(crate) fn new(receipt: &PlanarBooleanSegmentPairEnumerationReceipt) -> Self {
        Self {
            digest: receipt.segment_pair_enumeration_identity().to_string(),
            counters: WorkloadEvidenceStageCounters::boolean_segment_pair_enumeration(
                receipt.counters(),
            ),
        }
    }
}

impl BooleanEvidenceReceipt for SupportMismatchedSegmentPairEnumerationEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::SegmentPairEnumeration
    }

    fn evidence_identity(&self) -> &str {
        &self.digest
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Unsupported
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        self.counters
    }
}

pub(crate) struct WrongCounterFamilySegmentPairEnumerationEvidence {
    digest: String,
}

impl WrongCounterFamilySegmentPairEnumerationEvidence {
    pub(crate) fn new(receipt: &PlanarBooleanSegmentPairEnumerationReceipt) -> Self {
        Self {
            digest: receipt.segment_pair_enumeration_identity().to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for WrongCounterFamilySegmentPairEnumerationEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::SegmentPairEnumeration
    }

    fn evidence_identity(&self) -> &str {
        &self.digest
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_event_extraction_request()
    }
}
