use super::super::patches::{
    BoundedMaterializationLiveOutcome, DetailLiveOutcome, LiveBoundedMaterializationPatchError,
    LiveCollectionPatchError, LiveDetailPatchError, LivePatchPayload, OrderedCollectionLiveOutcome,
};
use super::super::promotion::{LiveQueryFamily, LiveQueryPlan};
use super::super::relevance::BridgeChangeSummary;
use super::super::telemetry::LivePolicyCounters;
use super::digest::{
    bounded_outcome_digest, detail_outcome_digest, live_execution_report,
    ordered_collection_outcome_digest, patch_envelope_from_payload,
    replay_bundle_from_patch_envelope, LivePatchConstructionBasis,
};
use super::report::LiveExecutionEnvelope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveExecutionError {
    Detail(LiveDetailPatchError),
    OrderedCollection(LiveCollectionPatchError),
    BoundedMaterialization(LiveBoundedMaterializationPatchError),
}

struct LiveExecutionDraft {
    outcome_kind: String,
    outcome_digest: String,
    payload: LivePatchPayload,
    counters: LivePolicyCounters,
}

pub(crate) fn execute_live_change(
    live: &LiveQueryPlan,
    change: &BridgeChangeSummary,
) -> Result<LiveExecutionEnvelope, LiveExecutionError> {
    let family = live_execution_family(live);
    let draft = execute_semantic_change(live, change, family)?;
    Ok(assemble_live_execution(live, draft))
}

fn live_execution_family(live: &LiveQueryPlan) -> LiveQueryFamily {
    live.descriptor().family().clone()
}

fn execute_semantic_change(
    live: &LiveQueryPlan,
    change: &BridgeChangeSummary,
    family: LiveQueryFamily,
) -> Result<LiveExecutionDraft, LiveExecutionError> {
    match family {
        LiveQueryFamily::Detail => execute_detail_change(live, change),
        LiveQueryFamily::OrderedCollection => execute_ordered_collection_change(live, change),
        LiveQueryFamily::BoundedMaterialization => {
            execute_bounded_materialization_change(live, change)
        }
    }
}

fn execute_detail_change(
    live: &LiveQueryPlan,
    change: &BridgeChangeSummary,
) -> Result<LiveExecutionDraft, LiveExecutionError> {
    let outcome = live
        .detail_live_outcome(change)
        .map_err(LiveExecutionError::Detail)?;
    let (outcome_kind, outcome_digest) = detail_outcome_digest(&outcome);
    Ok(LiveExecutionDraft {
        payload: detail_payload(&outcome),
        counters: LivePolicyCounters::from_detail_outcome(&outcome),
        outcome_kind,
        outcome_digest,
    })
}

fn detail_payload(outcome: &DetailLiveOutcome) -> LivePatchPayload {
    match outcome {
        DetailLiveOutcome::Patch(patch) => LivePatchPayload::Detail(patch.clone()),
        DetailLiveOutcome::Suppressed(reason) => LivePatchPayload::Suppressed(reason.clone()),
        DetailLiveOutcome::Refresh(fallback) => LivePatchPayload::Refresh(fallback.clone()),
    }
}

fn execute_ordered_collection_change(
    live: &LiveQueryPlan,
    change: &BridgeChangeSummary,
) -> Result<LiveExecutionDraft, LiveExecutionError> {
    let outcome = live
        .ordered_collection_live_outcome(change)
        .map_err(LiveExecutionError::OrderedCollection)?;
    let (outcome_kind, outcome_digest) = ordered_collection_outcome_digest(&outcome);
    Ok(LiveExecutionDraft {
        payload: ordered_collection_payload(&outcome),
        counters: LivePolicyCounters::from_ordered_collection_outcome(&outcome),
        outcome_kind,
        outcome_digest,
    })
}

fn ordered_collection_payload(outcome: &OrderedCollectionLiveOutcome) -> LivePatchPayload {
    match outcome {
        OrderedCollectionLiveOutcome::Patch(patch) => {
            LivePatchPayload::OrderedCollection(patch.clone())
        }
        OrderedCollectionLiveOutcome::Suppressed(reason) => {
            LivePatchPayload::Suppressed(reason.clone())
        }
        OrderedCollectionLiveOutcome::Refresh(fallback) => {
            LivePatchPayload::Refresh(fallback.clone())
        }
    }
}

fn execute_bounded_materialization_change(
    live: &LiveQueryPlan,
    change: &BridgeChangeSummary,
) -> Result<LiveExecutionDraft, LiveExecutionError> {
    let outcome = live
        .bounded_materialization_live_outcome(change)
        .map_err(LiveExecutionError::BoundedMaterialization)?;
    let (outcome_kind, outcome_digest) = bounded_outcome_digest(&outcome);
    Ok(LiveExecutionDraft {
        payload: bounded_materialization_payload(&outcome),
        counters: LivePolicyCounters::from_bounded_materialization_outcome(&outcome),
        outcome_kind,
        outcome_digest,
    })
}

fn bounded_materialization_payload(
    outcome: &BoundedMaterializationLiveOutcome,
) -> LivePatchPayload {
    match outcome {
        BoundedMaterializationLiveOutcome::Patch(patch) => {
            LivePatchPayload::BoundedMaterialization(patch.clone())
        }
        BoundedMaterializationLiveOutcome::Suppressed(reason) => {
            LivePatchPayload::Suppressed(reason.clone())
        }
        BoundedMaterializationLiveOutcome::Refresh(fallback) => {
            LivePatchPayload::Refresh(fallback.clone())
        }
    }
}

fn assemble_live_execution(
    live: &LiveQueryPlan,
    draft: LiveExecutionDraft,
) -> LiveExecutionEnvelope {
    let LiveExecutionDraft {
        outcome_kind,
        outcome_digest,
        payload,
        counters,
    } = draft;
    let construction_basis = execution_construction_basis(live, &outcome_kind, &outcome_digest);
    let patch_envelope = patch_envelope_from_payload(live, payload, construction_basis);
    let report = live_execution_report(live, outcome_kind, outcome_digest);
    let replay_bundle = replay_bundle_from_patch_envelope(patch_envelope.clone(), counters.clone());
    LiveExecutionEnvelope {
        report,
        patch_envelope,
        replay_bundle,
        counters,
    }
}

fn execution_construction_basis(
    live: &LiveQueryPlan,
    outcome_kind: &str,
    outcome_digest: &str,
) -> LivePatchConstructionBasis {
    let (basis_digest, replay_digest) = execution_progression_basis(live);
    LivePatchConstructionBasis {
        outcome_kind: outcome_kind.to_string(),
        outcome_digest: outcome_digest.to_string(),
        basis_digest,
        replay_digest,
    }
}

fn execution_progression_basis(live: &LiveQueryPlan) -> (String, String) {
    (
        live.progress_basis()
            .current_basis()
            .proof()
            .digest()
            .as_str()
            .to_string(),
        live.progress_basis().replay_digest().as_str().to_string(),
    )
}
