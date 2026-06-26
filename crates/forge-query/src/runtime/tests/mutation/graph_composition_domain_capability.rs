use super::super::support::*;
use crate::domain_capabilities::{
    admit_eligible_domain_capability_contribution,
    evaluate_requested_domain_capability_contribution,
    materialize_graph_composition_domain_invariant_denial,
    prepare_admitted_domain_capability_contribution_for_materialization,
    ForgeQueryInvariantCapabilityContributionAuthoring,
};
use forge_proof::TransitionOutcome;

#[test]
fn compose_graph_with_domain_invariant_denial_accepts_contributed_denial_artifact() {
    let mut workspace = stateful_bridge_task_edge_runtime()
        .workspace("tasks.graph-composition-domain-capability-denial")
        .expect("runtime should open a named workspace");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view(
            "tasks.graph-composition-domain-capability-denial-tasks",
            |q| {
                q.from("Task")
                    .select([
                        crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                            .unwrap(),
                        crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                            .unwrap(),
                    ])
                    .order_by(
                        crate::authoring::AspectFieldKey::from_authoring_parts("title", "value")
                            .unwrap(),
                    )
                    .schema_basis("tasks-graph-composition-domain-capability-denial-tasks")
            },
        )
        .expect("task live view should declare");
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view(
            "tasks.graph-composition-domain-capability-denial-edges",
            |q| {
                q.from("TaskEdge")
                    .select([
                        crate::authoring::AspectFieldKey::from_authoring_parts("edge", "kind")
                            .unwrap(),
                        crate::authoring::AspectFieldKey::from_authoring_parts(
                            "edge",
                            "source_identity",
                        )
                        .unwrap(),
                        crate::authoring::AspectFieldKey::from_authoring_parts(
                            "edge",
                            "target_identity",
                        )
                        .unwrap(),
                    ])
                    .order_by(
                        crate::authoring::AspectFieldKey::from_authoring_parts("edge", "kind")
                            .unwrap(),
                    )
                    .schema_basis("tasks-graph-composition-domain-capability-denial-edges")
            },
        )
        .expect("edge live view should declare");

    let denial = success(materialize_graph_composition_domain_invariant_denial(
        ready_lower_runtime_invariant(
            ForgeQueryInvariantCapabilityContributionAuthoring::graph_invariant_denial(
                "non_manifold_topology",
                ["HalfEdge", "HalfEdgeNextRelation"],
                ["draft-half-edge"],
                ["mixed_existing_and_symbolic_entity_identity_edges"],
                ["mixed_existing_target_verified_retarget"],
                "program-digest",
                "breadth-digest",
                "components=2;symbolic_entities=1;symbolic_relations=0;declared_collections=2;declared_symbols=1;target_combinations=1;lifecycle_families=1",
                "topology.non_manifold_edge_split",
                "loop successor rewire would create a non-manifold adjacency fanout",
            ),
        ),
    ));
    let expected_digest = denial.denial_digest().to_string();

    let error = workspace
        .compose_graph_with_domain_invariant_denial(
            |graph| {
                let task = graph.insert_entity("draft-task", "Task", |task| {
                    task.set_aspect(
                        test_aspect_touch("identity.id"),
                        test_authored_string_aspect_value("task-draft"),
                    )
                    .set_aspect(
                        test_aspect_touch("title.value"),
                        test_authored_string_aspect_value("Draft task"),
                    )
                })?;
                graph.insert_relation("TaskEdge", |edge| {
                    edge.set_aspect(
                        test_aspect_touch("edge.kind"),
                        test_authored_string_aspect_value("depends_on"),
                    )
                    .symbolic_entity_identity(test_aspect_touch("edge.source_identity"), &task)
                    .existing_entity_identity(
                        test_aspect_touch("edge.target_identity"),
                        test_entity_identity("task-existing"),
                    )
                })?;
                Ok(())
            },
            |_context| Err(denial),
        )
        .expect_err("contributed invariant denial should stop graph composition");

    match error {
        ForgeQueryRuntimeError::GraphCompositionDomainInvariantDenied(denial) => {
            assert_eq!(denial.invariant_family(), "non_manifold_topology");
            assert_eq!(denial.hook_family(), "domain_invariant_pack_hook");
            assert_eq!(denial.denial_digest(), expected_digest);
            assert_eq!(
                denial.domain_invariant_summary().declared_collections(),
                &["HalfEdge".to_string(), "HalfEdgeNextRelation".to_string()]
            );
            assert_eq!(
                denial
                    .domain_invariant_summary()
                    .target_combination_families(),
                &["mixed_existing_and_symbolic_entity_identity_edges".to_string()]
            );
        }
        other => panic!("expected contributed domain invariant denial, got {other:?}"),
    }
}

fn ready_lower_runtime_invariant(
    authoring: ForgeQueryInvariantCapabilityContributionAuthoring,
) -> crate::domain_capabilities::ForgeQueryMaterializationReadyInvariantCapabilityContribution<
    crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
> {
    let target = lower_runtime_target("boundary-graph");
    let requested = authoring.bind_to_lower_runtime_boundary_target(target.clone());
    let eligible = success(evaluate_requested_domain_capability_contribution(requested));
    let admitted = success(admit_eligible_domain_capability_contribution(eligible));
    success(prepare_admitted_domain_capability_contribution_for_materialization(admitted, target))
}

fn lower_runtime_target(
    label: &str,
) -> crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget {
    crate::domain_capabilities::ForgeQueryLowerRuntimeBoundaryBoundContributionTarget::for_lower_runtime_boundary_envelope(
        &lower_runtime_envelope(label),
    )
}

fn lower_runtime_envelope(
    label: &str,
) -> crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryEnvelope {
    let subject_identity =
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSubjectIdentity::compose(
            "graph-composition-domain-capability-fixture-subject",
        )
        .field_value(
            crate::evidence_identity::ForgeQueryEvidenceTag::new("fixture"),
            label,
        )
        .seal();
    let request = crate::lower_runtime_routing::ForgeQueryLowerRuntimeCapabilityRequest::new(
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeAuthorityOwner::Query,
        "signal-invalidation-routing",
        subject_identity,
    );
    let detail_identity = crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
        crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
    )
    .field_value(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("fixture_detail"),
        label,
    )
    .seal();
    let eligibility =
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
            request,
            &detail_identity,
        );
    let route = crate::lower_runtime_routing::ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility,
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
            "graph-composition-domain-capability-fixture-route",
            &detail_identity,
        ),
    );
    let retained_evidence =
        crate::lower_runtime_routing::forge_query_lower_runtime_retained_evidence_identity(
            "graph-composition-domain-capability-fixture",
            &crate::evidence_identity::ForgeQueryEvidenceIdentity::compose(
                crate::evidence_identity::ForgeQueryEvidenceScope::LowerRuntimeBoundaryEvidence,
            )
            .field_value(
                crate::evidence_identity::ForgeQueryEvidenceTag::new("fixture_retained"),
                label,
            )
            .seal(),
        );
    let boundary =
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
            &route,
            &retained_evidence,
        );

    crate::lower_runtime_routing::ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey::SignalInvalidationRouting,
        &route,
        &boundary,
        &retained_evidence,
    )
}

fn success<T, D, S, R, F, O>(outcome: TransitionOutcome<T, D, S, R, F, O>) -> T
where
    D: std::fmt::Debug,
    S: std::fmt::Debug,
    R: std::fmt::Debug,
    F: std::fmt::Debug,
{
    match outcome {
        TransitionOutcome::Success(value) => value,
        TransitionOutcome::Denied(denial) => panic!("expected success, got denial: {denial:?}"),
        TransitionOutcome::Stale(stale) => panic!("expected success, got stale: {stale:?}"),
        TransitionOutcome::RebindRequired(rebind) => {
            panic!("expected success, got rebind-required: {rebind:?}")
        }
        _ => panic!("expected success, got non-success transition outcome"),
    }
}
