use crate::facade::foundation::{
    LivePatchPayload, LivePolicyCounters, LiveReplayBundle, LiveReplayRun,
};

use super::super::super::profiles::CertificationProfile;
use super::super::model::{LiveBundleFamily, LiveCertificationBundle, LiveOutcomeKind};

pub(super) fn bundle_from_lane(
    profile: CertificationProfile,
    lane: &crate::facade::certification::LiveCertificationLane,
) -> LiveCertificationBundle {
    LiveCertificationBundle {
        profile,
        query_digest: lane.execution().replay_bundle().query_digest().to_string(),
        result_digest: lane.execution().replay_bundle().result_digest().to_string(),
        delivery_digest: lane
            .execution()
            .replay_bundle()
            .delivery_digest()
            .to_string(),
        replay_digest: lane.execution().replay_bundle().replay_digest().to_string(),
        replay_step_delivery_digests: Vec::new(),
        family: bundle_family(lane.execution().patch_envelope().family()),
        outcome_kind: outcome_kind_from_payload(lane.execution().patch_envelope().payload()),
        outcome_digest: lane.execution().report().outcome_digest().to_string(),
        basis_digest: lane.execution().replay_bundle().basis_digest().to_string(),
        subscription_digest: lane
            .execution()
            .replay_bundle()
            .subscription_digest()
            .to_string(),
        counter_snapshot: lane.execution().replay_bundle().counter_snapshot().clone(),
    }
}

pub(super) fn bundle_from_replay_run(
    profile: CertificationProfile,
    run: &LiveReplayRun,
) -> LiveCertificationBundle {
    let final_bundle = run
        .bundles()
        .last()
        .expect("replay run should emit at least one bundle");
    let mut counter_snapshot = LivePolicyCounters::default();
    for bundle in run.bundles() {
        counter_snapshot.absorb(bundle.counter_snapshot());
    }

    LiveCertificationBundle {
        profile,
        query_digest: final_bundle.query_digest().to_string(),
        result_digest: final_bundle.result_digest().to_string(),
        delivery_digest: final_bundle.delivery_digest().to_string(),
        replay_digest: final_bundle.replay_digest().to_string(),
        replay_step_delivery_digests: run
            .bundles()
            .iter()
            .map(|bundle| bundle.delivery_digest().to_string())
            .collect(),
        family: bundle_family(final_bundle.patch_envelope().family()),
        outcome_kind: replay_payload_kind(final_bundle),
        outcome_digest: final_bundle.delivery_digest().to_string(),
        basis_digest: final_bundle.basis_digest().to_string(),
        subscription_digest: final_bundle.subscription_digest().to_string(),
        counter_snapshot,
    }
}

fn replay_payload_kind(bundle: &LiveReplayBundle) -> LiveOutcomeKind {
    outcome_kind_from_payload(bundle.patch_envelope().payload())
}

fn bundle_family(family: &crate::facade::foundation::LiveQueryFamily) -> LiveBundleFamily {
    match family {
        crate::facade::foundation::LiveQueryFamily::Detail => LiveBundleFamily::Detail,
        crate::facade::foundation::LiveQueryFamily::OrderedCollection => {
            LiveBundleFamily::OrderedCollection
        }
        crate::facade::foundation::LiveQueryFamily::BoundedMaterialization => {
            LiveBundleFamily::BoundedMaterialization
        }
    }
}

fn outcome_kind_from_payload(payload: &LivePatchPayload) -> LiveOutcomeKind {
    match payload {
        LivePatchPayload::Detail(_)
        | LivePatchPayload::OrderedCollection(_)
        | LivePatchPayload::BoundedMaterialization(_) => LiveOutcomeKind::Patch,
        LivePatchPayload::Suppressed(_) => LiveOutcomeKind::Suppressed,
        LivePatchPayload::Refresh(_) => LiveOutcomeKind::Refresh,
        LivePatchPayload::Coalesced(_) => LiveOutcomeKind::CoalescedDelivery,
        LivePatchPayload::ProgressAdvance { .. } => LiveOutcomeKind::ProgressAdvance,
    }
}
