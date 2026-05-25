use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryCapabilityFamily, ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRelationalTruthContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryMixedAuthority,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.relational-routing"
    }

    fn display_name(&self) -> &'static str {
        "GeometryRelationalRoutingDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GeometryWorld {
    regime: &'static str,
}

impl GeometryWorld {
    pub(crate) fn named(regime: &'static str) -> Self {
        Self { regime }
    }
}

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
        ]
    }

    fn required_config_sections(&self) -> &'static [ForgeQueryConfigSectionFamily] {
        &[
            ForgeQueryConfigSectionFamily::Query,
            ForgeQueryConfigSectionFamily::Relational,
        ]
    }

    fn context_identity_digest(&self) -> String {
        format!("geometry.relational-routing.{}", self.regime)
    }
}

macro_rules! define_family {
    ($name:ident, $route_contract:expr, $relational_contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) struct $name;

        impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
            type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
            type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> ForgeQueryDeclarationRouteContract {
                $route_contract
            }

            fn relational_truth_contract() -> Option<ForgeQueryDeclarationRelationalTruthContract> {
                Some($relational_contract)
            }
        }
    };
}

define_family!(
    RuntimeFamily,
    ForgeQueryDeclarationRouteContract::relational_only(),
    ForgeQueryDeclarationRelationalTruthContract::authoritative_current_truth()
);
define_family!(
    GroupedFamily,
    ForgeQueryDeclarationRouteContract::relational_only(),
    ForgeQueryDeclarationRelationalTruthContract::grouped_truth()
);
define_family!(
    HistoryFamily,
    ForgeQueryDeclarationRouteContract::relational_only(),
    ForgeQueryDeclarationRelationalTruthContract::historical_truth()
);
define_family!(
    StrategyFamily,
    ForgeQueryDeclarationRouteContract::relational_only(),
    ForgeQueryDeclarationRelationalTruthContract::strategy_truth()
);
define_family!(
    BridgeSourceFamily,
    ForgeQueryDeclarationRouteContract::relational_only(),
    ForgeQueryDeclarationRelationalTruthContract::bridge_source_current_truth()
);
define_family!(
    MixedFamily,
    ForgeQueryDeclarationRouteContract::relational_and_bridge(),
    ForgeQueryDeclarationRelationalTruthContract::grouped_truth()
);
define_family!(
    SignalOnlyFamily,
    ForgeQueryDeclarationRouteContract::signal_only(),
    ForgeQueryDeclarationRelationalTruthContract::authoritative_current_truth()
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MixedAuthorityFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for MixedAuthorityFamily {
    type PrimaryAuthority = ForgeQueryMixedAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MixedAuthorityFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_and_bridge()
    }

    fn relational_truth_contract() -> Option<ForgeQueryDeclarationRelationalTruthContract> {
        Some(ForgeQueryDeclarationRelationalTruthContract::grouped_truth())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RoutingInput<F> {
    edge_ref: &'static str,
    _marker: PhantomData<F>,
}

impl<F> RoutingInput<F> {
    pub(crate) fn new(edge_ref: &'static str) -> Self {
        Self {
            edge_ref,
            _marker: PhantomData,
        }
    }
}

macro_rules! impl_declaration_input {
    ($($family:ty),+ $(,)?) => {
        $(
            impl ForgeQueryDeclarationInput<GeometryDomain> for RoutingInput<$family> {
                type Family = $family;

                fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
                    vec![ForgeQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
                }
            }
        )+
    };
}

impl_declaration_input!(
    RuntimeFamily,
    GroupedFamily,
    HistoryFamily,
    StrategyFamily,
    BridgeSourceFamily,
    MixedFamily,
    MixedAuthorityFamily,
    SignalOnlyFamily,
);

pub(crate) fn admitted_handle(
    regime: &'static str,
) -> ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld::named(regime))
        .validate()
        .expect("relational-routing world should validate")
        .admit()
        .expect("relational-routing world should admit")
}
