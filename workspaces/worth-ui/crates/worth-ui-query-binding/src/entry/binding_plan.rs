use std::collections::BTreeMap;

use crate::{
    WorthUiInstalledQueryBindingReference, WorthUiInstalledQueryView, WorthUiQueryViewDefinition,
    WorthUiQueryViewIdentity,
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
    references: BTreeMap<WorthUiQueryViewIdentity, WorthUiInstalledQueryBindingReference>,
}

impl WorthUiQueryBindingPlan {
    pub fn register_view(
        self,
        view: impl Into<WorthUiInstalledQueryView>,
    ) -> Result<Self, WorthUiQueryBindingRegistrationDenial> {
        let view = view.into();
        let (installed_domain, definition) = view.into_parts();
        let identity = definition.identity().clone();
        let reference = WorthUiInstalledQueryBindingReference::new(installed_domain, definition);
        match self {
            Self::QueryFree => {
                let mut references = BTreeMap::new();
                references.insert(identity, reference);
                Ok(Self::Installed(WorthUiInstalledQueryBindingPlan {
                    references,
                }))
            }
            Self::Installed(mut plan) => {
                if !plan
                    .references
                    .values()
                    .next()
                    .expect("an installed plan is created with its first reference")
                    .installed_domain()
                    .shares_authority_with(reference.installed_domain())
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
                WorthUiInstalledDownstreamQueryState::new(plan.references.clone()),
            )),
        }
    }
}
