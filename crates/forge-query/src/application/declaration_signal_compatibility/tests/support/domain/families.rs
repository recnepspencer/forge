use std::marker::PhantomData;

use crate::application::{
    ForgeQueryBridgeContinuationAuthority, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationCanonicalEntry,
    ForgeQueryDeclarationFamilyMarker, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationSignalCompatibilityContract, ForgeQueryMixedAuthority,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQuerySignalCompatiblePosture,
    ForgeQuerySignalDeferredPosture, ForgeQuerySignalNotCompatiblePosture,
};

use super::runtime::{
    signal_aspect_contract, signal_aspect_coverage, signal_dependency_aspects,
    signal_produced_aspects, GeometryDomain,
};

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

            fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
                signal_aspect_contract()
            }

            fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
                signal_aspect_coverage()
            }
        }
    };
}

define_family!(
    RuntimeFamily,
    ForgeQueryBridgeContinuationAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    Some(
        ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
            .with_aspects(signal_dependency_aspects(), signal_produced_aspects())
    )
);
define_family!(
    HistoricalFamily,
    ForgeQueryBridgeContinuationAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    Some(
        ForgeQueryDeclarationSignalCompatibilityContract::historical_derived_execution()
            .with_aspects(signal_dependency_aspects(), signal_produced_aspects())
    )
);
define_family!(
    PreviewFamily,
    ForgeQueryBridgeContinuationAuthority,
    ForgeQuerySignalCompatiblePosture,
    ForgeQueryDeclarationRouteContract::bridge_only(),
    Some(
        ForgeQueryDeclarationSignalCompatibilityContract::preview_derived_execution()
            .with_aspects(signal_dependency_aspects(), signal_produced_aspects())
    )
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
    Some(
        ForgeQueryDeclarationSignalCompatibilityContract::preview_derived_execution()
            .with_aspects(signal_dependency_aspects(), signal_produced_aspects())
    )
);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissingAspectFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for MissingAspectFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "MissingAspectFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(
            ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
                .with_aspects(signal_dependency_aspects(), signal_produced_aspects()),
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
pub struct ConflictingAspectFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for ConflictingAspectFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ConflictingAspectFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(
            ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
                .with_aspects(signal_dependency_aspects(), signal_produced_aspects()),
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
        ForgeQueryDeclarationAspectCoverage::from_slices(
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

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for ExpandedAspectFamily {
    type PrimaryAuthority = ForgeQueryBridgeContinuationAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "ExpandedAspectFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::bridge_only()
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(
            ForgeQueryDeclarationSignalCompatibilityContract::runtime_derived_execution()
                .with_aspects(
                    ForgeQueryDeclarationAspectContract::from_slices(
                        &["selection.active_face", "signal.dependency.runtime_inputs"],
                        &[
                            "selection.neighborhood.local_topology",
                            "signal.dependency.material_projection",
                        ],
                        &[],
                        &["signal.private_authority"],
                        &["signal.conflicting_dependency"],
                    ),
                    ForgeQueryDeclarationAspectContract::from_slices(
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

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
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

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
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
    MixedFamily,
    MissingAspectFamily,
    ConflictingAspectFamily,
    ExpandedAspectFamily
);
