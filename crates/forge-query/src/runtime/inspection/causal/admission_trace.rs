use std::collections::HashMap;

use crate::identity::hash_parts;

use super::admission_decision::{
    CausalInspectionAdmissionDecision, CausalInspectionAdmissionDecisionKind,
    CausalInspectionAdmissionSubject,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalDecisionTraceRow {
    key: String,
    span: String,
    decision: CausalInspectionAdmissionDecisionKind,
    authority: String,
    reason: String,
    row_digest: String,
}

impl CausalDecisionTraceRow {
    pub(super) fn new(
        key: &'static str,
        span: &'static str,
        decision: CausalInspectionAdmissionDecisionKind,
        authority: &'static str,
        reason: &'static str,
    ) -> Self {
        let row_digest = hash_parts(&[
            "causal_decision_trace_row_v1".to_string(),
            format!("key:{key}"),
            format!("span:{span}"),
            format!("decision:{}", decision.as_str()),
            format!("authority:{authority}"),
            format!("reason:{reason}"),
        ]);
        Self {
            key: key.to_string(),
            span: span.to_string(),
            decision,
            authority: authority.to_string(),
            reason: reason.to_string(),
            row_digest,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn span(&self) -> &str {
        &self.span
    }

    pub fn decision(&self) -> CausalInspectionAdmissionDecisionKind {
        self.decision
    }

    pub fn authority(&self) -> &str {
        &self.authority
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalDecisionTraceIndex {
    rows: Vec<CausalDecisionTraceRow>,
    lookup: HashMap<String, usize>,
    trace_digest: String,
}

impl CausalDecisionTraceIndex {
    pub(super) fn new(rows: Vec<CausalDecisionTraceRow>) -> Self {
        let lookup = rows
            .iter()
            .enumerate()
            .map(|(index, row)| (row.key().to_string(), index))
            .collect();
        let row_part = rows
            .iter()
            .map(CausalDecisionTraceRow::row_digest)
            .collect::<Vec<_>>()
            .join("|");
        let trace_digest = hash_parts(&[
            "causal_decision_trace_index_v1".to_string(),
            format!("rows:{row_part}"),
        ]);
        Self {
            rows,
            lookup,
            trace_digest,
        }
    }

    pub fn rows(&self) -> &[CausalDecisionTraceRow] {
        &self.rows
    }

    pub fn row_for_key(&self, key: &str) -> Option<&CausalDecisionTraceRow> {
        self.lookup.get(key).and_then(|index| self.rows.get(*index))
    }

    pub fn trace_digest(&self) -> &str {
        &self.trace_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionAdmissionCounters {
    causal_inspection_proof_transition_count: usize,
    causal_inspection_proof_outcome_count: usize,
    causal_inspection_proof_readmission_count: usize,
    causal_inspection_request_count: usize,
    causal_inspection_admission_count: usize,
    causal_inspection_advisory_count: usize,
    causal_inspection_denial_count: usize,
    causal_decision_trace_lookup_count: usize,
    causal_decision_trace_index_hit_count: usize,
    bridge_causal_envelope_request_count: usize,
    counter_snapshot: String,
}

impl CausalInspectionAdmissionCounters {
    pub(super) fn new(
        kind: CausalInspectionAdmissionDecisionKind,
        trace: &CausalDecisionTraceIndex,
    ) -> Self {
        let advisory_count = usize::from(kind == CausalInspectionAdmissionDecisionKind::Advisory);
        let denial_count = usize::from(kind == CausalInspectionAdmissionDecisionKind::Violation);
        let trace_lookup_count = trace.rows().len();
        let trace_hit_count = trace
            .rows()
            .iter()
            .filter(|row| trace.row_for_key(row.key()).is_some())
            .count();
        let counter_snapshot = hash_parts(&[
            "causal_inspection_admission_counters_v1".to_string(),
            "causal_inspection_proof_transition_count:1".to_string(),
            "causal_inspection_proof_outcome_count:1".to_string(),
            "causal_inspection_proof_readmission_count:0".to_string(),
            "causal_inspection_request_count:1".to_string(),
            "causal_inspection_admission_count:1".to_string(),
            format!("causal_inspection_advisory_count:{advisory_count}"),
            format!("causal_inspection_denial_count:{denial_count}"),
            format!("causal_decision_trace_lookup_count:{trace_lookup_count}"),
            format!("causal_decision_trace_index_hit_count:{trace_hit_count}"),
            "bridge_causal_envelope_request_count:0".to_string(),
        ]);
        Self {
            causal_inspection_proof_transition_count: 1,
            causal_inspection_proof_outcome_count: 1,
            causal_inspection_proof_readmission_count: 0,
            causal_inspection_request_count: 1,
            causal_inspection_admission_count: 1,
            causal_inspection_advisory_count: advisory_count,
            causal_inspection_denial_count: denial_count,
            causal_decision_trace_lookup_count: trace_lookup_count,
            causal_decision_trace_index_hit_count: trace_hit_count,
            bridge_causal_envelope_request_count: 0,
            counter_snapshot,
        }
    }

    pub fn causal_inspection_proof_transition_count(&self) -> usize {
        self.causal_inspection_proof_transition_count
    }

    pub fn causal_inspection_proof_outcome_count(&self) -> usize {
        self.causal_inspection_proof_outcome_count
    }

    pub fn causal_inspection_proof_readmission_count(&self) -> usize {
        self.causal_inspection_proof_readmission_count
    }

    pub fn causal_inspection_request_count(&self) -> usize {
        self.causal_inspection_request_count
    }

    pub fn causal_inspection_admission_count(&self) -> usize {
        self.causal_inspection_admission_count
    }

    pub fn causal_inspection_advisory_count(&self) -> usize {
        self.causal_inspection_advisory_count
    }

    pub fn causal_inspection_denial_count(&self) -> usize {
        self.causal_inspection_denial_count
    }

    pub fn causal_decision_trace_lookup_count(&self) -> usize {
        self.causal_decision_trace_lookup_count
    }

    pub fn causal_decision_trace_index_hit_count(&self) -> usize {
        self.causal_decision_trace_index_hit_count
    }

    pub fn bridge_causal_envelope_request_count(&self) -> usize {
        self.bridge_causal_envelope_request_count
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionAdmissionReceipt {
    receipt_digest: String,
    subject_digest: String,
    decision_digest: String,
    decision_trace_index_digest: String,
    counter_snapshot: String,
}

impl CausalInspectionAdmissionReceipt {
    pub(super) fn new(
        subject: &CausalInspectionAdmissionSubject,
        decision: &CausalInspectionAdmissionDecision,
        trace: &CausalDecisionTraceIndex,
        counters: &CausalInspectionAdmissionCounters,
    ) -> Self {
        let receipt_digest = hash_parts(&[
            "causal_inspection_admission_receipt_v1".to_string(),
            format!("subject:{}", subject.subject_digest()),
            format!("decision:{}", decision.decision_digest()),
            format!("trace:{}", trace.trace_digest()),
            format!("counters:{}", counters.counter_snapshot()),
        ]);
        Self {
            receipt_digest,
            subject_digest: subject.subject_digest().to_string(),
            decision_digest: decision.decision_digest().to_string(),
            decision_trace_index_digest: trace.trace_digest().to_string(),
            counter_snapshot: counters.counter_snapshot().to_string(),
        }
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn subject_digest(&self) -> &str {
        &self.subject_digest
    }

    pub fn decision_digest(&self) -> &str {
        &self.decision_digest
    }

    pub fn decision_trace_index_digest(&self) -> &str {
        &self.decision_trace_index_digest
    }

    pub fn counter_snapshot(&self) -> &str {
        &self.counter_snapshot
    }
}
