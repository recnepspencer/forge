use std::collections::BTreeMap;

use crate::{
    WorthUiInstalledQueryDomain, WorthUiInstalledQueryView, WorthUiQueryViewDefinition,
    WorthUiQueryViewIdentity,
};

use super::{
    WorthUiQueryBindingRegistrationDenial, WorthUiQueryBindingRegistrationDenialKind,
    WorthUiQueryBindingSubsystem, WorthUiRuntimeQueryBinding,
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
    definitions: BTreeMap<WorthUiQueryViewIdentity, WorthUiQueryViewDefinition>,
}

impl WorthUiQueryBindingPlan {
    pub fn register_view(
        self,
        view: impl Into<WorthUiInstalledQueryView>,
    ) -> Result<Self, WorthUiQueryBindingRegistrationDenial> {
        let view = view.into();
        let (installed_domain, definition) = view.into_parts();
        match self {
            Self::QueryFree => {
                let mut definitions = BTreeMap::new();
                definitions.insert(definition.identity().clone(), definition);
                Ok(Self::Installed(WorthUiInstalledQueryBindingPlan {
                    installed_domain,
                    definitions,
                }))
            }
            Self::Installed(mut plan) => {
                if !plan
                    .installed_domain
                    .shares_authority_with(&installed_domain)
                {
                    return Err(WorthUiQueryBindingRegistrationDenial {
                        kind: WorthUiQueryBindingRegistrationDenialKind::ForeignInstalledDomain,
                        identity: definition.identity().clone(),
                    });
                }
                if plan.definitions.contains_key(definition.identity()) {
                    return Err(WorthUiQueryBindingRegistrationDenial {
                        kind: WorthUiQueryBindingRegistrationDenialKind::DuplicateViewIdentity,
                        identity: definition.identity().clone(),
                    });
                }
                plan.definitions
                    .insert(definition.identity().clone(), definition);
                Ok(Self::Installed(plan))
            }
        }
    }

    pub fn is_query_free(&self) -> bool {
        matches!(self, Self::QueryFree)
    }

    pub fn definitions(&self) -> Vec<&WorthUiQueryViewDefinition> {
        match self {
            Self::QueryFree => Vec::new(),
            Self::Installed(plan) => plan.definitions.values().collect(),
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
            Self::Installed(plan) => plan.definitions.get(identity).and_then(|definition| {
                (definition.shape() == shape).then(|| {
                    crate::WorthUiInstalledQueryBindingReference::new(
                        plan.installed_domain.clone(),
                        definition.clone(),
                    )
                })
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
                plan.installed_domain
                    .shares_authority_with(reference.installed_domain())
                    && plan.definitions.get(reference.definition().identity())
                        == Some(reference.definition())
            }
        }
    }

    pub fn activate(&self) -> WorthUiRuntimeQueryBinding {
        match self {
            Self::QueryFree => WorthUiRuntimeQueryBinding::QueryFree,
            Self::Installed(plan) => {
                WorthUiRuntimeQueryBinding::Installed(Box::new(WorthUiQueryBindingSubsystem::new(
                    plan.installed_domain.clone(),
                    plan.definitions.clone(),
                )))
            }
        }
    }
}
