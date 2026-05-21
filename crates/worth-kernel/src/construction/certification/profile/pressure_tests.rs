use super::{
    prepare_primitive_construction_policy_pressure_report, PrimitiveConstructionPolicyPressureCase,
    PrimitiveConstructionPolicyPressureSetup,
};
use crate::spatial_intent::{
    SpatialArbitrationPosture, SpatialBlockedCapability, SpatialChosenIntentAuthority,
    SpatialIdentityContinuityClass, SpatialIntentCandidate, SpatialIntentEscalation,
    SpatialIntentPreviewCommitDisposition, SpatialIntentPreviewWarning, SpatialPreviewRichness,
};

#[test]
fn policy_pressure_report_proves_grazing_profile_deltas_on_same_setup() {
    let report = prepare_primitive_construction_policy_pressure_report().expect("report");
    let ask_first = report
        .row(PrimitiveConstructionPolicyPressureCase::GrazingAskFirst)
        .expect("ask first");
    let preserve = report
        .row(PrimitiveConstructionPolicyPressureCase::GrazingPreserveAmbiguity)
        .expect("preserve");
    let aggressive = report
        .row(PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnap)
        .expect("aggressive");
    let high_fidelity = report
        .row(PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnapHighFidelity)
        .expect("high fidelity");

    assert!(report.pressure_verified());
    assert_eq!(
        report
            .rows()
            .iter()
            .map(|row| row.case())
            .collect::<Vec<_>>(),
        vec![
            PrimitiveConstructionPolicyPressureCase::GrazingAskFirst,
            PrimitiveConstructionPolicyPressureCase::GrazingPreserveAmbiguity,
            PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnap,
            PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnapHighFidelity,
            PrimitiveConstructionPolicyPressureCase::HostFaceAskFirst,
            PrimitiveConstructionPolicyPressureCase::HostFaceBimHostFriendly,
            PrimitiveConstructionPolicyPressureCase::HostFaceBimHostHighFidelityAskFirst,
        ]
    );
    assert_eq!(
        ask_first.setup(),
        PrimitiveConstructionPolicyPressureSetup::GrazingContactMove
    );
    assert_eq!(ask_first.setup(), preserve.setup());
    assert_eq!(ask_first.setup(), aggressive.setup());
    assert_eq!(aggressive.setup(), high_fidelity.setup());
    assert_eq!(ask_first.setup_digest(), preserve.setup_digest());
    assert_eq!(ask_first.setup_digest(), aggressive.setup_digest());
    assert_eq!(aggressive.setup_digest(), high_fidelity.setup_digest());
    assert_eq!(
        ask_first.arbitration_posture(),
        SpatialArbitrationPosture::AskFirst
    );
    assert_eq!(
        preserve.arbitration_posture(),
        SpatialArbitrationPosture::PreserveAmbiguity
    );
    assert_eq!(
        aggressive.arbitration_posture(),
        SpatialArbitrationPosture::PreferSnap
    );
    assert_eq!(
        ask_first.escalation(),
        SpatialIntentEscalation::AskForClarification
    );
    assert_eq!(
        preserve.escalation(),
        SpatialIntentEscalation::PreserveCandidates
    );
    assert_eq!(
        aggressive.escalation(),
        SpatialIntentEscalation::AutoResolve(SpatialIntentCandidate::SnapFlush)
    );
    assert_eq!(
        aggressive.commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(SpatialIntentCandidate::SnapFlush)
    );
    assert_eq!(
        aggressive.continuity_class(),
        SpatialIdentityContinuityClass::AnchorContinuityPreserved
    );
    assert_eq!(
        ask_first.continuity_class(),
        SpatialIdentityContinuityClass::IdentityBlockedPendingChoice
    );
    assert_eq!(
        preserve.continuity_class(),
        SpatialIdentityContinuityClass::IdentityBlockedPendingChoice
    );
    assert_eq!(
        aggressive.policy_resolution_authority(),
        Some(SpatialChosenIntentAuthority::PolicyAutoResolve)
    );
    assert_eq!(
        aggressive.preview_richness(),
        SpatialPreviewRichness::Standard
    );
    assert_eq!(
        high_fidelity.preview_richness(),
        SpatialPreviewRichness::HighFidelity
    );
    assert!(high_fidelity
        .warnings()
        .contains(&SpatialIntentPreviewWarning::HighFidelityPreview));
}

#[test]
fn policy_pressure_report_proves_host_face_policy_deltas_on_same_setup() {
    let report = prepare_primitive_construction_policy_pressure_report().expect("report");
    let ask_first = report
        .row(PrimitiveConstructionPolicyPressureCase::HostFaceAskFirst)
        .expect("ask first");
    let bim = report
        .row(PrimitiveConstructionPolicyPressureCase::HostFaceBimHostFriendly)
        .expect("bim");
    let override_row = report
        .row(PrimitiveConstructionPolicyPressureCase::HostFaceBimHostHighFidelityAskFirst)
        .expect("override");

    assert_eq!(
        ask_first.setup(),
        PrimitiveConstructionPolicyPressureSetup::HostFaceAttachMove
    );
    assert_eq!(ask_first.setup(), bim.setup());
    assert_eq!(bim.setup(), override_row.setup());
    assert_eq!(ask_first.setup_digest(), bim.setup_digest());
    assert_eq!(bim.setup_digest(), override_row.setup_digest());
    assert_eq!(
        ask_first.arbitration_posture(),
        SpatialArbitrationPosture::AskFirst
    );
    assert_eq!(
        bim.arbitration_posture(),
        SpatialArbitrationPosture::PreferHostRelationships
    );
    assert_eq!(
        ask_first.escalation(),
        SpatialIntentEscalation::BlockedByMissingCapability(SpatialBlockedCapability::Join)
    );
    assert_eq!(
        ask_first.clarification_blocked_capability(),
        Some(SpatialBlockedCapability::Join)
    );
    assert_eq!(
        bim.escalation(),
        SpatialIntentEscalation::AutoResolve(SpatialIntentCandidate::AttachRelationally)
    );
    assert_eq!(
        bim.commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
            SpatialIntentCandidate::AttachRelationally
        )
    );
    assert_eq!(
        bim.continuity_class(),
        SpatialIdentityContinuityClass::IdentityReinterpreted
    );
    assert_eq!(
        override_row.escalation(),
        SpatialIntentEscalation::BlockedByMissingCapability(SpatialBlockedCapability::Join)
    );
    assert_eq!(
        override_row.arbitration_posture(),
        SpatialArbitrationPosture::AskFirst
    );
    assert_eq!(
        override_row.preview_richness(),
        SpatialPreviewRichness::HighFidelity
    );
    assert!(override_row
        .warnings()
        .contains(&SpatialIntentPreviewWarning::HighFidelityPreview));
}
