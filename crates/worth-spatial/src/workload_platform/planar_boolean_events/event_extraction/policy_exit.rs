use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

use super::{
    policy_exit_identity, EventExtractionIdentityBasis, PlanarBooleanEventExtractionCounters,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEventExtractionPolicyExitKind {
    ManualInspectionRequired,
    ImprintRequiredForCollinearOverlap,
    UnsupportedHighValenceRequiresPolicy,
}

impl PlanarBooleanEventExtractionPolicyExitKind {
    pub(crate) fn query_key(self) -> &'static str {
        match self {
            Self::ManualInspectionRequired => "manual_inspection_required",
            Self::ImprintRequiredForCollinearOverlap => "imprint_required_for_collinear_overlap",
            Self::UnsupportedHighValenceRequiresPolicy => {
                "unsupported_high_valence_requires_policy"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEventExtractionPolicyExit {
    kind: PlanarBooleanEventExtractionPolicyExitKind,
    policy_exit_identity: String,
    reduced_pair_identity: String,
    carrier_identity: Option<String>,
    segment_pair_identity: Option<String>,
    predicate_binding_identity: Option<String>,
    precision_basis_identity: Option<String>,
    workload_evidence_stage: WorkloadEvidenceStage,
    counters: PlanarBooleanEventExtractionCounters,
    human_reason: String,
}

pub(crate) struct PlanarBooleanEventExtractionPolicyExitInput {
    pub(crate) kind: PlanarBooleanEventExtractionPolicyExitKind,
    pub(crate) reduced_pair_identity: String,
    pub(crate) carrier_identity: Option<String>,
    pub(crate) segment_pair_identity: Option<String>,
    pub(crate) predicate_binding_identity: Option<String>,
    pub(crate) precision_basis_identity: Option<String>,
    pub(crate) workload_evidence_stage: WorkloadEvidenceStage,
    pub(crate) counters: PlanarBooleanEventExtractionCounters,
    pub(crate) human_reason: String,
}

impl PlanarBooleanEventExtractionPolicyExit {
    pub(crate) fn new(input: PlanarBooleanEventExtractionPolicyExitInput) -> Self {
        let basis = EventExtractionIdentityBasis {
            label: "planar-boolean:event-extraction-policy-exit",
            kind_key: "policy-exit",
            reduced_pair_identity: &input.reduced_pair_identity,
            carrier_identity: input.carrier_identity.as_deref(),
            segment_pair_identity: input.segment_pair_identity.as_deref(),
            predicate_binding_identity: input.predicate_binding_identity.as_deref(),
            precision_basis_identity: input.precision_basis_identity.as_deref(),
            workload_evidence_stage: input.workload_evidence_stage,
        };
        let policy_exit_identity = policy_exit_identity(input.kind, &basis);
        Self {
            kind: input.kind,
            policy_exit_identity,
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

    pub fn kind(&self) -> PlanarBooleanEventExtractionPolicyExitKind {
        self.kind
    }

    pub fn policy_exit_identity(&self) -> &str {
        &self.policy_exit_identity
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
