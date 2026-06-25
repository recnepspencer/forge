use forge_query::facade::ForgeQueryGraphObligationSelectionCounters;

use crate::topology_operators::TopologyTouchedGraphCounters;
use crate::validator_invariant_catalog::selection_from_touched_closure::WorthTopologyLegalitySelectionDenial;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologyLegalitySelectionCounters {
    touched_entity_count: usize,
    touched_relation_count: usize,
    touched_relation_kind_count: usize,
    touched_aspect_count: usize,
    touched_scope_count: usize,
    candidate_obligation_count: usize,
    selected_obligation_count: usize,
    denied_obligation_count: usize,
    missing_access_receipt_count: usize,
    budget_denial_count: usize,
    support_posture_denial_count: usize,
    whole_view_residue_count: usize,
    query_registration_full_scan_count: usize,
    counters_digest: String,
}

impl WorthTopologyLegalitySelectionCounters {
    pub(super) fn from_selection(
        touched_counters: TopologyTouchedGraphCounters,
        query_counters: &ForgeQueryGraphObligationSelectionCounters,
        selected_obligation_count: usize,
        denials: &[WorthTopologyLegalitySelectionDenial],
    ) -> Self {
        let missing_access_receipt_count = denials
            .iter()
            .filter(|denial| {
                denial.kind()
                    == crate::validator_invariant_catalog::WorthTopologyLegalitySelectionDenialKind::MissingAccessReceipt
            })
            .count();
        let budget_denial_count = denials
            .iter()
            .filter(|denial| {
                denial.kind()
                    == crate::validator_invariant_catalog::WorthTopologyLegalitySelectionDenialKind::BudgetExceeded
            })
            .count();
        let support_posture_denial_count =
            denials.len() - missing_access_receipt_count - budget_denial_count;
        let counters_digest = [
            "worth-topo-legality-selection-counters-v1".to_string(),
            format!("touched-entities:{}", touched_counters.entity_count()),
            format!("touched-relations:{}", touched_counters.relation_count()),
            format!(
                "touched-relation-kinds:{}",
                touched_counters.relation_kind_count()
            ),
            format!(
                "touched-aspects:{}",
                touched_counters.touched_aspect_count()
            ),
            format!("touched-scopes:{}", touched_counters.topology_scope_count()),
            format!(
                "candidate-obligations:{}",
                query_counters.deduplicated_candidate_count()
            ),
            format!("selected-obligations:{selected_obligation_count}"),
            format!("denied-obligations:{}", denials.len()),
            format!("missing-access-receipts:{missing_access_receipt_count}"),
            format!("budget-denials:{budget_denial_count}"),
            format!("support-posture-denials:{support_posture_denial_count}"),
            "whole-view-residue:0".to_string(),
            format!(
                "query-registration-full-scan:{}",
                query_counters.registration_full_scan_count()
            ),
        ]
        .join("|");
        Self {
            touched_entity_count: touched_counters.entity_count(),
            touched_relation_count: touched_counters.relation_count(),
            touched_relation_kind_count: touched_counters.relation_kind_count(),
            touched_aspect_count: touched_counters.touched_aspect_count(),
            touched_scope_count: touched_counters.topology_scope_count(),
            candidate_obligation_count: query_counters.deduplicated_candidate_count(),
            selected_obligation_count,
            denied_obligation_count: denials.len(),
            missing_access_receipt_count,
            budget_denial_count,
            support_posture_denial_count,
            whole_view_residue_count: 0,
            query_registration_full_scan_count: query_counters.registration_full_scan_count(),
            counters_digest,
        }
    }

    pub const fn touched_entity_count(&self) -> usize {
        self.touched_entity_count
    }

    pub const fn touched_relation_count(&self) -> usize {
        self.touched_relation_count
    }

    pub const fn touched_relation_kind_count(&self) -> usize {
        self.touched_relation_kind_count
    }

    pub const fn touched_aspect_count(&self) -> usize {
        self.touched_aspect_count
    }

    pub const fn touched_scope_count(&self) -> usize {
        self.touched_scope_count
    }

    pub const fn candidate_obligation_count(&self) -> usize {
        self.candidate_obligation_count
    }

    pub const fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub const fn denied_obligation_count(&self) -> usize {
        self.denied_obligation_count
    }

    pub const fn missing_access_receipt_count(&self) -> usize {
        self.missing_access_receipt_count
    }

    pub const fn budget_denial_count(&self) -> usize {
        self.budget_denial_count
    }

    pub const fn support_posture_denial_count(&self) -> usize {
        self.support_posture_denial_count
    }

    pub const fn whole_view_residue_count(&self) -> usize {
        self.whole_view_residue_count
    }

    pub const fn query_registration_full_scan_count(&self) -> usize {
        self.query_registration_full_scan_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }
}
