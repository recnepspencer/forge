use std::marker::PhantomData;

use crate::application::{
    WorthQueryBridgeContinuationAuthority, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationCanonicalEntry,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationRouteContract,
    WorthQueryDeclarationSignalCompatibilityContract, WorthQueryMixedAuthority,
    WorthQueryNeighborhoodCapableGrouping, WorthQuerySignalCompatiblePosture,
    WorthQuerySignalDeferredPosture, WorthQuerySignalNotCompatiblePosture,
};

use super::runtime::{
    signal_aspect_contract, signal_aspect_coverage, signal_dependency_aspects,
    signal_produced_aspects, GeometryDomain,
};

macro_rules! define_family {
    ($name:ident, $authority:ty, $signal:ty, $route:expr, $contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $name;

        impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = $authority;
            type SignalCompatibility = $signal;
            type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

            fn semantic_family_key() -> &'static str {
                stringify!($name)
            }

            fn legality_contract() -> WorthQueryDeclarationLegalityContract {
                WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
            }

            fn route_contract() -> WorthQueryDeclarationRouteContract {
                $route
            }

            fn signal_compatibility_contract(
            ) -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
                $contract
            }

            fn aspect_contract() -> WorthQueryDeclarationAspectContract {
                signal_aspect_contract()
            }

            fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
                signal_aspect_coverage()
            }
        }
    };
}

define_family!(
    RuntimeFamily,
    WorthQueryBridgeContinuationAuthority,
    WorthQuerySignalCompatiblePosture,
    WorthQueryDeclarationRouteContract::bridge_only(),
    Some(
        WorthQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
            .with_aspects(signal_dependency_aspects(), signal_produced_aspects())
    )
);
define_family!(
    HistoricalFamily,
    WorthQueryBridgeContinuationAuthority,
    WorthQuerySignalCompatiblePosture,
    WorthQueryDeclarationRouteContract::bridge_only(),
    Some(
        WorthQueryDeclarationSignalCompatibilityContract::historical_derived_execution()
            .with_aspects(signal_dependency_aspects(), signal_produced_aspects())
    )
);
define_family!(
    PreviewFamily,
    WorthQueryBridgeContinuationAuthority,
    WorthQuerySignalCompatiblePosture,
    WorthQueryDeclarationRouteContract::bridge_only(),
    Some(
        WorthQueryDeclarationSignalCompatibilityContract::preview_derived_execution()
            .with_aspects(signal_dependency_aspects(), signal_produced_aspects())
    )
);
define_family!(
    DeferredFamily,
    WorthQueryBridgeContinuationAuthority,
    WorthQuerySignalDeferredPosture,
    WorthQueryDeclarationRouteContract::bridge_only(),
    None
);
define_family!(
    IncompatibleFamily,
    WorthQueryBridgeContinuationAuthority,
    WorthQuerySignalNotCompatiblePosture,
    WorthQueryDeclarationRouteContract::bridge_only(),
    None
);
define_family!(
    MixedFamily,
    WorthQueryMixedAuthority,
    WorthQuerySignalCompatiblePosture,
    WorthQueryDeclarationRouteContract::relational_and_bridge(),
    Some(
        WorthQueryDeclarationSignalCompatibilityContract::preview_derived_execution()
            .with_aspects(signal_dependency_aspects(), signal_produced_aspects())
    )
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissingAspectFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for MissingAspectFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MissingAspectFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        Some(
            WorthQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
                .with_aspects(signal_dependency_aspects(), signal_produced_aspects()),
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
pub struct ConflictingAspectFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for ConflictingAspectFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ConflictingAspectFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        Some(
            WorthQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
                .with_aspects(signal_dependency_aspects(), signal_produced_aspects()),
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
        WorthQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.active_face",
                "signal.dependency.runtime_inputs",
                "signal.conflicting_dependency",
            ],
            &[],
            &["signal.conflicting_dependency"],
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExpandedAspectFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for ExpandedAspectFamily {
    type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ExpandedAspectFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        Some(
            WorthQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
                .with_aspects(
                    WorthQueryDeclarationAspectContract::from_slices(
                        &["selection.active_face", "signal.dependency.runtime_inputs"],
                        &[
                            "selection.neighborhood.local_topology",
                            "signal.dependency.material_projection",
                        ],
                        &[],
                        &["signal.private_authority"],
                        &["signal.conflicting_dependency"],
                    ),
                    WorthQueryDeclarationAspectContract::from_slices(
                        &["signal.produced.derived_face_preview"],
                        &[
                            "signal.produced.material_projection",
                            "signal.produced.analytics",
                        ],
                        &["signal.produced.preview.surface"],
                        &[],
                        &[],
                    ),
                ),
        )
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_face", "signal.dependency.runtime_inputs"],
            &[
                "selection.neighborhood.local_topology",
                "signal.dependency.material_projection",
            ],
            &["signal.preview.surface"],
            &["signal.private_authority"],
            &["signal.conflicting_dependency"],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.active_face",
                "signal.dependency.runtime_inputs",
                "selection.neighborhood.local_topology",
                "signal.dependency.material_projection",
                "signal.preview.surface",
            ],
            &[],
            &[],
        )
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Input<F>(pub &'static str, pub PhantomData<F>);

impl<F> Input<F> {
    pub fn new(edge_ref: &'static str) -> Self {
        Self(edge_ref, PhantomData)
    }
}

macro_rules! impl_input {
    ($($family:ty),+ $(,)?) => {$(
        impl WorthQueryDeclarationInput<GeometryDomain> for Input<$family> {
            type Family = $family;

            fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
                vec![WorthQueryDeclarationCanonicalEntry::text("edge_ref", self.0)]
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
    MixedFamily,
    MissingAspectFamily,
    ConflictingAspectFamily,
    ExpandedAspectFamily
);
