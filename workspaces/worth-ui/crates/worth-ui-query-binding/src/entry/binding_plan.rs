use std::collections::BTreeMap;

use crate::{
    UiCollectionProjectionRegistration, UiScalarProjectionRegistration,
    WorthUiInstalledQueryBindingReference, WorthUiInstalledQueryDomain, WorthUiInstalledQueryView,
    WorthUiQueryViewDefinition, WorthUiQueryViewIdentity,
};

use super::{
    WorthUiInstalledDownstreamQueryState, WorthUiQueryBindingRegistrationDenial,
    WorthUiQueryBindingRegistrationDenialKind, WorthUiRuntimeQueryBinding,
};

/// Stable app-owned binding plan. Query-free and installed Query posture are
/// explicit states; runtime authority never hides behind an optional field.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum WorthUiQueryBindingPlan {
    #[default]
    QueryFree,
    Installed(WorthUiInstalledQueryBindingPlan),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiInstalledQueryBindingPlan {
    installed_domain: WorthUiInstalledQueryDomain,
    references: BTreeMap<WorthUiQueryViewIdentity, WorthUiInstalledQueryBindingReference>,
    scalar_projections: BTreeMap<WorthUiQueryViewIdentity, UiScalarProjectionRegistration>,
    collection_projections: BTreeMap<WorthUiQueryViewIdentity, UiCollectionProjectionRegistration>,
}

impl WorthUiQueryBindingPlan {
    pub fn register_view(
        self,
        view: impl Into<WorthUiInstalledQueryView>,
    ) -> Result<Self, WorthUiQueryBindingRegistrationDenial> {
        let view = view.into();
        let (installed_domain, definition) = view.into_parts();
        let identity = definition.identity().clone();
        let reference =
            WorthUiInstalledQueryBindingReference::new(installed_domain.clone(), definition);
        match self {
            Self::QueryFree => {
                let mut references = BTreeMap::new();
                references.insert(identity, reference);
                Ok(Self::Installed(WorthUiInstalledQueryBindingPlan {
                    installed_domain,
                    references,
                    scalar_projections: BTreeMap::new(),
                    collection_projections: BTreeMap::new(),
                }))
            }
            Self::Installed(mut plan) => {
                if !plan
                    .installed_domain
                    .shares_authority_with(&installed_domain)
                {
                    return Err(WorthUiQueryBindingRegistrationDenial {
                        kind: WorthUiQueryBindingRegistrationDenialKind::ForeignInstalledDomain,
                        identity,
                    });
                }
                if plan.references.contains_key(&identity) {
                    return Err(WorthUiQueryBindingRegistrationDenial {
                        kind: WorthUiQueryBindingRegistrationDenialKind::DuplicateViewIdentity,
                        identity,
                    });
                }
                plan.references.insert(identity, reference);
                Ok(Self::Installed(plan))
            }
        }
    }

    pub fn register_scalar_projection(
        self,
        registration: UiScalarProjectionRegistration,
    ) -> Result<Self, WorthUiQueryBindingRegistrationDenial> {
        let (installed_domain, identity) = registration.view().clone().into_parts();
        self.register_projection(
            installed_domain,
            identity,
            ProjectionRegistration::Scalar(registration),
        )
    }

    pub fn register_collection_projection(
        self,
        registration: UiCollectionProjectionRegistration,
    ) -> Result<Self, WorthUiQueryBindingRegistrationDenial> {
        let (installed_domain, identity) = registration.view().clone().into_parts();
        self.register_projection(
            installed_domain,
            identity,
            ProjectionRegistration::Collection(registration),
        )
    }

    pub fn is_query_free(&self) -> bool {
        matches!(self, Self::QueryFree)
    }

    pub fn definitions(&self) -> Vec<&WorthUiQueryViewDefinition> {
        match self {
            Self::QueryFree => Vec::new(),
            Self::Installed(plan) => plan
                .references
                .values()
                .map(WorthUiInstalledQueryBindingReference::definition)
                .collect(),
        }
    }

    pub fn scalar_projection_registration(
        &self,
        identity: &WorthUiQueryViewIdentity,
    ) -> Option<&UiScalarProjectionRegistration> {
        match self {
            Self::QueryFree => None,
            Self::Installed(plan) => plan.scalar_projections.get(identity),
        }
    }

    pub fn collection_projection_registration(
        &self,
        identity: &WorthUiQueryViewIdentity,
    ) -> Option<&UiCollectionProjectionRegistration> {
        match self {
            Self::QueryFree => None,
            Self::Installed(plan) => plan.collection_projections.get(identity),
        }
    }

    /// Resolve one compact lowering reference from this exact installed plan.
    pub fn resolve_definition(
        &self,
        identity: &WorthUiQueryViewIdentity,
        shape: crate::WorthUiQueryViewShape,
    ) -> Option<crate::WorthUiInstalledQueryBindingReference> {
        match self {
            Self::QueryFree => None,
            Self::Installed(plan) => plan.references.get(identity).and_then(|reference| {
                (reference.definition().shape() == shape).then(|| reference.clone())
            }),
        }
    }

    /// Verify that a compact reference still belongs to this exact installed
    /// plan rather than a semantically equal foreign Query runtime.
    pub fn admits_reference(
        &self,
        reference: &crate::WorthUiInstalledQueryBindingReference,
    ) -> bool {
        match self {
            Self::QueryFree => false,
            Self::Installed(plan) => {
                plan.references.get(reference.definition().identity()) == Some(reference)
            }
        }
    }

    /// Prepare UI-owned downstream fact retention. This does not create a
    /// Query execution root; operation attempts enter Query through the
    /// operating-world gateway.
    pub fn prepare_downstream_state(&self) -> WorthUiRuntimeQueryBinding {
        match self {
            Self::QueryFree => WorthUiRuntimeQueryBinding::QueryFree,
            Self::Installed(plan) => WorthUiRuntimeQueryBinding::Installed(Box::new(
                WorthUiInstalledDownstreamQueryState::new(
                    plan.references.clone(),
                    plan.scalar_projections.clone(),
                    plan.collection_projections.clone(),
                ),
            )),
        }
    }

    fn register_projection(
        self,
        installed_domain: WorthUiInstalledQueryDomain,
        identity: WorthUiQueryViewIdentity,
        registration: ProjectionRegistration,
    ) -> Result<Self, WorthUiQueryBindingRegistrationDenial> {
        let mut plan = match self {
            Self::QueryFree => WorthUiInstalledQueryBindingPlan {
                installed_domain,
                references: BTreeMap::new(),
                scalar_projections: BTreeMap::new(),
                collection_projections: BTreeMap::new(),
            },
            Self::Installed(plan) => {
                if !plan
                    .installed_domain
                    .shares_authority_with(&installed_domain)
                {
                    return Err(WorthUiQueryBindingRegistrationDenial {
                        kind: WorthUiQueryBindingRegistrationDenialKind::ForeignInstalledDomain,
                        identity,
                    });
                }
                plan
            }
        };
        if plan.scalar_projections.contains_key(&identity)
            || plan.collection_projections.contains_key(&identity)
        {
            return Err(WorthUiQueryBindingRegistrationDenial {
                kind: WorthUiQueryBindingRegistrationDenialKind::DuplicateProjectionIdentity,
                identity,
            });
        }
        match registration {
            ProjectionRegistration::Scalar(registration) => {
                plan.scalar_projections.insert(identity, registration);
            }
            ProjectionRegistration::Collection(registration) => {
                plan.collection_projections.insert(identity, registration);
            }
        }
        Ok(Self::Installed(plan))
    }
}

enum ProjectionRegistration {
    Scalar(UiScalarProjectionRegistration),
    Collection(UiCollectionProjectionRegistration),
}

#[cfg(test)]
#[path = "binding_plan_tests.rs"]
mod tests;
