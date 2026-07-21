use crate::runtime::replacement::query_binding::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture, WorthUiQueryBindingPostureDriftFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingDriftDenial {
    identity: WorthUiQueryBindingIdentity,
    active_posture: Option<WorthUiQueryBindingPosture>,
    candidate_posture: Option<WorthUiQueryBindingPosture>,
    drift_families: Vec<WorthUiQueryBindingPostureDriftFamily>,
    reason: WorthUiQueryBindingDriftDenialReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBindingDriftDenialReason {
    UiLocalDenialPresentationWouldReplaceQueryRecovery,
    QuerySupportPostureNotAdmitted,
    MissingCandidatePostureForRebind,
    MissingActivePostureForRetirement,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiQueryLiveRebindPlanDenial {
    AmbiguousNodeReplacementPlan,
    ComparisonDigestMismatch {
        comparison_active_artifact_digest: u64,
        plan_active_artifact_digest: u64,
        comparison_candidate_artifact_digest: u64,
        plan_candidate_artifact_digest: u64,
    },
    NarrowingDigestMismatch {
        comparison_active_artifact_digest: u64,
        narrowing_active_artifact_digest: u64,
        comparison_candidate_artifact_digest: u64,
        narrowing_candidate_artifact_digest: u64,
    },
    AdmittedCandidateDigestMismatch {
        comparison_candidate_artifact_digest: u64,
        admitted_candidate_artifact_digest: u64,
    },
    AdmittedQuerySupportContractChanged {
        admitted_contract_identity: worth_ui_query_binding::WorthUiQueryBindingContractIdentity,
        current_contract_identity: worth_ui_query_binding::WorthUiQueryBindingContractIdentity,
    },
}

impl WorthUiQueryBindingDriftDenial {
    pub(crate) fn new(
        identity: WorthUiQueryBindingIdentity,
        active_posture: Option<WorthUiQueryBindingPosture>,
        candidate_posture: Option<WorthUiQueryBindingPosture>,
        drift_families: Vec<WorthUiQueryBindingPostureDriftFamily>,
        reason: WorthUiQueryBindingDriftDenialReason,
    ) -> Self {
        Self {
            identity,
            active_posture,
            candidate_posture,
            drift_families,
            reason,
        }
    }

    pub fn identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.identity
    }

    pub fn active_posture(&self) -> Option<&WorthUiQueryBindingPosture> {
        self.active_posture.as_ref()
    }

    pub fn candidate_posture(&self) -> Option<&WorthUiQueryBindingPosture> {
        self.candidate_posture.as_ref()
    }

    pub fn drift_families(&self) -> &[WorthUiQueryBindingPostureDriftFamily] {
        &self.drift_families
    }

    pub fn reason(&self) -> WorthUiQueryBindingDriftDenialReason {
        self.reason
    }
}

pub use WorthUiQueryBindingDriftDenialReason as WorthUiQueryBindingDriftDenialKind;
