mod build;

use crate::application::ForgeQueryAdmittedWorldBasis;
use crate::identity::hash_parts;
use crate::query_basis_lifecycle::{
    BasisIntentDenial, BasisLifecyclePosture, BasisOperationLaneRequest, BasisVisibility,
    DeniedBasisCapability, InspectionBasisCapability, LowerRuntimeBoundInspectionBasis,
    LowerRuntimeBoundObservationBasis, LowerRuntimeBoundSubscriptionActivationBasis,
    LowerRuntimeBoundSubscriptionDeclarationBasis, NormalizedBasisFamily,
    ObservationBasisCapability, ScopedInspectionBasis, ScopedObservationBasis, ScopedReplayBasis,
    ScopedSubscriptionActivationBasis, ScopedSubscriptionDeclarationBasis,
    SubscriptionActivationBasisCapability, SubscriptionDeclarationBasisCapability,
};
use crate::runtime::state_basis_classification::{
    authority_lane_for_denied_basis, authority_lane_for_intent_denial, state_kind_for_basis_denial,
    state_kind_for_basis_intent_denial,
};
use crate::runtime::{ForgeQueryAuthorityLane, ForgeQueryRuntimeStateKind};

use build::{from_admitted_capability, from_basis_admission, from_lower_runtime_bound_basis};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBasisLifecycleInspection {
    subject_label: &'static str,
    state_kind: ForgeQueryRuntimeStateKind,
    authority_lane: ForgeQueryAuthorityLane,
    basis_digest: String,
    shape_digest: String,
    family: Option<NormalizedBasisFamily>,
    operation_lane: Option<BasisOperationLaneRequest>,
    visibility: Option<BasisVisibility>,
    lifecycle_posture: Option<BasisLifecyclePosture>,
    lower_runtime_authority: Option<&'static str>,
    lower_runtime_binding_digest: Option<String>,
    support_digest: Option<String>,
    denial_digest: Option<String>,
    explanation: String,
    inspection_digest: String,
}

impl ForgeQueryBasisLifecycleInspection {
    pub(in crate::runtime) fn from_admitted_world_basis(
        basis: &ForgeQueryAdmittedWorldBasis,
    ) -> Self {
        let explanation = format!(
            "retained admitted world basis `{}` exposes basis lifecycle support `{}` and support snapshot `{}`",
            basis.domain_key(),
            basis.basis_lifecycle_support_digest(),
            basis.support_snapshot_digest()
        );
        let inspection_digest = hash_parts(&[
            "forge_query_basis_lifecycle_inspection_v1".to_string(),
            "subject:admitted_world_basis".to_string(),
            format!("state:{}", ForgeQueryRuntimeStateKind::Ready.as_str()),
            format!(
                "authority_lane:{}",
                ForgeQueryAuthorityLane::AuthoritativeTruth.as_str()
            ),
            format!("basis:{}", basis.basis_lifecycle_support_digest()),
            format!("shape:{}", basis.handle_identity_digest()),
            format!("support:{}", basis.support_snapshot_digest()),
        ]);
        Self {
            subject_label: "admitted_world_basis",
            state_kind: ForgeQueryRuntimeStateKind::Ready,
            authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            basis_digest: basis.basis_lifecycle_support_digest().to_string(),
            shape_digest: basis.handle_identity_digest().to_string(),
            family: None,
            operation_lane: None,
            visibility: None,
            lifecycle_posture: None,
            lower_runtime_authority: None,
            lower_runtime_binding_digest: None,
            support_digest: Some(basis.support_snapshot_digest().to_string()),
            denial_digest: None,
            explanation,
            inspection_digest,
        }
    }

    pub fn subject_label(&self) -> &'static str {
        self.subject_label
    }

    pub fn state_kind(&self) -> ForgeQueryRuntimeStateKind {
        self.state_kind
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn shape_digest(&self) -> &str {
        &self.shape_digest
    }

    pub fn family(&self) -> Option<&NormalizedBasisFamily> {
        self.family.as_ref()
    }

    pub fn operation_lane(&self) -> Option<&BasisOperationLaneRequest> {
        self.operation_lane.as_ref()
    }

    pub fn visibility(&self) -> Option<BasisVisibility> {
        self.visibility
    }

    pub fn lifecycle_posture(&self) -> Option<BasisLifecyclePosture> {
        self.lifecycle_posture
    }

    pub fn lower_runtime_authority(&self) -> Option<&'static str> {
        self.lower_runtime_authority
    }

    pub fn lower_runtime_binding_digest(&self) -> Option<&str> {
        self.lower_runtime_binding_digest.as_deref()
    }

    pub fn support_digest(&self) -> Option<&str> {
        self.support_digest.as_deref()
    }

    pub fn denial_digest(&self) -> Option<&str> {
        self.denial_digest.as_deref()
    }

    pub fn explanation(&self) -> &str {
        &self.explanation
    }

    pub fn inspection_digest(&self) -> &str {
        &self.inspection_digest
    }
}

impl From<&ObservationBasisCapability> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &ObservationBasisCapability) -> Self {
        from_basis_admission("observation_basis_capability", value.admission())
    }
}

impl From<&InspectionBasisCapability> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &InspectionBasisCapability) -> Self {
        from_basis_admission("inspection_basis_capability", value.admission())
    }
}

impl From<&SubscriptionDeclarationBasisCapability> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &SubscriptionDeclarationBasisCapability) -> Self {
        from_basis_admission(
            "subscription_declaration_basis_capability",
            value.admission(),
        )
    }
}

impl From<&SubscriptionActivationBasisCapability> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &SubscriptionActivationBasisCapability) -> Self {
        from_basis_admission(
            "subscription_activation_basis_capability",
            value.admission(),
        )
    }
}

impl From<&ScopedObservationBasis> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &ScopedObservationBasis) -> Self {
        from_basis_admission("scoped_observation_basis", value.admission())
    }
}

impl From<&ScopedInspectionBasis> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &ScopedInspectionBasis) -> Self {
        from_basis_admission("scoped_inspection_basis", value.admission())
    }
}

impl From<&ScopedReplayBasis> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &ScopedReplayBasis) -> Self {
        from_admitted_capability(
            "scoped_replay_basis",
            value.capability(),
            value.scoped_digest(),
            None,
        )
    }
}

impl From<&ScopedSubscriptionDeclarationBasis> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &ScopedSubscriptionDeclarationBasis) -> Self {
        from_admitted_capability(
            "scoped_subscription_declaration_basis",
            value.capability(),
            value.scoped_digest(),
            None,
        )
    }
}

impl From<&ScopedSubscriptionActivationBasis> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &ScopedSubscriptionActivationBasis) -> Self {
        from_admitted_capability(
            "scoped_subscription_activation_basis",
            value.capability(),
            value.scoped_digest(),
            None,
        )
    }
}

impl From<&LowerRuntimeBoundObservationBasis> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &LowerRuntimeBoundObservationBasis) -> Self {
        from_lower_runtime_bound_basis(
            "lower_runtime_bound_observation_basis",
            value.capability(),
            value.authority_name(),
            value.binding_digest(),
        )
    }
}

impl From<&LowerRuntimeBoundInspectionBasis> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &LowerRuntimeBoundInspectionBasis) -> Self {
        from_lower_runtime_bound_basis(
            "lower_runtime_bound_inspection_basis",
            value.capability(),
            value.authority_name(),
            value.binding_digest(),
        )
    }
}

impl From<&LowerRuntimeBoundSubscriptionDeclarationBasis> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &LowerRuntimeBoundSubscriptionDeclarationBasis) -> Self {
        from_lower_runtime_bound_basis(
            "lower_runtime_bound_subscription_declaration_basis",
            value.capability(),
            value.authority_name(),
            value.binding_digest(),
        )
    }
}

impl From<&LowerRuntimeBoundSubscriptionActivationBasis> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &LowerRuntimeBoundSubscriptionActivationBasis) -> Self {
        from_lower_runtime_bound_basis(
            "lower_runtime_bound_subscription_activation_basis",
            value.capability(),
            value.authority_name(),
            value.binding_digest(),
        )
    }
}

impl From<&DeniedBasisCapability> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &DeniedBasisCapability) -> Self {
        let explanation = format!(
            "query basis lifecycle denied `{}` on `{}` with `{}`",
            value.family().as_str(),
            value.operation_lane().as_str(),
            value.trace().rule_label()
        );
        let inspection_digest = hash_parts(&[
            "forge_query_basis_lifecycle_inspection_v1".to_string(),
            "subject:denied_basis_capability".to_string(),
            format!(
                "state:{}",
                state_kind_for_basis_denial(value.kind()).as_str()
            ),
            format!(
                "authority_lane:{}",
                authority_lane_for_denied_basis(value.family(), value.operation_lane()).as_str()
            ),
            format!("basis:{}", value.normalized_basis_intent_digest()),
            format!("shape:{}", value.failure_digest()),
            format!("family:{}", value.family().as_str()),
            format!("operation_lane:{}", value.operation_lane().as_str()),
            format!("denial:{}", value.failure_digest()),
        ]);
        Self {
            subject_label: "denied_basis_capability",
            state_kind: state_kind_for_basis_denial(value.kind()),
            authority_lane: authority_lane_for_denied_basis(value.family(), value.operation_lane()),
            basis_digest: value.normalized_basis_intent_digest().to_string(),
            shape_digest: value.failure_digest().to_string(),
            family: Some(value.family().clone()),
            operation_lane: Some(value.operation_lane().clone()),
            visibility: None,
            lifecycle_posture: None,
            lower_runtime_authority: None,
            lower_runtime_binding_digest: None,
            support_digest: None,
            denial_digest: Some(value.failure_digest().to_string()),
            explanation,
            inspection_digest,
        }
    }
}

impl From<&BasisIntentDenial> for ForgeQueryBasisLifecycleInspection {
    fn from(value: &BasisIntentDenial) -> Self {
        let explanation = format!(
            "query basis lifecycle normalization denied `{}` through `{}`",
            value.operation_lane().as_str(),
            value.source_path().as_str()
        );
        let inspection_digest = hash_parts(&[
            "forge_query_basis_lifecycle_inspection_v1".to_string(),
            "subject:basis_intent_denial".to_string(),
            format!(
                "state:{}",
                state_kind_for_basis_intent_denial(value.kind()).as_str()
            ),
            format!(
                "authority_lane:{}",
                authority_lane_for_intent_denial(value.kind(), value.operation_lane()).as_str()
            ),
            format!("basis:{}", value.raw_basis_intent_digest()),
            format!("shape:{}", value.failure_digest()),
            format!("operation_lane:{}", value.operation_lane().as_str()),
            format!("denial:{}", value.failure_digest()),
        ]);
        Self {
            subject_label: "basis_intent_denial",
            state_kind: state_kind_for_basis_intent_denial(value.kind()),
            authority_lane: authority_lane_for_intent_denial(value.kind(), value.operation_lane()),
            basis_digest: value.raw_basis_intent_digest().to_string(),
            shape_digest: value.failure_digest().to_string(),
            family: None,
            operation_lane: Some(value.operation_lane().clone()),
            visibility: None,
            lifecycle_posture: None,
            lower_runtime_authority: None,
            lower_runtime_binding_digest: None,
            support_digest: None,
            denial_digest: Some(value.failure_digest().to_string()),
            explanation,
            inspection_digest,
        }
    }
}
