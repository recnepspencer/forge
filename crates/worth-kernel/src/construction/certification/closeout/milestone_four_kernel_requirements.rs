use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;

use crate::construction::certification::{
    PrimitiveConstructionContinuityCase, PrimitiveConstructionIntentArbitrationPolicyCase,
    PrimitiveConstructionMotionResolutionPolicyCase, PrimitiveConstructionPolicyProfileCase,
    PrimitiveConstructionPreviewCase,
};

pub(crate) fn required_motion_cases() -> &'static [PrimitiveConstructionMotionResolutionPolicyCase]
{
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

pub(crate) fn required_arbitration_cases(
) -> &'static [PrimitiveConstructionIntentArbitrationPolicyCase] {
    &[
        PrimitiveConstructionIntentArbitrationPolicyCase::DirectMoveOnly,
        PrimitiveConstructionIntentArbitrationPolicyCase::GrazingSnapConflict,
        PrimitiveConstructionIntentArbitrationPolicyCase::OverlapBlockedCandidates,
        PrimitiveConstructionIntentArbitrationPolicyCase::HostPenetrationBlockedCut,
        PrimitiveConstructionIntentArbitrationPolicyCase::FrameAlignedIntent,
        PrimitiveConstructionIntentArbitrationPolicyCase::OverlapAdvancedCapabilities,
    ]
}

pub(crate) fn required_preview_cases() -> &'static [PrimitiveConstructionPreviewCase] {
    &[
        PrimitiveConstructionPreviewCase::GrazingAskFirst,
        PrimitiveConstructionPreviewCase::GrazingAggressiveSnap,
        PrimitiveConstructionPreviewCase::HostFaceBimAttach,
        PrimitiveConstructionPreviewCase::OverlapBlockedMerge,
        PrimitiveConstructionPreviewCase::OverlapHighFidelity,
    ]
}

pub(crate) fn required_continuity_cases() -> &'static [PrimitiveConstructionContinuityCase] {
    &[
        PrimitiveConstructionContinuityCase::MoveOnlyPreserved,
        PrimitiveConstructionContinuityCase::GrazingSnapAnchorContinuity,
        PrimitiveConstructionContinuityCase::HostAttachReinterpreted,
        PrimitiveConstructionContinuityCase::OverlapBlockedPendingChoice,
        PrimitiveConstructionContinuityCase::ExplicitMergeIdentityMerged,
        PrimitiveConstructionContinuityCase::ExplicitCutOpeningIdentitySplit,
    ]
}

pub(crate) fn required_policy_profile_cases() -> &'static [PrimitiveConstructionPolicyProfileCase] {
    &[
        PrimitiveConstructionPolicyProfileCase::ConservativeExactModeling,
        PrimitiveConstructionPolicyProfileCase::BimHostFriendly,
        PrimitiveConstructionPolicyProfileCase::AskFirstArbitration,
        PrimitiveConstructionPolicyProfileCase::AggressiveSnap,
        PrimitiveConstructionPolicyProfileCase::HighFidelityPreview,
    ]
}

pub(crate) fn required_realization_witness_kinds(
) -> &'static [PrimitiveRealizationExhaustionWitnessKind] {
    &[
        PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse,
        PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse,
        PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse,
    ]
}
