use super::super::super::policy::ResourceObservationPolicyDeclaration;
use super::super::errors::ResourcePolicyResolutionError;
use super::super::identity::{
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicySelectionBasis,
};
use super::super::reference::ValidatedResourcePolicyReference;
use super::super::registration::built_in_resource_policy_registration;
use super::super::registration::ResourcePolicyRegistration;
use super::super::registry::FrozenResourcePolicyRegistry;

impl FrozenResourcePolicyRegistry {
    pub(in crate::data::resource::policy_registry) fn resolve_observation(
        &self,
        policy: &ResourceObservationPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceObservationPolicyDeclaration::LifecycleOnly => self.built_in_policy(
                ResourcePolicyKind::Observation,
                "signal.resource.observation.lifecycle-only",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("observation:lifecycle-only"),
            )?,
            ResourceObservationPolicyDeclaration::LifecycleAndOutput => self.built_in_policy(
                ResourcePolicyKind::Observation,
                "signal.resource.observation.lifecycle-and-output",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("observation:lifecycle-and-output"),
            )?,
            ResourceObservationPolicyDeclaration::LifecycleOutputAndDeniedCompletion => self
                .built_in_policy(
                    ResourcePolicyKind::Observation,
                    "signal.resource.observation.lifecycle-output-and-denied-completion",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "observation:lifecycle-output-and-denied-completion",
                    ),
                )?,
            ResourceObservationPolicyDeclaration::LifecycleOutputAndRetrySchedule => self
                .built_in_policy(
                    ResourcePolicyKind::Observation,
                    "signal.resource.observation.lifecycle-output-and-retry-schedule",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "observation:lifecycle-output-and-retry-schedule",
                    ),
                )?,
            ResourceObservationPolicyDeclaration::LifecycleOutputAndDeniedCompletionAndRetrySchedule => self
                .built_in_policy(
                    ResourcePolicyKind::Observation,
                    "signal.resource.observation.lifecycle-output-and-denied-completion-and-retry-schedule",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "observation:lifecycle-output-and-denied-completion-and-retry-schedule",
                    ),
                )?,
            ResourceObservationPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Observation, name)?
            }
        })
    }
}

pub(super) fn built_in_registrations() -> Vec<ResourcePolicyRegistration> {
    [
        (
            10,
            ResourcePolicyKind::Observation,
            "signal.resource.observation.lifecycle-only",
            7,
        ),
        (
            11,
            ResourcePolicyKind::Observation,
            "signal.resource.observation.lifecycle-and-output",
            7,
        ),
        (
            39,
            ResourcePolicyKind::Observation,
            "signal.resource.observation.lifecycle-output-and-denied-completion",
            7,
        ),
        (
            40,
            ResourcePolicyKind::Observation,
            "signal.resource.observation.lifecycle-output-and-retry-schedule",
            7,
        ),
        (
            41,
            ResourcePolicyKind::Observation,
            "signal.resource.observation.lifecycle-output-and-denied-completion-and-retry-schedule",
            7,
        ),
    ]
    .into_iter()
    .map(|(id, kind, name, contract)| {
        built_in_resource_policy_registration(id, kind, name, contract)
    })
    .collect()
}
