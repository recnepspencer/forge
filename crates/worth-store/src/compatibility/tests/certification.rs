use super::super::{
    plan_first_ship_rolling_upgrade, plan_restore_compatibility, restore, rolling,
    ArtifactCompatibilityWindow, ArtifactFormatVersion, ArtifactSemanticVersion,
    CompatibilityAdmissionCounters, CompatibilityEdgeRegistry, CompatibilityFamilyKind,
    CompatibilityRejection, CompatibilityRejectionKind, CompatibilityRelation,
    DeclaredCompatibilityEdge, Milestone12CertificationLaneKind,
    Milestone12CertificationLaneOutcome, Milestone12CertificationLaneRejection,
    Milestone12CertificationLaneStatus, Milestone12CompatibilityMatrix,
    Milestone12CompatibilityMatrixStatus, ReaderCapabilitySet, RestoreBackupScope,
    RestoreCompatibilityTarget, RestorePublicationConflictSet, RollingUpgradeWindow,
    WriterCapabilitySet,
};
use super::{
    backup_manifest_for_family, milestone_12_certification_input,
    milestone_12_certification_outcomes,
    milestone_12_certification_outcomes_with_zero_counter_lane, milestone_12_complexity_surface,
    milestone_12_version_skew_report,
};
use super::{
    Milestone12AdmissionReport, Milestone12CertificationEvidenceBundle, Milestone12CounterContract,
    Milestone12CounterContractViolation, MILESTONE_12_ADMISSION_REPORT_COUNTER_FIELD_NAMES,
};

#[test]
fn compatibility_certification_lane_ids_are_stable_unique_and_mandatory() {
    let mandatory = Milestone12CertificationLaneKind::mandatory_phase_5a();
    assert_eq!(mandatory.len(), 23);
    let mut seen = std::collections::BTreeSet::new();
    for kind in mandatory {
        assert_eq!(kind.lane_id().as_str(), kind.label());
        assert!(
            seen.insert(kind.lane_id()),
            "duplicate lane {}",
            kind.label()
        );
    }
    assert!(seen.contains(&Milestone12CertificationLaneKind::CatalogCompleteness.lane_id()));
    assert!(seen.contains(&Milestone12CertificationLaneKind::RollingAdapterEdgeRejected.lane_id()));
    assert!(seen.contains(&Milestone12CertificationLaneKind::RestoreMissingEdgeRejected.lane_id()));
    assert!(seen.contains(&Milestone12CertificationLaneKind::DisasterRecoveryTruthWindow.lane_id()));
}

#[test]
fn compatibility_certification_matrix_requires_every_mandatory_lane() {
    let mut outcomes = milestone_12_certification_outcomes();
    let dropped = outcomes.pop().expect("fixture has mandatory lanes");
    assert_eq!(
        Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes),
        Err(Milestone12CertificationLaneRejection::MissingMandatoryLane)
    );

    outcomes.push(dropped.clone());
    outcomes.push(dropped);
    assert_eq!(
        Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes),
        Err(Milestone12CertificationLaneRejection::DuplicateLane)
    );
}

#[test]
fn compatibility_certification_matrix_is_complete_and_deterministic() {
    let mut outcomes = milestone_12_certification_outcomes();
    outcomes.reverse();
    let matrix = Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes)
        .expect("all mandatory lanes should produce a complete matrix");
    assert_eq!(
        matrix.status(),
        Milestone12CompatibilityMatrixStatus::Complete
    );
    assert_eq!(
        matrix.entries().len(),
        Milestone12CertificationLaneKind::mandatory_phase_5a().len()
    );
    let observed = matrix
        .entries()
        .iter()
        .map(|entry| entry.lane_id().as_str())
        .collect::<Vec<_>>();
    let mut sorted = observed.clone();
    sorted.sort();
    assert_eq!(observed, sorted);
}

#[test]
fn compatibility_certification_counter_contract_validates_report_shape() {
    let mut counters = CompatibilityAdmissionCounters::default();
    counters.record_relation_recheck();
    let report = Milestone12AdmissionReport::from_admission_counters(&counters);
    Milestone12CounterContract::phase_1()
        .validate_report(&report)
        .expect("phase-1 counter contract should cover report fields");

    let missing_counter_contract = Milestone12CounterContract {
        counter_names: MILESTONE_12_ADMISSION_REPORT_COUNTER_FIELD_NAMES
            .iter()
            .copied()
            .filter(|name| *name != "compatibility.restore.accept_count")
            .collect(),
    };
    assert_eq!(
        missing_counter_contract.validate_report(&report),
        Err(Milestone12CounterContractViolation::MissingReportCounter)
    );
}

#[test]
fn compatibility_certification_bundle_preserves_lane_evidence() {
    let outcomes = milestone_12_certification_outcomes();
    let matrix = Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes)
        .expect("fixture should contain every mandatory lane");
    let mut counters = CompatibilityAdmissionCounters::default();
    counters.record_relation_recheck();
    let bundle = Milestone12CertificationEvidenceBundle::from_parts(
        Milestone12AdmissionReport::from_admission_counters(&counters),
        matrix,
        milestone_12_version_skew_report(),
        milestone_12_complexity_surface(),
        outcomes,
    )
    .expect("complete matrix with counter evidence should build certification bundle");

    assert_eq!(
        bundle.lane_outcomes().len(),
        Milestone12CertificationLaneKind::mandatory_phase_5a().len()
    );
    assert_eq!(bundle.run_summary().accepted_lane_count(), 9);
    assert_eq!(bundle.run_summary().rejected_lane_count(), 11);
    assert_eq!(bundle.rolling_evidence().admitted_lane_count(), 1);
    assert_eq!(bundle.rolling_evidence().rejected_lane_count(), 3);
    assert_eq!(bundle.restore_evidence().admitted_lane_count(), 1);
    assert_eq!(bundle.restore_evidence().rejected_lane_count(), 3);
}

#[test]
fn compatibility_certification_bundle_rejects_matrix_outcome_mismatch() {
    let mut outcomes = milestone_12_certification_outcomes();
    let matrix = Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes)
        .expect("fixture should contain every mandatory lane");
    outcomes.pop();

    let mut counters = CompatibilityAdmissionCounters::default();
    counters.record_relation_recheck();
    assert_eq!(
        Milestone12CertificationEvidenceBundle::from_parts(
            Milestone12AdmissionReport::from_admission_counters(&counters),
            matrix,
            milestone_12_version_skew_report(),
            milestone_12_complexity_surface(),
            outcomes,
        ),
        Err(Milestone12CertificationLaneRejection::MatrixLaneMismatch)
    );
}

#[test]
fn compatibility_certification_bundle_rejects_counterless_lane_evidence() {
    let outcomes = vec![Milestone12CertificationLaneOutcome::accepted(
        Milestone12CertificationLaneKind::CatalogCompleteness,
        milestone_12_certification_input(),
        CompatibilityRelation::Native,
        &CompatibilityAdmissionCounters::default(),
    )];
    let rejection = Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes)
        .expect_err("single-lane fixture is intentionally incomplete");
    assert_eq!(
        rejection,
        Milestone12CertificationLaneRejection::MissingMandatoryLane
    );

    let outcomes = milestone_12_certification_outcomes_with_zero_counter_lane();
    let matrix = Milestone12CompatibilityMatrix::from_lane_outcomes(&outcomes)
        .expect("fixture should contain every mandatory lane");
    assert_eq!(
        Milestone12CertificationEvidenceBundle::from_parts(
            Milestone12AdmissionReport::from_admission_counters(
                &CompatibilityAdmissionCounters::default()
            ),
            matrix,
            milestone_12_version_skew_report(),
            milestone_12_complexity_surface(),
            outcomes,
        ),
        Err(Milestone12CertificationLaneRejection::CounterEvidenceMissing)
    );
}

#[test]
fn compatibility_certification_rolling_outcome_preserves_plan_relation() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let window = RollingUpgradeWindow::new(
        family_id.clone(),
        ArtifactCompatibilityWindow::new(
            ArtifactFormatVersion::new(1),
            ArtifactFormatVersion::new(2),
            ArtifactSemanticVersion::new(1),
            ArtifactSemanticVersion::new(2),
        ),
    );
    let reader = ReaderCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(1)]);
    let writer = WriterCapabilitySet::new(family_id.clone(), vec![ArtifactSemanticVersion::new(2)]);
    let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
        family_id,
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        CompatibilityRelation::ForwardRead,
    )]);
    let mut counters = CompatibilityAdmissionCounters::default();
    let plan = rolling::plan_first_ship_rolling_upgrade(
        &mut counters,
        &edge_registry,
        &window,
        &[reader],
        &[writer],
    )
    .expect("declared two-capability window should admit");
    let outcome = Milestone12CertificationLaneOutcome::from_rolling_plan(
        milestone_12_certification_input(),
        &plan,
        &counters,
    );
    assert_eq!(
        outcome.lane_kind(),
        Milestone12CertificationLaneKind::RollingTwoCapabilityAdmitted
    );
    assert_eq!(
        outcome.status(),
        Milestone12CertificationLaneStatus::Accepted
    );
    assert_eq!(outcome.relation(), Some(CompatibilityRelation::ForwardRead));
}

#[test]
fn compatibility_certification_restore_outcome_preserves_plan_relation() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let backup_manifest = backup_manifest_for_family(family_id.clone(), 1);
    let target =
        RestoreCompatibilityTarget::new(family_id.clone(), ArtifactSemanticVersion::new(2));
    let scope = RestoreBackupScope::new(vec![family_id.clone()]);
    let conflicts = RestorePublicationConflictSet::new(Vec::new());
    let edge_registry = CompatibilityEdgeRegistry::new(vec![DeclaredCompatibilityEdge::new(
        family_id,
        ArtifactSemanticVersion::new(1),
        ArtifactSemanticVersion::new(2),
        CompatibilityRelation::BackwardRead,
    )]);
    let mut counters = CompatibilityAdmissionCounters::default();
    let plan = restore::plan_restore_compatibility(
        &mut counters,
        &edge_registry,
        &scope,
        &backup_manifest,
        &target,
        &conflicts,
    )
    .expect("declared restore edge should admit");
    let outcome = Milestone12CertificationLaneOutcome::from_restore_plan(
        milestone_12_certification_input(),
        &plan,
        &counters,
    );
    assert_eq!(
        outcome.lane_kind(),
        Milestone12CertificationLaneKind::RestoreScopedBackupAdmitted
    );
    assert_eq!(
        outcome.status(),
        Milestone12CertificationLaneStatus::Accepted
    );
    assert_eq!(
        outcome.relation(),
        Some(CompatibilityRelation::BackwardRead)
    );
}

#[test]
fn compatibility_certification_rejection_outcome_preserves_missing_edge_kind() {
    let family_id = CompatibilityFamilyKind::CommitEnvelope.family_id();
    let mut counters = CompatibilityAdmissionCounters::default();
    counters.record_relation_recheck();
    counters.record_edge_missing_rejection();
    let rejection = CompatibilityRejection::new(
        CompatibilityRejectionKind::MissingCompatibilityEdge,
        family_id,
        "missing edge",
    );
    let outcome = Milestone12CertificationLaneOutcome::from_compatibility_rejection(
        Milestone12CertificationLaneKind::AuthoritativeMissingEdgeRejected,
        milestone_12_certification_input(),
        &rejection,
        &counters,
    );
    assert_eq!(
        outcome.status(),
        Milestone12CertificationLaneStatus::Rejected
    );
    assert_eq!(
        outcome.rejection_kind(),
        Some(CompatibilityRejectionKind::MissingCompatibilityEdge)
    );
    assert_eq!(outcome.counters().edge_missing_rejection_count, 1);
}
