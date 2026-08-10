use crate::harness::fixtures::{execution_preflights, preview_bridge::active_preview_artifacts};
use crate::preview::{
    admit_authoritative_preview_comparison_candidate, admit_preview_promotion_parity_comparison,
    admit_promotion_eligible_preview_session_plan_binding,
    admit_read_only_preview_session_plan_binding, admit_scoped_preview_live_session_plan,
    admit_scoped_preview_session_plan_binding_from_preview_binding,
    bind_preflight_to_preview_session, execute_promotion_eligible_preview_session_plan,
    execute_read_only_preview_session_plan, execute_scoped_preview_live_session_plan,
    PreviewEvaluationClass, PreviewSessionQueryContext,
};

use super::model::PreviewCertificationLane;

pub(super) struct PreviewCertificationLanes {
    pub(super) active: PreviewCertificationLane,
    pub(super) parity: PreviewCertificationLane,
    pub(super) promotable: PreviewCertificationLane,
    pub(super) promotion_parity: PreviewCertificationLane,
    pub(super) preview_live: PreviewCertificationLane,
    pub(super) parity_preview_live: PreviewCertificationLane,
    pub(super) preview_live_rebind: PreviewCertificationLane,
}

pub(super) fn build_lanes() -> PreviewCertificationLanes {
    let preflight = execution_preflights::direct_runtime_preflight();
    let parity_preflight = execution_preflights::replay_runtime_preflight();
    let (_runtime, active, execution_record) = active_preview_artifacts("preview-certification");
    let active_binding = bind_preflight_to_preview_session(
        preflight.clone(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("active preview certification binding should succeed");
    let active_execution = execute_read_only_preview_session_plan(
        &admit_read_only_preview_session_plan_binding(active_binding.clone())
            .expect("active read-only binding should admit"),
    )
    .expect("active preview execution should succeed");
    let parity_binding = bind_preflight_to_preview_session(
        parity_preflight.clone(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::read_only(),
        ),
    )
    .expect("parity preview certification binding should succeed");
    let parity_execution = execute_read_only_preview_session_plan(
        &admit_read_only_preview_session_plan_binding(parity_binding.clone())
            .expect("parity read-only binding should admit"),
    )
    .expect("parity preview execution should succeed");
    let promotable_binding = bind_preflight_to_preview_session(
        preflight.clone(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("promotion-eligible binding should succeed");
    let promotable_execution = execute_promotion_eligible_preview_session_plan(
        &admit_promotion_eligible_preview_session_plan_binding(promotable_binding.clone())
            .expect("promotion-eligible binding should admit"),
    )
    .expect("promotion-eligible preview execution should succeed");
    let parity_promotable_binding = bind_preflight_to_preview_session(
        parity_preflight.clone(),
        PreviewSessionQueryContext::active(
            &active,
            &execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("parity promotion-eligible binding should succeed");
    let parity_promotable_execution = execute_promotion_eligible_preview_session_plan(
        &admit_promotion_eligible_preview_session_plan_binding(parity_promotable_binding.clone())
            .expect("parity promotion-eligible binding should admit"),
    )
    .expect("parity promotion-eligible preview execution should succeed");
    let preview_live_binding = admit_scoped_preview_live_session_plan(
        admit_scoped_preview_session_plan_binding_from_preview_binding(promotable_binding.clone())
            .expect("preview-live should derive scoped preview binding"),
        crate::live::promote_preflight_bundle_to_live(&preflight)
            .expect("preview-live should reuse admitted detail live proof"),
    )
    .expect("preview-live admission should succeed");
    let preview_live = execute_scoped_preview_live_session_plan(&preview_live_binding)
        .expect("preview-live execution should succeed");
    let parity_preview_live_binding = admit_scoped_preview_live_session_plan(
        admit_scoped_preview_session_plan_binding_from_preview_binding(
            parity_promotable_binding.clone(),
        )
        .expect("parity preview-live should derive scoped preview binding"),
        crate::live::promote_preflight_bundle_to_live(&parity_preflight)
            .expect("parity preview-live should reuse admitted detail live proof"),
    )
    .expect("parity preview-live admission should succeed");
    let parity_preview_live =
        execute_scoped_preview_live_session_plan(&parity_preview_live_binding)
            .expect("parity preview-live execution should succeed");
    let promotion_candidate_execution = crate::execution::execute_preflight_bundle(&preflight)
        .expect("authoritative comparison candidate should execute");
    let promotion_candidate = admit_authoritative_preview_comparison_candidate(
        &preflight,
        &promotion_candidate_execution,
    )
    .expect("authoritative comparison candidate should admit");
    let promotion_parity =
        admit_preview_promotion_parity_comparison(&promotable_execution, &promotion_candidate)
            .expect("promotion parity comparison should admit");

    let (_rebind_old_runtime, rebind_old_active, rebind_old_execution_record) =
        active_preview_artifacts("preview-certification-live-rebind-old");
    let rebind_seed_binding = bind_preflight_to_preview_session(
        preflight.clone(),
        PreviewSessionQueryContext::active(
            &rebind_old_active,
            &rebind_old_execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    )
    .expect("rebind seed preview binding should admit");
    let rebind_seed_preview_live = admit_scoped_preview_live_session_plan(
        admit_scoped_preview_session_plan_binding_from_preview_binding(rebind_seed_binding)
            .expect("rebind seed should derive scoped preview binding"),
        crate::live::promote_preflight_bundle_to_live(&preflight)
            .expect("rebind seed should reuse live proof"),
    )
    .expect("rebind seed preview-live should admit");
    let (_rebind_new_runtime, rebind_new_active, rebind_new_execution_record) =
        active_preview_artifacts("preview-certification-live-rebind-new");
    let preview_live_explicit_rebind = match crate::preview::assess_preview_live_drift(
        &rebind_seed_preview_live,
        PreviewSessionQueryContext::active(
            &rebind_new_active,
            &rebind_new_execution_record,
            PreviewEvaluationClass::promotion_eligible(),
        ),
    ) {
        crate::preview::PreviewLiveDriftOutcome::ExplicitRebindAvailable(rebind) => rebind,
        other => panic!("preview-live drift should offer explicit rebind, got {other:?}"),
    };
    let preview_live_rebind_preview_execution = execute_promotion_eligible_preview_session_plan(
        &admit_promotion_eligible_preview_session_plan_binding(
            preview_live_explicit_rebind
                .rebound_preview_live()
                .scoped_binding()
                .preview_binding()
                .clone(),
        )
        .expect("rebound preview binding should admit"),
    )
    .expect("rebound preview execution should succeed");
    let preview_live_rebind_execution = execute_scoped_preview_live_session_plan(
        &admit_scoped_preview_live_session_plan(
            admit_scoped_preview_session_plan_binding_from_preview_binding(
                preview_live_explicit_rebind
                    .rebound_preview_live()
                    .scoped_binding()
                    .preview_binding()
                    .clone(),
            )
            .expect("rebound preview-live should derive scoped preview binding"),
            preview_live_explicit_rebind
                .rebound_preview_live()
                .live_plan()
                .clone(),
        )
        .expect("rebound preview-live should admit through scoped path"),
    )
    .expect("rebound preview-live execution should succeed");

    PreviewCertificationLanes {
        active: PreviewCertificationLane::from_execution(active_execution.as_preview_execution()),
        parity: PreviewCertificationLane::from_execution(parity_execution.as_preview_execution()),
        promotable: PreviewCertificationLane::from_execution(
            promotable_execution.as_preview_execution(),
        ),
        promotion_parity: PreviewCertificationLane::from_execution(
            promotable_execution.as_preview_execution(),
        )
        .with_promotion_parity(&promotion_parity),
        preview_live: PreviewCertificationLane::from_execution(
            promotable_execution.as_preview_execution(),
        )
        .with_preview_live(&preview_live),
        parity_preview_live: PreviewCertificationLane::from_execution(
            parity_promotable_execution.as_preview_execution(),
        )
        .with_preview_live(&parity_preview_live),
        preview_live_rebind: PreviewCertificationLane::from_execution(
            preview_live_rebind_preview_execution.as_preview_execution(),
        )
        .with_preview_live_rebind(
            &preview_live_rebind_execution,
            &preview_live_explicit_rebind,
        ),
    }
}
