use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

use super::{denial_identity, EventExtractionIdentityBasis, PlanarBooleanEventExtractionCounters};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEventExtractionDenialKind {
    ZeroLengthProjectedCarrier,
    SameOperandDuplicateUnsupported,
    PredicateAmbiguousNearContact,
    NearCoincidentWithoutCertifiedContact,
    IntervalCollapsedAfterNormalization,
    MissingTopologyProvenance,
    MixedReducedPairIdentity,
    MixedFrameIdentity,
    UnsupportedHighValencePosture,
}

impl PlanarBooleanEventExtractionDenialKind {
    pub(crate) fn query_key(self) -> &'static str {
        match self {
            Self::ZeroLengthProjectedCarrier => "zero_length_projected_carrier",
            Self::SameOperandDuplicateUnsupported => "same_operand_duplicate_unsupported",
            Self::PredicateAmbiguousNearContact => "predicate_ambiguous_near_contact",
            Self::NearCoincidentWithoutCertifiedContact => {
                "near_coincident_without_certified_contact"
            }
            Self::IntervalCollapsedAfterNormalization => "interval_collapsed_after_normalization",
            Self::MissingTopologyProvenance => "missing_topology_provenance",
            Self::MixedReducedPairIdentity => "mixed_reduced_pair_identity",
            Self::MixedFrameIdentity => "mixed_frame_identity",
            Self::UnsupportedHighValencePosture => "unsupported_high_valence_posture",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEventExtractionDenial {
    kind: PlanarBooleanEventExtractionDenialKind,
    denial_identity: String,
    reduced_pair_identity: String,
    carrier_identity: Option<String>,
    segment_pair_identity: Option<String>,
    predicate_binding_identity: Option<String>,
    precision_basis_identity: Option<String>,
    workload_evidence_stage: WorkloadEvidenceStage,
    counters: PlanarBooleanEventExtractionCounters,
    human_reason: String,
}

pub(crate) struct PlanarBooleanEventExtractionDenialInput {
    pub(crate) kind: PlanarBooleanEventExtractionDenialKind,
    pub(crate) reduced_pair_identity: String,
    pub(crate) carrier_identity: Option<String>,
    pub(crate) segment_pair_identity: Option<String>,
    pub(crate) predicate_binding_identity: Option<String>,
    pub(crate) precision_basis_identity: Option<String>,
    pub(crate) workload_evidence_stage: WorkloadEvidenceStage,
    pub(crate) counters: PlanarBooleanEventExtractionCounters,
    pub(crate) human_reason: String,
}

impl PlanarBooleanEventExtractionDenial {
    pub(crate) fn new(input: PlanarBooleanEventExtractionDenialInput) -> Self {
        let basis = EventExtractionIdentityBasis {
            label: "planar-boolean:event-extraction-denial",
            kind_key: "denial",
            reduced_pair_identity: &input.reduced_pair_identity,
            carrier_identity: input.carrier_identity.as_deref(),
            segment_pair_identity: input.segment_pair_identity.as_deref(),
            predicate_binding_identity: input.predicate_binding_identity.as_deref(),
            precision_basis_identity: input.precision_basis_identity.as_deref(),
            workload_evidence_stage: input.workload_evidence_stage,
        };
        let denial_identity = denial_identity(input.kind, &basis);
        Self {
            kind: input.kind,
            denial_identity,
            reduced_pair_identity: input.reduced_pair_identity,
            carrier_identity: input.carrier_identity,
            segment_pair_identity: input.segment_pair_identity,
            predicate_binding_identity: input.predicate_binding_identity,
            precision_basis_identity: input.precision_basis_identity,
            workload_evidence_stage: input.workload_evidence_stage,
            counters: input.counters,
            human_reason: input.human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanEventExtractionDenialKind {
        self.kind
    }

    pub fn denial_identity(&self) -> &str {
        &self.denial_identity
    }

    pub fn reduced_pair_identity(&self) -> &str {
        &self.reduced_pair_identity
    }

    pub fn carrier_identity(&self) -> Option<&str> {
        self.carrier_identity.as_deref()
    }

    pub fn segment_pair_identity(&self) -> Option<&str> {
        self.segment_pair_identity.as_deref()
    }

    pub fn predicate_binding_identity(&self) -> Option<&str> {
        self.predicate_binding_identity.as_deref()
    }

    pub fn precision_basis_identity(&self) -> Option<&str> {
        self.precision_basis_identity.as_deref()
    }

    pub fn workload_evidence_stage(&self) -> WorkloadEvidenceStage {
        self.workload_evidence_stage
    }

    pub fn counters(&self) -> PlanarBooleanEventExtractionCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
