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
use crate::query_basis_lifecycle::{
    AdmittedBasisCapability, BasisCapabilityAdmission, BasisIntentDenial, DeniedBasisCapability,
    InspectionBasisCapability, LowerRuntimeBoundInspectionBasis, LowerRuntimeBoundObservationBasis,
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
            self.basis_lifecycle_support_digest(),
            self.handle_identity_digest(),
            ForgeQueryAuthorityLane::AuthoritativeTruth,
            format!(
                "retained admitted world basis `{}` is ready with query basis lifecycle support `{}`",
                self.domain_key(),
                self.basis_lifecycle_support_digest()
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
                    basis_shape_digest(self.admission()),
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
            self.scoped_digest(),
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
            self.scoped_digest(),
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
                    self.scoped_digest(),
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
                    self.binding_digest()
                );
                Ok(match admission {
                    BasisCapabilityAdmission::Admitted(admitted) => {
                        ForgeQueryRuntimeStateSnapshot::ready(
                            admitted.normalized_basis_intent_digest(),
                            self.binding_digest(),
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
                            advisory.normalized_basis_intent_digest(),
                            self.binding_digest(),
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
            self.normalized_basis_intent_digest(),
            self.failure_digest(),
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
            self.raw_basis_intent_digest(),
            self.failure_digest(),
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
    result_shape_digest: &str,
) -> ForgeQueryRuntimeStateSnapshot {
    match admission {
        BasisCapabilityAdmission::Admitted(admitted) => {
            snapshot_for_admitted_basis(admitted, label, result_shape_digest)
        }
        BasisCapabilityAdmission::Advisory(advisory) => ForgeQueryRuntimeStateSnapshot::deferred(
            ForgeQueryRuntimeStateKind::Pending,
            advisory.normalized_basis_intent_digest(),
            result_shape_digest,
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
    result_shape_digest: &str,
) -> ForgeQueryRuntimeStateSnapshot {
    ForgeQueryRuntimeStateSnapshot::ready(
        admitted.normalized_basis_intent_digest(),
        result_shape_digest,
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

fn basis_shape_digest(admission: &BasisCapabilityAdmission) -> &str {
    match admission {
        BasisCapabilityAdmission::Admitted(admitted) => admitted.capability_digest(),
        BasisCapabilityAdmission::Advisory(advisory) => advisory.advisory_digest(),
    }
}
