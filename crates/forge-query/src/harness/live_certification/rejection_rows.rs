use super::bundles::{
    change_sequence_gap_rejection_bundle, detail_patch_bundle,
    forbidden_coalescing_rejection_bundle, forbidden_refresh_rejection_bundle,
    invalid_live_promotion_rejection_bundle, non_monotonic_sequence_rejection_bundle,
    progress_advance_bundle, raw_cdc_leakage_rejection_bundle, rejection_row,
    unsupported_live_family_rejection_bundle, unsupported_patch_family_rejection_bundle,
    width_overflow_rejection_bundle,
};
use super::model::{LivePerturbationClass, LiveRejectionRow};
use crate::harness::profiles::CertificationProfile;

pub(super) fn rejection_rows() -> Vec<LiveRejectionRow> {
    vec![
        rejection_row(
            "forbidden-width-budget-overflow-behavior",
            LivePerturbationClass::WidthOverflowRejection,
            detail_patch_bundle(CertificationProfile::DirectConstruction),
            width_overflow_rejection_bundle(CertificationProfile::BindingVariation),
            detail_patch_bundle(CertificationProfile::ReplayParity),
        ),
        rejection_row(
            "forbidden-coalescing-class",
            LivePerturbationClass::CoalescingRejection,
            detail_patch_bundle(CertificationProfile::DirectConstruction),
            forbidden_coalescing_rejection_bundle(CertificationProfile::BindingVariation),
            detail_patch_bundle(CertificationProfile::ReplayParity),
        ),
        rejection_row(
            "forbidden-refresh-escape-hatch",
            LivePerturbationClass::RefreshRejection,
            detail_patch_bundle(CertificationProfile::DirectConstruction),
            forbidden_refresh_rejection_bundle(CertificationProfile::BindingVariation),
            detail_patch_bundle(CertificationProfile::ReplayParity),
        ),
        rejection_row(
            "non-monotonic-change-sequence",
            LivePerturbationClass::NonMonotonicSequenceRejection,
            progress_advance_bundle(CertificationProfile::DirectConstruction),
            non_monotonic_sequence_rejection_bundle(CertificationProfile::BindingVariation),
            progress_advance_bundle(CertificationProfile::ReplayParity),
        ),
        rejection_row(
            "gapful-change-sequence",
            LivePerturbationClass::SequenceGapRejection,
            progress_advance_bundle(CertificationProfile::DirectConstruction),
            change_sequence_gap_rejection_bundle(CertificationProfile::BindingVariation),
            progress_advance_bundle(CertificationProfile::ReplayParity),
        ),
        rejection_row(
            "invalid-live-basis-promotion",
            LivePerturbationClass::InvalidLivePromotionRejection,
            detail_patch_bundle(CertificationProfile::DirectConstruction),
            invalid_live_promotion_rejection_bundle(CertificationProfile::BindingVariation),
            detail_patch_bundle(CertificationProfile::ReplayParity),
        ),
        rejection_row(
            "unsupported-patch-family",
            LivePerturbationClass::UnsupportedPatchFamilyRejection,
            detail_patch_bundle(CertificationProfile::DirectConstruction),
            unsupported_patch_family_rejection_bundle(CertificationProfile::BindingVariation),
            detail_patch_bundle(CertificationProfile::ReplayParity),
        ),
        rejection_row(
            "unsupported-live-family",
            LivePerturbationClass::UnsupportedLiveFamilyRejection,
            detail_patch_bundle(CertificationProfile::DirectConstruction),
            unsupported_live_family_rejection_bundle(CertificationProfile::BindingVariation),
            detail_patch_bundle(CertificationProfile::ReplayParity),
        ),
        rejection_row(
            "raw-cdc-leakage-forbidden",
            LivePerturbationClass::RawCdcLeakageRejection,
            detail_patch_bundle(CertificationProfile::DirectConstruction),
            raw_cdc_leakage_rejection_bundle(CertificationProfile::ReplayParity),
            detail_patch_bundle(CertificationProfile::ReplayParity),
        ),
    ]
}
