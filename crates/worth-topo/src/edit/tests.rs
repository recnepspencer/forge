use forge_relational::facade::history::BranchId;
use worth_schema::facade::{
    seed_minimal_topology, WorthAspect, WorthDiagnosticsAspect, WorthNamingAspect,
    WorthTopologyAspect, WorthTopologyEntityKind, WorthTopologyRelationKind,
};

use crate::edit::{
    WorthBoundaryMembershipKind, WorthTopologyDerivedRegion, WorthTopologyEditApplicationMode,
    WorthTopologyEditBatch, WorthTopologyEditChangedScope, WorthTopologyEditContract,
    WorthTopologyEditFamily, WorthTopologyEditNamingOutcome, WorthTopologyEditNamingScope,
    WorthTopologyEditRunner,
};

#[test]
fn create_topology_entity_contract_is_topology_only_and_naming_aware() {
    let contract =
        WorthTopologyEditContract::create_topology_entity("m3.contract.vertex", WorthTopologyEntityKind::Vertex);

    assert_eq!(contract.family, WorthTopologyEditFamily::CreateTopologyEntity);
    assert!(contract
        .touched_aspects()
        .contains(&WorthAspect::Topology(WorthTopologyAspect::Structure)));
    assert!(contract
        .touched_aspects()
        .contains(&WorthAspect::Naming(WorthNamingAspect::PersistentName)));
    assert!(contract
        .touched_aspects()
        .contains(&WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions)));
    assert_eq!(
        contract.changed_scopes(),
        &[
            WorthTopologyEditChangedScope::Entity,
            WorthTopologyEditChangedScope::Naming,
        ]
    );
    assert_eq!(
        contract.naming_scopes(),
        &[WorthTopologyEditNamingScope::EditedEntityNames]
    );
    assert_eq!(
        contract.derived_regions(),
        &[
            WorthTopologyDerivedRegion::EditLocalNeighborhoodRegion,
            WorthTopologyDerivedRegion::NamingContinuityRegion,
        ]
    );
}

#[test]
fn boundary_membership_contract_exposes_boundary_scope_and_regions() {
    let contract = WorthTopologyEditContract::attach_boundary_membership(
        "m3.boundary.loop",
        WorthBoundaryMembershipKind::LoopOwnsHalfEdge,
        forge_relational::facade::identity::EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            1,
            1,
        ),
        forge_relational::facade::identity::EntityId::new(
            forge_relational::facade::identity::PartitionId::main(),
            2,
            1,
        ),
    );

    assert_eq!(contract.family, WorthTopologyEditFamily::AttachBoundaryMembership);
    assert!(contract
        .touched_aspects()
        .contains(&WorthAspect::Topology(WorthTopologyAspect::Boundary)));
    assert!(contract
        .changed_scopes()
        .contains(&WorthTopologyEditChangedScope::Loop));
    assert!(contract
        .derived_regions()
        .contains(&WorthTopologyDerivedRegion::LoopRegion));
}

#[test]
fn edit_runner_applies_topology_only_create_contract_through_authority() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let _seeded = seed_minimal_topology(&mut runtime, "m3-edit-mainline").expect("seed minimal topology");

    let batch = WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::create_topology_entity(
        "m3-edit-mainline.added_vertex",
        WorthTopologyEntityKind::Vertex,
    )])
    .expect("non-empty edit batch");

    let verified = WorthTopologyEditRunner::new(&mut runtime)
        .apply_traced(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("mainline local edit should commit")
        .into_primary_result();

    assert_eq!(verified.branch_id.0, "main");
    let read = runtime
        .read_truth()
        .read_snapshot(&verified.persisted_truth.snapshot)
        .expect("verified edit snapshot should remain readable");
    assert!(read.entities().iter().any(|record| {
        record
            .payload
            .as_json()
            .and_then(|json| json.get("label"))
            .and_then(|value| value.as_str())
            .is_some_and(|label| label == "m3-edit-mainline.added_vertex")
    }));
    assert!(read.entities().iter().any(|record| {
        record
            .payload
            .as_json()
            .and_then(|json| json.get("label"))
            .and_then(|value| value.as_str())
            .is_some_and(|label| label == "m3-edit-mainline.added_vertex.persistent_name")
    }));
}

#[test]
fn edit_runner_applies_branch_local_contract_on_real_branch() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let _seeded = seed_minimal_topology(&mut runtime, "m3-edit-branch").expect("seed minimal topology");
    runtime
        .history_authority()
        .create_branch(BranchId("feature".to_string()), &BranchId("main".to_string()))
        .expect("feature branch should be creatable");

    let batch = WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::create_topology_entity(
        "m3-edit-branch.added_vertex",
        WorthTopologyEntityKind::Vertex,
    )])
    .expect("non-empty edit batch");

    let verified = WorthTopologyEditRunner::new(&mut runtime)
        .apply_traced(
            batch,
            WorthTopologyEditApplicationMode::BranchLocal(BranchId("feature".to_string())),
        )
        .expect("branch-local edit should commit")
        .into_primary_result();

    assert_eq!(verified.branch_id.0, "feature");
    assert_eq!(verified.read_basis.branch_id().0, "feature");
}

#[test]
fn apply_and_inspect_surfaces_runtime_trace_for_create_entity_success() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let _seeded = seed_minimal_topology(&mut runtime, "m3-edit-inspect").expect("seed minimal topology");
    let batch = WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::create_topology_entity(
        "m3-edit-inspect.added_vertex",
        WorthTopologyEntityKind::Vertex,
    )])
    .expect("non-empty edit batch");

    let applied = WorthTopologyEditRunner::new(&mut runtime)
        .apply_and_inspect_traced(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("create inspection should remain runtime-valid")
        .into_primary_result();

    assert_eq!(applied.naming_report.rows.len(), 1);
    assert_eq!(
        applied.naming_report.rows[0].outcome,
        WorthTopologyEditNamingOutcome::Preserved
    );
    assert_eq!(applied.derived_validation_report.rows.len(), 5);
}

#[test]
fn apply_and_inspect_rejects_identity_collision_against_existing_truth() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let _seeded =
        seed_minimal_topology(&mut runtime, "m3-edit-collision").expect("seed minimal topology");

    let batch = WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::create_topology_entity(
        "m3-edit-collision.vertex",
        WorthTopologyEntityKind::Vertex,
    )])
    .expect("non-empty edit batch");

    let error = WorthTopologyEditRunner::new(&mut runtime)
        .apply_and_inspect_traced(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect_err("identity-colliding create should surface a traced failure");

    let trace = error.trace().expect("runtime trace should be preserved");
    assert_eq!(trace.mode, WorthTopologyEditApplicationMode::Mainline);
    assert_eq!(trace.families, vec![WorthTopologyEditFamily::CreateTopologyEntity]);
    assert_eq!(trace.naming_report.rows.len(), 1);
    assert!(trace.verified_commit.is_none());
    assert_eq!(
        trace.naming_report.rows[0].outcome,
        WorthTopologyEditNamingOutcome::Rejected
    );
    assert!(trace.decision_trace.is_some());
    assert!(trace.integrity_markers.is_some());
    assert!(trace.performance_accounting.is_some());

    let authority = error
        .authority_error()
        .expect("collision should surface through the authority boundary");
    assert!(matches!(
        authority,
        worth_schema::facade::WorthTopologyAuthorityError::DuplicateLiveEntityLabel(_)
    ));
}

#[test]
fn apply_returns_verified_commit_with_runtime_commit_evidence() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let seeded =
        seed_minimal_topology(&mut runtime, "m3-edit-delete").expect("seed minimal topology");

    let batch = WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::retire_topology_entity(
        seeded.vertex,
        WorthTopologyEntityKind::Vertex,
    )])
    .expect("non-empty edit batch");

    let verified = WorthTopologyEditRunner::new(&mut runtime)
        .apply_traced(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("delete currently commits through the authority boundary")
        .into_primary_result();

    assert_eq!(verified.branch_id.0, "main");
    assert_eq!(verified.commits.len(), 1);
    assert!(verified.commits[0].commit_summary().phase_count > 0);
    assert!(verified.commits[0].history_summary().is_some());
    assert!(verified.commits[0].publication_summary().is_some());
}

#[test]
fn apply_and_inspect_reports_ambiguous_naming_for_local_rewire_family() {
    let mut runtime = crate::facade::worth_milestone_one_runtime_builder()
        .expect("worth milestone one runtime builder")
        .build();
    let seeded = seed_minimal_topology(&mut runtime, "m3-edit-rewire").expect("seed minimal topology");
    let read_view = runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .expect("seeded snapshot should remain readable");
    let next_relation = read_view
        .relations()
        .iter()
        .find(|relation| {
            relation.kind.kind_id
                == worth_schema::facade::WorthRelationKind::Topology(WorthTopologyRelationKind::HalfEdgeNext)
                    .kind_id()
        })
        .expect("seeded topology should contain one half-edge next relation")
        .relation_id;

    let batch = WorthTopologyEditBatch::new(vec![WorthTopologyEditContract::rewire_loop_successor(
        next_relation,
        crate::edit::WorthLoopSuccessorKind::Next,
        seeded.half_edge,
        seeded.half_edge,
    )])
    .expect("non-empty edit batch");

    let applied = WorthTopologyEditRunner::new(&mut runtime)
        .apply_and_inspect_traced(batch, WorthTopologyEditApplicationMode::Mainline)
        .expect("self-rewire should remain valid and inspectable")
        .into_primary_result();

    assert_eq!(applied.naming_report.rows.len(), 1);
    assert_eq!(
        applied.naming_report.rows[0].outcome,
        WorthTopologyEditNamingOutcome::Ambiguous
    );
}
