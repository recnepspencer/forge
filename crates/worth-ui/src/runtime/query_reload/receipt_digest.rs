use crate::runtime::{
    WorthUiQueryBindingChangedFacts, WorthUiQueryBindingDriftDenialKind,
    WorthUiQueryBindingPostureDriftFamily, WorthUiQueryBindingRebindReason,
    WorthUiQueryBindingRetirementReason, WorthUiQueryLiveRebindOutcome, WorthUiQueryLiveRebindPlan,
    WorthUiQueryRuntimeFactLoweringCounters, WorthUiQueryRuntimeFactLoweringInput,
    WorthUiQueryRuntimeFactLoweringStatus, WorthUiQuerySupportDenialKind,
    WorthUiQuerySupportDenialReceipt, WorthUiQuerySupportReceipt, WorthUiQuerySupportStatus,
};

pub(super) fn query_lowering_receipt_digest(
    status: WorthUiQueryRuntimeFactLoweringStatus,
    support_receipt: WorthUiQuerySupportReceipt,
    changed_facts: &WorthUiQueryBindingChangedFacts,
    support_denials: &[WorthUiQuerySupportDenialReceipt],
    counters: WorthUiQueryRuntimeFactLoweringCounters,
    input: &WorthUiQueryRuntimeFactLoweringInput,
    query_proofs_consumed: bool,
    live_rebind_plan_inspected: bool,
) -> u64 {
    let mut digest = 0x7175_6572_795f_7007u64;
    digest ^= status as u64;
    digest = digest.rotate_left(7) ^ support_receipt.receipt_digest();
    digest = digest.rotate_left(11) ^ changed_facts.changed_facts().digest().value();
    digest = digest.rotate_left(13) ^ changed_facts.active_artifact_digest_before();
    digest = digest.rotate_left(17) ^ changed_facts.candidate_artifact_digest_after();
    digest = digest.rotate_left(19) ^ counter_digest(counters);
    if live_rebind_plan_inspected {
        digest = digest.rotate_left(23) ^ query_live_rebind_plan_digest(input.live_rebind_plan());
    }
    if query_proofs_consumed {
        digest = digest.rotate_left(29) ^ query_proof_input_digest(input);
    }
    for denial in support_denials {
        digest = digest.rotate_left(5) ^ query_denial_kind_digest(denial.kind());
        digest = digest.rotate_left(7) ^ query_support_status_digest(denial.support_status());
        digest = digest.rotate_left(19) ^ denial.support_receipt_digest();
        digest = digest.rotate_left(23) ^ denial.denied_binding_count() as u64;
    }
    digest
}

fn counter_digest(counters: WorthUiQueryRuntimeFactLoweringCounters) -> u64 {
    let mut digest = 0x636f_756e_7473_7007u64;
    digest = digest.rotate_left(5) ^ counters.bindings_compared() as u64;
    digest = digest.rotate_left(7) ^ counters.live_rebind_entries() as u64;
    digest = digest.rotate_left(11) ^ counters.consumed_projection_fact_count() as u64;
    digest = digest.rotate_left(13) ^ counters.consumed_state_snapshot_count() as u64;
    digest = digest.rotate_left(17) ^ counters.consumed_effect_posture_count() as u64;
    digest = digest.rotate_left(19) ^ counters.virtualized_frame_target_count() as u64;
    digest = digest.rotate_left(23) ^ counters.changed_fact_count() as u64;
    digest.rotate_left(29) ^ counters.support_denial_count() as u64
}

fn query_live_rebind_plan_digest(plan: &WorthUiQueryLiveRebindPlan) -> u64 {
    let mut digest = 0x6c69_7665_706c_7007u64;
    digest = digest.rotate_left(5) ^ plan.active_artifact_digest();
    digest = digest.rotate_left(7) ^ plan.candidate_artifact_digest();
    let counters = plan.counters();
    digest = digest.rotate_left(11) ^ counters.bindings_planned() as u64;
    digest = digest.rotate_left(13) ^ counters.preserved_binding_count() as u64;
    digest = digest.rotate_left(17) ^ counters.rebound_binding_count() as u64;
    digest = digest.rotate_left(19) ^ counters.retired_binding_count() as u64;
    digest = digest.rotate_left(23) ^ counters.denied_binding_count() as u64;
    for entry in plan.entries() {
        digest = digest.rotate_left(29) ^ stable_text_digest(entry.identity().view_binding_id());
        digest = digest.rotate_left(31) ^ query_live_rebind_outcome_digest(entry.outcome());
    }
    digest
}

fn query_live_rebind_outcome_digest(outcome: &WorthUiQueryLiveRebindOutcome) -> u64 {
    match outcome {
        WorthUiQueryLiveRebindOutcome::Preserve(preservation) => {
            0x1000
                ^ stable_text_digest(preservation.identity().view_binding_id()).rotate_left(5)
                ^ stable_text_digest(preservation.preservation_receipt()).rotate_left(7)
                ^ stable_text_digest(preservation.preserved_posture().live_compatibility_digest())
                    .rotate_left(11)
        }
        WorthUiQueryLiveRebindOutcome::Rebind(rebind) => {
            let mut digest = 0x2000
                ^ stable_text_digest(rebind.identity().view_binding_id()).rotate_left(5)
                ^ query_rebind_reason_digest(rebind.reason()).rotate_left(7);
            for drift in rebind.drift_families() {
                digest = digest.rotate_left(11) ^ query_drift_family_digest(*drift);
            }
            digest
        }
        WorthUiQueryLiveRebindOutcome::Retire(retirement) => {
            0x3000
                ^ stable_text_digest(retirement.identity().view_binding_id()).rotate_left(5)
                ^ query_retirement_reason_digest(retirement.reason()).rotate_left(7)
        }
        WorthUiQueryLiveRebindOutcome::Deny(denial) => {
            let mut digest = 0x4000
                ^ stable_text_digest(denial.identity().view_binding_id()).rotate_left(5)
                ^ query_drift_denial_kind_digest(denial.reason()).rotate_left(7);
            for drift in denial.drift_families() {
                digest = digest.rotate_left(11) ^ query_drift_family_digest(*drift);
            }
            digest
        }
    }
}

fn query_proof_input_digest(input: &WorthUiQueryRuntimeFactLoweringInput) -> u64 {
    let mut digest = 0x5155_4552_5950_7007u64;
    for receipt in input.projection_fact_receipts() {
        digest = digest.rotate_left(7) ^ stable_text_digest(receipt.receipt_identity());
        digest = digest.rotate_left(11) ^ receipt.receipt_digest();
    }
    for receipt in input.state_snapshot_receipts() {
        digest = digest.rotate_left(13) ^ stable_text_digest(receipt.receipt_identity());
        digest = digest.rotate_left(17) ^ receipt.receipt_digest();
    }
    for receipt in input.effect_posture_receipts() {
        digest = digest.rotate_left(19) ^ stable_text_digest(receipt.receipt_identity());
        digest = digest.rotate_left(23) ^ receipt.receipt_digest();
    }
    for target in input.virtualized_frame_targets() {
        digest = digest.rotate_left(29) ^ stable_text_digest(&target.digest_basis());
    }
    digest
}

fn query_denial_kind_digest(kind: WorthUiQuerySupportDenialKind) -> u64 {
    match kind {
        WorthUiQuerySupportDenialKind::Deferred => 1,
        WorthUiQuerySupportDenialKind::Unsupported => 2,
        WorthUiQuerySupportDenialKind::LiveRebindDenied => 3,
    }
}

fn query_support_status_digest(status: WorthUiQuerySupportStatus) -> u64 {
    match status {
        WorthUiQuerySupportStatus::Supported => 1,
        WorthUiQuerySupportStatus::Deferred => 2,
        WorthUiQuerySupportStatus::Unsupported => 3,
    }
}

fn query_drift_family_digest(drift: WorthUiQueryBindingPostureDriftFamily) -> u64 {
    match drift {
        WorthUiQueryBindingPostureDriftFamily::SupportAdmission => 1,
        WorthUiQueryBindingPostureDriftFamily::BasisCapability => 2,
        WorthUiQueryBindingPostureDriftFamily::LiveCompatibility => 3,
        WorthUiQueryBindingPostureDriftFamily::AsyncResultState => 4,
        WorthUiQueryBindingPostureDriftFamily::Recovery => 5,
        WorthUiQueryBindingPostureDriftFamily::Inspection => 6,
        WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption => 7,
        WorthUiQueryBindingPostureDriftFamily::DenialPresentation => 8,
    }
}

fn query_rebind_reason_digest(reason: WorthUiQueryBindingRebindReason) -> u64 {
    match reason {
        WorthUiQueryBindingRebindReason::FreshCandidateBinding => 1,
        WorthUiQueryBindingRebindReason::QueryIdentityChanged => 2,
        WorthUiQueryBindingRebindReason::QueryOwnedPostureDrift => 3,
    }
}

fn query_retirement_reason_digest(reason: WorthUiQueryBindingRetirementReason) -> u64 {
    match reason {
        WorthUiQueryBindingRetirementReason::CandidateRemovedQueryBinding => 1,
    }
}

fn query_drift_denial_kind_digest(kind: WorthUiQueryBindingDriftDenialKind) -> u64 {
    match kind {
        WorthUiQueryBindingDriftDenialKind::UiLocalDenialPresentationWouldReplaceQueryRecovery => 1,
        WorthUiQueryBindingDriftDenialKind::QuerySupportPostureNotAdmitted => 2,
        WorthUiQueryBindingDriftDenialKind::MissingCandidatePostureForRebind => 3,
        WorthUiQueryBindingDriftDenialKind::MissingActivePostureForRetirement => 4,
    }
}

fn stable_text_digest(text: &str) -> u64 {
    text.bytes().fold(0x7374_6162_6c65_7007u64, |digest, byte| {
        digest.rotate_left(5) ^ u64::from(byte)
    })
}
