use super::fact_lowering::lower_admitted_query_runtime_facts;
use super::lowering_counters::WorthUiQueryRuntimeFactLoweringCounters;
use super::receipt_digest::query_lowering_receipt_digest;
use crate::runtime::{
    WorthUiQueryBindingChangedFacts, WorthUiQueryLiveRebindOutcome,
    WorthUiQueryRuntimeFactLoweringInput, WorthUiQuerySupportDenialReceipt,
    WorthUiQuerySupportReceipt, WorthUiRuntimeFactSet,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryRuntimeFactLoweringStatus {
    AdmittedChanged,
    EquivalentNoOp,
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryRuntimeFactLoweringReceipt {
    status: WorthUiQueryRuntimeFactLoweringStatus,
    changed_facts: WorthUiQueryBindingChangedFacts,
    support_denials: Vec<WorthUiQuerySupportDenialReceipt>,
    counters: WorthUiQueryRuntimeFactLoweringCounters,
    support_receipt_digest: u64,
    active_artifact_digest_before: u64,
    candidate_artifact_digest_after: u64,
    receipt_digest: u64,
}

impl WorthUiQueryRuntimeFactLoweringReceipt {
    pub(crate) fn lower(input: WorthUiQueryRuntimeFactLoweringInput) -> Self {
        let support_receipt = input.support_receipt();
        let mut support_denials = support_denials_from_support_receipt(support_receipt);
        let mut changed_facts = WorthUiRuntimeFactSet::empty();
        let mut query_proofs_consumed = false;
        let mut live_rebind_plan_inspected = false;

        if support_denials.is_empty() {
            live_rebind_plan_inspected = true;
            let denied_live_rebind_count = denied_live_rebind_count(&input);
            if denied_live_rebind_count > 0 {
                support_denials.push(WorthUiQuerySupportDenialReceipt::live_rebind_denied(
                    support_receipt.receipt_digest(),
                    support_receipt.runtime_hook_count(),
                    denied_live_rebind_count,
                ));
            } else {
                changed_facts = lower_admitted_query_runtime_facts(&input);
                query_proofs_consumed = true;
            }
        }

        let counters = WorthUiQueryRuntimeFactLoweringCounters::from_input(
            &input,
            changed_facts.len(),
            support_denials.len(),
            query_proofs_consumed,
        );
        let status = query_lowering_status(changed_facts.len(), support_denials.len());
        let active_artifact_digest_before = input.binding_comparison().active_artifact_digest();
        let candidate_artifact_digest_after =
            input.binding_comparison().candidate_artifact_digest();
        let changed_facts = WorthUiQueryBindingChangedFacts::from_comparison_facts(
            changed_facts,
            active_artifact_digest_before,
            candidate_artifact_digest_after,
        );
        let receipt_digest = query_lowering_receipt_digest(
            status,
            support_receipt,
            &changed_facts,
            &support_denials,
            counters,
            &input,
            query_proofs_consumed,
            live_rebind_plan_inspected,
        );
        Self {
            status,
            changed_facts,
            support_denials,
            counters,
            support_receipt_digest: support_receipt.receipt_digest(),
            active_artifact_digest_before,
            candidate_artifact_digest_after,
            receipt_digest,
        }
    }

    pub fn status(&self) -> WorthUiQueryRuntimeFactLoweringStatus {
        self.status
    }

    pub fn changed_facts(&self) -> &WorthUiQueryBindingChangedFacts {
        &self.changed_facts
    }

    pub fn support_denials(&self) -> &[WorthUiQuerySupportDenialReceipt] {
        &self.support_denials
    }

    pub fn counters(&self) -> WorthUiQueryRuntimeFactLoweringCounters {
        self.counters
    }

    pub fn support_receipt_digest(&self) -> u64 {
        self.support_receipt_digest
    }

    pub fn active_artifact_digest_before(&self) -> u64 {
        self.active_artifact_digest_before
    }

    pub fn candidate_artifact_digest_after(&self) -> u64 {
        self.candidate_artifact_digest_after
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

fn support_denials_from_support_receipt(
    support_receipt: WorthUiQuerySupportReceipt,
) -> Vec<WorthUiQuerySupportDenialReceipt> {
    WorthUiQuerySupportDenialReceipt::support_not_admitted(
        support_receipt.status(),
        support_receipt.receipt_digest(),
        support_receipt.runtime_hook_count(),
    )
    .into_iter()
    .collect()
}

fn denied_live_rebind_count(input: &WorthUiQueryRuntimeFactLoweringInput) -> usize {
    input
        .live_rebind_plan()
        .entries()
        .iter()
        .filter(|entry| matches!(entry.outcome(), WorthUiQueryLiveRebindOutcome::Deny(_)))
        .count()
}

fn query_lowering_status(
    changed_fact_count: usize,
    support_denial_count: usize,
) -> WorthUiQueryRuntimeFactLoweringStatus {
    if support_denial_count > 0 {
        WorthUiQueryRuntimeFactLoweringStatus::Denied
    } else if changed_fact_count == 0 {
        WorthUiQueryRuntimeFactLoweringStatus::EquivalentNoOp
    } else {
        WorthUiQueryRuntimeFactLoweringStatus::AdmittedChanged
    }
}
