use super::super::super::policy::ResourceStaleAfterPolicyDeclaration;
use super::super::errors::ResourcePolicyResolutionError;
use super::super::identity::{
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicySelectionBasis,
};
use super::super::reference::ValidatedResourcePolicyReference;
use super::super::registration::built_in_resource_policy_registration;
use super::super::registration::ResourcePolicyRegistration;
use super::super::registry::FrozenResourcePolicyRegistry;

impl FrozenResourcePolicyRegistry {
    pub(in crate::data::resource::policy_registry) fn resolve_stale_after(
        &self,
        policy: &ResourceStaleAfterPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceStaleAfterPolicyDeclaration::Disabled => self.built_in_policy(
                ResourcePolicyKind::StaleAfter,
                "signal.resource.stale-after.disabled",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("stale-after:disabled"),
            )?,
            ResourceStaleAfterPolicyDeclaration::RuntimeStaleAfter { stale_after } => self
                .built_in_policy(
                    ResourcePolicyKind::StaleAfter,
                    "signal.resource.stale-after.runtime-stale-after",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(format!(
                        "stale-after:runtime-stale-after:{}",
                        stale_after.get()
                    )),
                )?,
            ResourceStaleAfterPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::StaleAfter, name)?
            }
        })
    }
}

pub(super) fn built_in_registrations() -> Vec<ResourcePolicyRegistration> {
    [
        (
            6,
            ResourcePolicyKind::StaleAfter,
            "signal.resource.stale-after.disabled",
            7,
        ),
        (
            7,
            ResourcePolicyKind::StaleAfter,
            "signal.resource.stale-after.runtime-stale-after",
            7,
        ),
    ]
    .into_iter()
    .map(|(id, kind, name, contract)| {
        built_in_resource_policy_registration(id, kind, name, contract)
    })
    .collect()
}
