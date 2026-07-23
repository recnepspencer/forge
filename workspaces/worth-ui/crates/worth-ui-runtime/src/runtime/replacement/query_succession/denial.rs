use crate::runtime::replacement::query_binding::{
    WorthUiQueryBindingIdentity, WorthUiQueryBindingUiRequirements,
    WorthUiQueryBindingUiRequirementsDriftFamily,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingDriftDenial {
    identity: WorthUiQueryBindingIdentity,
    active_ui_requirements: Option<WorthUiQueryBindingUiRequirements>,
    candidate_ui_requirements: Option<WorthUiQueryBindingUiRequirements>,
    drift_families: Vec<WorthUiQueryBindingUiRequirementsDriftFamily>,
    reason: WorthUiQueryBindingDriftDenialReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBindingDriftDenialReason {
    MissingCandidateUiRequirementsForRebind,
    MissingActiveUiRequirementsForRetirement,
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
}

impl WorthUiQueryBindingDriftDenial {
    pub(crate) fn new(
        identity: WorthUiQueryBindingIdentity,
        active_ui_requirements: Option<WorthUiQueryBindingUiRequirements>,
        candidate_ui_requirements: Option<WorthUiQueryBindingUiRequirements>,
        drift_families: Vec<WorthUiQueryBindingUiRequirementsDriftFamily>,
        reason: WorthUiQueryBindingDriftDenialReason,
    ) -> Self {
        Self {
            identity,
            active_ui_requirements,
            candidate_ui_requirements,
            drift_families,
            reason,
        }
    }

    pub fn identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.identity
    }

    pub fn active_ui_requirements(&self) -> Option<&WorthUiQueryBindingUiRequirements> {
        self.active_ui_requirements.as_ref()
    }

    pub fn candidate_ui_requirements(&self) -> Option<&WorthUiQueryBindingUiRequirements> {
        self.candidate_ui_requirements.as_ref()
    }

    pub fn drift_families(&self) -> &[WorthUiQueryBindingUiRequirementsDriftFamily] {
        &self.drift_families
    }

    pub fn reason(&self) -> WorthUiQueryBindingDriftDenialReason {
        self.reason
    }
}

pub use WorthUiQueryBindingDriftDenialReason as WorthUiQueryBindingDriftDenialKind;
