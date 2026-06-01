use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationBridgeContinuationContract, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationLegalityContract, ForgeQueryDeclarationProgressionContract,
    ForgeQueryDeclarationRelationalTruthContract, ForgeQueryDeclarationRouteContract,
    ForgeQueryDeclarationSignalCompatibilityContract, ForgeQueryMixedAuthority,
    ForgeQueryNeighborhoodCapableGrouping, ForgeQuerySignalCompatiblePosture,
};

use super::GeometryDomain;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AuthorityRichFamily;

impl ForgeQueryDeclarationFamilyMarker<GeometryDomain> for AuthorityRichFamily {
    type PrimaryAuthority = ForgeQueryMixedAuthority;
    type SignalCompatibility = ForgeQuerySignalCompatiblePosture;
    type GroupedPosture = ForgeQueryNeighborhoodCapableGrouping;

    fn semantic_family_key() -> &'static str {
        "AuthorityRichFamily"
    }

    fn legality_contract() -> ForgeQueryDeclarationLegalityContract {
        ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }

    fn route_contract() -> ForgeQueryDeclarationRouteContract {
        ForgeQueryDeclarationRouteContract::relational_and_bridge()
    }

    fn progression_contract(
        _handle_identity_digest: &str,
        _operating_context_identity_digest: &str,
    ) -> ForgeQueryDeclarationProgressionContract {
        ForgeQueryDeclarationProgressionContract::admitted_current()
    }

    fn aspect_contract() -> ForgeQueryDeclarationAspectContract {
        ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_face"],
            &["selection.neighborhood"],
            &["continuation.preview_ready", "signal.material_edit"],
            &["selection.private_authority"],
            &[],
        )
    }

    fn aspect_coverage() -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_slices(
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

    fn relational_truth_contract() -> Option<ForgeQueryDeclarationRelationalTruthContract> {
        Some(
            ForgeQueryDeclarationRelationalTruthContract::grouped_truth().with_required_aspects(
                ForgeQueryDeclarationAspectContract::from_slices(
                    &["selection.active_face"],
                    &["selection.neighborhood"],
                    &[],
                    &[],
                    &[],
                ),
            ),
        )
    }

    fn bridge_continuation_contract() -> Option<ForgeQueryDeclarationBridgeContinuationContract> {
        Some(
            ForgeQueryDeclarationBridgeContinuationContract::preview_session()
                .with_required_aspects(ForgeQueryDeclarationAspectContract::from_slices(
                    &["continuation.preview_ready"],
                    &[],
                    &[],
                    &[],
                    &[],
                )),
        )
    }

    fn signal_compatibility_contract() -> Option<ForgeQueryDeclarationSignalCompatibilityContract> {
        Some(
            ForgeQueryDeclarationSignalCompatibilityContract::preview_derived_execution()
                .with_aspects(
                    ForgeQueryDeclarationAspectContract::from_slices(
                        &["signal.material_edit"],
                        &[],
                        &[],
                        &[],
                        &[],
                    ),
                    ForgeQueryDeclarationAspectContract::from_slices(
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
