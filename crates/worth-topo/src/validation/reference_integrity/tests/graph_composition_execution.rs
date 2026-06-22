use super::*;
use forge_relational::facade::runtime::{
    InvariantCostClass, InvariantExecutionPoint, InvariantExecutionResult, InvariantFailureEffect,
    InvariantReportedRule, InvariantVerdict,
};

#[test]
fn graph_composition_denies_real_loop_wiring_rule_before_commit_backstop() {
    let runtime = milestone_one_runtime_builder()
        .expect("milestone one runtime builder")
        .build();
    let plan = merged_plan_from_raw_intent(91, broken_loop_wiring_intent("graph.loop"));

    let graph_result = runtime.validation().graph_composition_plan(&plan);
    let commit_result = runtime.validation().commit_boundary_plan(&plan);

    assert_custom_rule_violation(
        &graph_result,
        ".m1.topology.loop_wiring",
        InvariantExecutionPoint::GraphComposition,
        InvariantCostClass::Touched,
    );
    assert_custom_rule_violation(
        &commit_result,
        ".m1.topology.loop_wiring",
        InvariantExecutionPoint::CommitBoundary,
        InvariantCostClass::Global,
    );
}

#[test]
fn graph_composition_denies_real_ownership_rule_before_commit_backstop() {
    let runtime = milestone_one_runtime_builder()
        .expect("milestone one runtime builder")
        .build();
    let plan = merged_plan_from_raw_intent(92, missing_owner_intent("graph.owner"));

    let graph_result = runtime.validation().graph_composition_plan(&plan);
    let commit_result = runtime.validation().commit_boundary_plan(&plan);

    assert_custom_rule_violation(
        &graph_result,
        ".m1.topology.ownership_surface",
        InvariantExecutionPoint::GraphComposition,
        InvariantCostClass::Touched,
    );
    assert_custom_rule_violation(
        &commit_result,
        ".m1.topology.ownership_surface",
        InvariantExecutionPoint::CommitBoundary,
        InvariantCostClass::Global,
    );
}

fn assert_custom_rule_violation(
    result: &InvariantExecutionResult,
    rule_id: &str,
    execution_point: InvariantExecutionPoint,
    max_cost: InvariantCostClass,
) {
    assert_eq!(result.metadata().execution_point(), execution_point);
    assert_eq!(result.metadata().max_cost(), max_cost);
    assert!(
        result.results().iter().any(|check| {
            check.execution_point == execution_point
                && check.failure_effect == InvariantFailureEffect::BlockCommit
                && matches!(&check.rule, InvariantReportedRule::Custom(identity) if identity.rule_id.as_str() == rule_id)
                && matches!(check.verdict, InvariantVerdict::Violation(_))
        }),
        "expected {rule_id} violation at {execution_point:?}, got {:?}",
        result.results()
    );
}

fn broken_loop_wiring_intent(stem: &str) -> RawTopologyIntent {
    let topology_keys = [
        format!("{stem}.model"),
        format!("{stem}.body"),
        format!("{stem}.lump"),
        format!("{stem}.region"),
        format!("{stem}.shell"),
        format!("{stem}.face"),
        format!("{stem}.loop"),
        format!("{stem}.wire"),
        format!("{stem}.he"),
        format!("{stem}.edge"),
        format!("{stem}.vertex"),
    ];
    let topology_key_refs = topology_keys.iter().map(String::as_str).collect::<Vec<_>>();

    RawTopologyIntent::new(
        vec![
            entity(&format!("{stem}.model"), TopologyEntityKind::Model),
            entity(&format!("{stem}.body"), TopologyEntityKind::Body),
            entity(&format!("{stem}.lump"), TopologyEntityKind::Lump),
            entity(&format!("{stem}.region"), TopologyEntityKind::Region),
            entity(&format!("{stem}.shell"), TopologyEntityKind::Shell),
            entity(&format!("{stem}.face"), TopologyEntityKind::Face),
            entity(&format!("{stem}.loop"), TopologyEntityKind::Loop),
            entity(&format!("{stem}.wire"), TopologyEntityKind::Wire),
            entity(&format!("{stem}.he"), TopologyEntityKind::HalfEdge),
            entity(&format!("{stem}.edge"), TopologyEntityKind::Edge),
            entity(&format!("{stem}.vertex"), TopologyEntityKind::Vertex),
            relation(
                &format!("{stem}.model.owns_body"),
                TopologyRelationKind::ModelOwnsBody,
                &format!("{stem}.model"),
                &format!("{stem}.body"),
            ),
            relation(
                &format!("{stem}.body.owns_lump"),
                TopologyRelationKind::BodyOwnsLump,
                &format!("{stem}.body"),
                &format!("{stem}.lump"),
            ),
            relation(
                &format!("{stem}.lump.owns_region"),
                TopologyRelationKind::LumpOwnsRegion,
                &format!("{stem}.lump"),
                &format!("{stem}.region"),
            ),
            relation(
                &format!("{stem}.region.owns_shell"),
                TopologyRelationKind::RegionOwnsShell,
                &format!("{stem}.region"),
                &format!("{stem}.shell"),
            ),
            relation(
                &format!("{stem}.shell.owns_face"),
                TopologyRelationKind::ShellOwnsFace,
                &format!("{stem}.shell"),
                &format!("{stem}.face"),
            ),
            relation(
                &format!("{stem}.face.outer_loop"),
                TopologyRelationKind::FaceOuterLoop,
                &format!("{stem}.face"),
                &format!("{stem}.loop"),
            ),
            relation(
                &format!("{stem}.loop.owns_he"),
                TopologyRelationKind::LoopOwnsHalfEdge,
                &format!("{stem}.loop"),
                &format!("{stem}.he"),
            ),
            relation(
                &format!("{stem}.wire.owns_he"),
                TopologyRelationKind::WireOwnsHalfEdge,
                &format!("{stem}.wire"),
                &format!("{stem}.he"),
            ),
            relation(
                &format!("{stem}.he.radial"),
                TopologyRelationKind::HalfEdgeRadialNext,
                &format!("{stem}.he"),
                &format!("{stem}.he"),
            ),
            relation(
                &format!("{stem}.he.edge"),
                TopologyRelationKind::HalfEdgeUsesEdge,
                &format!("{stem}.he"),
                &format!("{stem}.edge"),
            ),
            relation(
                &format!("{stem}.he.start"),
                TopologyRelationKind::HalfEdgeStartsAtVertex,
                &format!("{stem}.he"),
                &format!("{stem}.vertex"),
            ),
            relation(
                &format!("{stem}.he.end"),
                TopologyRelationKind::HalfEdgeEndsAtVertex,
                &format!("{stem}.he"),
                &format!("{stem}.vertex"),
            ),
        ]
        .into_iter()
        .chain(naming_bundle(&topology_key_refs))
        .collect(),
        MutationOrigin::LocalEdit,
    )
}

fn missing_owner_intent(stem: &str) -> RawTopologyIntent {
    let topology_keys = [format!("{stem}.model"), format!("{stem}.body")];
    let topology_key_refs = topology_keys.iter().map(String::as_str).collect::<Vec<_>>();

    RawTopologyIntent::new(
        vec![
            entity(&format!("{stem}.model"), TopologyEntityKind::Model),
            entity(&format!("{stem}.body"), TopologyEntityKind::Body),
        ]
        .into_iter()
        .chain(naming_bundle(&topology_key_refs))
        .collect(),
        MutationOrigin::LocalEdit,
    )
}
