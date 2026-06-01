use super::*;

#[test]
fn durability_contract_checkpoint_tail_recovery_preserves_post_checkpoint_commits() {
    let mut runtime = persisted_runtime_with_test_schema();
    let main = create_entity_outcome(&mut runtime, "main-a");
    let _checkpoint = runtime.durability_authority().checkpoint().unwrap();
    let later = create_entity_outcome(&mut runtime, "main-b");
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    assert_eq!(outcome.recovered_commits, 2);
    assert_eq!(outcome.latest_commit, Some(later.commit.clone()));
    assert_eq!(
        recovered
            .history()
            .branch_head(&BranchId("main".to_string())),
        Some(&later.commit)
    );
    assert_eq!(
        recovered
            .replay()
            .canonical_commit_envelope(main.commit.commit_id)
            .unwrap()
            .commit
            .commit_id,
        main.commit.commit_id
    );
}

#[test]
fn durability_contract_checkpoint_recovers_index_metadata() {
    let mut runtime = persisted_runtime_with_test_schema();
    let commit = create_entity_outcome(&mut runtime, "indexed");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity-name".to_string(),
        kind: DerivedIndexKind::EntityField {
            field_locator: aspect_field_locator(aspect_key("name"), field_key("name")),
        },
        branch_scoped: false,
    });
    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: commit.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    runtime.durability_authority().checkpoint().unwrap();
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();

    let index_access = recovered.index_access();
    let generation = index_access
        .latest_generation(index.index_id, &BranchId("main".to_string()))
        .unwrap();
    assert_eq!(generation.generation_id, build.generations[0].generation_id);
    assert_eq!(generation.source_commit_id, commit.commit.commit_id);
}

#[test]
fn durability_contract_checkpoint_recovers_lineage_metadata() {
    let mut runtime = persisted_runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    let second = create_entity_outcome(&mut runtime, "second");
    let first_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&first)[0])
        .unwrap()
        .lineage_id;
    let second_lineage = runtime
        .lineage_access()
        .for_record(changed_entities(&second)[0])
        .unwrap()
        .lineage_id;
    let candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![first_lineage],
        vec![second_lineage],
        "recover-me",
    );
    runtime
        .lineage_authority()
        .promote_correspondence(candidate.candidate_id, second.commit.clone())
        .unwrap();
    let rejected_candidate = runtime.lineage_authority().record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![LineageId(999)],
        vec![LineageId(1000)],
        "reject-me",
    );
    let rejected_resolution = runtime
        .lineage_authority()
        .promote_correspondence(rejected_candidate.candidate_id, second.commit.clone());
    assert_eq!(
        rejected_resolution,
        Err(CorrespondencePromotionRejectionClass::MissingLineageReference)
    );
    let checkpoint = runtime.durability_authority().checkpoint().unwrap();
    assert_eq!(
        checkpoint
            .lineage
            .digest_basis()
            .published_lineage_commit_count,
        runtime
            .history()
            .commit_envelopes_snapshot()
            .iter()
            .filter(|envelope| envelope.has_lineage_authority())
            .count()
    );
    assert_eq!(
        checkpoint
            .lineage
            .digest_basis()
            .published_lineage_event_count,
        runtime
            .history()
            .commit_envelopes_snapshot()
            .iter()
            .map(|envelope| envelope.lineage_digest_basis().lineage_event_count())
            .sum::<usize>()
    );
    assert_eq!(
        checkpoint
            .lineage
            .digest_basis()
            .published_lineage_decision_count,
        runtime
            .history()
            .commit_envelopes_snapshot()
            .iter()
            .map(|envelope| envelope.lineage_digest_basis().lineage_decision_count())
            .sum::<usize>()
    );
    assert_eq!(
        checkpoint.lineage.counters().node_count,
        checkpoint.lineage.nodes().len()
    );
    assert_eq!(
        checkpoint.lineage.counters().correspondence_candidate_count,
        checkpoint.lineage.correspondence_candidates().len()
    );
    assert_eq!(
        checkpoint.lineage.counters().rejected_decision_count,
        checkpoint.lineage.rejected_decisions().len()
    );
    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered.durability_authority().recover(plan).unwrap();
    let graph = recovered
        .lineage_access()
        .graph(crate::facade::lineage::LineageGraphRequest {
            branch_id: BranchId("main".to_string()),
            traversal_basis:
                crate::facade::lineage::LineageGraphTraversalBasis::FullBranchGraphMaterialization,
        });

    assert_eq!(graph.nodes.len(), 2);
    assert!(graph
        .events
        .iter()
        .any(|event| event.kind == LineageEventKind::Correspond));
    assert!(graph
        .correspondence_candidates
        .iter()
        .any(|entry| entry.candidate_id == candidate.candidate_id));
    assert!(recovered
        .lineage_access()
        .rejected_decisions_snapshot()
        .iter()
        .any(|decision| {
            decision.kind == LineageDecisionKind::CorrespondencePromotionRejected
                && decision.candidate_id == Some(rejected_candidate.candidate_id)
                && decision.rejection_class
                    == Some(CorrespondencePromotionRejectionClass::MissingLineageReference)
        }));
}

#[test]
fn durability_contract_corrupt_latest_checkpoint_falls_back_to_prior_valid_checkpoint() {
    let mut runtime = persisted_runtime_with_test_schema();
    let first = create_entity_outcome(&mut runtime, "first");
    runtime.durability_authority().checkpoint().unwrap();
    let second = create_entity_outcome(&mut runtime, "second");
    runtime.durability_authority().checkpoint().unwrap();
    let store = runtime
        .durability()
        .recovery_plan(
            crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
        )
        .store
        .unwrap();
    let latest_checkpoint = store.checkpoints.last().unwrap();
    std::fs::write(&latest_checkpoint.path, b"{not-json").unwrap();

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    let outcome = recovered.durability_authority().recover(plan).unwrap();

    assert_eq!(outcome.latest_commit, Some(second.commit.clone()));
    assert!(!outcome
        .integrity_report
        .skipped_corrupt_checkpoints
        .is_empty());
    assert_eq!(
        recovered
            .history()
            .branch_head(&BranchId("main".to_string())),
        Some(&second.commit)
    );
    assert!(recovered
        .replay()
        .canonical_commit_envelope(first.commit.commit_id)
        .is_some());
}

#[test]
fn durability_contract_compaction_only_removes_segments_covered_by_checkpoint() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "a");
    create_entity_outcome(&mut runtime, "b");
    runtime.durability_authority().checkpoint().unwrap();
    create_entity_outcome(&mut runtime, "c");
    let before = runtime
        .durability()
        .recovery_plan(
            crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
        )
        .store
        .unwrap();

    let compaction = runtime.durability_authority().compact_store().unwrap();
    let after = runtime
        .durability()
        .recovery_plan(
            crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
        )
        .store
        .unwrap();

    assert!(!before.segments.is_empty());
    assert!(after.segments.len() <= before.segments.len());
    assert_eq!(after.segments.len(), compaction.retained_segments.len());
}
