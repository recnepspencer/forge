use worth_primitives::PrimitiveWitnessDescriptor;

use super::support::{birth_synopsis, BirthCounts};
use crate::construction::{
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    TopologyPrimitiveConstructionBirthFamily,
};
use crate::facade::TopologyRuntimeAdapters;
use crate::projection::runtime_boundary::query_runtime::topology_runtime_without_primitive_birth_compose_obligation;
use crate::test_support::primitive_corpus::validated_topology::build_test_runtime;

#[test]
fn admitted_birth_compose_fails_closed_without_primitive_birth_obligation_registration() {
    let runtime = build_test_runtime().expect("topology relational test runtime");
    let mut workspace = topology_runtime_without_primitive_birth_compose_obligation(
        TopologyRuntimeAdapters::current_head(runtime),
        "primitive-birth-compose.missing-registration",
    )
    .expect("topology query runtime workspace");
    let counts = BirthCounts {
        supported_vertex_count: 8,
        supported_edge_count: 8,
        supported_loop_count: 1,
        supported_wire_count: 1,
        supported_face_count: 0,
        supported_shell_count: 0,
        supported_body_count: 1,
    };
    let synopsis = birth_synopsis(
        TopologyPrimitiveConstructionBirthFamily::WireBody,
        PrimitiveWitnessDescriptor::WireBody { edge_count: 8 },
        "planar_wire_body",
        counts,
    );
    let handoff = prepare_primitive_construction_query_admitted_handoff_from_synopsis(
        &synopsis,
        "birth-completeness",
        "birth-mapping",
        counts.supported_loop_count,
        counts.supported_body_count,
    )
    .expect("family synopsis should admit to topology handoff");
    let declared_touched_basis =
        super::super::TopologyPrimitiveConstructionBirthDeclaredTouchedBasis::from_admitted_handoff(
            &handoff,
        )
        .expect("admitted construction handoff should lower to touched basis");

    let error = super::super::execute_primitive_construction_birth_compose(
        &mut workspace,
        handoff,
        declared_touched_basis,
    )
    .expect_err("compose must fail closed when primitive birth obligation is not registered");

    assert!(matches!(
        error,
        super::super::TopologyPrimitiveConstructionBirthComposeExecutionError::MissingGraphObligationEvidence {
            family: TopologyPrimitiveConstructionBirthFamily::WireBody
        }
    ));
}
