use std::marker::PhantomData;

use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationRelationalTruthContract, WorthQueryDeclarationRouteContract,
    WorthQueryMixedAuthority, WorthQueryNeighborhoodCapableGrouping,
    WorthQueryRelationalTruthAuthority, WorthQuerySignalCompatiblePosture,
};

use super::runtime::{relational_aspect_contract, relational_aspect_coverage, GeometryDomain};

macro_rules! define_family {
    ($name:ident, $route_contract:expr, $relational_contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) struct $name;

        impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
            type SignalCompatibility = WorthQuerySignalCompatiblePosture;
            type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn legality_contract() -> WorthQueryDeclarationLegalityContract {
                WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> WorthQueryDeclarationRouteContract {
                $route_contract
            }

            fn relational_truth_contract() -> Option<WorthQueryDeclarationRelationalTruthContract> {
                Some($relational_contract)
            }

            fn aspect_contract() -> WorthQueryDeclarationAspectContract {
                relational_aspect_contract()
            }

            fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
                relational_aspect_coverage()
            }
        }
    };
}

define_family!(
    RuntimeFamily,
    WorthQueryDeclarationRouteContract::relational_only(),
    WorthQueryDeclarationRelationalTruthContract::authoritative_current_truth()
);
define_family!(
    GroupedFamily,
    WorthQueryDeclarationRouteContract::relational_only(),
    WorthQueryDeclarationRelationalTruthContract::grouped_truth()
);
define_family!(
    HistoryFamily,
    WorthQueryDeclarationRouteContract::relational_only(),
    WorthQueryDeclarationRelationalTruthContract::historical_truth()
);
define_family!(
    StrategyFamily,
    WorthQueryDeclarationRouteContract::relational_only(),
    WorthQueryDeclarationRelationalTruthContract::strategy_truth()
);
define_family!(
    BridgeSourceFamily,
    WorthQueryDeclarationRouteContract::relational_only(),
    WorthQueryDeclarationRelationalTruthContract::bridge_source_current_truth()
);
define_family!(
    MixedFamily,
    WorthQueryDeclarationRouteContract::relational_and_bridge(),
    WorthQueryDeclarationRelationalTruthContract::grouped_truth()
);
define_family!(
    SignalOnlyFamily,
    WorthQueryDeclarationRouteContract::signal_only(),
    WorthQueryDeclarationRelationalTruthContract::authoritative_current_truth()
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MixedAuthorityFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for MixedAuthorityFamily {
    type PrimaryAuthority = WorthQueryMixedAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MixedAuthorityFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_and_bridge()
    }

    fn relational_truth_contract() -> Option<WorthQueryDeclarationRelationalTruthContract> {
        Some(WorthQueryDeclarationRelationalTruthContract::grouped_truth())
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        relational_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MissingAspectFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for MissingAspectFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MissingAspectFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }

    fn relational_truth_contract() -> Option<WorthQueryDeclarationRelationalTruthContract> {
        Some(
            WorthQueryDeclarationRelationalTruthContract::authoritative_current_truth()
                .with_required_aspects(WorthQueryDeclarationAspectContract::from_slices(
                    &["selection.relational.authority_slice"],
                    &[],
                    &[],
                    &[],
                    &[],
                )),
        )
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(&["selection.active_face"], &[], &[])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ConflictingAspectFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for ConflictingAspectFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ConflictingAspectFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }

    fn relational_truth_contract() -> Option<WorthQueryDeclarationRelationalTruthContract> {
        Some(
            WorthQueryDeclarationRelationalTruthContract::authoritative_current_truth()
                .with_required_aspects(WorthQueryDeclarationAspectContract::from_slices(
                    &[],
                    &[],
                    &[],
                    &[],
                    &["selection.conflicting_preview"],
                )),
        )
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        relational_aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_face", "selection.conflicting_preview"],
            &[],
            &["selection.conflicting_preview"],
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ExpandedAspectFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for ExpandedAspectFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ExpandedAspectFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }

    fn relational_truth_contract() -> Option<WorthQueryDeclarationRelationalTruthContract> {
        Some(WorthQueryDeclarationRelationalTruthContract::authoritative_current_truth())
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &[
                "selection.active_face",
                "selection.neighborhood.local_topology",
            ],
            &["selection.material_edit", "selection.face_boundary"],
            &["selection.preview.surface"],
            &["selection.private_authority"],
            &["selection.conflicting_preview"],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.active_face",
                "selection.neighborhood.local_topology",
                "selection.material_edit",
                "selection.face_boundary",
                "selection.preview.surface",
            ],
            &[],
            &[],
        )
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
            impl WorthQueryDeclarationInput<GeometryDomain> for RoutingInput<$family> {
                type Family = $family;

                fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                    vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", self.edge_ref)]
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
    MissingAspectFamily,
    ConflictingAspectFamily,
    ExpandedAspectFamily,
);
