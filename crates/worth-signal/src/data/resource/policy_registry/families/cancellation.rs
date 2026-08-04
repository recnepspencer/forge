use super::super::super::policy::ResourceCancellationPolicyDeclaration;
use super::super::errors::ResourcePolicyResolutionError;
use super::super::identity::{
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicySelectionBasis,
};
use super::super::reference::ValidatedResourcePolicyReference;
use super::super::registration::built_in_resource_policy_registration;
use super::super::registration::ResourcePolicyRegistration;
use super::super::registry::FrozenResourcePolicyRegistry;

impl FrozenResourcePolicyRegistry {
    pub(in crate::data::resource::policy_registry) fn resolve_cancellation(
        &self,
        policy: &ResourceCancellationPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceCancellationPolicyDeclaration::RuntimeDenialOnly => self.built_in_policy(
                ResourcePolicyKind::Cancellation,
                "signal.resource.cancellation.runtime-denial-only",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("cancellation:runtime-denial-only"),
            )?,
            ResourceCancellationPolicyDeclaration::BestEffortHostSignalAndRuntimeDenial => self
                .built_in_policy(
                    ResourcePolicyKind::Cancellation,
                    "signal.resource.cancellation.best-effort-host-signal-and-runtime-denial",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new(
                        "cancellation:best-effort-host-signal-and-runtime-denial",
                    ),
                )?,
            ResourceCancellationPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Cancellation, name)?
            }
        })
    }
}

pub(super) fn built_in_registrations() -> Vec<ResourcePolicyRegistration> {
    [
        (
            4,
            ResourcePolicyKind::Cancellation,
            "signal.resource.cancellation.runtime-denial-only",
            3,
        ),
        (
            5,
            ResourcePolicyKind::Cancellation,
            "signal.resource.cancellation.best-effort-host-signal-and-runtime-denial",
            3,
        ),
    ]
    .into_iter()
    .map(|(id, kind, name, contract)| {
        built_in_resource_policy_registration(id, kind, name, contract)
    })
    .collect()
}
