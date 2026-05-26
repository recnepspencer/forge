use std::marker::PhantomData;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade,
    ForgeQueryBridgeContinuationAuthority, ForgeQueryCapabilityFamily,
    ForgeQueryConfigSectionFamily, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationSignalCompatibilityContract, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryMixedAuthority,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQuerySignalCompatiblePosture,
    ForgeQuerySignalDeferredPosture, ForgeQuerySignalNotCompatiblePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str {
        "test.geometry.signal-compatibility"
    }

    fn display_name(&self) -> &'static str {
        "GeometrySignalCompatibilityDomain"
    }

    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[ForgeQueryCapabilityFamily::QueryComposition]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GeometryWorld(pub &'static str);

impl ForgeQueryDomainOperatingContext<GeometryDomain> for GeometryWorld {
    fn required_capability_families(&self) -> &'static [ForgeQueryCapabilityFamily] {
        &[
            ForgeQueryCapabilityFamily::HistoricalEvaluation,
            ForgeQueryCapabilityFamily::WorkflowOrchestration,
            ForgeQueryCapabilityFamily::PreviewSession,
            ForgeQueryCapabilityFamily::QueryComposition,
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
        format!("signal.compatibility.{}", self.0)
    }
}

macro_rules! define_family {
    ($name:ident, $authority:ty, $signal:ty, $route:expr, $contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name;

        impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = $authority;
            type SignalCompatibility = $signal;
            type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
                ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> ForgeQueryDeclarationRouteContract {
                $route
            }

            fn signal_compatibility_contract(
            ) -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
                $contract
            }
        }
    };
}

define_family!(
    RuntimeFamily,
    ForgeQueryBridgeContinuationAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    Some(ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution())
);
define_family!(
    HistoricalFamily,
    ForgeQueryBridgeContinuationAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    Some(ForgeQueryDeclarationSignalCompatibilityContract::historical_derived_execution())
);
define_family!(
    PreviewFamily,
    ForgeQueryBridgeContinuationAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    Some(ForgeQueryDeclarationSignalCompatibilityContract::preview_derived_execution())
);
define_family!(
    DeferredFamily,
    ForgeQueryBridgeContinuationAuthority,
    ForgeQuerySignalDeferredPosture,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    None
);
define_family!(
    IncompatibleFamily,
    ForgeQueryBridgeContinuationAuthority,
    ForgeQuerySignalNotCompatiblePosture,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    None
);
define_family!(
    MixedFamily,
    ForgeQueryMixedAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryDeclarationRouteContract::relational_and_bridge(),
    Some(ForgeQueryDeclarationSignalCompatibilityContract::preview_derived_execution())
);

#[derive(Clone, Eq, PartialEq)]
pub struct Input<F>(pub &'static str, pub PhantomData<F>);

impl<F> Input<F> {
    pub fn new(edge_ref: &'static str) -> Self {
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
    RuntimeFamily,
    HistoricalFamily,
    PreviewFamily,
    DeferredFamily,
    IncompatibleFamily,
    MixedFamily
);

pub fn handle(
    regime: &'static str,
) -> ForgeQueryAdmittedConfiguredDomainHandle<GeometryDomain, GeometryWorld> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(GeometryDomain)
        .with_operating_context(GeometryWorld(regime))
        .validate()
        .expect("signal compatibility world should validate")
        .admit()
        .expect("signal compatibility world should admit")
}
