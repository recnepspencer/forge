use forge_query::facade::{
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationSupportLane,
};

use super::{
    primitive_construction_birth_touch_descriptor,
    primitive_construction_graph_obligation_adoption_proof,
    primitive_construction_graph_obligation_audit_sources,
    primitive_construction_graph_obligation_catalog,
    primitive_construction_graph_obligation_execution_matrix,
    primitive_construction_graph_obligation_local_ceremony_audit,
    primitive_construction_graph_obligation_replay_pair,
    primitive_construction_graph_obligation_residue_manifest,
    primitive_construction_graph_obligation_selector_coverage,
    primitive_construction_graph_obligation_selector_precision_matrix,
    primitive_construction_graph_obligation_support_matrix,
    primitive_construction_graph_obligation_support_pin,
    primitive_construction_phase_eighteen_family_count_gap,
    PHASE_EIGHTEEN_SPEC_PRIMITIVE_FAMILY_COUNT,
};
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::request::PRIMITIVE_CONSTRUCTION_FAMILIES;
use crate::construction::result::prepare_primitive_construction_result;
use crate::construction::specs::SimplexSolidSpec;

#[test]
fn kernel_construction_catalog_covers_current_primitive_family_set_exactly() {
    let catalog = primitive_construction_graph_obligation_catalog();
    let catalog_families = catalog
        .rows()
        .iter()
        .map(|row| row.family())
        .collect::<Vec<_>>();

    assert_eq!(catalog_families, PRIMITIVE_CONSTRUCTION_FAMILIES);
    assert_eq!(catalog.rows().len(), PRIMITIVE_CONSTRUCTION_FAMILIES.len());
    assert_eq!(PHASE_EIGHTEEN_SPEC_PRIMITIVE_FAMILY_COUNT, 7);
    assert_eq!(primitive_construction_phase_eighteen_family_count_gap(), 1);
    assert!(catalog
        .rows()
        .iter()
        .all(|row| row.descriptor_source().contains("compose")));
}

#[test]
fn kernel_construction_adoption_proof_selects_real_primitive_birth_obligation() {
    let proof = primitive_construction_graph_obligation_adoption_proof()
        .expect("kernel construction adoption proof");

    assert_eq!(
        proof.manifest().consumer_name(),
        "worth-kernel.primitive-construction"
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 1);
    assert_eq!(proof.local_ceremony_audit().evaluated_source_count(), 9);
    assert!(proof.local_ceremony_audit().is_clean());
    assert_eq!(proof.residue_manifest().rows().len(), 4);
    assert!(proof
        .residue_manifest()
        .rows()
        .iter()
        .any(|row| row.class() == "kernel-handoff-only-result-helper"));
    assert!(proof
        .residue_manifest()
        .rows()
        .iter()
        .any(|row| row.class() == "kernel-motion-preflight-sequencing"));
    assert!(proof.residue_manifest().rows().iter().any(|row| row.class()
        == "kernel-primitive-family-cardinality-gap"
        && row.current_count() == 1));
    assert!(proof
        .residue_manifest()
        .rows()
        .iter()
        .any(|row| row.class() == "kernel-birth-selector-conjunction-gap"));
}

#[test]
fn milestone_9_9_graph_obligation_kernel_closeout_is_certifiable_by_query_kit() {
    let proof = primitive_construction_graph_obligation_adoption_proof()
        .expect("kernel construction graph obligation adoption proof");
    let residue = primitive_construction_graph_obligation_residue_manifest()
        .expect("kernel construction graph obligation residue manifest");

    assert_eq!(
        proof.manifest().consumer_name(),
        "worth-kernel.primitive-construction"
    );
    assert_eq!(
        proof.manifest().residue_manifest_digest(),
        residue.manifest_digest()
    );
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 1);
    assert!(proof.local_ceremony_audit().is_evaluated());
    assert!(proof.local_ceremony_audit().is_clean());
    assert!(residue.rows().iter().all(|row| {
        !row.introduced_in().is_empty()
            && row.current_count() <= row.must_not_exceed_count()
            && !row.removal_trigger().is_empty()
    }));
}

#[test]
fn kernel_construction_support_pin_matches_phase_eighteen_support_matrix() {
    primitive_construction_graph_obligation_support_pin()
        .evaluate(&primitive_construction_graph_obligation_support_matrix())
        .expect("primitive construction birth support pin should match matrix");
}

#[test]
fn kernel_construction_selector_coverage_matches_registration_selector() {
    let coverage = primitive_construction_graph_obligation_selector_coverage();
    let catalog = primitive_construction_graph_obligation_catalog();
    let selector_digest = primitive_construction_birth_touch_descriptor()
        .expect("descriptor")
        .descriptor_digest()
        .to_string();

    assert_eq!(coverage.row_count(), PRIMITIVE_CONSTRUCTION_FAMILIES.len());
    assert!(catalog.rows().iter().all(|row| {
        row.registration().support_posture().lane()
            == ForgeQueryGraphObligationSupportLane::GraphComposition
    }));
    assert!(selector_digest.starts_with("forge.query.evidence"));
}

#[test]
fn kernel_construction_selector_precision_matrix_documents_current_collection_boundary() {
    let rows = primitive_construction_graph_obligation_selector_precision_matrix()
        .expect("selector precision matrix");

    assert_eq!(rows.len(), 6);
    assert!(rows.iter().all(|row| !row.descriptor_digest().is_empty()));
    assert!(rows
        .iter()
        .all(|row| row.selected_count() == row.expected_selected_count()));
    assert_eq!(
        rows.iter()
            .filter(|row| row.selected_count() == 0)
            .map(|row| row.label())
            .collect::<Vec<_>>(),
        vec!["unrelated-collection"]
    );
}

#[test]
fn handoff_only_result_remains_visible_residue_not_covered_execution() {
    let result = prepare_primitive_construction_result(PrimitiveConstructionIntent::simplex_solid(
        SimplexSolidSpec {
            scale: 1.0,
            auxiliary_altitude_component: 1.0,
        },
    ))
    .expect("handoff-only construction result");
    let residue = primitive_construction_graph_obligation_residue_manifest()
        .expect("kernel residue manifest");

    assert!(result.topology_compose_evidence().is_none());
    assert!(residue
        .rows()
        .iter()
        .any(|row| row.class() == "kernel-handoff-only-result-helper"));
}

#[test]
fn executed_result_and_outcome_matrix_covers_every_current_family() {
    let rows = primitive_construction_graph_obligation_execution_matrix();
    let executed_families = rows.iter().map(|row| row.family()).collect::<Vec<_>>();

    assert_eq!(executed_families, PRIMITIVE_CONSTRUCTION_FAMILIES);
    assert_eq!(rows.len(), PRIMITIVE_CONSTRUCTION_FAMILIES.len());
    for row in rows {
        assert_eq!(row.selected_count(), 1);
        assert!(!row.result_digest().is_empty());
        assert!(!row.outcome_digest().is_empty());
        assert!(!row.evidence_digest().is_empty());
        assert!(!row.envelope_digest().is_empty());
        assert!(!row.selected_row_digest().is_empty());
        assert!(!row.rule_identity_digest().is_empty());
        assert_eq!(
            row.obligation_kind(),
            ForgeQueryGraphObligationKind::AdvisoryObligation
        );
        assert_eq!(
            row.support_lane(),
            ForgeQueryGraphObligationSupportLane::GraphComposition
        );
        assert_eq!(
            row.execution_status(),
            Some(ForgeQueryGraphObligationExecutionStatus::Executed)
        );
        assert_eq!(row.verdict(), "advise");
        assert_eq!(row.verdict_context(), Some("advisory-obligation-selected"));
        assert!(row.has_authoritative_dispatch_identity());
    }
}

#[test]
fn executed_obligation_selection_replays_to_same_authoritative_row_identity() {
    for family in PRIMITIVE_CONSTRUCTION_FAMILIES {
        let (first, second) = primitive_construction_graph_obligation_replay_pair(family);

        assert_eq!(first.selected_count(), 1);
        assert_eq!(second.selected_count(), 1);
        assert_eq!(first.selected_row_digest(), second.selected_row_digest());
        assert_eq!(first.rule_identity_digest(), second.rule_identity_digest());
        assert_eq!(first.obligation_kind(), second.obligation_kind());
        assert_eq!(first.support_lane(), second.support_lane());
        assert_eq!(first.execution_status(), second.execution_status());
        assert_eq!(first.verdict(), second.verdict());
        assert_eq!(first.verdict_context(), second.verdict_context());
    }
}

#[test]
fn local_ceremony_audit_rejects_seeded_kernel_shadow_authority() {
    let seeded = primitive_construction_graph_obligation_audit_sources().source_file(
        "seeded.local-legality",
        "seeded.rs",
        "fn bypass() { local_legality_graph(); }",
    );
    let audit =
        forge_query::facade::consumer_kit::ForgeQueryGraphObligationLocalCeremonyAudit::evaluate(
            &seeded,
        );

    assert!(
        !primitive_construction_graph_obligation_local_ceremony_audit()
            .findings()
            .iter()
            .any(|finding| finding.pattern() == "local_legality_graph")
    );
    assert!(audit
        .findings()
        .iter()
        .any(|finding| finding.pattern() == "local_legality_graph"));
}

#[test]
fn motion_admission_support_contains_no_unreachable_sequencing_folklore() {
    let audit_sources = primitive_construction_graph_obligation_audit_sources();

    assert!(audit_sources
        .sources()
        .iter()
        .filter(|source| source.label().contains("compound-lowering"))
        .all(|source| !source.source().contains("unreachable!")));
}
