use worth_foundational::{
    boundary_evidence_api::common_path, materialize_diagnostic_support_report, performance,
    performance_api, prepare_canonical_basis_sequence, AdmissionReadinessProfile,
    BoundaryArtifactField, BoundaryArtifactId, BoundaryArtifactLocator, BoundaryHandle,
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalizationRuleVersion, CertificationPostureProfile,
    CompatibilityPostureProfile, EquivalenceBasisId, FoundationalAuthoritativePerformanceClaim,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceReceiptBoundary,
    FoundationalBoundaryEvidenceSourceBasis, FoundationalCommitId, FoundationalCommitParentBasis,
    FoundationalCommitParentageLocator, FoundationalCounterBackedPerformanceReceipt,
    FoundationalDiagnosticCounterSnapshot, FoundationalDiagnosticDeliveryClass,
    FoundationalDiagnosticOutcomeKind, FoundationalDiagnosticPartiality,
    FoundationalDiagnosticSubject, FoundationalDiagnosticSupportClaimStrength,
    FoundationalDiagnosticSupportInput, FoundationalDiagnosticSurfaceAvailability,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceContractName,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
    FoundationalProfileSet, FoundationalProfileSetInput, FoundationalTransitionLocator,
    InternedString, RetentionDeliveryProfile, SupportPostureProfile,
};
use worth_proof::TransitionOutcome;
use worth_store_aspect_native::{
    project_store_boundary_fact_to_terminal_json, StoreCompletedBoundaryReceiptEvidence,
    StoreDiagnosticSupportReportEvidence, StorePerformanceReceiptEvidence,
    StorePhysicalBoundaryWitness, StoreS0ReadinessHandoffArtifact, StoreS0ReadinessHandoffDenial,
};
use worth_store_contracts::{StorePhysicalAuthorityWitness, ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE};
use worth_store_readiness::{
    accept_s0_aspect_native_gate_handoff, reconstruct_s0_handoff_verdict_from_native_evidence,
    reject_terminal_json_projection_as_s0_handoff, S0AspectNativeGateHandoff,
    S0AspectNativeGateHandoffDenial,
};
use worth_store_s0_reclassification::S0HandoffDeniedInputKind;
use worth_store_test_support::NativeStoreAspectFixture;

use crate::certify_s0_handoff_gate_proof_evidence;

#[test]
fn s0_handoff_is_native_boundary_artifact() {
    let artifact = native_handoff_artifact();

    assert_eq!(
        artifact.canonical_basis().payload().domain(),
        CanonicalBasisDomain::Future("store.readiness.handoff")
    );
    assert_eq!(artifact.completed_receipts().len(), 1);
    assert_eq!(artifact.diagnostics().len(), 1);
    assert_eq!(artifact.performance().len(), 1);
}

#[test]
fn s0_handoff_artifact_denies_symbolic_readiness_without_boundary_receipts() {
    assert_handoff_artifact_denial(
        readiness_basis(),
        Vec::new(),
        vec![diagnostic_evidence()],
        vec![performance_evidence()],
        StoreS0ReadinessHandoffDenial::MissingBoundaryReceipt,
    );
}

#[test]
fn s0_handoff_artifact_denies_symbolic_readiness_without_diagnostics() {
    assert_handoff_artifact_denial(
        readiness_basis(),
        vec![completed_receipt_evidence()],
        Vec::new(),
        vec![performance_evidence()],
        StoreS0ReadinessHandoffDenial::MissingDiagnosticEvidence,
    );
}

#[test]
fn s0_handoff_artifact_denies_symbolic_readiness_without_performance_evidence() {
    assert_handoff_artifact_denial(
        readiness_basis(),
        vec![completed_receipt_evidence()],
        vec![diagnostic_evidence()],
        Vec::new(),
        StoreS0ReadinessHandoffDenial::MissingPerformanceEvidence,
    );
}

#[test]
fn s0_rejects_terminal_projection_as_handoff() {
    let fixture = NativeStoreAspectFixture::scalar_string("s0-terminal-denial");
    let projection = project_store_boundary_fact_to_terminal_json(fixture.boundary_fact()).unwrap();

    let denial = reject_terminal_json_projection_as_s0_handoff(projection);

    assert_eq!(
        denial,
        S0AspectNativeGateHandoffDenial::TerminalJsonProjectionInput
    );
}

#[test]
fn s0_handoff_replays_from_native_evidence() {
    let handoff = S0AspectNativeGateHandoff::new(
        native_handoff_artifact(),
        certify_s0_handoff_gate_proof_evidence().unwrap(),
    )
    .unwrap();

    let direct = accept_s0_aspect_native_gate_handoff(handoff.clone());
    let replayed = reconstruct_s0_handoff_verdict_from_native_evidence(&handoff);

    assert_eq!(direct, replayed);
    assert_eq!(replayed.canonical_basis_entry_count(), 3);
    assert_eq!(replayed.receipt_count(), 1);
    assert_eq!(replayed.diagnostic_count(), 1);
    assert_eq!(replayed.performance_receipt_count(), 1);
    assert_eq!(replayed.denied_input_count(), 5);
    assert!(replayed.residue_scan_occurrence_count() > 0);
    assert_eq!(replayed.foundational_adoption_family_count(), 6);
}

#[test]
fn s0_handoff_uses_current_gate_proof_evidence_not_symbolic_labels() {
    let evidence = certify_s0_handoff_gate_proof_evidence().unwrap();

    assert!(
        evidence
            .current_residue_scan()
            .classified_occurrence_count()
            > 0
    );
    assert!(
        evidence
            .terminal_projection_boundary()
            .terminal_boundary_count()
            > 0
    );
    assert_eq!(evidence.foundational_adoption().adopted_family_count(), 6);
    assert_eq!(evidence.public_facade().exported_surface_count(), 3);
    assert_eq!(evidence.native_harness().native_fixture_surface_count(), 2);
    assert_eq!(evidence.negative_proof_count(), 5);
    for denied_input in S0HandoffDeniedInputKind::REQUIRED {
        assert!(evidence.contains_negative_proof(denied_input));
    }
}

#[test]
fn s0_handoff_tests_do_not_use_symbolic_required_negative_proof_shortcut() {
    let source = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("s0_handoff_contract_tests.rs"),
    )
    .unwrap();

    assert!(!source.contains(&["S0HandoffNegativeProofSet", "required"].join("::")));
}

fn native_handoff_artifact(
) -> StoreS0ReadinessHandoffArtifact<FoundationalAuthoritativePerformanceClaim> {
    StoreS0ReadinessHandoffArtifact::new(
        readiness_basis(),
        vec![completed_receipt_evidence()],
        vec![diagnostic_evidence()],
        vec![performance_evidence()],
    )
    .unwrap()
}

fn assert_handoff_artifact_denial(
    canonical_basis: worth_foundational::CanonicalBasisReadyArtifact,
    completed_receipts: Vec<StoreCompletedBoundaryReceiptEvidence>,
    diagnostics: Vec<StoreDiagnosticSupportReportEvidence>,
    performance: Vec<StorePerformanceReceiptEvidence<FoundationalAuthoritativePerformanceClaim>>,
    expected_denial: StoreS0ReadinessHandoffDenial,
) {
    let denial = StoreS0ReadinessHandoffArtifact::<FoundationalAuthoritativePerformanceClaim>::new(
        canonical_basis,
        completed_receipts,
        diagnostics,
        performance,
    )
    .unwrap_err();

    assert_eq!(denial, expected_denial);
}

fn completed_receipt_evidence() -> StoreCompletedBoundaryReceiptEvidence {
    StoreCompletedBoundaryReceiptEvidence::new(completed_receipt(), physical_witness())
}

fn diagnostic_evidence() -> StoreDiagnosticSupportReportEvidence {
    StoreDiagnosticSupportReportEvidence::new(support_report(), physical_witness())
}

fn performance_evidence(
) -> StorePerformanceReceiptEvidence<FoundationalAuthoritativePerformanceClaim> {
    StorePerformanceReceiptEvidence::new(performance_receipt(), physical_witness())
}

fn readiness_basis() -> worth_foundational::CanonicalBasisReadyArtifact {
    expect_success(
        prepare_canonical_basis_sequence(
            CanonicalizationRuleVersion::new("store-s0-handoff-v1").unwrap(),
            CanonicalBasisDomain::Future("store.readiness.handoff"),
            [
                basis_text_entry("canonical-basis", "native"),
                basis_text_entry("diagnostics", "support-report"),
                basis_text_entry("performance", "counter-backed"),
            ],
        ),
        "readiness basis",
    )
}

fn basis_text_entry(locus: &'static str, value: &'static str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::Future("store.readiness.handoff"),
        CanonicalBasisLocus::Named(InternedString::from(locus)),
        CanonicalBasisEntryKind::Future("store-s0-handoff-field"),
        CanonicalBasisValue::ExactText(InternedString::from(value)),
    )
}

fn completed_receipt() -> worth_foundational::FoundationalBoundaryEvidenceCompletedReceiptArtifact {
    common_path::receipt()
        .execution(receipt_boundary())
        .with_provenance(provenance())
        .completed_receipt()
        .clone()
}

fn receipt_boundary() -> FoundationalBoundaryEvidenceReceiptBoundary {
    FoundationalBoundaryEvidenceReceiptBoundary::transition(
        FoundationalTransitionLocator::CommitParentage(FoundationalCommitParentageLocator::new(
            FoundationalCommitId::new(BoundaryHandle::new(111)),
            FoundationalCommitParentBasis::new(EquivalenceBasisId::new(11)),
        )),
    )
}

fn provenance() -> worth_foundational::FoundationalBoundaryEvidenceProvenanceArtifact {
    expect_success(
        common_path::provenance()
            .historical(FoundationalBoundaryEvidenceSourceBasis::boundary_artifact(
                BoundaryArtifactLocator::new(
                    BoundaryArtifactId::new(11),
                    BoundaryArtifactField::Payload,
                ),
            ))
            .with_freshness(FoundationalBoundaryEvidenceFreshnessPosture::StaleRetained),
        "provenance",
    )
}

fn support_report() -> worth_foundational::FoundationalDiagnosticSupportReport {
    materialize_diagnostic_support_report(
        FoundationalDiagnosticSupportInput::new(
            FoundationalDiagnosticSubject::BoundaryArtifact {
                artifact_locator: BoundaryArtifactLocator::new(
                    BoundaryArtifactId::new(12),
                    BoundaryArtifactField::Payload,
                ),
            },
            FoundationalDiagnosticOutcomeKind::Accepted,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            FoundationalDiagnosticSurfaceAvailability::retained_hot(),
            FoundationalDiagnosticSupportClaimStrength::DescriptiveOnly,
            FoundationalDiagnosticPartiality::Complete,
            FoundationalDiagnosticCounterSnapshot::new(1, 0, 0, 0, 0, 0),
            Vec::new(),
        ),
        diagnostic_profile(),
        FoundationalDiagnosticDeliveryClass::MustBeHot,
    )
    .unwrap()
}

fn diagnostic_profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: worth_foundational::DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::Uncertified,
    })
    .unwrap()
}

fn performance_receipt(
) -> FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim> {
    let counter_name = FoundationalPerformanceCounterName::new("store.s0.handoff.accept").unwrap();
    let bundle = performance_api::lower_lane::basis::performance_bundle(performance_claim())
        .attach_contract_name(FoundationalPerformanceContractName::new("store.s0.handoff").unwrap())
        .attach_counter_spec(FoundationalPerformanceCounterSpec::new(
            counter_name.clone(),
            FoundationalPerformanceWorkClass::AuthoritativeMutation,
            1,
        ))
        .finish()
        .unwrap();

    performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle)
        .attach_counter_row(FoundationalPerformanceCounterRow::new(counter_name, 1))
        .finish()
        .unwrap()
}

fn performance_claim() -> FoundationalAuthoritativePerformanceClaim {
    performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::AuthoritativeMutation)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .unwrap()
}

fn physical_witness() -> StorePhysicalBoundaryWitness {
    StorePhysicalBoundaryWitness::from_physical_authority(
        StorePhysicalAuthorityWitness::for_aspect_native_boundary(
            ROADMAP_2_ASPECT_NATIVE_GATE_SCOPE,
        )
        .unwrap(),
    )
    .unwrap()
}

fn expect_success<T, E>(outcome: TransitionOutcome<T, E>, label: &str) -> T {
    match outcome {
        TransitionOutcome::Success(value) => value,
        _ => panic!("{label} should succeed"),
    }
}
