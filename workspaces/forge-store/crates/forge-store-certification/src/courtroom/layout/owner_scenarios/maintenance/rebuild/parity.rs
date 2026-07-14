use forge_store_layout_indexes::{
    layout_parity_verification, layout_rebuild_admission, layout_rebuild_candidate_readmission,
    layout_rebuild_execution, DerivedIndexCandidateDeclaration, DerivedIndexParityBasis,
    DerivedIndexParityRow, DerivedIndexRebuildReceipt, DerivedIndexRebuildSourceInput,
    ObserveOwnerCase,
};

use super::super::super::LayoutOwnerObservationLedger;
use super::fixture_inputs;

pub(super) fn execute(ledger: &mut LayoutOwnerObservationLedger) {
    let exact = root_execution(11);
    record(ledger, readmit_generated(exact));

    let execution = root_execution(11);
    let exact = execution.rebuilt_basis();
    let coverage_mismatch = coverage_mismatch(exact);
    record(ledger, readmit(execution, coverage_mismatch));

    let execution = root_execution(11);
    let foreign = root_execution(12);
    let exact = execution.rebuilt_basis();
    let key_mismatch = basis(
        vec![DerivedIndexParityRow::new(
            foreign.rebuilt_basis().ordered_rows()[0].key().clone(),
            exact.ordered_rows()[0].value_fingerprint(),
        )],
        exact,
        exact.counter_shape().to_vec(),
    );
    record(ledger, readmit(execution, key_mismatch));

    let execution = root_execution(11);
    let exact = execution.rebuilt_basis();
    let value_mismatch = basis(
        vec![DerivedIndexParityRow::new(
            exact.ordered_rows()[0].key().clone(),
            "sha256:hostile-rebuild-value",
        )],
        exact,
        exact.counter_shape().to_vec(),
    );
    record(ledger, readmit(execution, value_mismatch));

    let execution = root_execution(11);
    let exact = execution.rebuilt_basis();
    let counter_mismatch = basis(exact.ordered_rows().to_vec(), exact, vec![999]);
    record(ledger, readmit(execution, counter_mismatch));
}

fn root_execution(page: u64) -> DerivedIndexRebuildReceipt {
    let strategy = fixture_inputs::btree_strategy();
    let source = fixture_inputs::root_source(page);
    let materialization = fixture_inputs::root_materialization(&strategy, &source);
    let plan = layout_rebuild_admission()
        .admit_plan(fixture_inputs::root_request(
            &strategy,
            materialization,
            DerivedIndexRebuildSourceInput::PhysicalRootManifest { source },
        ))
        .into_admitted()
        .expect("ordinary rebuild must admit");
    layout_rebuild_execution().execute(plan).into_rebuilt()
}

fn basis(
    rows: Vec<DerivedIndexParityRow>,
    exact: &DerivedIndexParityBasis,
    counters: Vec<u64>,
) -> DerivedIndexParityBasis {
    DerivedIndexParityBasis::new(
        rows,
        exact.coverage().clone(),
        exact.cost_envelope_compliant(),
        counters,
    )
    .expect("hostile candidate remains canonically shaped")
}

fn coverage_mismatch(exact: &DerivedIndexParityBasis) -> DerivedIndexParityBasis {
    let lsm = fixture_inputs::lsm_materialization();
    DerivedIndexParityBasis::new(
        exact.ordered_rows().to_vec(),
        lsm.coverage().clone(),
        exact.cost_envelope_compliant(),
        exact.counter_shape().to_vec(),
    )
    .expect("production LSM coverage is a canonical hostile candidate")
}

fn readmit_generated(
    execution: DerivedIndexRebuildReceipt,
) -> forge_store_layout_indexes::DerivedIndexCandidateReadmissionReceipt {
    let declaration = execution.candidate_declaration();
    layout_rebuild_candidate_readmission().readmit(execution, declaration)
}

fn readmit(
    execution: DerivedIndexRebuildReceipt,
    candidate: DerivedIndexParityBasis,
) -> forge_store_layout_indexes::DerivedIndexCandidateReadmissionReceipt {
    layout_rebuild_candidate_readmission().readmit(
        execution,
        DerivedIndexCandidateDeclaration::from_canonical_basis(candidate),
    )
}

fn record(
    ledger: &mut LayoutOwnerObservationLedger,
    candidate: forge_store_layout_indexes::DerivedIndexCandidateReadmissionReceipt,
) {
    let outcome = layout_parity_verification().verify(candidate);
    ledger.record_derived_index_parity(outcome.owner_case_observation());
}
