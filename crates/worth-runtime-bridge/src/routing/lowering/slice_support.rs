use worth_foundational::facade::{AspectFieldLocator, AspectLocator, AspectMask, ProjectionMask};

use crate::input::envelope::BridgeCommittedPatchTarget;
use crate::mapping::TruthDeltaSurfaceKind;
use crate::routing::matching::FineGrainedMatchStatus;

pub(super) fn subscription_slice_canonical_basis(
    slice_target_identity: &str,
    snapshot_read_contract_basis: &str,
    match_status: FineGrainedMatchStatus,
) -> String {
    format!(
        "subscription-slice|slice-target={}|read-contract={}|match={}",
        slice_target_identity,
        snapshot_read_contract_basis,
        canonical_match_status_label(match_status),
    )
}

pub(super) fn subscription_committed_patch_target(
    aspect_locator: &AspectLocator,
    field_locator: Option<&AspectFieldLocator>,
    projection_mask: &AspectMask<ProjectionMask>,
    surface_kind: TruthDeltaSurfaceKind,
) -> BridgeCommittedPatchTarget {
    assert_subscription_slice_target_shape(field_locator, projection_mask, surface_kind);
    BridgeCommittedPatchTarget::from_admitted_target_shape(
        aspect_locator.clone(),
        field_locator.cloned(),
        projection_mask,
        surface_kind,
    )
}

pub(super) fn assert_subscription_slice_target_shape(
    field_locator: Option<&AspectFieldLocator>,
    projection_mask: &AspectMask<ProjectionMask>,
    surface_kind: TruthDeltaSurfaceKind,
) {
    match (surface_kind, field_locator) {
        (TruthDeltaSurfaceKind::EntityField, Some(locator)) => {
            assert_eq!(
                projection_mask.paths(),
                std::slice::from_ref(locator.field_path()),
                "field subscription slices must project exactly their foundational field path"
            );
        }
        (TruthDeltaSurfaceKind::EntityField, None) => {
            panic!("field subscription slices require a foundational field locator");
        }
        (_, Some(_)) => {
            panic!("non-field subscription slices must not carry a foundational field locator");
        }
        (_, None) => {
            assert!(
                projection_mask.is_whole_aspect(),
                "non-field subscription slices must use whole-aspect projection masks"
            );
        }
    }
}

fn canonical_match_status_label(status: FineGrainedMatchStatus) -> &'static str {
    match status {
        FineGrainedMatchStatus::Matched => "matched",
        FineGrainedMatchStatus::WideningAdmitted => "widening-admitted",
        FineGrainedMatchStatus::SuppressedByRegistrationPolicy => {
            "suppressed-by-registration-policy"
        }
        FineGrainedMatchStatus::UnsupportedSurfaceCategory => "unsupported-surface-category",
        FineGrainedMatchStatus::AmbiguousRegistration => "ambiguous-registration",
    }
}
