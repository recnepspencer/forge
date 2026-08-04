use super::super::patches::{
    BoundedMaterializationLiveOutcome, DetailLiveOutcome, LivePatchEnvelope, LivePatchPayload,
    OrderedCollectionLiveOutcome,
};
use super::super::promotion::LiveQueryPlan;
use super::super::telemetry::LivePolicyCounters;
use super::replay::LiveReplayBundle;
use super::report::LiveExecutionReport;
use crate::identity::hash_parts;

pub(crate) struct LivePatchConstructionBasis {
    pub(crate) outcome_kind: String,
    pub(crate) outcome_digest: String,
    pub(crate) basis_digest: String,
    pub(crate) replay_digest: String,
}

pub(crate) fn live_execution_report(
    live: &LiveQueryPlan,
    outcome_kind: String,
    outcome_digest: String,
) -> LiveExecutionReport {
    let replay_digest = live.progress_basis().replay_digest().as_str().to_string();
    let result_digest = semantic_result_digest(
        live,
        live.progress_basis()
            .current_basis()
            .proof()
            .digest()
            .as_str(),
        &outcome_kind,
        &outcome_digest,
    );
    LiveExecutionReport {
        query_digest: live.descriptor().query_digest().as_str().to_string(),
        result_digest,
        delivery_digest: outcome_digest.clone(),
        replay_digest,
        family: live.descriptor().family().clone(),
        outcome_kind,
        outcome_digest,
        basis_digest: live
            .progress_basis()
            .current_basis()
            .proof()
            .digest()
            .as_str()
            .to_string(),
        subscription_digest: live.subscription_digest().as_str().to_string(),
    }
}

pub(crate) fn patch_envelope_from_payload(
    live: &LiveQueryPlan,
    payload: LivePatchPayload,
    basis: LivePatchConstructionBasis,
) -> LivePatchEnvelope {
    let result_digest = semantic_result_digest(
        live,
        &basis.basis_digest,
        &basis.outcome_kind,
        &basis.outcome_digest,
    );

    LivePatchEnvelope {
        query_digest: live.descriptor().query_digest().as_str().to_string(),
        result_digest,
        delivery_digest: basis.outcome_digest,
        replay_digest: basis.replay_digest,
        basis_digest: basis.basis_digest,
        subscription_digest: live.subscription_digest().as_str().to_string(),
        family: live.descriptor().family().clone(),
        payload,
    }
}

fn semantic_result_digest(
    live: &LiveQueryPlan,
    basis_digest: &str,
    outcome_kind: &str,
    outcome_digest: &str,
) -> String {
    hash_parts(&[
        format!("query:{}", live.descriptor().query_digest().as_str()),
        format!("family:{}", live.descriptor().family().as_str()),
        format!("basis:{basis_digest}"),
        format!("outcome_kind:{outcome_kind}"),
        format!("delivery:{outcome_digest}"),
    ])
}

pub(crate) fn replay_bundle_from_patch_envelope(
    patch_envelope: LivePatchEnvelope,
    counter_snapshot: LivePolicyCounters,
) -> LiveReplayBundle {
    LiveReplayBundle {
        query_digest: patch_envelope.query_digest().to_string(),
        result_digest: patch_envelope.result_digest().to_string(),
        delivery_digest: patch_envelope.delivery_digest().to_string(),
        replay_digest: patch_envelope.replay_digest().to_string(),
        basis_digest: patch_envelope.basis_digest().to_string(),
        subscription_digest: patch_envelope.subscription_digest().to_string(),
        counter_snapshot,
        patch_envelope,
    }
}

pub(crate) fn detail_outcome_digest(outcome: &DetailLiveOutcome) -> (String, String) {
    match outcome {
        DetailLiveOutcome::Patch(patch) => {
            ("patch".to_string(), patch.digest().as_str().to_string())
        }
        DetailLiveOutcome::Suppressed(reason) => ("suppressed".to_string(), format!("{reason:?}")),
        DetailLiveOutcome::Refresh(fallback) => (
            "refresh".to_string(),
            format!(
                "{}:{}",
                fallback.admission_class().as_str(),
                fallback.admission_status().as_str()
            ),
        ),
    }
}

pub(crate) fn ordered_collection_outcome_digest(
    outcome: &OrderedCollectionLiveOutcome,
) -> (String, String) {
    match outcome {
        OrderedCollectionLiveOutcome::Patch(patch) => {
            ("patch".to_string(), patch.digest().as_str().to_string())
        }
        OrderedCollectionLiveOutcome::Suppressed(reason) => {
            ("suppressed".to_string(), format!("{reason:?}"))
        }
        OrderedCollectionLiveOutcome::Refresh(fallback) => (
            "refresh".to_string(),
            format!(
                "{}:{}",
                fallback.admission_class().as_str(),
                fallback.admission_status().as_str()
            ),
        ),
    }
}

pub(crate) fn bounded_outcome_digest(
    outcome: &BoundedMaterializationLiveOutcome,
) -> (String, String) {
    match outcome {
        BoundedMaterializationLiveOutcome::Patch(patch) => {
            ("patch".to_string(), patch.digest().as_str().to_string())
        }
        BoundedMaterializationLiveOutcome::Suppressed(reason) => {
            ("suppressed".to_string(), format!("{reason:?}"))
        }
        BoundedMaterializationLiveOutcome::Refresh(fallback) => (
            "refresh".to_string(),
            format!(
                "{}:{}",
                fallback.admission_class().as_str(),
                fallback.admission_status().as_str()
            ),
        ),
    }
}
