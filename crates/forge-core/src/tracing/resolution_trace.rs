use serde::{Deserialize, Serialize};

use crate::provenance::SnapshotHandleRef;
use crate::tracing::{DecisionId, DecisionKind, DecisionTier, EntityKind, TracedDecision};

/// Outcome category for persistent-name/selector resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionOutcome {
    Resolved,
    Ambiguous,
    Missing,
    Incompatible,
}

/// Resolver path used to produce a candidate or final resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionRoute {
    DirectPersistentName,
    LineageReidentified,
    Hybrid,
    None,
}

/// Typed candidate match kind (avoid string parsing in audit/replay tooling).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionMatchKind {
    ExactPersistentName,
    SelectorMatch,
    LineageDescendant,
    LineageAncestor,
    AliasMatch { alias_kind: String },
    Other { tag: String },
}

/// Serializable summary of a name/selector query for trace/audit correlation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionQuerySummary {
    PersistentName {
        entity_kind: EntityKind,
        ancestry_hash_hex: String,
        ordinal: u32,
    },
    Selector {
        entity_kind: Option<EntityKind>,
        selector_kind: String,
    },
}

/// Compact candidate summary stored in typed trace adjuncts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionCandidateSummary {
    pub entity_kind: EntityKind,
    pub persistent_ref: String,
    pub snapshot_ref: SnapshotHandleRef,
    pub route: ResolutionRoute,
    pub match_kind: ResolutionMatchKind,
}

impl ResolutionCandidateSummary {
    /// Canonical deterministic ordering key for trace payload candidate lists.
    pub fn sort_key(&self) -> (u8, &str, u32, u32, u8, String) {
        (
            self.entity_kind as u8,
            self.persistent_ref.as_str(),
            self.snapshot_ref.index,
            self.snapshot_ref.generation,
            self.route as u8,
            match &self.match_kind {
                ResolutionMatchKind::ExactPersistentName => "exact".to_string(),
                ResolutionMatchKind::SelectorMatch => "selector".to_string(),
                ResolutionMatchKind::LineageDescendant => "lineage_desc".to_string(),
                ResolutionMatchKind::LineageAncestor => "lineage_anc".to_string(),
                ResolutionMatchKind::AliasMatch { alias_kind } => format!("alias:{alias_kind}"),
                ResolutionMatchKind::Other { tag } => format!("other:{tag}"),
            },
        )
    }
}

/// Typed adjunct payload attached to a trace decision for persistent-name resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionTracePayload {
    pub decision_id: DecisionId,
    pub query: ResolutionQuerySummary,
    pub outcome: ResolutionOutcome,
    pub final_route: ResolutionRoute,
    pub routes_attempted: Vec<ResolutionRoute>,
    pub candidate_count: u32,
    pub ordered_candidates: Vec<ResolutionCandidateSummary>,
    pub operation_scope_id: Option<String>,
    pub source_scope_id: Option<String>,
    pub candidate_set_hash: Option<u64>,
}

impl ResolutionTracePayload {
    /// Keep candidate summaries in canonical order before persistence/comparison.
    pub fn canonicalize(&mut self) {
        self.ordered_candidates.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
        self.candidate_count = self.ordered_candidates.len() as u32;
    }

    pub fn validate_against_decision(
        &self,
        decision: &TracedDecision,
    ) -> Result<(), ResolutionTraceConsistencyError> {
        if self.decision_id != decision.get_id() {
            return Err(ResolutionTraceConsistencyError::DecisionIdMismatch);
        }
        match (self.outcome, decision.get_kind(), decision.get_tier()) {
            (ResolutionOutcome::Resolved, DecisionKind::Exact, _) => {}
            (ResolutionOutcome::Ambiguous, DecisionKind::Forced { .. }, DecisionTier::Escalated) => {}
            (ResolutionOutcome::Missing, DecisionKind::Forced { .. }, DecisionTier::Escalated) => {}
            (ResolutionOutcome::Incompatible, DecisionKind::Forced { .. }, DecisionTier::Escalated) => {}
            _ => return Err(ResolutionTraceConsistencyError::DecisionKindTierMismatch),
        }
        let mut clone = self.clone();
        clone.canonicalize();
        if clone.ordered_candidates != self.ordered_candidates || clone.candidate_count != self.candidate_count {
            return Err(ResolutionTraceConsistencyError::CandidatesNotCanonical);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionTraceConsistencyError {
    DecisionIdMismatch,
    DecisionKindTierMismatch,
    CandidatesNotCanonical,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::{DecisionContext, DecisionTier};

    fn sample_candidate(persistent: &str, idx: u32, gen: u32) -> ResolutionCandidateSummary {
        ResolutionCandidateSummary {
            entity_kind: EntityKind::Face,
            persistent_ref: persistent.to_string(),
            snapshot_ref: SnapshotHandleRef::new(EntityKind::Face, idx, gen),
            route: ResolutionRoute::DirectPersistentName,
            match_kind: ResolutionMatchKind::ExactPersistentName,
        }
    }

    #[test]
    fn resolution_trace_payload_canonicalizes_candidate_order() {
        let mut payload = ResolutionTracePayload {
            decision_id: DecisionId(7),
            query: ResolutionQuerySummary::Selector {
                entity_kind: Some(EntityKind::Face),
                selector_kind: "by_feature".into(),
            },
            outcome: ResolutionOutcome::Ambiguous,
            final_route: ResolutionRoute::DirectPersistentName,
            routes_attempted: vec![ResolutionRoute::DirectPersistentName],
            candidate_count: 0,
            ordered_candidates: vec![
                sample_candidate("face:b", 12, 1),
                sample_candidate("face:a", 9, 2),
            ],
            operation_scope_id: Some("sheet_region_merge".into()),
            source_scope_id: None,
            candidate_set_hash: None,
        };
        payload.canonicalize();
        assert_eq!(payload.candidate_count, 2);
        assert_eq!(payload.ordered_candidates[0].persistent_ref, "face:a");
    }

    #[test]
    fn resolution_trace_payload_validator_rejects_noncanonical_candidates() {
        let payload = ResolutionTracePayload {
            decision_id: DecisionId(7),
            query: ResolutionQuerySummary::PersistentName {
                entity_kind: EntityKind::Face,
                ancestry_hash_hex: "deadbeef".into(),
                ordinal: 0,
            },
            outcome: ResolutionOutcome::Missing,
            final_route: ResolutionRoute::None,
            routes_attempted: vec![ResolutionRoute::DirectPersistentName],
            candidate_count: 2,
            ordered_candidates: vec![
                sample_candidate("face:b", 12, 1),
                sample_candidate("face:a", 9, 1),
            ],
            operation_scope_id: None,
            source_scope_id: None,
            candidate_set_hash: None,
        };
        let decision = TracedDecision::new(
            DecisionId(7),
            DecisionKind::Forced { reason: "ResolutionMissing".into() },
            DecisionTier::Escalated,
            0.0,
            DecisionContext::Degeneracy { description: "persistent_name".into() },
        );
        assert_eq!(
            payload.validate_against_decision(&decision),
            Err(ResolutionTraceConsistencyError::CandidatesNotCanonical)
        );
    }
}
