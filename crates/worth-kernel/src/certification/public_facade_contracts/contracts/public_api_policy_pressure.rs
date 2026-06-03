use worth_kernel::facade::{
    authoring::{intents::*, policy::*},
    certification::policy::*,
    diagnostics::{arbitration::*, policy::*},
};

#[test]
fn kernel_public_facade_exports_profile_aware_conflict_artifact_surface() {
    let preserve = PrimitiveIntentConflict::analyze_with_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        SpatialIntentPolicyProfile::conservative_exact_modeling().derive(
            SpatialIntentPolicyProfileOverride::new()
                .with_name("conservative_preserve_ambiguity")
                .with_arbitration_posture(SpatialArbitrationPosture::PreserveAmbiguity),
        ),
    );
    let aggressive = PrimitiveIntentConflict::analyze_with_profile(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        SpatialIntentPolicyProfile::aggressive_snap(),
    );

    assert_eq!(
        preserve.escalation(),
        SpatialIntentEscalation::PreserveCandidates
    );
    assert_eq!(
        aggressive.escalation(),
        SpatialIntentEscalation::AutoResolve(SpatialIntentCandidate::SnapFlush)
    );
}

#[test]
fn kernel_public_facade_exports_policy_pressure_siege_surface() {
    let report = prepare_primitive_construction_policy_pressure_report().expect("report");
    let grazing_ask = report
        .row(PrimitiveConstructionPolicyPressureCase::GrazingAskFirst)
        .expect("grazing ask");
    let host_bim = report
        .row(PrimitiveConstructionPolicyPressureCase::HostFaceBimHostFriendly)
        .expect("host bim");
    let host_override = report
        .row(PrimitiveConstructionPolicyPressureCase::HostFaceBimHostHighFidelityAskFirst)
        .expect("host override");

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
        grazing_ask.arbitration_posture(),
        SpatialArbitrationPosture::AskFirst
    );
    assert_eq!(
        host_bim.escalation(),
        SpatialIntentEscalation::AutoResolve(SpatialIntentCandidate::AttachRelationally)
    );
    assert_eq!(
        host_override.escalation(),
        SpatialIntentEscalation::BlockedByMissingCapability(SpatialBlockedCapability::Join)
    );
}

#[test]
fn kernel_public_facade_exports_policy_pressure_delta_surface() {
    let report = prepare_primitive_construction_policy_pressure_delta_report().expect("report");
    let grazing = report
        .row(PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsAggressiveSnap)
        .expect("grazing");
    let host = report
        .row(PrimitiveConstructionPolicyPressureDeltaCase::HostFaceAskFirstVsBimHostFriendly)
        .expect("host");

    assert!(report.delta_verified());
    assert_eq!(
        report.rows().iter().map(|row| row.case()).collect::<Vec<_>>(),
        vec![
            PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsPreservedAmbiguity,
            PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsAggressiveSnap,
            PrimitiveConstructionPolicyPressureDeltaCase::GrazingAggressiveSnapVsHighFidelity,
            PrimitiveConstructionPolicyPressureDeltaCase::HostFaceAskFirstVsBimHostFriendly,
            PrimitiveConstructionPolicyPressureDeltaCase::HostFaceBimHostFriendlyVsHighFidelityAskFirst,
        ]
    );
    assert_eq!(
        grazing.left_row().setup_digest(),
        grazing.right_row().setup_digest()
    );
    assert_eq!(
        grazing.left_row().escalation(),
        SpatialIntentEscalation::AskForClarification
    );
    assert_eq!(
        grazing.right_row().escalation(),
        SpatialIntentEscalation::AutoResolve(SpatialIntentCandidate::SnapFlush)
    );
    assert_eq!(
        host.left_row().escalation(),
        SpatialIntentEscalation::BlockedByMissingCapability(SpatialBlockedCapability::Join)
    );
    assert_eq!(
        host.right_row().escalation(),
        SpatialIntentEscalation::AutoResolve(SpatialIntentCandidate::AttachRelationally)
    );
    assert_eq!(
        host.left_row().commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldBlockOnCapability(
            SpatialBlockedCapability::Join
        )
    );
    assert_eq!(
        host.right_row().commit_disposition(),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
            SpatialIntentCandidate::AttachRelationally
        )
    );
    assert_eq!(
        host.right_row().policy_resolution_authority(),
        Some(SpatialChosenIntentAuthority::PolicyAutoResolve)
    );
    assert_eq!(
        grazing.right_row().continuity_class(),
        SpatialIdentityContinuityClass::AnchorContinuityPreserved
    );
    assert_eq!(
        host.right_row().continuity_class(),
        SpatialIdentityContinuityClass::IdentityReinterpreted
    );
}

#[test]
fn kernel_public_facade_exports_policy_pressure_bundle_surface() {
    let bundle = prepare_primitive_construction_policy_pressure_report_bundle().expect("bundle");

    assert_eq!(
        bundle.direct_report().report_digest(),
        bundle.delta_report().direct_report().report_digest()
    );
    assert_eq!(bundle.required_direct_cases().len(), 7);
    assert_eq!(bundle.required_delta_cases().len(), 5);
    assert_eq!(
        bundle.truth().required_direct_cases(),
        bundle.required_direct_cases()
    );
}
