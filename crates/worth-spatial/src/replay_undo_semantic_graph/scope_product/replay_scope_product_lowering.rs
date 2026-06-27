use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_replay_scope_identity, ReplayScopeIdentity, ReplayScopeIdentityInput,
    ReplayUndoSemanticGraphEquivalenceBasis, ReplayUndoSemanticGraphLocalityScope,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    SpatialReplayScopeProduct, SpatialReplayScopeProductCounters, SpatialReplayScopeProductIdentity,
};
use crate::replay_undo_semantic_graph::SpatialReplaySelectedPlan;

pub fn lower_spatial_replay_scope_product_from_selected_plan<'a>(
    replay_plan: &SpatialReplaySelectedPlan<'a>,
) -> SpatialReplayScopeProduct<'a> {
    let admitted_input = replay_plan.admitted_input();
    let equivalence_basis = lower_spatial_replay_equivalence_basis_from_selected_plan(replay_plan);
    let scope_identity =
        admit_replay_scope_identity(ReplayScopeIdentityInput::new(equivalence_basis.clone()));
    let counters = SpatialReplayScopeProductCounters::new(
        equivalence_basis.touched_subjects().len(),
        admitted_input
            .lookup_consumed_workload_handoff()
            .counters()
            .covered_family_count(),
        admitted_input
            .lookup_consumed_workload_handoff()
            .counters()
            .indexed_lookup_count(),
        admitted_input
            .lookup_consumed_workload_handoff()
            .counters()
            .topology_receipt_ref_count(),
        admitted_input
            .lookup_consumed_workload_handoff()
            .counters()
            .raw_row_scan_count(),
        admitted_input
            .lookup_consumed_workload_handoff()
            .counters()
            .broad_receipt_scan_count(),
        admitted_input
            .lookup_consumed_workload_handoff()
            .counters()
            .caller_owned_scan_count(),
        usize::from(admitted_input.retained_replay_receipt().is_some()),
    );
    let scope_product_identity = SpatialReplayScopeProductIdentity::new(
        lower_spatial_replay_scope_product_identity(replay_plan, &scope_identity, &counters),
    );

    SpatialReplayScopeProduct::new(
        replay_plan.family_identity(),
        replay_plan.covered_lookup_identity(),
        replay_plan.workload_dependency_posture(),
        replay_plan
            .admitted_input_semantic_graph_identity()
            .to_string(),
        replay_plan.selected_plan_identity().to_string(),
        replay_plan
            .lookup_consumed_workload_handoff_identity()
            .to_string(),
        admitted_input.prior_proof_identity().clone(),
        admitted_input.stage_index_identity().clone(),
        admitted_input.lookup_consumed_workload_handoff(),
        admitted_input.retained_replay_receipt(),
        counters,
        equivalence_basis,
        scope_identity,
        scope_product_identity,
    )
}

pub fn lower_spatial_replay_scope_identity_from_scope_product(
    scope_product: &SpatialReplayScopeProduct<'_>,
) -> ReplayScopeIdentity {
    scope_product.scope_identity().clone()
}

pub fn lower_spatial_replay_equivalence_basis_from_scope_product(
    scope_product: &SpatialReplayScopeProduct<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    scope_product.equivalence_basis().clone()
}

pub fn lower_spatial_replay_equivalence_basis_from_selected_plan(
    replay_plan: &SpatialReplaySelectedPlan<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    let admitted_input = replay_plan.admitted_input();
    ReplayUndoSemanticGraphEquivalenceBasis::new(
        ReplayUndoSemanticGraphLocalityScope::SpatialTouchAuthority,
        crate::replay_undo_semantic_graph::lower_spatial_touched_subjects(
            admitted_input.spatial_touch_authority(),
        ),
        admitted_input.prior_proof_identity().clone(),
        Some(admitted_input.stage_index_identity().clone()),
    )
}

fn lower_spatial_replay_scope_product_identity(
    replay_plan: &SpatialReplaySelectedPlan<'_>,
    scope_identity: &ReplayScopeIdentity,
    counters: &SpatialReplayScopeProductCounters,
) -> String {
    let admitted_input = replay_plan.admitted_input();
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:replay-undo-semantic-graph:scope-product:v1".to_string(),
            format!("family:{}", replay_plan.family_identity().as_str()),
            format!("selected-plan:{}", replay_plan.selected_plan_identity()),
            format!(
                "admitted:{}",
                replay_plan.admitted_input_semantic_graph_identity()
            ),
            format!("scope:{}", scope_identity.digest()),
            format!(
                "lookup-handoff:{}",
                replay_plan.lookup_consumed_workload_handoff_identity()
            ),
            format!(
                "retained-replay:{}",
                replay_plan
                    .retained_replay_receipt_identity()
                    .unwrap_or("not-required")
            ),
            format!(
                "covered-lookup:{}",
                replay_plan.covered_lookup_identity().as_str()
            ),
            format!("touched-subjects:{}", counters.touched_subject_count()),
            format!(
                "retained-binding-count:{}",
                counters.retained_replay_binding_count()
            ),
            format!(
                "stage-receipt:{}",
                admitted_input
                    .lookup_consumed_workload_handoff()
                    .stage_receipt_identity()
            ),
        ],
    )
}
