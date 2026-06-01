use std::marker::PhantomData;

use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationLegalityContract,
    ForgeQueryDeclarationRelationalTruthContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryMixedAuthority, ForgeQueryNeighborhoodCapableGrouping,
    ForgeQueryRelationalTruthAuthority, ForgeQuerySignalCompatiblePosture,
};

use super::runtime::{relational_aspect_contract, relational_aspect_coverage, GeometryDomain};

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

            fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
                relational_aspect_contract()
            }

            fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
                relational_aspect_coverage()
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

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        relational_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MissingAspectFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for MissingAspectFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MissingAspectFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }

    fn relational_truth_contract() -> Option<ForgeQueryDeclarationRelationalTruthContract> {
        Some(
            ForgeQueryDeclarationRelationalTruthContract::authoritative_current_truth()
                .with_required_aspects(ForgeQueryDeclarationAspectContract::from_slices(
                    &["selection.relational.authority_slice"],
                    &[],
                    &[],
                    &[],
                    &[],
                )),
        )
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &[],
            &[],
            &[],
            &[],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(&["selection.active_face"], &[], &[])
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ConflictingAspectFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for ConflictingAspectFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ConflictingAspectFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }

    fn relational_truth_contract() -> Option<ForgeQueryDeclarationRelationalTruthContract> {
        Some(
            ForgeQueryDeclarationRelationalTruthContract::authoritative_current_truth()
                .with_required_aspects(ForgeQueryDeclarationAspectContract::from_slices(
                    &[],
                    &[],
                    &[],
                    &[],
                    &["selection.conflicting_preview"],
                )),
        )
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        relational_aspect_contract()
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_face", "selection.conflicting_preview"],
            &[],
            &["selection.conflicting_preview"],
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ExpandedAspectFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for ExpandedAspectFamily {
    type PrimaryAuthority = ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ExpandedAspectFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_only()
    }

    fn relational_truth_contract() -> Option<ForgeQueryDeclarationRelationalTruthContract> {
        Some(ForgeQueryDeclarationRelationalTruthContract::authoritative_current_truth())
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
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

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
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
    MissingAspectFamily,
    ConflictingAspectFamily,
    ExpandedAspectFamily,
);
