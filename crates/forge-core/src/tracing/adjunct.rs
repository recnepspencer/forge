//! Versioned typed adjunct payload transport for trace decisions.
//!
//! Adjuncts carry domain-specific structured payloads (policy resolution,
//! provenance, replay witnesses, etc.) alongside `TracedDecision` records
//! without forcing every payload family into the core `TracedDecision` schema.

use serde::{Deserialize, Serialize};

use super::{DecisionId, PolicyDecisionTracePayload};

/// Stable payload kind tag for `PolicyDecisionTracePayload`.
pub const POLICY_DECISION_TRACE_PAYLOAD_KIND: &str = "policy_decision";

/// Version of the serialized `PolicyDecisionTracePayload` adjunct schema.
pub const POLICY_DECISION_TRACE_PAYLOAD_VERSION: u32 = 2;

/// Versioned typed adjunct payload attached to a trace decision.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceAdjunctRecord {
    pub decision_id: DecisionId,
    /// Stable snake_case payload family identifier.
    pub payload_kind: String,
    /// Payload-family-local schema version.
    pub payload_version: u32,
    /// Serialized typed payload transport.
    pub payload_json: serde_json::Value,
}

impl TraceAdjunctRecord {
    pub fn new(
        decision_id: DecisionId,
        payload_kind: impl Into<String>,
        payload_version: u32,
        payload_json: serde_json::Value,
    ) -> Self {
        Self {
            decision_id,
            payload_kind: payload_kind.into(),
            payload_version,
            payload_json,
        }
    }

    /// Canonical sort key used to keep adjunct ordering deterministic.
    pub fn sort_key(&self) -> (u64, &str, u32) {
        (self.decision_id.0, self.payload_kind.as_str(), self.payload_version)
    }

    /// Build a versioned adjunct record from a typed policy payload.
    pub fn from_policy_payload(payload: &PolicyDecisionTracePayload) -> Self {
        let payload_json = serde_json::to_value(payload)
            .expect("PolicyDecisionTracePayload must serialize for trace adjunct transport");
        Self::new(
            payload.decision_id,
            POLICY_DECISION_TRACE_PAYLOAD_KIND,
            POLICY_DECISION_TRACE_PAYLOAD_VERSION,
            payload_json,
        )
    }

    /// Decode a typed policy payload from an adjunct record.
    pub fn as_policy_payload(&self) -> Option<Result<PolicyDecisionTracePayload, serde_json::Error>> {
        if self.payload_kind != POLICY_DECISION_TRACE_PAYLOAD_KIND {
            return None;
        }
        Some(serde_json::from_value(self.payload_json.clone()))
    }
}

/// Deterministic adjunct collection for transport and persistence.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TraceAdjunctSet {
    records: Vec<TraceAdjunctRecord>,
}

impl TraceAdjunctSet {
    pub fn new() -> Self {
        Self { records: Vec::new() }
    }

    pub fn from_records(mut records: Vec<TraceAdjunctRecord>) -> Self {
        records.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
        Self { records }
    }

    pub fn insert(&mut self, record: TraceAdjunctRecord) {
        self.records.push(record);
        self.records.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
    }

    pub fn records(&self) -> &[TraceAdjunctRecord] {
        &self.records
    }

    pub fn into_records(self) -> Vec<TraceAdjunctRecord> {
        self.records
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::PolicyKind;
    use crate::tracing::{
        CandidateValueSummary, DecisionContext, DecisionKind, DecisionTier, PolicyResolutionOutcome,
        PolicyResolutionSource, PolicyTraceConsistencyError, TracedDecision,
    };

    fn sample_policy_payload() -> PolicyDecisionTracePayload {
        PolicyDecisionTracePayload {
            decision_id: DecisionId(12),
            policy_kind: PolicyKind::CoincidentGeometry,
            operation_scope_id: Some("sheet_region_merge".into()),
            query_location: [1.0, 2.0, 3.0],
            measured_margin: 1.0e-7,
            threshold: Some(1.0e-6),
            overridable: true,
            candidate_summary: CandidateValueSummary::EnumTag {
                type_name: "WeakSimpleCertificate".into(),
                variant: "WeaklySimple".into(),
            },
            outcome: PolicyResolutionOutcome::AcceptedPotentialValue,
            source: PolicyResolutionSource::DefaultPolicy,
            source_scope: None,
            default_used: true,
        }
    }

    #[test]
    fn trace_adjunct_set_orders_records_deterministically() {
        let mut set = TraceAdjunctSet::new();
        set.insert(TraceAdjunctRecord::new(
            DecisionId(9),
            "z_payload",
            1,
            serde_json::json!({"v": 1}),
        ));
        set.insert(TraceAdjunctRecord::new(
            DecisionId(3),
            "b_payload",
            2,
            serde_json::json!({"v": 2}),
        ));
        set.insert(TraceAdjunctRecord::new(
            DecisionId(3),
            "a_payload",
            1,
            serde_json::json!({"v": 3}),
        ));

        let keys: Vec<_> = set.records().iter().map(|r| r.sort_key()).collect();
        assert_eq!(keys, vec![(3, "a_payload", 1), (3, "b_payload", 2), (9, "z_payload", 1)]);
    }

    #[test]
    fn policy_trace_payload_round_trips_through_trace_adjunct_record() {
        let payload = sample_policy_payload();
        let record = TraceAdjunctRecord::from_policy_payload(&payload);
        let decoded = record
            .as_policy_payload()
            .expect("policy kind")
            .expect("decode policy payload");
        assert_eq!(decoded, payload);
    }

    #[test]
    fn unknown_adjunct_version_round_trips_without_semantic_interpretation() {
        let record = TraceAdjunctRecord::new(
            DecisionId(5),
            "future_payload",
            99,
            serde_json::json!({"nested": {"x": 1}, "arr": [1,2,3]}),
        );
        let json = serde_json::to_string(&record).expect("serialize adjunct");
        let restored: TraceAdjunctRecord = serde_json::from_str(&json).expect("deserialize adjunct");
        assert_eq!(restored, record);
        assert!(restored.as_policy_payload().is_none(), "unknown adjunct kind remains opaque");
    }

    #[test]
    fn policy_adjunct_contradiction_is_detectable_via_typed_validator() {
        let mut payload = sample_policy_payload();
        payload.default_used = false; // contradicts DefaultPolicy source
        let record = TraceAdjunctRecord::from_policy_payload(&payload);
        let decoded = record
            .as_policy_payload()
            .expect("policy kind")
            .expect("decode policy payload");

        let decision = TracedDecision::new(
            DecisionId(12),
            DecisionKind::PolicyApplied {
                policy: PolicyKind::CoincidentGeometry,
                default_used: false,
            },
            DecisionTier::PolicyApplied,
            1.0e-7,
            DecisionContext::Tolerance {
                measured: 1.0e-7,
                threshold: 1.0e-6,
            },
        );
        assert_eq!(
            decoded.validate_against_decision(&decision),
            Err(PolicyTraceConsistencyError::SourceDefaultMismatch)
        );
    }
}
