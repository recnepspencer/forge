use worth_foundational::{FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole};
use worth_proof::{ProofOutcomeKind, TransitionOutcome};
use worth_store_physical_integrity::{
    FoundationalBoundaryRoleMapping, IntegrityDiagnosticReport, IntegrityEvidenceCounters,
    IntegrityProofProgressionReport, OfflineScrubInspectionInput,
    PhysicalIntegrityEvidenceAuthority, PhysicalIntegrityEvidenceDenial,
    PhysicalIntegrityEvidenceEquivalence, PhysicalIntegrityEvidenceProfile, ScrubPlan,
    ScrubPlanRequest, ScrubWindowOrdinal, StoreExecutedIntegrityEvidence,
    StorePlannedWorkBoundaryKind, StorePlannedWorkBoundaryReport,
};

use crate::courtroom::harness::test_support::physical_container_integrity_test_support::{
    inspect_page_report, page_payload_with_record,
};
use crate::courtroom::layout::derived_index_damage_tests::inspect_damaged_derived_index_with_authority;
use crate::{PhysicalScenarioPlannedWorkBoundaryReport, PhysicalScenarioQualityHarness};

use super::foundational_integrity_evidence_support::{
    planned_work_scenario_definition, seal_intact_page_report, with_scrub_plan_authority,
};

#[test]
fn executed_findings_materialize_same_foundational_basis_through_independent_constructors() {
    let payload = page_payload_with_record(b"phase-13-authority");
    let report = inspect_page_report(&payload);
    let authority = PhysicalIntegrityEvidenceAuthority::store_local();

    let first = authority
        .materialize(
            StoreExecutedIntegrityEvidence::authoritative_page(&report),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap();
    let second = authority
        .materialize(
            StoreExecutedIntegrityEvidence::authoritative_page_boundary(&report),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap();

    PhysicalIntegrityEvidenceEquivalence::from_independent_materializations(
        first.clone(),
        second.clone(),
    )
    .unwrap();
    assert_eq!(first.diagnostic_report(), second.diagnostic_report());
    assert_eq!(first.provenance(), second.provenance());
    assert_eq!(first.performance_receipt(), second.performance_receipt());
    assert_eq!(
        first.certification_receipt(),
        second.certification_receipt()
    );
}

#[test]
fn same_materialization_path_is_not_independent_evidence() {
    let payload = page_payload_with_record(b"phase-13-self-compare");
    let report = inspect_page_report(&payload);
    let authority = PhysicalIntegrityEvidenceAuthority::store_local();

    let first = authority
        .materialize(
            StoreExecutedIntegrityEvidence::authoritative_page(&report),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap();
    let second = authority
        .materialize(
            StoreExecutedIntegrityEvidence::authoritative_page(&report),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap();

    let denial =
        PhysicalIntegrityEvidenceEquivalence::from_independent_materializations(first, second)
            .unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityEvidenceDenial::SameMaterializationPath
    );
}

#[test]
fn independent_materializations_must_have_the_same_evidence_basis() {
    let payload = page_payload_with_record(b"phase-13-basis-mismatch");
    let page_report = inspect_page_report(&payload);
    let derived_report = inspect_damaged_derived_index_with_authority(9);
    let authority = PhysicalIntegrityEvidenceAuthority::store_local();

    let page_evidence = authority
        .materialize(
            StoreExecutedIntegrityEvidence::authoritative_page(&page_report),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap();
    let derived_evidence = authority
        .materialize(
            StoreExecutedIntegrityEvidence::rebuildable_derived_report(&derived_report).unwrap(),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap();

    let denial = PhysicalIntegrityEvidenceEquivalence::from_independent_materializations(
        page_evidence,
        derived_evidence,
    )
    .unwrap_err();
    assert_eq!(
        denial,
        PhysicalIntegrityEvidenceDenial::EvidenceBasisMismatch
    );
}

#[test]
fn reduced_profile_drops_only_optional_forensic_material() {
    let payload = page_payload_with_record(b"phase-13-profile");
    let report = inspect_page_report(&payload);
    let authority = PhysicalIntegrityEvidenceAuthority::store_local();

    let full = authority
        .materialize(
            StoreExecutedIntegrityEvidence::authoritative_page(&report),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap();
    let reduced = authority
        .materialize(
            StoreExecutedIntegrityEvidence::authoritative_page(&report),
            PhysicalIntegrityEvidenceProfile::reduced(),
        )
        .unwrap();

    assert!(full.optional_forensic_material_count() > 0);
    assert_eq!(reduced.optional_forensic_material_count(), 0);
    assert_eq!(full.integrity_outcome(), reduced.integrity_outcome());
    assert_eq!(full.locality(), reduced.locality());
    assert_eq!(full.counters(), reduced.counters());
    assert_eq!(full.denial_count(), reduced.denial_count());
    assert_eq!(full.diagnostic_report(), reduced.diagnostic_report());
    assert_eq!(full.provenance(), reduced.provenance());
    assert_eq!(full.performance_receipt(), reduced.performance_receipt());
    assert_eq!(
        full.certification_receipt(),
        reduced.certification_receipt()
    );
}

#[test]
fn role_mapping_keeps_store_authority_and_exports_descriptive_roles_distinct() {
    assert_eq!(
        FoundationalBoundaryRoleMapping::store_physical_authority().role(),
        FoundationalBoundaryArtifactRole::AuthoritativeCurrent
    );
    assert_eq!(
        FoundationalBoundaryRoleMapping::store_derived_projection().role(),
        FoundationalBoundaryArtifactRole::DerivedProjection
    );
    assert_eq!(
        FoundationalBoundaryRoleMapping::store_support_only().role(),
        FoundationalBoundaryArtifactRole::SupportOnly
    );
    assert_eq!(
        FoundationalBoundaryRoleMapping::store_planned_work().role(),
        FoundationalBoundaryArtifactRole::PlannedWork
    );
    assert_eq!(
        FoundationalBoundaryRoleMapping::store_receipt_evidence().role(),
        FoundationalBoundaryArtifactRole::ReceiptEvidence
    );
}

#[test]
fn planned_work_role_is_derived_from_a_real_pre_execution_scrub_plan() {
    with_scrub_plan_authority(|allocation, policy| {
        let input = OfflineScrubInspectionInput::from_declared_windows(vec![
            (ScrubWindowOrdinal::from_zero_based(0), b"alpha".as_slice()),
            (ScrubWindowOrdinal::from_zero_based(1), b"bravo".as_slice()),
        ])
        .unwrap();
        let plan =
            ScrubPlan::build(ScrubPlanRequest::offline(allocation, input, policy)).unwrap();
        let report = StorePlannedWorkBoundaryReport::from_scrub_plan(&plan);

        assert_eq!(report.kind(), StorePlannedWorkBoundaryKind::ScrubPlan);
        assert_eq!(
            report.mapping().role(),
            FoundationalBoundaryArtifactRole::PlannedWork
        );
        assert_eq!(
            report.mapping().category(),
            FoundationalBoundaryArtifactCategory::Summary
        );
        assert_eq!(report.planned_window_count(), 2);
        assert_eq!(report.planned_byte_count(), 10);
        assert_eq!(
            report.claim().basis(),
            StorePlannedWorkBoundaryReport::from_scrub_plan(&plan)
                .claim()
                .basis()
        );
    });
}

#[test]
fn planned_work_role_is_derived_from_a_real_pre_execution_scenario_plan() {
    let harness = PhysicalScenarioQualityHarness::cross_cutting_scenario();
    let plan = harness.lower(planned_work_scenario_definition()).unwrap();
    let report = PhysicalScenarioPlannedWorkBoundaryReport::from_scenario_plan(&plan);

    assert_eq!(report.kind(), StorePlannedWorkBoundaryKind::ScenarioPlan);
    assert_eq!(
        report.mapping().role(),
        FoundationalBoundaryArtifactRole::PlannedWork
    );
    assert_eq!(
        report.mapping().category(),
        FoundationalBoundaryArtifactCategory::Summary
    );
    assert_eq!(report.plan_identity(), plan.identity());
    assert_eq!(report.planned_step_count(), plan.story_steps().len() as u64);
    assert_eq!(
        report.required_oracle_count(),
        plan.required_oracles().len() as u64
    );
    assert_eq!(
        report.expected_counter_count(),
        plan.expected_counters().len() as u64
    );
    assert_eq!(
        report.basis(),
        PhysicalScenarioPlannedWorkBoundaryReport::from_scenario_plan(&plan).basis()
    );
}

#[test]
fn rebuildable_derived_reports_export_as_derived_projection_only() {
    let report = inspect_damaged_derived_index_with_authority(7);
    let authority = PhysicalIntegrityEvidenceAuthority::store_local();
    let source = StoreExecutedIntegrityEvidence::rebuildable_derived_report(&report).unwrap();
    let evidence = authority
        .materialize(source, PhysicalIntegrityEvidenceProfile::full())
        .unwrap();

    assert_eq!(
        evidence.boundary_role(),
        FoundationalBoundaryArtifactRole::DerivedProjection
    );
    assert!(matches!(
        evidence.counters(),
        IntegrityEvidenceCounters::DerivedIndex(counters)
            if counters.rebuildable_classifications() == 1
    ));
}

#[test]
fn diagnostics_and_receipts_remain_export_evidence_not_store_authority() {
    let payload = page_payload_with_record(b"phase-13-diagnostic");
    let report = inspect_page_report(&payload);
    let authority = PhysicalIntegrityEvidenceAuthority::store_local();
    let evidence = authority
        .materialize(
            StoreExecutedIntegrityEvidence::authoritative_page(&report),
            PhysicalIntegrityEvidenceProfile::full(),
        )
        .unwrap();
    let diagnostic = IntegrityDiagnosticReport::from_executed_evidence(&evidence);
    let support = authority
        .materialize(
            StoreExecutedIntegrityEvidence::support_diagnostic(&diagnostic),
            PhysicalIntegrityEvidenceProfile::reduced(),
        )
        .unwrap();
    assert_eq!(
        support.boundary_role(),
        FoundationalBoundaryArtifactRole::SupportOnly
    );

    let record = seal_intact_page_report(&report);
    let receipt = authority
        .materialize(
            StoreExecutedIntegrityEvidence::receipt_evidence(&record),
            PhysicalIntegrityEvidenceProfile::reduced(),
        )
        .unwrap();
    assert_eq!(
        receipt.boundary_role(),
        FoundationalBoundaryArtifactRole::ReceiptEvidence
    );

    let proof_report = IntegrityProofProgressionReport::from_evidence(&receipt);
    assert!(proof_report.proof_outcome().is_success());
    assert_eq!(proof_report.proof_outcome_kind(), ProofOutcomeKind::Success);
    match proof_report.proof_outcome().as_raw() {
        TransitionOutcome::Success(snapshot) => {
            assert_eq!(snapshot.category(), receipt.category());
            assert_eq!(snapshot.boundary_role(), receipt.boundary_role());
            assert_eq!(snapshot.outcome(), receipt.integrity_outcome());
            assert_eq!(snapshot.locality(), receipt.locality());
            assert_eq!(snapshot.counters(), receipt.counters());
        }
        other => panic!("expected successful proof outcome, got {other:?}"),
    }
    assert!(!proof_report.claims_store_authority());
    assert!(!proof_report.claims_repair_or_recovery());
}
