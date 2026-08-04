use super::super::super::policy::ResourceRevalidationPolicyDeclaration;
use super::super::errors::ResourcePolicyResolutionError;
use super::super::identity::{
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicySelectionBasis,
};
use super::super::reference::ValidatedResourcePolicyReference;
use super::super::registration::built_in_resource_policy_registration;
use super::super::registration::ResourcePolicyRegistration;
use super::super::registry::FrozenResourcePolicyRegistry;

impl FrozenResourcePolicyRegistry {
    pub(in crate::data::resource::policy_registry) fn resolve_revalidation(
        &self,
        policy: &ResourceRevalidationPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceRevalidationPolicyDeclaration::ExplicitIntentOnly => self.built_in_policy(
                ResourcePolicyKind::Revalidation,
                "signal.resource.revalidation.explicit-intent-only",
                ResourcePolicySelectionBasis::BuiltInDefault,
                ResourcePolicyDigest::new("revalidation:explicit-intent-only"),
            )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("revalidation:explicit-or-active-handle-forced"),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrStaleAfterFulfilled => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-stale-after-fulfilled",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-stale-after-fulfilled",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrStaleAfterFulfilledOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-stale-after-fulfilled-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-stale-after-fulfilled-or-active-handle-forced",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChange => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-dependency-change",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("revalidation:explicit-or-dependency-change"),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-dependency-change-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-dependency-change-or-active-handle-forced",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrObserverDemand => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-observer-demand",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("revalidation:explicit-or-observer-demand"),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrObserverDemandOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-observer-demand-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-observer-demand-or-active-handle-forced",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrObserverDemand => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-dependency-change-or-observer-demand",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-dependency-change-or-observer-demand",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrDependencyChangeOrObserverDemandOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-dependency-change-or-observer-demand-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-dependency-change-or-observer-demand-or-active-handle-forced",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrTerminalState => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-terminal-state",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("revalidation:explicit-or-terminal-state"),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrTerminalStateOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-terminal-state-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-terminal-state-or-active-handle-forced",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrFulfilledLifecycle => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-fulfilled-lifecycle",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("revalidation:explicit-or-fulfilled-lifecycle"),
                )?,
            ResourceRevalidationPolicyDeclaration::ExplicitOrFulfilledLifecycleOrActiveHandleForced => self
                .built_in_policy(
                    ResourcePolicyKind::Revalidation,
                    "signal.resource.revalidation.explicit-or-fulfilled-lifecycle-or-active-handle-forced",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "revalidation:explicit-or-fulfilled-lifecycle-or-active-handle-forced",
                    ),
                )?,
            ResourceRevalidationPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Revalidation, name)?
            }
        })
    }
}

pub(super) fn built_in_registrations() -> Vec<ResourcePolicyRegistration> {
    [
        (9, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-intent-only", 7),
        (25, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-active-handle-forced", 7),
        (26, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-stale-after-fulfilled", 7),
        (27, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-stale-after-fulfilled-or-active-handle-forced", 7),
        (28, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-dependency-change", 7),
        (29, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-dependency-change-or-active-handle-forced", 7),
        (30, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-observer-demand", 7),
        (31, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-observer-demand-or-active-handle-forced", 7),
        (36, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-dependency-change-or-observer-demand", 7),
        (37, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-dependency-change-or-observer-demand-or-active-handle-forced", 7),
        (32, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-terminal-state", 7),
        (33, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-terminal-state-or-active-handle-forced", 7),
        (34, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-fulfilled-lifecycle", 7),
        (35, ResourcePolicyKind::Revalidation, "signal.resource.revalidation.explicit-or-fulfilled-lifecycle-or-active-handle-forced", 7),
    ]
    .into_iter()
    .map(|(id, kind, name, contract)| {
        built_in_resource_policy_registration(id, kind, name, contract)
    })
    .collect()
}
