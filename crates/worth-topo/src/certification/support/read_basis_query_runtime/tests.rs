use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::HistoricalReadBasisQueryRuntime;
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::validation::reference_integrity::milestone_one_runtime_builder;

#[test]
fn historical_read_basis_query_runtime_opens_one_staged_query_boundary() {
    let mut runtime = milestone_one_runtime_builder()
        .expect(" milestone one runtime builder")
        .build();
    let verified = seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        "historical-read-basis-query-runtime",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("verified primitive");

    let mut query_runtime = HistoricalReadBasisQueryRuntime::open(
        &runtime,
        verified.read_basis().clone(),
        ".historical-read-basis.query-runtime",
    )
    .expect("read-basis query runtime should open");
    let surface_evidence = query_runtime
        .query_surface_evidence()
        .expect("surface evidence should be ready");

    assert_eq!(
        surface_evidence.validation_state().kind().as_str(),
        "ready",
        "validation surface should already be query-ready inside the shared read-basis runtime seam",
    );
    assert_eq!(
        surface_evidence.equivalence_state().kind().as_str(),
        "ready",
        "equivalence surface should already be query-ready inside the shared read-basis runtime seam",
    );
    assert!(
        surface_evidence
            .validation_inspection()
            .materialized_row_count()
            <= 1,
        "validation inspection should stay scoped to one read-basis query surface",
    );
    assert!(
        surface_evidence
            .equivalence_inspection()
            .materialized_row_count()
            <= 1,
        "equivalence inspection should stay scoped to one read-basis query surface",
    );
    assert_eq!(
        query_runtime.read_basis().snapshot(),
        verified.read_basis().snapshot(),
        "shared read-basis runtime seam should preserve the requested snapshot authority",
    );
}
