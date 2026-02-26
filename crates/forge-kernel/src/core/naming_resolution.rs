use serde::{Deserialize, Serialize};

use forge_core::errors::PersistentResolutionOriginKind;
use forge_core::provenance::SnapshotHandleRef;
use forge_core::tracing::{
    DecisionContext, DecisionId, DecisionKind, DecisionTier, EntityKind,
    ResolutionCandidateSummary, ResolutionMatchKind, ResolutionOutcome, ResolutionQuerySummary,
    ResolutionRoute, ResolutionTracePayload, TracedDecision,
};
use forge_topo::topology::attributes::EntityKey;
use forge_topo::topology::naming::{PersistentName, Selector};

/// Typed query input for persistent-name/selector resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionQuery {
    PersistentName(PersistentName),
    Selector(Selector),
}

impl ResolutionQuery {
    pub fn to_trace_summary(&self) -> ResolutionQuerySummary {
        match self {
            ResolutionQuery::PersistentName(name) => ResolutionQuerySummary::PersistentName {
                entity_kind: name.get_kind(),
                ancestry_hash_hex: format!("{:032x}", name.get_ancestry_hash()),
                ordinal: name.get_ordinal(),
            },
            ResolutionQuery::Selector(sel) => ResolutionQuerySummary::Selector {
                entity_kind: selector_entity_kind(sel),
                selector_kind: selector_kind_tag(sel).to_string(),
                selector_fingerprint: Some(hash_selector(sel)),
            },
        }
    }
}

/// Compute a deterministic fingerprint of a Selector strictly from its topological criteria.
///
/// INV-1 Guard: This completely ignores any future explicit or implicit float-derived
/// or geometric properties unless explicitly allowed. The hash covers ONLY structural
/// and feature/operation identities.
fn hash_selector(sel: &Selector) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    let mut mix = |val: u64| {
        h ^= val;
        h = h.wrapping_mul(0x100000001b3);
    };

    match sel {
        Selector::ByAncestry { hash, kind } => {
            mix(1);
            mix((*hash & 0xFFFFFFFFFFFFFFFF) as u64);
            mix((*hash >> 64) as u64);
            mix(*kind as u64);
        }
        Selector::ByFeature { feature_id, kind } => {
            mix(2);
            mix(*feature_id);
            mix(*kind as u64);
        }
        Selector::ByOperation { op_name, kind } => {
            mix(3);
            for b in op_name.as_bytes() {
                mix(*b as u64);
            }
            mix(*kind as u64);
        }
        Selector::And(a, b) => {
            mix(4);
            mix(hash_selector(a));
            mix(hash_selector(b));
        }
        Selector::Or(a, b) => {
            mix(5);
            mix(hash_selector(a));
            mix(hash_selector(b));
        }
    }
    h
}

/// Resolver path used for a result/candidate.
pub type ResolverRoute = ResolutionRoute;
/// Typed match-kind reused in trace payloads.
pub type ResolverMatchKind = ResolutionMatchKind;

/// Typed incompatibility result for resolver contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionIncompatibility {
    UnsupportedEntityKind {
        requested: EntityKind,
    },
    MissingLineageStore,
    SubstrateUnavailable,
    UnsupportedEntityOrigin {
        origin: PersistentResolutionOriginKind,
    },
    SchemaVersionMismatch {
        expected: u32,
        actual: u32,
    },
    LineageStoreVersionMismatch {
        expected: u32,
        actual: u32,
    },
    LegacyIndexOnlyLineageHistory,
    UnsupportedResolverMode {
        mode: String,
    },
    Other {
        code: String,
        detail: String,
    },
}

/// Machine-readable evidence describing resolver passes and filters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ResolutionEvidence {
    pub routes_attempted: Vec<ResolverRoute>,
    pub initial_candidate_count: u32,
    pub surviving_candidate_count: u32,
    pub filters_applied: Vec<String>,
    pub notes: Vec<String>,
}

impl ResolutionEvidence {
    pub fn with_counts(initial: usize, surviving: usize) -> Self {
        Self {
            initial_candidate_count: initial as u32,
            surviving_candidate_count: surviving as u32,
            ..Self::default()
        }
    }
}

/// Typed candidate payload for deterministic, traceable resolution outcomes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionCandidate {
    pub entity_kind: EntityKind,
    pub persistent_ref: String,
    pub snapshot_ref: SnapshotHandleRef,
    pub route: ResolverRoute,
    pub match_kind: ResolverMatchKind,
    /// Optional provenance/evidence summary (typed string code + detail) for audit use.
    pub provenance_tag: Option<String>,
    pub provenance_detail: Option<String>,
}

impl ResolutionCandidate {
    pub fn sort_key(&self) -> (u8, &str, u32, u32, u8, String) {
        (
            self.entity_kind as u8,
            self.persistent_ref.as_str(),
            self.snapshot_ref.index,
            self.snapshot_ref.generation,
            self.route as u8,
            match &self.match_kind {
                ResolverMatchKind::ExactPersistentName => "exact".to_string(),
                ResolverMatchKind::SelectorMatch => "selector".to_string(),
                ResolverMatchKind::LineageDescendant => "lineage_desc".to_string(),
                ResolverMatchKind::LineageAncestor => "lineage_anc".to_string(),
                ResolverMatchKind::AliasMatch { alias_kind } => format!("alias:{alias_kind}"),
                ResolverMatchKind::Other { tag } => format!("other:{tag}"),
            },
        )
    }

    pub fn to_trace_summary(&self) -> ResolutionCandidateSummary {
        ResolutionCandidateSummary {
            entity_kind: self.entity_kind,
            persistent_ref: self.persistent_ref.clone(),
            snapshot_ref: self.snapshot_ref,
            route: self.route,
            match_kind: self.match_kind.clone(),
        }
    }
}

/// Deterministic candidate container used by resolver implementations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ResolutionCandidates {
    ordered: Vec<ResolutionCandidate>,
}

impl ResolutionCandidates {
    pub fn new() -> Self {
        Self {
            ordered: Vec::new(),
        }
    }

    pub fn from_vec(mut candidates: Vec<ResolutionCandidate>) -> Self {
        candidates.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
        Self {
            ordered: candidates,
        }
    }

    pub fn push(&mut self, candidate: ResolutionCandidate) {
        self.ordered.push(candidate);
        self.ordered.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
    }

    pub fn as_slice(&self) -> &[ResolutionCandidate] {
        &self.ordered
    }
    pub fn len(&self) -> usize {
        self.ordered.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ordered.is_empty()
    }
    pub fn into_vec(self) -> Vec<ResolutionCandidate> {
        self.ordered
    }

    pub fn candidate_set_hash(&self) -> u64 {
        let mut h: u64 = 0xcbf29ce484222325;
        for c in &self.ordered {
            for word in [
                c.entity_kind as u64,
                c.snapshot_ref.kind as u64,
                c.snapshot_ref.index as u64,
                c.snapshot_ref.generation as u64,
                c.route as u64,
            ] {
                h ^= word;
                h = h.wrapping_mul(0x100000001b3);
            }
            for b in c.persistent_ref.as_bytes() {
                h ^= *b as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
        }
        h
    }
}

/// Reusable typed result family for persistent-name/selector resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionResult<T> {
    Resolved {
        value: T,
        route: ResolverRoute,
        evidence: ResolutionEvidence,
    },
    Ambiguous {
        query: ResolutionQuery,
        candidates: ResolutionCandidates,
        evidence: ResolutionEvidence,
    },
    Missing {
        query: ResolutionQuery,
        evidence: ResolutionEvidence,
    },
    Incompatible {
        query: ResolutionQuery,
        incompatibility: ResolutionIncompatibility,
    },
}

impl<T> ResolutionResult<T> {
    pub fn outcome(&self) -> ResolutionOutcome {
        match self {
            ResolutionResult::Resolved { .. } => ResolutionOutcome::Resolved,
            ResolutionResult::Ambiguous { .. } => ResolutionOutcome::Ambiguous,
            ResolutionResult::Missing { .. } => ResolutionOutcome::Missing,
            ResolutionResult::Incompatible { .. } => ResolutionOutcome::Incompatible,
        }
    }
}

impl ResolutionResult<ResolutionCandidate> {
    /// Build the typed trace adjunct payload for this resolution result.
    pub fn to_trace_payload(
        &self,
        decision_id: forge_core::DecisionId,
        operation_scope_id: Option<String>,
        source_scope_id: Option<String>,
    ) -> ResolutionTracePayload {
        match self {
            ResolutionResult::Resolved {
                value,
                route,
                evidence,
            } => {
                let mut payload = ResolutionTracePayload {
                    decision_id,
                    query: ResolutionQuerySummary::Selector {
                        entity_kind: Some(value.entity_kind),
                        selector_kind: "resolved_candidate".into(),
                        selector_fingerprint: None,
                    },
                    outcome: ResolutionOutcome::Resolved,
                    final_route: *route,
                    routes_attempted: evidence.routes_attempted.clone(),
                    candidate_count: 1,
                    ordered_candidates: vec![value.to_trace_summary()],
                    operation_scope_id,
                    source_scope_id,
                    candidate_set_hash: None,
                };
                payload.canonicalize();
                payload
            }
            ResolutionResult::Ambiguous {
                query,
                candidates,
                evidence,
            } => {
                let mut payload = ResolutionTracePayload {
                    decision_id,
                    query: query.to_trace_summary(),
                    outcome: ResolutionOutcome::Ambiguous,
                    final_route: ResolverRoute::None,
                    routes_attempted: evidence.routes_attempted.clone(),
                    candidate_count: candidates.len() as u32,
                    ordered_candidates: candidates
                        .as_slice()
                        .iter()
                        .map(ResolutionCandidate::to_trace_summary)
                        .collect(),
                    operation_scope_id,
                    source_scope_id,
                    candidate_set_hash: Some(candidates.candidate_set_hash()),
                };
                payload.canonicalize();
                payload
            }
            ResolutionResult::Missing { query, evidence } => ResolutionTracePayload {
                decision_id,
                query: query.to_trace_summary(),
                outcome: ResolutionOutcome::Missing,
                final_route: ResolverRoute::None,
                routes_attempted: evidence.routes_attempted.clone(),
                candidate_count: 0,
                ordered_candidates: Vec::new(),
                operation_scope_id,
                source_scope_id,
                candidate_set_hash: Some(0),
            },
            ResolutionResult::Incompatible { query, .. } => ResolutionTracePayload {
                decision_id,
                query: query.to_trace_summary(),
                outcome: ResolutionOutcome::Incompatible,
                final_route: ResolverRoute::None,
                routes_attempted: Vec::new(),
                candidate_count: 0,
                ordered_candidates: Vec::new(),
                operation_scope_id,
                source_scope_id,
                candidate_set_hash: None,
            },
        }
    }
}

pub fn build_resolution_decision(
    decision_id: DecisionId,
    result: &ResolutionResult<ResolutionCandidate>,
) -> TracedDecision {
    match result {
        ResolutionResult::Resolved { .. } => TracedDecision::new(
            decision_id,
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: "Persistent-name resolution succeeded".into(),
            },
        ),
        ResolutionResult::Ambiguous { candidates, .. } => TracedDecision::new(
            decision_id,
            DecisionKind::Forced {
                reason: "NameResolutionAmbiguous".into(),
            },
            DecisionTier::Escalated,
            candidates.len() as f64,
            DecisionContext::Degeneracy {
                description: format!(
                    "Persistent-name resolution ambiguous ({} candidates)",
                    candidates.len()
                ),
            },
        ),
        ResolutionResult::Missing { .. } => TracedDecision::new(
            decision_id,
            DecisionKind::Forced {
                reason: "NameResolutionMissing".into(),
            },
            DecisionTier::Escalated,
            0.0,
            DecisionContext::Degeneracy {
                description: "Persistent-name resolution missing".into(),
            },
        ),
        ResolutionResult::Incompatible {
            incompatibility, ..
        } => TracedDecision::new(
            decision_id,
            DecisionKind::Forced {
                reason: "NameResolutionIncompatible".into(),
            },
            DecisionTier::Escalated,
            0.0,
            DecisionContext::Degeneracy {
                description: format!(
                    "Persistent-name resolution incompatible: {:?}",
                    incompatibility
                ),
            },
        ),
    }
}

pub fn snapshot_ref_from_entity_key(key: EntityKey) -> SnapshotHandleRef {
    match key {
        EntityKey::Face(fid) => {
            SnapshotHandleRef::new(EntityKind::Face, fid.index(), fid.generation())
        }
        EntityKey::Edge(eid) => {
            SnapshotHandleRef::new(EntityKind::Edge, eid.index(), eid.generation())
        }
        EntityKey::Vertex(vid) => {
            SnapshotHandleRef::new(EntityKind::Vertex, vid.index(), vid.generation())
        }
        EntityKey::Shell(sid) => {
            SnapshotHandleRef::new(EntityKind::Shell, sid.index(), sid.generation())
        }
    }
}

fn selector_entity_kind(sel: &Selector) -> Option<EntityKind> {
    match sel {
        Selector::ByAncestry { kind, .. } => Some(*kind),
        Selector::ByFeature { kind, .. } => Some(*kind),
        Selector::ByOperation { kind, .. } => Some(*kind),
        Selector::And(a, b) => {
            let ka = selector_entity_kind(a);
            let kb = selector_entity_kind(b);
            if ka == kb {
                ka
            } else {
                None
            }
        }
        Selector::Or(a, b) => {
            let ka = selector_entity_kind(a);
            let kb = selector_entity_kind(b);
            if ka == kb {
                ka
            } else {
                None
            }
        }
    }
}

fn selector_kind_tag(sel: &Selector) -> &'static str {
    match sel {
        Selector::ByAncestry { .. } => "by_ancestry",
        Selector::ByFeature { .. } => "by_feature",
        Selector::ByOperation { .. } => "by_operation",
        Selector::And(_, _) => "and",
        Selector::Or(_, _) => "or",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_core::DecisionId;
    use forge_topo::FaceId;

    fn face_candidate(name: &str, idx: u32, gen: u32) -> ResolutionCandidate {
        ResolutionCandidate {
            entity_kind: EntityKind::Face,
            persistent_ref: name.into(),
            snapshot_ref: SnapshotHandleRef::new(EntityKind::Face, idx, gen),
            route: ResolverRoute::DirectPersistentName,
            match_kind: ResolverMatchKind::ExactPersistentName,
            provenance_tag: None,
            provenance_detail: None,
        }
    }

    #[test]
    fn resolution_candidates_are_sorted_deterministically() {
        let set = ResolutionCandidates::from_vec(vec![
            face_candidate("face:b", 7, 1),
            face_candidate("face:a", 11, 1),
            face_candidate("face:a", 3, 2),
        ]);
        let refs: Vec<_> = set
            .as_slice()
            .iter()
            .map(|c| {
                (
                    &c.persistent_ref,
                    c.snapshot_ref.index,
                    c.snapshot_ref.generation,
                )
            })
            .collect();
        assert_eq!(
            refs,
            vec![
                (&"face:a".to_string(), 3, 2),
                (&"face:a".to_string(), 11, 1),
                (&"face:b".to_string(), 7, 1)
            ]
        );
    }

    #[test]
    fn ambiguous_result_trace_payload_is_canonical_and_hashed() {
        let result = ResolutionResult::<ResolutionCandidate>::Ambiguous {
            query: ResolutionQuery::PersistentName(PersistentName::new(0x12, EntityKind::Face, 0)),
            candidates: ResolutionCandidates::from_vec(vec![
                face_candidate("face:z", 9, 1),
                face_candidate("face:a", 2, 1),
            ]),
            evidence: ResolutionEvidence {
                routes_attempted: vec![ResolverRoute::DirectPersistentName],
                initial_candidate_count: 2,
                surviving_candidate_count: 2,
                filters_applied: vec![],
                notes: vec![],
            },
        };
        let payload =
            result.to_trace_payload(DecisionId(5), Some("sheet_region_merge".into()), None);
        assert_eq!(payload.ordered_candidates[0].persistent_ref, "face:a");
        assert_eq!(payload.candidate_count, 2);
        assert!(payload.candidate_set_hash.is_some());
    }

    #[test]
    fn snapshot_ref_conversion_preserves_generation() {
        let face = FaceId::from_raw_parts(42, 9);
        let snap = snapshot_ref_from_entity_key(EntityKey::Face(face));
        assert_eq!(snap.kind, EntityKind::Face);
        assert_eq!(snap.index, 42);
        assert_eq!(snap.generation, 9);
    }
}
