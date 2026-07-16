use std::collections::BTreeMap;

use crate::{
    WorthUiInstalledQueryDomain, WorthUiInstalledQueryView,
    WorthUiQueryMeasurementFactSettlement, WorthUiQueryMeasurementFactSettlementDenial,
    WorthUiQueryProjectionOutcome,
    WorthUiQueryViewDefinition, WorthUiQueryViewIdentity,
};
use crate::prerequisites::WorthUiQueryAllocationSourceAuthority;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryBindingRegistrationDenialKind {
    ForeignInstalledDomain,
    DuplicateViewIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingRegistrationDenial {
    kind: WorthUiQueryBindingRegistrationDenialKind,
    identity: WorthUiQueryViewIdentity,
}

impl WorthUiQueryBindingRegistrationDenial {
    pub fn kind(&self) -> WorthUiQueryBindingRegistrationDenialKind {
        self.kind
    }

    pub fn identity(&self) -> &WorthUiQueryViewIdentity {
        &self.identity
    }
}

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
        view: WorthUiInstalledQueryView,
    ) -> Result<Self, WorthUiQueryBindingRegistrationDenial> {
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
                if !plan.installed_domain.shares_authority_with(&installed_domain) {
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

    pub fn activate(&self) -> WorthUiRuntimeQueryBinding {
        match self {
            Self::QueryFree => WorthUiRuntimeQueryBinding::QueryFree,
            Self::Installed(plan) => WorthUiRuntimeQueryBinding::Installed(
                WorthUiQueryBindingSubsystem {
                    installed_domain: plan.installed_domain.clone(),
                    definitions: plan.definitions.clone(),
                    allocation_source_authority: Default::default(),
                },
            ),
        }
    }
}

/// Runtime binding state. Only the installed variant can consume a projection.
#[derive(Debug)]
pub enum WorthUiRuntimeQueryBinding {
    QueryFree,
    Installed(WorthUiQueryBindingSubsystem),
}

#[derive(Debug)]
pub struct WorthUiQueryBindingSubsystem {
    installed_domain: WorthUiInstalledQueryDomain,
    definitions: BTreeMap<WorthUiQueryViewIdentity, WorthUiQueryViewDefinition>,
    allocation_source_authority: WorthUiQueryAllocationSourceAuthority,
}

impl WorthUiRuntimeQueryBinding {
    pub fn admit(
        &mut self,
        outcome: WorthUiQueryProjectionOutcome,
    ) -> Result<WorthUiQueryMeasurementFactSettlement, WorthUiQueryMeasurementFactSettlementDenial>
    {
        match self {
            Self::QueryFree => Err(WorthUiQueryMeasurementFactSettlementDenial::QueryNotInstalled),
            Self::Installed(binding) => binding.admit(outcome),
        }
    }
}

impl WorthUiQueryBindingSubsystem {
    fn admit(
        &mut self,
        outcome: WorthUiQueryProjectionOutcome,
    ) -> Result<WorthUiQueryMeasurementFactSettlement, WorthUiQueryMeasurementFactSettlementDenial>
    {
        let (definition, outcome, installed_execution) = outcome.into_parts();
        if installed_execution.installed_authority()
            != &self.installed_domain.handle().authority_witness()
        {
            return Err(
                WorthUiQueryMeasurementFactSettlementDenial::InstalledAuthorityMismatch,
            );
        }
        if self.definitions.get(definition.identity()) != Some(&definition) {
            return Err(WorthUiQueryMeasurementFactSettlementDenial::UnregisteredView);
        }
        self.allocation_source_authority
            .admit(definition, outcome, installed_execution)
    }
}
