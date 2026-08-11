use crate::application::{
    WorthQueryBridgeContinuationAuthority, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationBridgeContinuationContract,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationLegalityContract,
    WorthQueryDeclarationRouteContract, WorthQueryMixedAuthority,
    WorthQueryNeighborhoodCapableGrouping, WorthQueryRelationalTruthAuthority,
    WorthQuerySignalCompatiblePosture,
};

use super::super::runtime::{bridge_aspect_contract, bridge_aspect_coverage, GeometryDomain};

macro_rules! define_family {
    ($name:ident, $route_contract:expr, $bridge_contract:expr) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub(crate) struct $name;

        impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for $name {
            type PrimaryAuthority = WorthQueryBridgeContinuationAuthority;
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

            fn bridge_continuation_contract(
            ) -> Option<WorthQueryDeclarationBridgeContinuationContract> {
                Some($bridge_contract)
            }

            fn aspect_contract() -> WorthQueryDeclarationAspectContract {
                bridge_aspect_contract()
            }

            fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
                bridge_aspect_coverage()
            }
        }
    };
}

define_family!(
    RuntimeRouteFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::runtime_route_current()
);
define_family!(
    TruthViewCurrentFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::truth_view_current()
);
define_family!(
    TruthViewHistoricalFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::truth_view_historical()
);
define_family!(
    PreviewSessionFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::preview_session()
);
define_family!(
    PreviewPromotionFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::preview_promotion()
);
define_family!(
    SubscriptionPreparationFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::subscription_preparation()
);
define_family!(
    WritebackPreparationFamily,
    WorthQueryDeclarationRouteContract::bridge_only(),
    WorthQueryDeclarationBridgeContinuationContract::writeback_preparation()
);
define_family!(
    SignalOnlyFamily,
    WorthQueryDeclarationRouteContract::signal_only(),
    WorthQueryDeclarationBridgeContinuationContract::runtime_route_current()
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

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::preview_session())
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
        bridge_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct RelationalOnlyFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for RelationalOnlyFamily {
    type PrimaryAuthority = WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "RelationalOnlyFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_only()
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        bridge_aspect_contract()
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        bridge_aspect_coverage()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MissingAspectFamily;

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

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(
            WorthQueryDeclarationBridgeContinuationContract::preview_session()
                .with_required_aspects(WorthQueryDeclarationAspectContract::from_slices(
                    &["continuation.bridge.authority_slice"],
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

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(
            WorthQueryDeclarationBridgeContinuationContract::preview_session()
                .with_required_aspects(WorthQueryDeclarationAspectContract::from_slices(
                    &["continuation.conflicting_preview"],
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
        WorthQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_face", "continuation.conflicting_preview"],
            &[],
            &["continuation.conflicting_preview"],
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ExpandedAspectFamily;

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

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(WorthQueryDeclarationBridgeContinuationContract::preview_session())
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_face", "continuation.preview_ready"],
            &[
                "selection.neighborhood.local_topology",
                "continuation.preview_material",
            ],
            &["continuation.preview.surface"],
            &["continuation.private_branch"],
            &["continuation.conflicting_preview"],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.active_face",
                "continuation.preview_ready",
                "selection.neighborhood.local_topology",
                "continuation.bridge.audit_lane",
                "continuation.preview_material",
                "continuation.preview.surface",
            ],
            &[],
            &[],
        )
    }
}
