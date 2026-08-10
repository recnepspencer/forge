use crate::facade::foundation::{
    LiveExecutionEnvelope, LivePatchPayload, LiveQueryFamily, RegionScopedLiveExecutionEnvelope,
    StreamLoweredDeliveryContract,
};
use crate::harness::live_certification::{
    LiveBundleFamily, LiveCertificationBundle, LiveOutcomeKind,
};
use crate::harness::profiles::CertificationProfile;

pub(super) fn bundle_from_live_execution(
    profile: CertificationProfile,
    execution: &LiveExecutionEnvelope,
) -> LiveCertificationBundle {
    LiveCertificationBundle {
        profile,
        query_digest: execution.replay_bundle().query_digest().to_string(),
        result_digest: execution.replay_bundle().result_digest().to_string(),
        delivery_digest: execution.replay_bundle().delivery_digest().to_string(),
        replay_digest: execution.replay_bundle().replay_digest().to_string(),
        replay_step_delivery_digests: Vec::new(),
        family: bundle_family(execution.patch_envelope().family()),
        outcome_kind: outcome_kind_from_payload(execution.patch_envelope().payload()),
        outcome_digest: execution.report().outcome_digest().to_string(),
        basis_digest: execution.replay_bundle().basis_digest().to_string(),
        subscription_digest: execution.replay_bundle().subscription_digest().to_string(),
        counter_snapshot: execution.replay_bundle().counter_snapshot().clone(),
    }
}

pub(super) fn bundle_from_region_execution(
    profile: CertificationProfile,
    execution: &RegionScopedLiveExecutionEnvelope,
) -> LiveCertificationBundle {
    let replay_record = execution.region_scoped_replay_bundle().replay_record();
    let (outcome_kind, outcome_digest) = match execution.patch_envelope().payload() {
        LivePatchPayload::Detail(_)
        | LivePatchPayload::OrderedCollection(_)
        | LivePatchPayload::BoundedMaterialization(_) => (
            LiveOutcomeKind::Patch,
            execution.patch_envelope().delivery_digest().to_string(),
        ),
        LivePatchPayload::Suppressed(_) => (
            LiveOutcomeKind::Suppressed,
            execution.patch_envelope().delivery_digest().to_string(),
        ),
        LivePatchPayload::Refresh(_) => (
            LiveOutcomeKind::Refresh,
            execution.patch_envelope().delivery_digest().to_string(),
        ),
        LivePatchPayload::ProgressAdvance { .. } => (
            LiveOutcomeKind::ProgressAdvance,
            execution.patch_envelope().delivery_digest().to_string(),
        ),
        LivePatchPayload::Coalesced(_) => (
            LiveOutcomeKind::CoalescedDelivery,
            execution.patch_envelope().delivery_digest().to_string(),
        ),
    };
    LiveCertificationBundle {
        profile,
        query_digest: replay_record.query_digest().to_string(),
        result_digest: execution.replay_bundle().result_digest().to_string(),
        delivery_digest: replay_record.delivery_digest().to_string(),
        replay_digest: replay_record.replay_digest().to_string(),
        replay_step_delivery_digests: Vec::new(),
        family: bundle_family(execution.patch_envelope().family()),
        outcome_kind,
        outcome_digest,
        basis_digest: execution.replay_bundle().basis_digest().to_string(),
        subscription_digest: execution.replay_bundle().subscription_digest().to_string(),
        counter_snapshot: execution
            .region_scoped_replay_bundle()
            .counter_snapshot()
            .clone(),
    }
}

pub(super) fn bundle_from_stream_contract(
    profile: CertificationProfile,
    execution: &RegionScopedLiveExecutionEnvelope,
    contract: &StreamLoweredDeliveryContract,
) -> LiveCertificationBundle {
    LiveCertificationBundle {
        profile,
        query_digest: contract
            .query_delivery_contract()
            .query_digest()
            .to_string(),
        result_digest: execution.replay_bundle().result_digest().to_string(),
        delivery_digest: contract
            .query_delivery_contract()
            .delivery_digest()
            .to_string(),
        replay_digest: contract.replay_record().replay_digest().to_string(),
        replay_step_delivery_digests: Vec::new(),
        family: bundle_family(contract.query_delivery_contract().family()),
        outcome_kind: LiveOutcomeKind::StreamLoweredDelivery,
        outcome_digest: contract.stream_contract_digest().to_string(),
        basis_digest: execution.replay_bundle().basis_digest().to_string(),
        subscription_digest: execution.replay_bundle().subscription_digest().to_string(),
        counter_snapshot: contract.counter_snapshot().clone(),
    }
}

fn bundle_family(family: &LiveQueryFamily) -> LiveBundleFamily {
    match family {
        LiveQueryFamily::Detail => LiveBundleFamily::Detail,
        LiveQueryFamily::OrderedCollection => LiveBundleFamily::OrderedCollection,
        LiveQueryFamily::BoundedMaterialization => LiveBundleFamily::BoundedMaterialization,
    }
}

fn outcome_kind_from_payload(payload: &LivePatchPayload) -> LiveOutcomeKind {
    match payload {
        LivePatchPayload::Detail(_)
        | LivePatchPayload::OrderedCollection(_)
        | LivePatchPayload::BoundedMaterialization(_) => LiveOutcomeKind::Patch,
        LivePatchPayload::Suppressed(_) => LiveOutcomeKind::Suppressed,
        LivePatchPayload::Refresh(_) => LiveOutcomeKind::Refresh,
        LivePatchPayload::ProgressAdvance { .. } => LiveOutcomeKind::ProgressAdvance,
        LivePatchPayload::Coalesced(_) => LiveOutcomeKind::CoalescedDelivery,
    }
}
