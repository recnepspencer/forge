use crate::facade::{BranchId, LineageResolutionStatus};

// CONTRACT: lineage
// LANES: success, failure_boundary, determinism

#[test]
fn lineage_contract_correspondence_stays_advisory_until_promoted() {
    let mut runtime = super::runtime_with_test_schema();
    let first = super::create_entity_outcome(&mut runtime, "left");
    let second = super::create_entity_outcome(&mut runtime, "right");
    let left_lineage = runtime
        .lineage_for_record(super::changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let right_lineage = runtime
        .lineage_for_record(super::changed_entities(&second)[0])
        .unwrap()
        .lineage_id;
    let candidate = runtime.record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![left_lineage],
        vec![right_lineage],
        "candidate",
    );
    let graph_before = runtime.lineage_graph(&BranchId("main".to_string()));
    let resolution = runtime
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    let graph_after = runtime.lineage_graph(&BranchId("main".to_string()));

    assert_eq!(graph_before.events.len(), 2);
    assert_eq!(graph_before.correspondence_candidates.len(), 1);
    assert_eq!(resolution.status, LineageResolutionStatus::Promoted);
    assert_eq!(graph_after.events.len(), 3);
}

#[test]
fn lineage_contract_failure_invalid_references_do_not_promote() {
    let mut runtime = super::runtime_with_test_schema();
    let commit = super::create_entity_outcome(&mut runtime, "anchor");
    let candidate = runtime.record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![crate::facade::LineageId(999)],
        vec![crate::facade::LineageId(1000)],
        "invalid",
    );

    let resolution = runtime.promote_correspondence(candidate.candidate_id, commit.commit.clone());

    assert!(resolution.is_none());
    assert!(runtime
        .diagnostics()
        .by_scope(crate::facade::DiagnosticsScope::Lineage)
        .iter()
        .any(|artifact| artifact
            .entries
            .iter()
            .any(|entry| entry.code == crate::facade::DiagnosticCode::InvariantViolation)));
}
