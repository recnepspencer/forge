use super::super::super::policy::ResourceSupersessionPolicyDeclaration;
use super::super::errors::ResourcePolicyResolutionError;
use super::super::identity::{
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicySelectionBasis,
};
use super::super::reference::ValidatedResourcePolicyReference;
use super::super::registration::built_in_resource_policy_registration;
use super::super::registration::ResourcePolicyRegistration;
use super::super::registry::FrozenResourcePolicyRegistry;

impl FrozenResourcePolicyRegistry {
    pub(in crate::data::resource::policy_registry) fn resolve_supersession(
        &self,
        policy: &ResourceSupersessionPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceSupersessionPolicyDeclaration::NewGenerationSupersedesPrior => self
                .built_in_policy(
                    ResourcePolicyKind::Supersession,
                    "signal.resource.supersession.new-generation-supersedes-prior",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new("supersession:new-generation-supersedes-prior"),
                )?,
            ResourceSupersessionPolicyDeclaration::OverlappingGenerationRetainsOldHostWork => self
                .built_in_policy(
                    ResourcePolicyKind::Supersession,
                    "signal.resource.supersession.overlapping-generation-retains-old-host-work",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "supersession:overlapping-generation-retains-old-host-work",
                    ),
                )?,
            ResourceSupersessionPolicyDeclaration::OverlappingGenerationCancelsOldHostWork => self
                .built_in_policy(
                    ResourcePolicyKind::Supersession,
                    "signal.resource.supersession.overlapping-generation-cancels-old-host-work",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "supersession:overlapping-generation-cancels-old-host-work",
                    ),
                )?,
            ResourceSupersessionPolicyDeclaration::IntentEquivalentCoalescesToActive => self
                .built_in_policy(
                    ResourcePolicyKind::Supersession,
                    "signal.resource.supersession.intent-equivalent-coalesces-to-active",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("supersession:intent-equivalent-coalesces-to-active"),
                )?,
            ResourceSupersessionPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Supersession, name)?
            }
        })
    }
}

pub(super) fn built_in_registrations() -> Vec<ResourcePolicyRegistration> {
    [
        (
            8,
            ResourcePolicyKind::Supersession,
            "signal.resource.supersession.new-generation-supersedes-prior",
            1,
        ),
        (
            22,
            ResourcePolicyKind::Supersession,
            "signal.resource.supersession.overlapping-generation-retains-old-host-work",
            1,
        ),
        (
            23,
            ResourcePolicyKind::Supersession,
            "signal.resource.supersession.overlapping-generation-cancels-old-host-work",
            1,
        ),
        (
            24,
            ResourcePolicyKind::Supersession,
            "signal.resource.supersession.intent-equivalent-coalesces-to-active",
            1,
        ),
    ]
    .into_iter()
    .map(|(id, kind, name, contract)| {
        built_in_resource_policy_registration(id, kind, name, contract)
    })
    .collect()
}
