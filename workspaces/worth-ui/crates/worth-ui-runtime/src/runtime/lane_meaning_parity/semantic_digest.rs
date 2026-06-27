use crate::runtime::lane_meaning_parity::hash_fold::WorthUiLaneParityHashFold;
use crate::runtime::{
    WorthUiCrossLaneSemanticReference, WorthUiExecutionPlan, WorthUiPlanNodeInputFamily,
    WorthUiQueryBindingComparisonEntry, WorthUiQueryBindingPosture, WorthUiQueryLiveRebindEntry,
    WorthUiQueryLiveRebindOutcome,
};

pub(super) enum WorthUiQueryReferenceSide {
    Active,
    Candidate,
}

pub(super) fn digest_identity_basis(identity_basis: &str) -> u64 {
    let mut digest = WorthUiLaneParityHashFold::new(0x2fac_c3c5_19ae_8211);
    digest.fold_str(identity_basis);
    digest.finish()
}

pub(super) fn digest_plan_family(
    plan: &WorthUiExecutionPlan,
    family: WorthUiPlanNodeInputFamily,
) -> u64 {
    let mut digest = WorthUiLaneParityHashFold::new(0xa64d_781d_e156_9a2b);
    let nodes = plan
        .topology()
        .traversal_order()
        .iter()
        .filter(|node| node.family().input_family() == family);
    for node in nodes {
        digest.fold(0x10);
        digest.fold(u64::from(node.runtime_handle().plan_index()));
        digest.fold(node.runtime_handle().plan_generation().as_u64());
    }
    digest.finish()
}

pub(super) fn digest_query_posture_entry(
    entry: &WorthUiQueryBindingComparisonEntry,
    side: WorthUiQueryReferenceSide,
) -> u64 {
    let mut digest = WorthUiLaneParityHashFold::new(0x76de_81c2_9c7a_3451);
    digest.fold_str(entry.identity().view_binding_id());
    digest.fold_str(entry.identity().query_capability_digest());
    digest.fold_str(entry.identity().query_composition_profile_digest());
    digest.fold_str(entry.identity().result_shape_digest());
    let posture = match side {
        WorthUiQueryReferenceSide::Active => entry.active_posture(),
        WorthUiQueryReferenceSide::Candidate => entry.candidate_posture(),
    };
    if let Some(posture) = posture {
        fold_query_posture(&mut digest, posture);
    }
    digest.finish()
}

pub(super) fn digest_query_rebind_entry(entry: &WorthUiQueryLiveRebindEntry) -> u64 {
    let mut digest = WorthUiLaneParityHashFold::new(0x76de_81c2_9c7a_3451);
    digest.fold_str(entry.identity().view_binding_id());
    digest.fold_str(entry.identity().query_capability_digest());
    digest.fold_str(entry.identity().query_composition_profile_digest());
    digest.fold_str(entry.identity().result_shape_digest());
    match entry.outcome() {
        WorthUiQueryLiveRebindOutcome::Preserve(preservation) => {
            fold_query_posture(&mut digest, preservation.preserved_posture());
        }
        WorthUiQueryLiveRebindOutcome::Rebind(rebind) => {
            fold_query_posture(&mut digest, rebind.candidate_posture());
        }
        WorthUiQueryLiveRebindOutcome::Retire(retirement) => {
            fold_query_posture(&mut digest, retirement.active_posture());
        }
        WorthUiQueryLiveRebindOutcome::Deny(_) => digest.fold(0),
    }
    digest.finish()
}

pub(super) fn digest_references(references: &[WorthUiCrossLaneSemanticReference]) -> u64 {
    let mut digest = WorthUiLaneParityHashFold::new(0xf2b5_1d78_92e1_aaa9);
    for reference in references {
        digest.fold(reference.family() as u64);
        digest.fold_str(reference.identity());
        digest.fold(reference.active_digest());
        digest.fold(reference.candidate_digest());
        digest.fold(reference.authority() as u64);
    }
    digest.finish()
}

fn fold_query_posture(
    digest: &mut WorthUiLaneParityHashFold,
    posture: &WorthUiQueryBindingPosture,
) {
    digest.fold_str(posture.support_admission_digest());
    digest.fold_str(posture.basis_capability_digest());
    digest.fold_str(posture.live_compatibility_digest());
    digest.fold_str(posture.async_result_state_digest());
    digest.fold_str(posture.recovery_digest());
    digest.fold_str(posture.inspection_digest());
    digest.fold_str(posture.projection_consumption_digest());
}
