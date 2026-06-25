use super::super::primitive_construction_graph_obligation_residue_manifest;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::request::PrimitiveConstructionFamily;
use crate::construction::result::prepare_primitive_construction_result;
use crate::construction::specs::SimplexSolidSpec;
use crate::query_obligation_selection::selection_substrate::{
    QueryObligationSelectionInput, QueryObligationSelectionSubstrate,
};

use super::super::primitive_construction_touched_basis_fixture::primitive_construction_touched_basis_for_family;

#[test]
fn kernel_construction_query_substrate_selects_real_primitive_birth_obligation() {
    let declared_touched_basis =
        primitive_construction_touched_basis_for_family(PrimitiveConstructionFamily::SimplexSolid);
    let input =
        QueryObligationSelectionInput::from_topology_touched_basis(declared_touched_basis.proof())
            .expect("primitive touched basis should become selection input");
    let selected = QueryObligationSelectionSubstrate::select_execution_backed_obligations(input)
        .expect("primitive construction Query substrate selection");
    let proof = selected.query_proof();

    assert_eq!(
        proof.manifest().consumer_name(),
        "worth-kernel.query-obligation-selection.primitive-construction"
    );
    assert!(proof.execution_proof().has_real_executor_rows());
    assert_eq!(proof.execution_proof().selected_obligation_count(), 1);
    assert_eq!(proof.in_memory_proof().selected_obligation_count(), 1);
    assert_eq!(proof.local_ceremony_audit().evaluated_source_count(), 8);
    assert!(proof.local_ceremony_audit().is_clean());
    assert_eq!(proof.residue_manifest().rows().len(), 3);
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
    assert!(!proof
        .residue_manifest()
        .rows()
        .iter()
        .any(|row| row.class() == "kernel-birth-selector-conjunction-gap"));
}

#[test]
fn primitive_graph_obligation_closeout_is_execution_backed_not_in_memory_authority() {
    let declared_touched_basis =
        primitive_construction_touched_basis_for_family(PrimitiveConstructionFamily::SimplexSolid);
    let input =
        QueryObligationSelectionInput::from_topology_touched_basis(declared_touched_basis.proof())
            .expect("primitive touched basis should become selection input");
    let selected = QueryObligationSelectionSubstrate::select_execution_backed_obligations(input)
        .expect("primitive construction Query substrate selection");
    let proof = selected.query_proof();
    let residue = primitive_construction_graph_obligation_residue_manifest()
        .expect("kernel construction graph obligation residue manifest");

    assert_eq!(
        proof.manifest().consumer_name(),
        "worth-kernel.query-obligation-selection.primitive-construction"
    );
    assert_eq!(
        proof.manifest().residue_manifest_digest(),
        residue.manifest_digest()
    );
    assert!(proof.execution_proof().has_real_executor_rows());
    assert_eq!(proof.execution_proof().rows().len(), 1);
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

    assert!(!result.result_digest().is_empty());
    assert!(residue
        .rows()
        .iter()
        .any(|row| row.class() == "kernel-handoff-only-result-helper"));
}
