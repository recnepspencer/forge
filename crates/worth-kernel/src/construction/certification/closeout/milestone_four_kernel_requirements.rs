use forge_query::facade::{ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus};
use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;

use crate::construction::certification::{
    PrimitiveConstructionContinuityCase, PrimitiveConstructionContinuitySurfaceReport,
    PrimitiveConstructionIntentArbitrationPolicyCase,
    PrimitiveConstructionIntentArbitrationPolicyReport,
    PrimitiveConstructionMotionResolutionPolicyCase,
    PrimitiveConstructionMotionResolutionPolicyReport, PrimitiveConstructionPolicyProfileCase,
    PrimitiveConstructionPolicyProfileSurfaceReport, PrimitiveConstructionPreviewCase,
    PrimitiveConstructionPreviewSurfaceReport,
    PrimitiveConstructionRealizationExhaustionWitnessReport,
};
use crate::construction::query::PrimitiveConstructionQueryBoundaryGapRegister;

pub fn motion_policy_inventory_present(
    report: &PrimitiveConstructionMotionResolutionPolicyReport,
) -> bool {
    required_motion_cases()
        .iter()
        .all(|case| report.row(*case).is_some())
}

pub fn arbitration_policy_inventory_present(
    report: &PrimitiveConstructionIntentArbitrationPolicyReport,
) -> bool {
    required_arbitration_cases()
        .iter()
        .all(|case| report.row(*case).is_some())
}

pub fn preview_inventory_present(report: &PrimitiveConstructionPreviewSurfaceReport) -> bool {
    required_preview_cases()
        .iter()
        .all(|case| report.row(*case).is_some())
}

pub fn continuity_inventory_present(report: &PrimitiveConstructionContinuitySurfaceReport) -> bool {
    required_continuity_cases()
        .iter()
        .all(|case| report.row(*case).is_some())
}

pub fn policy_profile_inventory_present(
    report: &PrimitiveConstructionPolicyProfileSurfaceReport,
) -> bool {
    required_policy_profile_cases()
        .iter()
        .all(|case| report.row(*case).is_some())
}

pub fn realization_exhaustion_inventory_present(
    report: &PrimitiveConstructionRealizationExhaustionWitnessReport,
) -> bool {
    required_realization_witness_kinds()
        .iter()
        .all(|kind| report.row_for(*kind).is_some())
}

pub fn query_boundary_closeout_verified(
    report: &PrimitiveConstructionQueryBoundaryGapRegister,
) -> bool {
    let rows = report.rows();
    let write = rows
        .iter()
        .find(|row| row.family() == ForgeQueryRuntimeFacadeFamily::Write);
    let inspect = rows
        .iter()
        .find(|row| row.family() == ForgeQueryRuntimeFacadeFamily::Inspect);
    let branch = rows
        .iter()
        .find(|row| row.family() == ForgeQueryRuntimeFacadeFamily::BranchPreview);
    let temporal = rows
        .iter()
        .find(|row| row.family() == ForgeQueryRuntimeFacadeFamily::Temporal);
    rows.len() == 6
        && write.is_some_and(|row| row.gap_status().as_str() == "closed")
        && inspect.is_some_and(|row| row.gap_status().as_str() == "closed")
        && branch.is_some_and(|row| row.gap_status().as_str() == "closed")
        && temporal.is_some_and(|row| {
            row.gap_status().as_str() == "deferred_unsupported_neighbor"
                && row.support_status() != ForgeQueryRuntimeFamilySupportStatus::Supported
        })
        && report.unresolved_gap_count() >= 1
}

fn required_motion_cases() -> &'static [PrimitiveConstructionMotionResolutionPolicyCase] {
    &[
        PrimitiveConstructionMotionResolutionPolicyCase::DirectMove,
        PrimitiveConstructionMotionResolutionPolicyCase::FrameReorient,
        PrimitiveConstructionMotionResolutionPolicyCase::CarrierReorient,
        PrimitiveConstructionMotionResolutionPolicyCase::FallbackPointsToward,
        PrimitiveConstructionMotionResolutionPolicyCase::AmbiguousMove,
        PrimitiveConstructionMotionResolutionPolicyCase::UndefinedReorient,
        PrimitiveConstructionMotionResolutionPolicyCase::UnsupportedReorient,
        PrimitiveConstructionMotionResolutionPolicyCase::ExhaustedRotate,
        PrimitiveConstructionMotionResolutionPolicyCase::CoincidentPointsToward,
    ]
}

fn required_arbitration_cases() -> &'static [PrimitiveConstructionIntentArbitrationPolicyCase] {
    &[
        PrimitiveConstructionIntentArbitrationPolicyCase::DirectMoveOnly,
        PrimitiveConstructionIntentArbitrationPolicyCase::GrazingSnapConflict,
        PrimitiveConstructionIntentArbitrationPolicyCase::OverlapBlockedCandidates,
        PrimitiveConstructionIntentArbitrationPolicyCase::HostPenetrationBlockedCut,
        PrimitiveConstructionIntentArbitrationPolicyCase::FrameAlignedIntent,
        PrimitiveConstructionIntentArbitrationPolicyCase::OverlapAdvancedCapabilities,
    ]
}

fn required_preview_cases() -> &'static [PrimitiveConstructionPreviewCase] {
    &[
        PrimitiveConstructionPreviewCase::GrazingAskFirst,
        PrimitiveConstructionPreviewCase::GrazingAggressiveSnap,
        PrimitiveConstructionPreviewCase::HostFaceBimAttach,
        PrimitiveConstructionPreviewCase::OverlapBlockedMerge,
        PrimitiveConstructionPreviewCase::OverlapHighFidelity,
    ]
}

fn required_continuity_cases() -> &'static [PrimitiveConstructionContinuityCase] {
    &[
        PrimitiveConstructionContinuityCase::MoveOnlyPreserved,
        PrimitiveConstructionContinuityCase::GrazingSnapAnchorContinuity,
        PrimitiveConstructionContinuityCase::HostAttachReinterpreted,
        PrimitiveConstructionContinuityCase::OverlapBlockedPendingChoice,
        PrimitiveConstructionContinuityCase::ExplicitMergeIdentityMerged,
        PrimitiveConstructionContinuityCase::ExplicitCutOpeningIdentitySplit,
    ]
}

fn required_policy_profile_cases() -> &'static [PrimitiveConstructionPolicyProfileCase] {
    &[
        PrimitiveConstructionPolicyProfileCase::ConservativeExactModeling,
        PrimitiveConstructionPolicyProfileCase::BimHostFriendly,
        PrimitiveConstructionPolicyProfileCase::AskFirstArbitration,
        PrimitiveConstructionPolicyProfileCase::AggressiveSnap,
        PrimitiveConstructionPolicyProfileCase::HighFidelityPreview,
    ]
}

fn required_realization_witness_kinds() -> &'static [PrimitiveRealizationExhaustionWitnessKind] {
    &[
        PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse,
        PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse,
        PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse,
    ]
}
