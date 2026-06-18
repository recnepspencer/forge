use worth_kernel::workload_composition::PlanarBooleanEventExtractionRequest;
use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

pub(crate) struct CounterlessEventExtractionRequestEvidence {
    digest: String,
}

impl CounterlessEventExtractionRequestEvidence {
    pub(crate) fn new(event_request: &PlanarBooleanEventExtractionRequest) -> Self {
        Self {
            digest: event_request
                .event_extraction_request_identity()
                .to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for CounterlessEventExtractionRequestEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::EventExtractionRequest
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

impl BooleanEvidenceRowAuthority for CounterlessEventExtractionRequestEvidence {}

pub(crate) struct SupportMismatchedEventExtractionRequestEvidence {
    digest: String,
}

impl SupportMismatchedEventExtractionRequestEvidence {
    pub(crate) fn new(event_request: &PlanarBooleanEventExtractionRequest) -> Self {
        Self {
            digest: event_request
                .event_extraction_request_identity()
                .to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for SupportMismatchedEventExtractionRequestEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::EventExtractionRequest
    }

    fn evidence_identity(&self) -> &str {
        &self.digest
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Unsupported
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_event_extraction_request()
    }
}

impl BooleanEvidenceRowAuthority for SupportMismatchedEventExtractionRequestEvidence {}

pub(crate) struct WrongCounterFamilyEventExtractionRequestEvidence {
    digest: String,
}

impl WrongCounterFamilyEventExtractionRequestEvidence {
    pub(crate) fn new(event_request: &PlanarBooleanEventExtractionRequest) -> Self {
        Self {
            digest: event_request
                .event_extraction_request_identity()
                .to_string(),
        }
    }
}

impl BooleanEvidenceReceipt for WrongCounterFamilyEventExtractionRequestEvidence {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::EventExtractionRequest
    }

    fn evidence_identity(&self) -> &str {
        &self.digest
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_reduced_operand_pair()
    }
}

impl BooleanEvidenceRowAuthority for WrongCounterFamilyEventExtractionRequestEvidence {}
