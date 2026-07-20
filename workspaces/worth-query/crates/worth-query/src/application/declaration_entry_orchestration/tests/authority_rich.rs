use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationBridgeContinuationContract, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationLegalityContract, WorthQueryDeclarationProgressionContract,
    WorthQueryDeclarationRelationalTruthContract, WorthQueryDeclarationRouteContract,
    WorthQueryDeclarationSignalCompatibilityContract, WorthQueryMixedAuthority,
    WorthQueryNeighborhoodCapableGrouping, WorthQuerySignalCompatiblePosture,
};

use super::GeometryDomain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AuthorityRichFamily;

impl WorthQueryDeclarationFamilyMarker<GeometryDomain> for AuthorityRichFamily {
    type PrimaryAuthority = WorthQueryMixedAuthority;
    type SignalCompatibility = WorthQuerySignalCompatiblePosture;
    type GroupedPosture = WorthQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AuthorityRichFamily"
    }

    fn legality_contract() -> WorthQueryDeclarationLegalityContract {
        WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> WorthQueryDeclarationRouteContract {
        WorthQueryDeclarationRouteContract::relational_and_bridge()
    }

    fn progression_contract(
        _handle_identity_digest: &str,
        _operating_context_identity_digest: &str,
    ) -> WorthQueryDeclarationProgressionContract {
        WorthQueryDeclarationProgressionContract::admitted_current()
    }

    fn aspect_contract() -> WorthQueryDeclarationAspectContract {
        WorthQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &["selection.neighborhood"],
            &["continuation.preview_ready", "signal.material_edit"],
            &["selection.private_authority"],
            &[],
        )
    }

    fn aspect_coverage() -> WorthQueryDeclarationAspectCoverage {
        WorthQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.active_face",
                "selection.neighborhood",
                "continuation.preview_ready",
                "signal.material_edit",
                "selection.private_authority",
            ],
            &["selection.private_authority"],
            &[],
        )
    }

    fn relational_truth_contract() -> Option<WorthQueryDeclarationRelationalTruthContract> {
        Some(
            WorthQueryDeclarationRelationalTruthContract::grouped_truth().with_required_aspects(
                WorthQueryDeclarationAspectContract::from_slices(
                    &["selection.active_face"],
                    &["selection.neighborhood"],
                    &[],
                    &[],
                    &[],
                ),
            ),
        )
    }

    fn bridge_continuation_contract() -> Option<WorthQueryDeclarationBridgeContinuationContract> {
        Some(
            WorthQueryDeclarationBridgeContinuationContract::preview_session()
                .with_required_aspects(WorthQueryDeclarationAspectContract::from_slices(
                    &["continuation.preview_ready"],
                    &[],
                    &[],
                    &[],
                    &[],
                )),
        )
    }

    fn signal_compatibility_contract() -> Option<WorthQueryDeclarationSignalCompatibilityContract> {
        Some(
            WorthQueryDeclarationSignalCompatibilityContract::preview_derived_execution()
                .with_aspects(
                    WorthQueryDeclarationAspectContract::from_slices(
                        &["signal.material_edit"],
                        &[],
                        &[],
                        &[],
                        &[],
                    ),
                    WorthQueryDeclarationAspectContract::from_slices(
                        &["signal.preview_patch"],
                        &[],
                        &[],
                        &[],
                        &[],
                    ),
                ),
        )
    }
}
