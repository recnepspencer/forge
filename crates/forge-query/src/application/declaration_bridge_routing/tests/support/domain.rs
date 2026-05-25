use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationBridgeContinuationContract,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRouteContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryMixedAuthority,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQueryRelationalTruthAuthority,
    ForgeQuerySignalCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.bridge-routing"
    }

    fn display_name(&self) -> &'static str {
        "GeometryBridgeRoutingDomain"
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
            ForgeQueryCapabilityFamily::PreviewSession,
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
        format!("geometry.bridge-routing.{}", self.regime)
    }
}

macro_rules! define_family {
    ($name:ident, $route_contract:expr, $bridge_contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) struct $name;

        impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
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

            fn bridge_continuation_contract(
            ) -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
                Some($bridge_contract)
            }
        }
    };
}

define_family!(
    RuntimeRouteFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current()
);
define_family!(
    TruthViewCurrentFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::truth_view_current()
);
define_family!(
    TruthViewHistoricalFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::truth_view_historical()
);
define_family!(
    PreviewSessionFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::preview_session()
);
define_family!(
    PreviewPromotionFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::preview_promotion()
);
define_family!(
    SubscriptionPreparationFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::subscription_preparation()
);
define_family!(
    WritebackPreparationFamily,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    ForgeQueryDeclarationBridgeContinuationContract::writeback_preparation()
);
define_family!(
    SignalOnlyFamily,
    ForgeQueryDeclarationRouteContract::signal_only(),
    ForgeQueryDeclarationBridgeContinuationContract::runtime_route_current()
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

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(ForgeQueryDeclarationBridgeContinuationContract::preview_session())
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
    RuntimeRouteFamily,
    TruthViewCurrentFamily,
    TruthViewHistoricalFamily,
    PreviewSessionFamily,
    PreviewPromotionFamily,
    SubscriptionPreparationFamily,
    WritebackPreparationFamily,
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
        .expect("bridge-routing world should validate")
        .admit()
        .expect("bridge-routing world should admit")
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RelationalOnlyFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for RelationalOnlyFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "RelationalOnlyFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }
}

impl ForgeQueryDeclarationInput<GeometryDomain> for RoutingInput<RelationalOnlyFamily> {
    type Family = RelationalOnlyFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![ForgeQueryDeclarationCanonicalEntry::text(
            "edge_ref",
            self.edge_ref,
        )]
    }
}
