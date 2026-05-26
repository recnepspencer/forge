use std::marker::PhantomData;

use crate::application::{
    ForgeQueryApplicationFacade, ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily,
    ForgeQueryDeclarationBridgeContinuationContract, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
    ForgeQuerySignalNotCompatiblePosture, ForgeQuerySingleOnlyGrouping,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.entry-orchestration"
    }
    fn display_name(&self) -> &'static str {
        "GeometryEntryOrchestrationDomain"
    }
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CollaborativeWorld(pub &'static str);

impl ForgeQueryDomainOperatingContext<GeometryDomain> for CollaborativeWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::QueryComposition,
            ForgeQueryCapabilityFamily::LiveQuery,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
            ForgeQueryConfigSectionFamily::RuntimeBridge,
            ForgeQueryConfigSectionFamily::Signal,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("entry.orchestration.{}", self.0)
    }
}

macro_rules! define_family {
    ($name:ident, $authority:ty, $signal:ty, $grouped:ty, $route:expr, $progression:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(super) struct $name;

        impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = $authority;
            type SignalCompatibility = $signal;
            type GroupedPosture = $grouped;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }
            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }
            fn route_contract() -> ForgeQueryDeclarationRouteContract {
                $route
            }
            fn progression_contract(
                _handle_identity_digest: &str,
                operating_context_identity_digest: &str,
            ) -> ForgeQueryDeclarationProgressionContract {
                let _ = operating_context_identity_digest;
                $progression
            }
            fn bridge_continuation_contract(
            ) -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
                None
            }
        }
    };
}

define_family!(
    AdmittedFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture,
    ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryDeclarationRouteContract::relational_only(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);
define_family!(
    DeferredFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
    ForgeQueryDeclarationRouteContract::relational_only(),
    ForgeQueryDeclarationProgressionContract::deferred_support()
);
define_family!(
    DeniedFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
    ForgeQueryDeclarationRouteContract::relational_only(),
    ForgeQueryDeclarationProgressionContract::denied_boundary()
);
define_family!(
    StaleFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
    ForgeQueryDeclarationRouteContract::relational_only(),
    ForgeQueryDeclarationProgressionContract::stale_readable()
);
define_family!(
    FailedFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
    ForgeQueryDeclarationRouteContract::relational_only(),
    ForgeQueryDeclarationProgressionContract::failed_transition()
);
define_family!(
    ExplicitIntentFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
    ForgeQueryDeclarationRouteContract::required_relational_intent(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);
define_family!(
    DeferredRouteFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
    ForgeQueryDeclarationRouteContract::deferred_auto(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);
define_family!(
    UnsupportedReceiptFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
    ForgeQueryDeclarationRouteContract::signal_only(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);
define_family!(
    ExpensiveAutomationFamily,
    ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalNotCompatiblePosture,
    ForgeQuerySingleOnlyGrouping,
    ForgeQueryDeclarationRouteContract::expensive_by_default_for_tests(),
    ForgeQueryDeclarationProgressionContract::admitted_current()
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct WorldSensitiveFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for WorldSensitiveFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = ForgeQuerySingleOnlyGrouping;
    fn semantic_family_key() -> &'static str {
        "WorldSensitiveFamily"
    }
    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
    fn progression_contract(
        _handle_identity_digest: &str,
        operating_context_identity_digest: &str,
    ) -> ForgeQueryDeclarationProgressionContract {
        if operating_context_identity_digest.contains("restricted") {
            ForgeQueryDeclarationProgressionContract::rebind_required()
        } else {
            ForgeQueryDeclarationProgressionContract::admitted_current()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Input<F>(pub &'static str, pub PhantomData<F>);

impl<F> Input<F> {
    pub(super) fn new(edge_ref: &'static str) -> Self {
        Self(edge_ref, PhantomData)
    }
}

macro_rules! impl_input {
    ($($family:ty),+ $(,)?) => {$(
        impl ForgeQueryDeclarationInput<GeometryDomain> for Input<$family> {
            type Family = $family;
            fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.0)]
            }
        }
    )+};
}

impl_input!(
    AdmittedFamily,
    DeferredFamily,
    DeniedFamily,
    StaleFamily,
    FailedFamily,
    ExplicitIntentFamily,
    DeferredRouteFamily,
    UnsupportedReceiptFamily,
    ExpensiveAutomationFamily,
    WorldSensitiveFamily
);

pub(super) fn admitted_handle(
    regime: &'static str,
) -> crate::application::ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, CollaborativeWorld>
{
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(CollaborativeWorld(regime))
        .validate()
        .expect("world should validate")
        .admit()
        .expect("world should admit")
}
