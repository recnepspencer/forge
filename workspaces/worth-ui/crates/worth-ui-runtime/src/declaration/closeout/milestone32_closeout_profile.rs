use crate::declaration::{
    UiDeclarationClosedSemanticLane, UiDeclarationCloseoutGuarantee, UiDeclarationCloseoutNonGoal,
    UiDeclarationCloseoutReport, UiDeclarationFamilyCatalog,
};

const MILESTONE32_CLOSED_SEMANTIC_LANES: &[UiDeclarationClosedSemanticLane] = &[
    UiDeclarationClosedSemanticLane::Identity,
    UiDeclarationClosedSemanticLane::FamilyAuthority,
    UiDeclarationClosedSemanticLane::AspectContracts,
    UiDeclarationClosedSemanticLane::StructuralIntent,
    UiDeclarationClosedSemanticLane::QueryBindingPosture,
    UiDeclarationClosedSemanticLane::ServiceUsagePosture,
    UiDeclarationClosedSemanticLane::TouchMeaningPosture,
    UiDeclarationClosedSemanticLane::MeasurementPolicyPosture,
    UiDeclarationClosedSemanticLane::HostCapabilityPosture,
    UiDeclarationClosedSemanticLane::SupportSnapshot,
];

const MILESTONE32_GUARANTEES: &[UiDeclarationCloseoutGuarantee] = &[
    UiDeclarationCloseoutGuarantee::LowersOnceFromSemanticDslAuthority,
    UiDeclarationCloseoutGuarantee::LaneSpecificDigestLocality,
    UiDeclarationCloseoutGuarantee::NoLaterSourceReopening,
    UiDeclarationCloseoutGuarantee::GraphHandoffConsumesCanonicalDeclarationAuthorityOnly,
];

const MILESTONE32_NON_GOALS: &[UiDeclarationCloseoutNonGoal] = &[
    UiDeclarationCloseoutNonGoal::GraphTruth,
    UiDeclarationCloseoutNonGoal::GraphNodeIdentity,
    UiDeclarationCloseoutNonGoal::ParticipationTruth,
    UiDeclarationCloseoutNonGoal::MountedTruth,
    UiDeclarationCloseoutNonGoal::MeasuredTruth,
    UiDeclarationCloseoutNonGoal::RuntimeParticipationExecution,
];

pub(crate) const MILESTONE32_CLOSEOUT_PROFILE: UiDeclarationCloseoutReport =
    UiDeclarationCloseoutReport::new(
        UiDeclarationFamilyCatalog::closed_initial_set(),
        MILESTONE32_CLOSED_SEMANTIC_LANES,
        MILESTONE32_GUARANTEES,
        MILESTONE32_NON_GOALS,
    );
