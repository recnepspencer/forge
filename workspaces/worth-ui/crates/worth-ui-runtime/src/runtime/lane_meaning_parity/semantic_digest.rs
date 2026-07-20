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
    digest.fold(plan.regional_family_count(family) as u64);
    digest.fold(plan.regional_family_semantic_digest(family));
    digest.finish()
}

pub(super) fn digest_query_posture_entry(
    entry: &WorthUiQueryBindingComparisonEntry,
    side: WorthUiQueryReferenceSide,
) -> u64 {
    let mut digest = WorthUiLaneParityHashFold::new(0x76de_81c2_9c7a_3451);
    digest.fold(entry.identity().canonical_identity());
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
    digest.fold(entry.identity().canonical_identity());
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
    digest.fold(posture.canonical_identity());
}
