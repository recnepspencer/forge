use serde::{Deserialize, Serialize};

use crate::tracing::{DecisionId, DecisionKind, DecisionTier, EntityKind, TracedDecision};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationOutcome {
    Resolved,
    Ambiguous,
    Missing,
    Incompatible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationModeSummary {
    Descendants,
    Ancestors,
    Hybrid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationOriginKindSummary {
    EulerOperator,
    GeometricIntersection,
    ConstraintSolver,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationCompatibilitySummary {
    Available,
    Unavailable,
    SchemaVersionMismatch {
        recorded: u32,
        supported: u32,
    },
    MissingLinkage {
        kind: EntityKind,
    },
    UnsupportedMode {
        mode: ReidentificationModeSummary,
    },
    UnsupportedEntityOrigin {
        origin: ReidentificationOriginKindSummary,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationFailureCauseSummary {
    EntityDeleted,
    ToleranceSnapVariant,
    UnsupportedOriginClass {
        origin: ReidentificationOriginKindSummary,
    },
    SubstrateNotBuilt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReidentificationTracePayload {
    pub decision_id: DecisionId,
    pub query_entity_kind: EntityKind,
    pub query_ancestry_hash_hex: String,
    pub query_ordinal: u32,
    pub outcome: ReidentificationOutcome,
    pub compatibility: ReidentificationCompatibilitySummary,
    pub suspected_cause: Option<ReidentificationFailureCauseSummary>,
    pub mode_used: ReidentificationModeSummary,
    pub records_scanned: u32,
    pub candidates_pre_filter: u32,
    pub candidates_post_filter: u32,
    pub index_schema_version: Option<u32>,
    pub operation_scope_id: Option<String>,
    pub source_scope_id: Option<String>,
}

impl ReidentificationTracePayload {
    pub fn validate_against_decision(
        &self,
        decision: &TracedDecision,
    ) -> Result<(), ReidentificationTraceConsistencyError> {
        if self.decision_id != decision.get_id() {
            return Err(ReidentificationTraceConsistencyError::DecisionIdMismatch);
        }
        match (self.outcome, decision.get_kind(), decision.get_tier()) {
            (ReidentificationOutcome::Resolved, DecisionKind::Exact, _) => {}
            (
                ReidentificationOutcome::Ambiguous,
                DecisionKind::Forced { .. },
                DecisionTier::Escalated,
            ) => {}
            (
                ReidentificationOutcome::Missing,
                DecisionKind::Forced { .. },
                DecisionTier::Escalated,
            ) => {}
            (
                ReidentificationOutcome::Incompatible,
                DecisionKind::Forced { .. },
                DecisionTier::Escalated,
            ) => {}
            _ => return Err(ReidentificationTraceConsistencyError::DecisionKindTierMismatch),
        }
        if self.query_ancestry_hash_hex.len() != 32
            || !self
                .query_ancestry_hash_hex
                .chars()
                .all(|c| c.is_ascii_hexdigit())
        {
            return Err(ReidentificationTraceConsistencyError::InvalidQueryHashHex);
        }
        if self.candidates_post_filter > self.candidates_pre_filter
            || self.candidates_pre_filter > self.records_scanned
        {
            return Err(ReidentificationTraceConsistencyError::CountsInconsistent);
        }
        if matches!(
            self.outcome,
            ReidentificationOutcome::Resolved | ReidentificationOutcome::Ambiguous
        ) && !matches!(
            self.compatibility,
            ReidentificationCompatibilitySummary::Available
        ) {
            return Err(ReidentificationTraceConsistencyError::OutcomeCompatibilityMismatch);
        }
        if matches!(self.outcome, ReidentificationOutcome::Incompatible)
            && matches!(
                self.compatibility,
                ReidentificationCompatibilitySummary::Available
            )
        {
            return Err(ReidentificationTraceConsistencyError::OutcomeCompatibilityMismatch);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationTraceConsistencyError {
    DecisionIdMismatch,
    DecisionKindTierMismatch,
    InvalidQueryHashHex,
    CountsInconsistent,
    OutcomeCompatibilityMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::{DecisionContext, DecisionId};

    fn sample_payload() -> ReidentificationTracePayload {
        ReidentificationTracePayload {
            decision_id: DecisionId(9),
            query_entity_kind: EntityKind::Face,
            query_ancestry_hash_hex: "000000000000000000000000deadbeef".into(),
            query_ordinal: 0,
            outcome: ReidentificationOutcome::Missing,
            compatibility: ReidentificationCompatibilitySummary::Available,
            suspected_cause: Some(ReidentificationFailureCauseSummary::EntityDeleted),
            mode_used: ReidentificationModeSummary::Descendants,
            records_scanned: 2,
            candidates_pre_filter: 2,
            candidates_post_filter: 0,
            index_schema_version: Some(1),
            operation_scope_id: Some("sheet_region_merge".into()),
            source_scope_id: Some("surviving_face".into()),
        }
    }

    #[test]
    fn reidentification_trace_payload_validator_accepts_missing_forced_decision() {
        let payload = sample_payload();
        let decision = TracedDecision::new(
            DecisionId(9),
            DecisionKind::Forced {
                reason: "ReidentificationMissing".into(),
            },
            DecisionTier::Escalated,
            0.0,
            DecisionContext::Degeneracy {
                description: "persistent_name_reidentification".into(),
            },
        );

        assert_eq!(payload.validate_against_decision(&decision), Ok(()));
    }

    #[test]
    fn reidentification_trace_payload_validator_rejects_outcome_compatibility_drift() {
        let mut payload = sample_payload();
        payload.outcome = ReidentificationOutcome::Incompatible;
        payload.compatibility = ReidentificationCompatibilitySummary::Available;
        let decision = TracedDecision::new(
            DecisionId(9),
            DecisionKind::Forced {
                reason: "ReidentificationIncompatible".into(),
            },
            DecisionTier::Escalated,
            0.0,
            DecisionContext::Degeneracy {
                description: "persistent_name_reidentification".into(),
            },
        );

        assert_eq!(
            payload.validate_against_decision(&decision),
            Err(ReidentificationTraceConsistencyError::OutcomeCompatibilityMismatch)
        );
    }
}
