use super::super::super::policy::ResourceOutputContinuityPolicyDeclaration;
use super::super::errors::ResourcePolicyResolutionError;
use super::super::identity::{
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicySelectionBasis,
};
use super::super::reference::ValidatedResourcePolicyReference;
use super::super::registration::built_in_resource_policy_registration;
use super::super::registration::ResourcePolicyRegistration;
use super::super::registry::FrozenResourcePolicyRegistry;

impl FrozenResourcePolicyRegistry {
    pub(in crate::data::resource::policy_registry) fn resolve_output_continuity(
        &self,
        policy: &ResourceOutputContinuityPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceOutputContinuityPolicyDeclaration::PreserveLifecycleOutputSeparation => self
                .built_in_policy(
                    ResourcePolicyKind::OutputContinuity,
                    "signal.resource.output-continuity.preserve-lifecycle-output-separation",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new(
                        "output-continuity:preserve-lifecycle-output-separation",
                    ),
                )?,
            ResourceOutputContinuityPolicyDeclaration::HideWhilePending => self.built_in_policy(
                ResourcePolicyKind::OutputContinuity,
                "signal.resource.output-continuity.hide-while-pending",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("output-continuity:hide-while-pending"),
            )?,
            ResourceOutputContinuityPolicyDeclaration::HideAfterRejection => self.built_in_policy(
                ResourcePolicyKind::OutputContinuity,
                "signal.resource.output-continuity.hide-after-rejection",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("output-continuity:hide-after-rejection"),
            )?,
            ResourceOutputContinuityPolicyDeclaration::HideAfterTimeout => self.built_in_policy(
                ResourcePolicyKind::OutputContinuity,
                "signal.resource.output-continuity.hide-after-timeout",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("output-continuity:hide-after-timeout"),
            )?,
            ResourceOutputContinuityPolicyDeclaration::HideAfterCancellation => self
                .built_in_policy(
                    ResourcePolicyKind::OutputContinuity,
                    "signal.resource.output-continuity.hide-after-cancellation",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("output-continuity:hide-after-cancellation"),
                )?,
            ResourceOutputContinuityPolicyDeclaration::HideAfterSupersession => self
                .built_in_policy(
                    ResourcePolicyKind::OutputContinuity,
                    "signal.resource.output-continuity.hide-after-supersession",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("output-continuity:hide-after-supersession"),
                )?,
            ResourceOutputContinuityPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::OutputContinuity, name)?
            }
        })
    }
}

pub(super) fn built_in_registrations() -> Vec<ResourcePolicyRegistration> {
    [
        (
            12,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.preserve-lifecycle-output-separation",
            7,
        ),
        (
            38,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.hide-while-pending",
            7,
        ),
        (
            45,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.hide-after-rejection",
            7,
        ),
        (
            42,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.hide-after-timeout",
            7,
        ),
        (
            43,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.hide-after-cancellation",
            7,
        ),
        (
            44,
            ResourcePolicyKind::OutputContinuity,
            "signal.resource.output-continuity.hide-after-supersession",
            7,
        ),
    ]
    .into_iter()
    .map(|(id, kind, name, contract)| {
        built_in_resource_policy_registration(id, kind, name, contract)
    })
    .collect()
}
