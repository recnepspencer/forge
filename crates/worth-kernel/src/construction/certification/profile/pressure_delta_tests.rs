use super::{
    prepare_primitive_construction_policy_pressure_delta_report,
    PrimitiveConstructionPolicyPressureDeltaCase, PrimitiveConstructionPolicyPressureSetup,
};
use crate::spatial_intent::{
    SpatialArbitrationPosture, SpatialBlockedCapability, SpatialChosenIntentAuthority,
    SpatialIdentityContinuityClass, SpatialIntentCandidate, SpatialIntentEscalation,
    SpatialIntentPreviewCommitDisposition, SpatialIntentPreviewWarning, SpatialPreviewRichness,
};

#[test]
fn policy_pressure_delta_report_proves_same_setup_semantic_deltas_exactly() {
    let report = prepare_primitive_construction_policy_pressure_delta_report().expect("report");
    let preserve = report
        .row(PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsPreservedAmbiguity)
        .expect("preserve");
    let aggressive = report
        .row(PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsAggressiveSnap)
        .expect("aggressive");
    let high_fidelity = report
        .row(PrimitiveConstructionPolicyPressureDeltaCase::GrazingAggressiveSnapVsHighFidelity)
        .expect("high fidelity");

    assert!(report.delta_verified());
    assert_eq!(
        report
            .rows()
            .iter()
            .map(|row| row.case())
            .collect::<Vec<_>>(),
        vec![
            PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsPreservedAmbiguity,
            PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsAggressiveSnap,
            PrimitiveConstructionPolicyPressureDeltaCase::GrazingAggressiveSnapVsHighFidelity,
            PrimitiveConstructionPolicyPressureDeltaCase::HostFaceAskFirstVsBimHostFriendly,
            PrimitiveConstructionPolicyPressureDeltaCase::HostFaceBimHostFriendlyVsHighFidelityAskFirst,
        ]
    );
    assert_eq!(
        preserve.setup(),
        PrimitiveConstructionPolicyPressureSetup::GrazingContactMove
    );
    assert_eq!(
        preserve.left_row().setup_digest(),
        preserve.right_row().setup_digest()
    );
    assert_eq!(
        preserve.left_row().escalation(),
        SpatialIntentEscalation::AskForClarification
    );
    assert_eq!(
        preserve.right_row().escalation(),
        SpatialIntentEscalation::PreserveCandidates
    );
    assert_eq!(
        preserve.right_row().commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldPreserveCandidates
    );
    assert_eq!(
        aggressive.right_row().commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(SpatialIntentCandidate::SnapFlush)
    );
    assert_eq!(
        aggressive.right_row().policy_resolution_authority(),
        Some(SpatialChosenIntentAuthority::PolicyAutoResolve)
    );
    assert_eq!(
        aggressive.left_row().continuity_class(),
        SpatialIdentityContinuityClass::IdentityBlockedPendingChoice
    );
    assert_eq!(
        aggressive.right_row().continuity_class(),
        SpatialIdentityContinuityClass::AnchorContinuityPreserved
    );
    assert_eq!(
        high_fidelity.left_row().preview_richness(),
        SpatialPreviewRichness::Standard
    );
    assert_eq!(
        high_fidelity.right_row().preview_richness(),
        SpatialPreviewRichness::HighFidelity
    );
    assert_eq!(
        aggressive.left_row().arbitration_posture(),
        SpatialArbitrationPosture::AskFirst
    );
    assert_eq!(
        aggressive.right_row().arbitration_posture(),
        SpatialArbitrationPosture::PreferSnap
    );
    assert_eq!(
        high_fidelity.left_row().commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(SpatialIntentCandidate::SnapFlush)
    );
    assert_eq!(
        high_fidelity.right_row().commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(SpatialIntentCandidate::SnapFlush)
    );
    assert_eq!(
        high_fidelity.left_row().continuity_class(),
        SpatialIdentityContinuityClass::AnchorContinuityPreserved
    );
    assert_eq!(
        high_fidelity.right_row().continuity_class(),
        SpatialIdentityContinuityClass::AnchorContinuityPreserved
    );
    assert!(!high_fidelity
        .left_row()
        .warnings()
        .contains(&SpatialIntentPreviewWarning::HighFidelityPreview));
    assert!(high_fidelity
        .right_row()
        .warnings()
        .contains(&SpatialIntentPreviewWarning::HighFidelityPreview));
}

#[test]
fn policy_pressure_delta_report_proves_host_face_policy_flips_without_geometry_drift() {
    let report = prepare_primitive_construction_policy_pressure_delta_report().expect("report");
    let attach = report
        .row(PrimitiveConstructionPolicyPressureDeltaCase::HostFaceAskFirstVsBimHostFriendly)
        .expect("attach");
    let override_row = report
        .row(
            PrimitiveConstructionPolicyPressureDeltaCase::HostFaceBimHostFriendlyVsHighFidelityAskFirst,
        )
        .expect("override");

    assert_eq!(
        attach.setup(),
        PrimitiveConstructionPolicyPressureSetup::HostFaceAttachMove
    );
    assert_eq!(
        attach.left_row().setup_digest(),
        attach.right_row().setup_digest()
    );
    assert_eq!(
        attach.left_row().escalation(),
        SpatialIntentEscalation::BlockedByMissingCapability(SpatialBlockedCapability::Join)
    );
    assert_eq!(
        attach.left_row().clarification_blocked_capability(),
        Some(SpatialBlockedCapability::Join)
    );
    assert_eq!(
        attach.left_row().commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldBlockOnCapability(
            SpatialBlockedCapability::Join
        )
    );
    assert_eq!(
        attach.right_row().escalation(),
        SpatialIntentEscalation::AutoResolve(SpatialIntentCandidate::AttachRelationally)
    );
    assert_eq!(
        attach.right_row().policy_resolution_authority(),
        Some(SpatialChosenIntentAuthority::PolicyAutoResolve)
    );
    assert_eq!(
        attach.right_row().commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
            SpatialIntentCandidate::AttachRelationally
        )
    );
    assert_eq!(
        attach.left_row().continuity_class(),
        SpatialIdentityContinuityClass::IdentityBlockedPendingChoice
    );
    assert_eq!(
        attach.right_row().continuity_class(),
        SpatialIdentityContinuityClass::IdentityReinterpreted
    );
    assert_eq!(
        override_row.left_row().commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
            SpatialIntentCandidate::AttachRelationally
        )
    );
    assert_eq!(
        override_row.right_row().commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldBlockOnCapability(
            SpatialBlockedCapability::Join
        )
    );
    assert_eq!(
        override_row.left_row().continuity_class(),
        SpatialIdentityContinuityClass::IdentityReinterpreted
    );
    assert_eq!(
        override_row.right_row().continuity_class(),
        SpatialIdentityContinuityClass::IdentityBlockedPendingChoice
    );
    assert_eq!(
        override_row.right_row().preview_richness(),
        SpatialPreviewRichness::HighFidelity
    );
    assert_eq!(
        override_row.right_row().escalation(),
        SpatialIntentEscalation::BlockedByMissingCapability(SpatialBlockedCapability::Join)
    );
    assert_eq!(
        override_row.right_row().clarification_blocked_capability(),
        Some(SpatialBlockedCapability::Join)
    );
    assert!(!override_row
        .left_row()
        .warnings()
        .contains(&SpatialIntentPreviewWarning::HighFidelityPreview));
    assert!(override_row
        .right_row()
        .warnings()
        .contains(&SpatialIntentPreviewWarning::HighFidelityPreview));
}
