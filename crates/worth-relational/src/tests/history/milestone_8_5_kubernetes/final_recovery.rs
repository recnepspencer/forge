use super::*;

pub(super) struct FinalRecoveryCertificationInput {
    pub(super) runtime: RelationalRuntime,
    pub(super) recovered_root: std::path::PathBuf,
    pub(super) entity: crate::facade::identity::EntityId,
    pub(super) main_branch: BranchId,
    pub(super) controller_branch: BranchId,
    pub(super) overlap_main_head: crate::history::data::CommitReference,
    pub(super) overlap_controller_head: crate::history::data::CommitReference,
    pub(super) narrowed_main_head: crate::history::data::CommitReference,
    pub(super) narrowed_controller_head: crate::history::data::CommitReference,
    pub(super) rebroadened_main_head: crate::history::data::CommitReference,
    pub(super) rebroadened_controller_head: crate::history::data::CommitReference,
    pub(super) broad_intent_commit: CommitResult,
    pub(super) first_converge_commit: CommitResult,
    pub(super) rebroadened_intent_commit: CommitResult,
    pub(super) revalidation_commit: CommitResult,
    pub(super) live_bundle: KubernetesIntentCertificationBundle,
}

pub(super) fn certify_final_recovery(
    input: FinalRecoveryCertificationInput,
) -> KubernetesIntentCertificationBundle {
    let FinalRecoveryCertificationInput {
        mut runtime,
        recovered_root,
        entity,
        main_branch,
        controller_branch,
        overlap_main_head,
        overlap_controller_head,
        narrowed_main_head,
        narrowed_controller_head,
        rebroadened_main_head,
        rebroadened_controller_head,
        broad_intent_commit,
        first_converge_commit,
        rebroadened_intent_commit,
        revalidation_commit,
        live_bundle,
    } = input;

    let mut recovered = recover_stage(&mut runtime, recovered_root);
    let recovered_overlap_from_final = recover_stage_from_final_history(
        &recovered,
        unique_test_store_path("worth-relational-m8-5-kubernetes-style-final-overlap"),
        overlap_controller_head,
        overlap_main_head,
    );
    let recovered_overlap_planning = planning_for(
        &recovered_overlap_from_final,
        controller_branch.clone(),
        main_branch.clone(),
    );
    assert_strategy_conflict(
        &recovered_overlap_planning,
        entity,
        "final-history recovered overlap",
    );
    assert_eq!(
        planning_evidence(&recovered_overlap_planning),
        live_bundle.overlap_conflict
    );
    let recovered_narrowed_from_final = recover_stage_from_final_history(
        &recovered,
        unique_test_store_path("worth-relational-m8-5-kubernetes-style-final-narrowed"),
        narrowed_controller_head,
        narrowed_main_head,
    );
    let recovered_narrowed_planning = planning_for(
        &recovered_narrowed_from_final,
        controller_branch.clone(),
        main_branch.clone(),
    );
    assert_non_strategy_conflict(
        &recovered_narrowed_planning,
        entity,
        "final-history recovered narrowed",
    );
    assert_eq!(
        planning_evidence(&recovered_narrowed_planning),
        live_bundle.narrowed_non_conflict
    );
    let recovered_rebroadened_from_final = recover_stage_from_final_history(
        &recovered,
        unique_test_store_path("worth-relational-m8-5-kubernetes-style-final-rebroadened"),
        rebroadened_controller_head,
        rebroadened_main_head,
    );
    let recovered_rebroadened_planning = planning_for(
        &recovered_rebroadened_from_final,
        controller_branch.clone(),
        main_branch.clone(),
    );
    assert_strategy_conflict(
        &recovered_rebroadened_planning,
        entity,
        "final-history recovered rebroadened",
    );
    assert_eq!(
        planning_evidence(&recovered_rebroadened_planning),
        live_bundle.rebroadened_conflict
    );
    let recovered_revalidated_planning =
        planning_for(&recovered, controller_branch.clone(), main_branch.clone());
    assert_exact_shared_truth(
        &recovered_revalidated_planning,
        entity,
        "recovered revalidated shared truth",
    );

    let recovered_broad_intent_replay = replay_commit(
        &mut recovered,
        broad_intent_commit.commit.commit_id,
        main_branch.clone(),
    );
    let recovered_first_converge_replay = replay_commit(
        &mut recovered,
        first_converge_commit.commit.commit_id,
        controller_branch.clone(),
    );
    let recovered_rebroadened_intent_replay = replay_commit(
        &mut recovered,
        rebroadened_intent_commit.commit.commit_id,
        main_branch.clone(),
    );
    let recovered_revalidation_replay = replay_commit(
        &mut recovered,
        revalidation_commit.commit.commit_id,
        controller_branch.clone(),
    );
    assert_strategy_replay_clean(&recovered_broad_intent_replay, "recovered broad intent");
    assert_strategy_replay_clean(&recovered_first_converge_replay, "recovered first converge");
    assert_strategy_replay_clean(
        &recovered_rebroadened_intent_replay,
        "recovered rebroadened intent",
    );
    let recovered_current = recovered
        .read_truth()
        .read_version(recovered.current_version_id());
    assert_eq!(
        planning_evidence(&recovered_revalidated_planning),
        live_bundle.revalidated_shared_truth
    );
    let recovered_revalidation_envelope = recovered
        .replay()
        .canonical_commit_envelope(revalidation_commit.commit.commit_id)
        .cloned()
        .expect("recovered revalidation envelope");
    assert_eq!(
        KubernetesNoopEvidence {
            strategy_artifacts: recovered_revalidation_envelope
                .strategy_artifacts
                .as_ref()
                .expect("recovered revalidation strategy artifacts")
                .clone(),
            changed_record_count: recovered_revalidation_envelope
                .patch
                .authoritative_record_patches
                .len(),
            patch_record_count: recovered_revalidation_envelope
                .patch
                .authoritative_record_patches
                .len(),
        },
        live_bundle.revalidation_noop
    );
    assert_eq!(
        recovered_broad_intent_replay,
        live_bundle.broad_intent_replay
    );
    assert_eq!(
        recovered_first_converge_replay,
        live_bundle.first_converge_replay
    );
    assert_eq!(
        recovered_rebroadened_intent_replay,
        live_bundle.rebroadened_intent_replay
    );
    assert_eq!(
        recovered_revalidation_replay,
        live_bundle.revalidation_noop_replay
    );
    assert_strategy_replay_clean(
        &recovered_revalidation_replay,
        "recovered revalidation converge",
    );
    assert_eq!(
        KubernetesBranchHeadEvidence {
            main: recovered.history().branch_head(&main_branch).cloned(),
            controller: recovered.history().branch_head(&controller_branch).cloned(),
        },
        live_bundle.branch_heads
    );
    let recovered_entity = recovered_current
        .get_entity(entity)
        .expect("recovered entity visible");
    assert_eq!(
        KubernetesVisibleTruthEvidence {
            entity_name: read_entity_name(recovered_entity),
            replicas_canonical_bytes: replicas_canonical_bytes(recovered_entity),
        },
        live_bundle.visible_truth
    );
    live_bundle
}
