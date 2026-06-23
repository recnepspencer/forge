use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanSplitNamedArtifactKind, PlanarBooleanSplitPersistentNamingCounters,
};
use crate::workload_platform::planar_boolean_events::PlanarBooleanLoopRole;
use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    prepared_loop_continuation_subject, LoopFixtureEntryOrder, PreparedLoopContinuationIndexSubject,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoop, PlanarBooleanAdmittedReconstructedLoopSet,
    PlanarBooleanBornLoopSet, PlanarBooleanDegenerateLoopOutcome,
    PlanarBooleanDegenerateLoopOutcomeKind, PlanarBooleanDegenerateLoopOutcomeSet,
    PlanarBooleanDeniedLoopCandidateSet, PlanarBooleanLoopClassifiedProductKind,
    PlanarBooleanLoopContainmentEvidencePosture, PlanarBooleanLoopContainmentEvidencePostureKind,
    PlanarBooleanLoopContainmentEvidencePostureSet, PlanarBooleanLoopIdentityBoundary,
    PlanarBooleanLoopIdentityMintingDenial, PlanarBooleanLoopIdentityMintingDenialKind,
    PlanarBooleanLoopIdentityMintingInput, PlanarBooleanLoopNamingAuthoritySupport,
    PlanarBooleanLoopRoleOutcome, PlanarBooleanLoopRoleOutcomeKind,
    PlanarBooleanLoopRoleOutcomeSet, PlanarBooleanSourceLoopSplitAttribution,
    PlanarBooleanSourceLoopSplitAttributionCounters, PlanarBooleanSourceLoopSplitAttributionKind,
    PlanarBooleanSourceLoopSplitAttributionRow,
};

#[test]
fn loop_identity_boundary_mints_for_admitted_reconstructed_loop_with_real_naming_support() {
    let fixture = admitted_loop_fixture();

    let boundary = mint_fixture_boundary(&fixture, &fixture.naming_support)
        .expect("admitted reconstructed loop should mint identities");

    assert_eq!(boundary.loop_identity_map().rows().len(), 1);
    assert!(!boundary.loop_identity_map().rows()[0]
        .canonical_loop_identity()
        .is_empty());
    assert_eq!(
        boundary.loop_identity_map().rows()[0].tracked_loop_identity(),
        fixture.reconstructed_loops.rows()[0].reconstructed_loop_identity()
    );
    assert!(!boundary.persistent_name_propagation_map().rows().is_empty());
    assert_eq!(
        boundary.persistent_name_propagation_map().rows().len(),
        boundary.subshape_signature_map().rows().len()
    );

    let counters = boundary.counters();
    assert_eq!(counters.admitted_loops_considered(), 1);
    assert_eq!(counters.denied_candidates_indexed(), 0);
    assert_eq!(
        counters.split_name_rows_indexed(),
        fixture.naming_support.persistent_name_rows().len()
    );
    assert_eq!(counters.loop_identities_minted(), 1);
    assert_eq!(
        counters.propagated_name_rows_emitted(),
        boundary.persistent_name_propagation_map().rows().len()
    );
    assert_eq!(
        counters.subshape_signature_rows_emitted(),
        boundary.subshape_signature_map().rows().len()
    );
    assert_eq!(counters.missing_name_seed_denials(), 0);
    assert_eq!(counters.foreign_lineage_denials(), 0);
}

#[test]
fn loop_identity_boundary_denies_admitted_loop_without_split_naming_seed() {
    let fixture = admitted_loop_fixture();
    let stripped_naming = fixture.prepared.subject.naming.with_rows_for_tests(
        Vec::new(),
        PlanarBooleanSplitPersistentNamingCounters::default(),
    );
    let naming_support =
        PlanarBooleanLoopNamingAuthoritySupport::admit_from_split_receipt_and_provenance(
            &stripped_naming,
            &fixture.prepared.source_provenance,
            &fixture.split_attribution,
        )
        .expect("empty naming receipt should still lower into support");

    let denial = mint_fixture_boundary(&fixture, &naming_support)
        .expect_err("admitted loop without a split naming seed must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopIdentityMintingDenialKind::MissingSplitNamingSeed
    );
    assert_eq!(denial.counters().admitted_loops_considered(), 1);
    assert_eq!(denial.counters().split_name_rows_indexed(), 0);
    assert_eq!(denial.counters().loop_identities_minted(), 1);
    assert_eq!(denial.counters().missing_name_seed_denials(), 1);
}

#[test]
fn loop_identity_boundary_denies_foreign_naming_lineage() {
    let fixture = admitted_loop_fixture();
    let mut persistent_name_rows = fixture
        .prepared
        .subject
        .naming
        .persistent_name_rows()
        .to_vec();
    let foreign_index = persistent_name_rows
        .iter()
        .position(|row| row.artifact_identity() == fixture.seed_artifact_identity)
        .expect("fixture should choose a real seed artifact from persistent naming");
    persistent_name_rows[foreign_index] = persistent_name_rows[foreign_index]
        .with_source_edge_identity_for_tests("foreign-source-edge");
    let foreign_naming = fixture.prepared.subject.naming.with_rebuilt_rows_for_tests(
        persistent_name_rows,
        fixture.prepared.subject.naming.counters(),
    );
    let naming_support =
        PlanarBooleanLoopNamingAuthoritySupport::admit_from_split_receipt_and_provenance(
            &foreign_naming,
            &fixture.prepared.source_provenance,
            &fixture.split_attribution,
        )
        .expect("foreign lineage should survive support lowering and deny during minting");

    let denial = mint_fixture_boundary(&fixture, &naming_support)
        .expect_err("foreign naming lineage must deny admitted loop minting");

    assert_eq!(
        denial.kind(),
        PlanarBooleanLoopIdentityMintingDenialKind::ForeignNamingLineage
    );
    assert_eq!(denial.counters().admitted_loops_considered(), 1);
    assert_eq!(denial.counters().loop_identities_minted(), 1);
    assert_eq!(
        denial.counters().split_name_rows_indexed(),
        foreign_naming.persistent_name_rows().len()
    );
    assert_eq!(denial.counters().foreign_lineage_denials(), 1);
}

struct AdmittedLoopFixture {
    prepared: PreparedLoopContinuationIndexSubject,
    reconstructed_loops: PlanarBooleanAdmittedReconstructedLoopSet,
    born_loops: PlanarBooleanBornLoopSet,
    role_outcomes: PlanarBooleanLoopRoleOutcomeSet,
    degenerate_outcomes: PlanarBooleanDegenerateLoopOutcomeSet,
    denied_loop_candidates: PlanarBooleanDeniedLoopCandidateSet,
    split_attribution: PlanarBooleanSourceLoopSplitAttribution,
    naming_support: PlanarBooleanLoopNamingAuthoritySupport,
    seed_artifact_identity: String,
}

fn admitted_loop_fixture() -> AdmittedLoopFixture {
    let prepared = prepared_loop_continuation_subject(LoopFixtureEntryOrder::Canonical);
    let request_identity = prepared.request.request_identity().to_string();
    let source_carrier = prepared
        .source_provenance
        .source_loop_carriers()
        .rows()
        .first()
        .expect("fixture should expose one source carrier");
    let fragment_identities = prepared
        .subject
        .naming
        .persistent_name_rows()
        .iter()
        .filter(|row| row.artifact_kind() == PlanarBooleanSplitNamedArtifactKind::SplitFragment)
        .take(2)
        .map(|row| row.artifact_identity().to_string())
        .collect::<Vec<_>>();
    let split_vertex_identities = prepared
        .subject
        .naming
        .persistent_name_rows()
        .iter()
        .filter(|row| row.artifact_kind() == PlanarBooleanSplitNamedArtifactKind::SplitVertex)
        .take(1)
        .map(|row| row.artifact_identity().to_string())
        .collect::<Vec<_>>();
    let seed_artifact_identity = fragment_identities
        .first()
        .cloned()
        .expect("fixture should emit at least one named split fragment");
    let loop_identity = "reconstructed-loop:phase-thirteen-admitted".to_string();
    let role_outcome_identity = "role-outcome:phase-thirteen-admitted".to_string();
    let containment_identity = "containment:phase-thirteen-admitted".to_string();
    let reconstructed_loops = PlanarBooleanAdmittedReconstructedLoopSet::new(
        "reconstructed-set:phase-thirteen-admitted".to_string(),
        request_identity.clone(),
        vec![PlanarBooleanAdmittedReconstructedLoop::new(
            loop_identity.clone(),
            "loop-candidate:phase-thirteen-admitted".to_string(),
            source_carrier.source_loop_identity().to_string(),
            source_carrier.source_face_identity().to_string(),
            "local-frame:phase-thirteen-admitted".to_string(),
            "precision-basis:phase-thirteen-admitted".to_string(),
            fragment_identities.clone(),
            split_vertex_identities.clone(),
        )],
    );
    let born_loops = PlanarBooleanBornLoopSet::new(
        "born-set:phase-thirteen-admitted".to_string(),
        request_identity.clone(),
        Vec::new(),
    );
    let role_outcomes = PlanarBooleanLoopRoleOutcomeSet::new(
        "role-set:phase-thirteen-admitted".to_string(),
        request_identity.clone(),
        vec![PlanarBooleanLoopRoleOutcome::new(
            role_outcome_identity.clone(),
            loop_identity.clone(),
            PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
            vec!["island:phase-thirteen-admitted".to_string()],
            vec![source_carrier.source_loop_identity().to_string()],
            Some(PlanarBooleanLoopRole::OuterBoundary),
            PlanarBooleanLoopRoleOutcomeKind::PreservedSourceRole,
        )],
    );
    let _containment = PlanarBooleanLoopContainmentEvidencePostureSet::new(
        "containment-set:phase-thirteen-admitted".to_string(),
        request_identity.clone(),
        vec![PlanarBooleanLoopContainmentEvidencePosture::new(
            containment_identity.clone(),
            loop_identity.clone(),
            PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
            vec!["island:phase-thirteen-admitted".to_string()],
            vec![source_carrier.source_loop_identity().to_string()],
            PlanarBooleanLoopContainmentEvidencePostureKind::PreservedSourceContainmentEvidence,
        )],
    );
    let degenerate_outcomes = PlanarBooleanDegenerateLoopOutcomeSet::new(
        "degenerate-set:phase-thirteen-admitted".to_string(),
        request_identity.clone(),
        vec![PlanarBooleanDegenerateLoopOutcome::new(
            "degenerate-outcome:phase-thirteen-admitted".to_string(),
            loop_identity.clone(),
            PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
            vec![source_carrier.source_loop_identity().to_string()],
            "local-frame:phase-thirteen-admitted".to_string(),
            "precision-basis:phase-thirteen-admitted".to_string(),
            fragment_identities,
            split_vertex_identities,
            Some(role_outcome_identity),
            Some(containment_identity),
            PlanarBooleanDegenerateLoopOutcomeKind::AdmittedForIdentityMinting,
            "fixture admits the reconstructed loop into phase thirteen".to_string(),
        )],
    );
    let denied_loop_candidates = PlanarBooleanDeniedLoopCandidateSet::new(
        "denied-loop-candidate-set:phase-thirteen-admitted".to_string(),
        request_identity.clone(),
        "walk-outcomes:phase-thirteen-admitted".to_string(),
        Vec::new(),
    );
    let split_attribution = PlanarBooleanSourceLoopSplitAttribution::new(
        "split-attribution:phase-thirteen-admitted".to_string(),
        request_identity,
        vec![PlanarBooleanSourceLoopSplitAttributionRow::new(
            "attribution:phase-thirteen-admitted".to_string(),
            source_carrier.source_loop_identity().to_string(),
            vec!["island:phase-thirteen-admitted".to_string()],
            PlanarBooleanSourceLoopSplitAttributionKind::Preserved,
        )],
        PlanarBooleanSourceLoopSplitAttributionCounters::default(),
    );
    let naming_support =
        PlanarBooleanLoopNamingAuthoritySupport::admit_from_split_receipt_and_provenance(
            &prepared.subject.naming,
            &prepared.source_provenance,
            &split_attribution,
        )
        .expect("fixture should lower real naming support");

    AdmittedLoopFixture {
        prepared,
        reconstructed_loops,
        born_loops,
        role_outcomes,
        degenerate_outcomes,
        denied_loop_candidates,
        split_attribution,
        naming_support,
        seed_artifact_identity,
    }
}

fn mint_fixture_boundary(
    fixture: &AdmittedLoopFixture,
    naming_support: &PlanarBooleanLoopNamingAuthoritySupport,
) -> Result<PlanarBooleanLoopIdentityBoundary, PlanarBooleanLoopIdentityMintingDenial> {
    PlanarBooleanLoopIdentityBoundary::mint(
        PlanarBooleanLoopIdentityMintingInput::from_phase_twelve_products_and_naming_support(
            &fixture.reconstructed_loops,
            &fixture.born_loops,
            &fixture.role_outcomes,
            &fixture.degenerate_outcomes,
            &fixture.denied_loop_candidates,
            naming_support,
            &fixture.split_attribution,
        ),
    )
}
