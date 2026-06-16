use super::evidence_identities::{
    runtime_state_snapshot_basis_label_identity, runtime_state_snapshot_result_shape_label_identity,
};
use super::state::ForgeQueryRuntimeStateTarget;
use super::state_basis_classification::{
    authority_lane_for_basis_family, authority_lane_for_denied_basis,
    authority_lane_for_intent_denial, state_kind_for_basis_denial,
    state_kind_for_basis_intent_denial,
};
use super::{
    ForgeQueryAuthorityLane, ForgeQueryRuntime, ForgeQueryRuntimeError, ForgeQueryRuntimeStateKind,
    ForgeQueryRuntimeStateSnapshot,
};
use crate::application::ForgeQueryAdmittedWorldBasis;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::query_basis_lifecycle::{
    AdmittedBasisCapability, BasisCapabilityAdmission, BasisIntentDenial, BasisIntentDenialKind,
    DeniedBasisCapability, DeniedBasisCapabilityKind, InspectionBasisCapability,
    LowerRuntimeBoundInspectionBasis, LowerRuntimeBoundObservationBasis,
    LowerRuntimeBoundSubscriptionActivationBasis, LowerRuntimeBoundSubscriptionDeclarationBasis,
    MaterializationBasisCapability, MutationPreparationBasisCapability, ObservationBasisCapability,
    PreviewCloseoutBasisCapability, ReplayBasisCapability, ScopedCertificationBasis,
    ScopedInspectionBasis, ScopedMaterializationBasis, ScopedMutationPreparationBasis,
    ScopedObservationBasis, ScopedPreviewCloseoutBasis, ScopedReplayBasis,
    ScopedSubscriptionActivationBasis, ScopedSubscriptionDeclarationBasis,
    SubscriptionActivationBasisCapability, SubscriptionDeclarationBasisCapability,
};

impl ForgeQueryRuntimeStateTarget for &ForgeQueryAdmittedWorldBasis {
    fn into_state_snapshot(
        self,
        _runtime: &ForgeQueryRuntime,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        Ok(ForgeQueryRuntimeStateSnapshot::ready(
            runtime_state_snapshot_basis_label_identity(self.basis_lifecycle_support_identity()),
            runtime_state_snapshot_result_shape_label_identity(self.handle_identity()),
            ForgeQueryAuthorityLane::AuthoritativeTruth,
            format!(
                "retained admitted world basis `{}` is ready with query basis lifecycle support `{}`",
                self.domain_key(),
                self.basis_lifecycle_support_for_reporting()
            ),
        ))
    }
}

macro_rules! impl_basis_capability_state_target {
    ($target:ty, $label:literal) => {
        impl ForgeQueryRuntimeStateTarget for $target {
            fn into_state_snapshot(
                self,
                _runtime: &ForgeQueryRuntime,
            ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
                Ok(snapshot_for_basis_admission(
                    self.admission(),
                    $label,
                    basis_shape_identity(self.admission()),
                ))
            }
        }
    };
}

impl_basis_capability_state_target!(&ObservationBasisCapability, "observation basis capability");
impl_basis_capability_state_target!(
    &MutationPreparationBasisCapability,
    "mutation-preparation basis capability"
);
impl_basis_capability_state_target!(&ReplayBasisCapability, "replay basis capability");
impl_basis_capability_state_target!(&InspectionBasisCapability, "inspection basis capability");
impl_basis_capability_state_target!(
    &MaterializationBasisCapability,
    "materialization basis capability"
);
impl_basis_capability_state_target!(
    &SubscriptionDeclarationBasisCapability,
    "subscription-declaration basis capability"
);
impl_basis_capability_state_target!(
    &SubscriptionActivationBasisCapability,
    "subscription-activation basis capability"
);
impl_basis_capability_state_target!(
    &PreviewCloseoutBasisCapability,
    "preview-closeout basis capability"
);

impl ForgeQueryRuntimeStateTarget for &ScopedObservationBasis {
    fn into_state_snapshot(
        self,
        _runtime: &ForgeQueryRuntime,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        Ok(snapshot_for_basis_admission(
            self.admission(),
            "scoped observation basis",
            scoped_result_shape_identity(self.admission()),
        ))
    }
}

impl ForgeQueryRuntimeStateTarget for &ScopedInspectionBasis {
    fn into_state_snapshot(
        self,
        _runtime: &ForgeQueryRuntime,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        Ok(snapshot_for_basis_admission(
            self.admission(),
            "scoped inspection basis",
            scoped_result_shape_identity(self.admission()),
        ))
    }
}

macro_rules! impl_scoped_admitted_basis_state_target {
    ($target:ty, $label:literal) => {
        impl ForgeQueryRuntimeStateTarget for $target {
            fn into_state_snapshot(
                self,
                _runtime: &ForgeQueryRuntime,
            ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
                Ok(snapshot_for_admitted_basis(
                    self.capability(),
                    $label,
                    self.capability().snapshot_result_shape_identity(),
                ))
            }
        }
    };
}

impl_scoped_admitted_basis_state_target!(
    &ScopedMutationPreparationBasis,
    "scoped mutation-preparation basis"
);
impl_scoped_admitted_basis_state_target!(&ScopedReplayBasis, "scoped replay basis");
impl_scoped_admitted_basis_state_target!(
    &ScopedMaterializationBasis,
    "scoped materialization basis"
);
impl_scoped_admitted_basis_state_target!(
    &ScopedSubscriptionDeclarationBasis,
    "scoped subscription-declaration basis"
);
impl_scoped_admitted_basis_state_target!(
    &ScopedSubscriptionActivationBasis,
    "scoped subscription-activation basis"
);
impl_scoped_admitted_basis_state_target!(
    &ScopedPreviewCloseoutBasis,
    "scoped preview-closeout basis"
);
impl_scoped_admitted_basis_state_target!(&ScopedCertificationBasis, "scoped certification basis");

macro_rules! impl_bound_basis_state_target {
    ($target:ty, $label:literal) => {
        impl ForgeQueryRuntimeStateTarget for $target {
            fn into_state_snapshot(
                self,
                _runtime: &ForgeQueryRuntime,
            ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
                let admission = self.capability();
                let explanation = format!(
                    "{} is ready through lower-runtime `{}` evidence with `{}` binding",
                    $label,
                    self.authority_name(),
                    self.binding_for_reporting()
                );
                Ok(match admission {
                    BasisCapabilityAdmission::Admitted(admitted) => {
                        ForgeQueryRuntimeStateSnapshot::ready(
                            runtime_state_snapshot_basis_label_identity(
                                &admitted.snapshot_basis_identity(),
                            ),
                            runtime_state_snapshot_result_shape_label_identity(
                                self.binding_identity(),
                            ),
                            authority_lane_for_basis_family(
                                admitted.family(),
                                admitted.lifecycle_posture(),
                                admitted.operation_lane(),
                            ),
                            explanation,
                        )
                    }
                    BasisCapabilityAdmission::Advisory(advisory) => {
                        ForgeQueryRuntimeStateSnapshot::deferred(
                            ForgeQueryRuntimeStateKind::Pending,
                            runtime_state_snapshot_basis_label_identity(
                                &advisory.snapshot_basis_identity(),
                            ),
                            runtime_state_snapshot_result_shape_label_identity(
                                self.binding_identity(),
                            ),
                            authority_lane_for_basis_family(
                                advisory.family(),
                                advisory.lifecycle_posture(),
                                advisory.operation_lane(),
                            ),
                            explanation,
                        )
                    }
                })
            }
        }
    };
}

impl_bound_basis_state_target!(
    &LowerRuntimeBoundObservationBasis,
    "lower-runtime-bound observation basis"
);
impl_bound_basis_state_target!(
    &LowerRuntimeBoundInspectionBasis,
    "lower-runtime-bound inspection basis"
);
impl_bound_basis_state_target!(
    &LowerRuntimeBoundSubscriptionDeclarationBasis,
    "lower-runtime-bound subscription-declaration basis"
);
impl_bound_basis_state_target!(
    &LowerRuntimeBoundSubscriptionActivationBasis,
    "lower-runtime-bound subscription-activation basis"
);

impl ForgeQueryRuntimeStateTarget for &DeniedBasisCapability {
    fn into_state_snapshot(
        self,
        _runtime: &ForgeQueryRuntime,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        Ok(ForgeQueryRuntimeStateSnapshot::deferred(
            state_kind_for_basis_denial(self.kind()),
            denied_basis_snapshot_basis_identity(self),
            denied_basis_snapshot_result_shape_identity(self),
            authority_lane_for_denied_basis(self.family(), self.operation_lane()),
            format!(
                "query basis lifecycle denied `{}` on `{}` with `{}`",
                self.family().as_str(),
                self.operation_lane().as_str(),
                self.trace().rule_label()
            ),
        ))
    }
}

impl ForgeQueryRuntimeStateTarget for &BasisIntentDenial {
    fn into_state_snapshot(
        self,
        _runtime: &ForgeQueryRuntime,
    ) -> Result<ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        Ok(ForgeQueryRuntimeStateSnapshot::deferred(
            state_kind_for_basis_intent_denial(self.kind()),
            basis_intent_denial_snapshot_basis_identity(self),
            basis_intent_denial_snapshot_result_shape_identity(self),
            authority_lane_for_intent_denial(self.kind(), self.operation_lane()),
            format!(
                "query basis lifecycle normalization denied `{}` through `{}`",
                self.operation_lane().as_str(),
                self.source_path().as_str()
            ),
        ))
    }
}

fn snapshot_for_basis_admission(
    admission: &BasisCapabilityAdmission,
    label: &str,
    result_shape_identity: ForgeQueryEvidenceIdentity,
) -> ForgeQueryRuntimeStateSnapshot {
    match admission {
        BasisCapabilityAdmission::Admitted(admitted) => {
            snapshot_for_admitted_basis(admitted, label, result_shape_identity)
        }
        BasisCapabilityAdmission::Advisory(advisory) => ForgeQueryRuntimeStateSnapshot::deferred(
            ForgeQueryRuntimeStateKind::Pending,
            runtime_state_snapshot_basis_label_identity(&advisory.snapshot_basis_identity()),
            runtime_state_snapshot_result_shape_label_identity(&result_shape_identity),
            authority_lane_for_basis_family(
                advisory.family(),
                advisory.lifecycle_posture(),
                advisory.operation_lane(),
            ),
            format!(
                "{} remains advisory for `{}` visibility with `{}` lifecycle posture",
                label,
                advisory.visibility().as_str(),
                advisory.lifecycle_posture().as_str()
            ),
        ),
    }
}

fn snapshot_for_admitted_basis(
    admitted: &AdmittedBasisCapability,
    label: &str,
    result_shape_identity: ForgeQueryEvidenceIdentity,
) -> ForgeQueryRuntimeStateSnapshot {
    ForgeQueryRuntimeStateSnapshot::ready(
        runtime_state_snapshot_basis_label_identity(&admitted.snapshot_basis_identity()),
        runtime_state_snapshot_result_shape_label_identity(&result_shape_identity),
        authority_lane_for_basis_family(
            admitted.family(),
            admitted.lifecycle_posture(),
            admitted.operation_lane(),
        ),
        format!(
            "{} is ready for `{}` visibility with `{}` lifecycle posture",
            label,
            admitted.visibility().as_str(),
            admitted.lifecycle_posture().as_str()
        ),
    )
}

fn basis_shape_identity(admission: &BasisCapabilityAdmission) -> ForgeQueryEvidenceIdentity {
    match admission {
        BasisCapabilityAdmission::Admitted(admitted) => admitted.snapshot_result_shape_identity(),
        BasisCapabilityAdmission::Advisory(advisory) => advisory.snapshot_result_shape_identity(),
    }
}

fn scoped_result_shape_identity(
    admission: &BasisCapabilityAdmission,
) -> ForgeQueryEvidenceIdentity {
    basis_shape_identity(admission)
}

fn denied_basis_snapshot_basis_identity(
    denied: &DeniedBasisCapability,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RawBasisIntent)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "basis_denial_snapshot_basis_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            denied.family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("operation_lane"),
            denied.operation_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("kind"),
            denied_basis_kind_label(denied.kind()),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("rule"),
            denied.trace().rule_label(),
        )
        .seal()
}

fn denied_basis_snapshot_result_shape_identity(
    denied: &DeniedBasisCapability,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RawBasisIntent)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "basis_denial_snapshot_result_shape_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            denied.family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("operation_lane"),
            denied.operation_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("kind"),
            denied_basis_kind_label(denied.kind()),
        )
        .seal()
}

fn basis_intent_denial_snapshot_basis_identity(
    denial: &BasisIntentDenial,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RawBasisIntent)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "basis_intent_denial_snapshot_basis_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("operation_lane"),
            denial.operation_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("kind"),
            basis_intent_denial_kind_label(denial.kind()),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_path"),
            denial.source_path().as_str(),
        )
        .seal()
}

fn basis_intent_denial_snapshot_result_shape_identity(
    denial: &BasisIntentDenial,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::RawBasisIntent)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "basis_intent_denial_snapshot_result_shape_v1",
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("operation_lane"),
            denial.operation_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("kind"),
            basis_intent_denial_kind_label(denial.kind()),
        )
        .seal()
}

fn denied_basis_kind_label(kind: &DeniedBasisCapabilityKind) -> &'static str {
    match kind {
        DeniedBasisCapabilityKind::Stale { .. } => "stale",
        DeniedBasisCapabilityKind::Inaccessible { .. } => "inaccessible",
        DeniedBasisCapabilityKind::PolicyMasked { .. } => "policy_masked",
        DeniedBasisCapabilityKind::TenantMismatched { .. } => "tenant_mismatched",
        DeniedBasisCapabilityKind::SchemaIncompatible { .. } => "schema_incompatible",
        DeniedBasisCapabilityKind::OperationIneligible { .. } => "operation_ineligible",
        DeniedBasisCapabilityKind::LowerRuntimeBindingMissing { .. } => {
            "lower_runtime_binding_missing"
        }
        DeniedBasisCapabilityKind::LowerRuntimeBindingMismatch { .. } => {
            "lower_runtime_binding_mismatch"
        }
        DeniedBasisCapabilityKind::LowerRuntimeCapabilityUnsupported { .. } => {
            "lower_runtime_capability_unsupported"
        }
        DeniedBasisCapabilityKind::HistoricalReplayUnsupported { .. } => {
            "historical_replay_unsupported"
        }
        DeniedBasisCapabilityKind::PreviewDrifted { .. } => "preview_drifted",
        DeniedBasisCapabilityKind::DurableOverclaim { .. } => "durable_overclaim",
    }
}

fn basis_intent_denial_kind_label(kind: &BasisIntentDenialKind) -> &'static str {
    match kind {
        BasisIntentDenialKind::MalformedIdentifier { .. } => "malformed_identifier",
        BasisIntentDenialKind::UnsupportedCompatibilityFamily { .. } => {
            "unsupported_compatibility_family"
        }
        BasisIntentDenialKind::UnsupportedFutureNeighbor { .. } => "unsupported_future_neighbor",
    }
}
