use forge_query::facade::{ForgeQueryGraphObligationSupportLane, ForgeQueryRuntimeError};
use worth_primitives::PrimitiveWitnessDescriptor;

use super::support::{birth_synopsis, topology_workspace, BirthCounts};
use crate::construction::{
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    TopologyPrimitiveConstructionBirthFamily,
};

#[test]
fn shell_with_hole_layout_violation_denies_at_compose_graph_obligation() {
    let mut workspace = topology_workspace("shell-with-hole-layout-violation");
    let synopsis = birth_synopsis(
        TopologyPrimitiveConstructionBirthFamily::ShellWithHole,
        PrimitiveWitnessDescriptor::ShellWithHole {
            outer_loop_edge_count: 4,
            hole_loop_edge_counts: vec![4],
        },
        "planar_shell_layout_violation_body",
        BirthCounts {
            supported_vertex_count: 8,
            supported_edge_count: 8,
            supported_loop_count: 2,
            supported_wire_count: 0,
            supported_face_count: 1,
            supported_shell_count: 1,
            supported_body_count: 1,
        },
    );
    let handoff = prepare_primitive_construction_query_admitted_handoff_from_synopsis(
        &synopsis,
        "birth-completeness",
        "birth-mapping",
        2,
        1,
    )
    .expect("layout-violation synopsis should still reach compose for runtime denial");
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
    .expect_err("layout violation must be denied by runtime graph obligation");

    match error {
        super::super::TopologyPrimitiveConstructionBirthComposeExecutionError::Runtime(
            ForgeQueryRuntimeError::GraphObligationDenied(denial),
        ) => {
            let row = denial
                .rows()
                .first()
                .expect("runtime denial should carry the blocking obligation row");
            assert_eq!(
                row.verdict_context(),
                Some("selected-obligation-unsupported")
            );
            assert_eq!(
                row.rule_name(),
                "primitive-construction-birth-layout-violation.graph-obligation"
            );
            assert_eq!(
                row.support_lane(),
                ForgeQueryGraphObligationSupportLane::GraphComposition
            );
        }
        other => panic!("expected typed graph obligation denial, got {other:?}"),
    }
}
