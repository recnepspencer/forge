use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityFactError;
use crate::planar_contracts::clean_fail_boundary::PlanarCleanFailBoundaryReceipt;
use crate::planar_contracts::coplanar_overlap_contract::{
    CoplanarOverlapBooleanResult, CoplanarOverlapContractReceipt, CoplanarOverlapDenial,
    CoplanarOverlapImprintAction,
};
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleReceipt;
use crate::workload_platform::coplanar_overlap_storm::CoplanarOverlapStormWorkloadError;
use crate::workload_platform::planar_boolean_overlap_region_extraction::CoplanarOverlapOperatorReceipt;
use crate::workload_platform::user_response::{
    WorthPolicyDecision, WorthUserOutcome, WorthUserOutcomeCauseKind, WorthUserOutcomeKind,
    WorthUserResponseSource, WorthUserResponseWorkload,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoplanarOverlapUserOutcomeKind {
    ContractsCertified,
    PolicyDecisionRequired,
    NoOptions,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoplanarOverlapNoOptionsCause {
    DirtyInput,
    UnsupportedInput,
    DeniedMovementOrRotation,
    PredicateUncertain,
    PredicateEvaluationFailed,
    PredicateAuthorityNotBound,
    IntegrityMismatch,
    MissingEvidence,
    OverlapDenied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoplanarOverlapUserDecision {
    TreatCandidateLoopAsInsideFace,
    TreatCandidateLoopAsOutsideFace,
    PauseForManualInspection,
}

impl CoplanarOverlapUserDecision {
    pub const fn label(self) -> &'static str {
        match self {
            Self::TreatCandidateLoopAsInsideFace => "Treat the candidate loop as inside this face.",
            Self::TreatCandidateLoopAsOutsideFace => {
                "Treat the candidate loop as outside this face."
            }
            Self::PauseForManualInspection => "Pause boolean certification for manual inspection.",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoplanarOverlapUserOutcome {
    shared: WorthUserOutcome,
    decisions: Vec<CoplanarOverlapUserDecision>,
    boolean_result: Option<CoplanarOverlapBooleanResult>,
    imprint_action: Option<CoplanarOverlapImprintAction>,
}

impl CoplanarOverlapUserOutcome {
    pub fn from_overlap_receipt(receipt: &CoplanarOverlapContractReceipt) -> Self {
        let shared = shared_outcome(WorthUserResponseSource::from_overlap_receipt(receipt));
        let decisions = overlap_decisions(shared.choices());
        Self {
            shared,
            decisions,
            boolean_result: receipt.boolean_result(),
            imprint_action: receipt.imprint_action(),
        }
    }

    pub fn from_operator_receipt(receipt: &CoplanarOverlapOperatorReceipt) -> Self {
        Self {
            shared: shared_outcome(WorthUserResponseSource::from_coplanar_overlap_operator(
                receipt,
            )),
            decisions: Vec::new(),
            boolean_result: None,
            imprint_action: None,
        }
    }

    pub fn from_storm_workload_error(error: CoplanarOverlapStormWorkloadError) -> Self {
        Self::from_source(WorthUserResponseSource::from_coplanar_overlap_storm_error(
            error,
        ))
    }

    pub fn from_clean_fail_boundary(receipt: &PlanarCleanFailBoundaryReceipt) -> Self {
        Self::from_source(WorthUserResponseSource::from_clean_fail_boundary(receipt))
    }

    pub fn from_overlap_denial(
        denial: &CoplanarOverlapDenial,
        diagnostic: &PlanarDiagnosticBundleReceipt,
    ) -> Self {
        Self::from_source(WorthUserResponseSource::from_overlap_denial(
            denial, diagnostic,
        ))
    }

    pub fn from_predicate_authority_error(error: &PlanarPredicateAuthorityFactError) -> Self {
        Self::from_source(WorthUserResponseSource::from_predicate_authority_error(
            error,
        ))
    }

    pub fn kind(&self) -> CoplanarOverlapUserOutcomeKind {
        match self.shared.kind() {
            WorthUserOutcomeKind::Admitted => CoplanarOverlapUserOutcomeKind::ContractsCertified,
            WorthUserOutcomeKind::PolicyRequired => {
                CoplanarOverlapUserOutcomeKind::PolicyDecisionRequired
            }
            _ => CoplanarOverlapUserOutcomeKind::NoOptions,
        }
    }

    pub fn no_options_cause(&self) -> Option<CoplanarOverlapNoOptionsCause> {
        self.shared
            .cause()
            .map(|cause| overlap_no_options_cause(cause.kind()))
    }

    pub fn message(&self) -> &str {
        self.shared.human_response().summary()
    }

    pub fn evidence_digest(&self) -> &str {
        self.shared.evidence().digest()
    }

    pub fn decisions(&self) -> &[CoplanarOverlapUserDecision] {
        &self.decisions
    }

    pub fn boolean_result(&self) -> Option<CoplanarOverlapBooleanResult> {
        self.boolean_result
    }

    pub fn imprint_action(&self) -> Option<CoplanarOverlapImprintAction> {
        self.imprint_action
    }

    pub fn shared_outcome(&self) -> &WorthUserOutcome {
        &self.shared
    }

    fn from_source(source: WorthUserResponseSource) -> Self {
        Self {
            shared: shared_outcome(source),
            decisions: Vec::new(),
            boolean_result: None,
            imprint_action: None,
        }
    }
}

fn shared_outcome(source: WorthUserResponseSource) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(source)
        .declared("explain coplanar overlap outcome")
        .respond()
        .expect("overlap user response should be certifiable")
        .outcome()
        .clone()
}

fn overlap_no_options_cause(cause: WorthUserOutcomeCauseKind) -> CoplanarOverlapNoOptionsCause {
    match cause {
        WorthUserOutcomeCauseKind::DirtyInput => CoplanarOverlapNoOptionsCause::DirtyInput,
        WorthUserOutcomeCauseKind::UnsupportedInput => {
            CoplanarOverlapNoOptionsCause::UnsupportedInput
        }
        WorthUserOutcomeCauseKind::DeniedMovementOrRotation => {
            CoplanarOverlapNoOptionsCause::DeniedMovementOrRotation
        }
        WorthUserOutcomeCauseKind::PredicateUncertain => {
            CoplanarOverlapNoOptionsCause::PredicateUncertain
        }
        WorthUserOutcomeCauseKind::PredicateEvaluationFailed => {
            CoplanarOverlapNoOptionsCause::PredicateEvaluationFailed
        }
        WorthUserOutcomeCauseKind::PredicateAuthorityNotBound => {
            CoplanarOverlapNoOptionsCause::PredicateAuthorityNotBound
        }
        WorthUserOutcomeCauseKind::IntegrityMismatch => {
            CoplanarOverlapNoOptionsCause::IntegrityMismatch
        }
        WorthUserOutcomeCauseKind::MissingEvidence => {
            CoplanarOverlapNoOptionsCause::MissingEvidence
        }
        _ => CoplanarOverlapNoOptionsCause::OverlapDenied,
    }
}

fn overlap_decision(decision: WorthPolicyDecision) -> CoplanarOverlapUserDecision {
    match decision {
        WorthPolicyDecision::TreatCandidateLoopAsInsideFace => {
            CoplanarOverlapUserDecision::TreatCandidateLoopAsInsideFace
        }
        WorthPolicyDecision::TreatCandidateLoopAsOutsideFace => {
            CoplanarOverlapUserDecision::TreatCandidateLoopAsOutsideFace
        }
        WorthPolicyDecision::PauseForManualInspection => {
            CoplanarOverlapUserDecision::PauseForManualInspection
        }
    }
}

fn overlap_decisions(decisions: &[WorthPolicyDecision]) -> Vec<CoplanarOverlapUserDecision> {
    decisions.iter().copied().map(overlap_decision).collect()
}
