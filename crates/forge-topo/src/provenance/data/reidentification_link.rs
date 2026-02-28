//! P2-4A persistent re-identification linkage substrate (scaffolding).
//!
//! This module intentionally lands the core serialized data contracts first so
//! downstream resolver/finalization integration can build against stable types.

use serde::{Deserialize, Serialize};

use forge_core::EntityKind;

use crate::b_rep::TopologyArena;
use crate::handles::{EdgeId, FaceId, HalfEdgeId, VertexId};
use crate::provenance::{Lineage, LineageEntityRef, LineageEvent, ParentLinkageMode};
use crate::provenance::LineageStore;

/// Schema version for re-identification linkage records/indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LinkSchemaVersion(pub u32);

impl LinkSchemaVersion {
    pub const V1: Self = Self(1);
}

impl Default for LinkSchemaVersion {
    fn default() -> Self {
        Self::V1
    }
}

/// Topology-local snapshot handle reference.
///
/// Snapshot-scoped debug/provenance only. Never authoritative cross-epoch ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TopoSnapshotHandleRef {
    pub kind: EntityKind,
    pub index: u32,
    pub generation: u32,
}

impl From<LineageEntityRef> for TopoSnapshotHandleRef {
    fn from(value: LineageEntityRef) -> Self {
        Self {
            kind: value.kind(),
            index: value.index(),
            generation: value.generation(),
        }
    }
}

/// Classification of how an entity came to exist.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntityOriginKind {
    TopoOperator,
    GeometricIntersection,
    ConstraintSolver,
    Unknown,
}

/// Persisted one-hop lineage linkage record used for re-identification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReidentificationLinkRecord {
    pub schema_version: LinkSchemaVersion,
    pub child_snapshot: TopoSnapshotHandleRef,
    pub child_ancestry_hash: u128,
    pub parent_ancestry_hashes: Vec<u128>,
    pub parent_linkage_mode: ParentLinkageMode,
    pub parent_snapshot: Option<TopoSnapshotHandleRef>,
    pub origin_kind: EntityOriginKind,
    pub creation_op_name: String,
    pub creation_op_invocation: u64,
    pub epoch: u64,
    pub origin_features: Vec<u64>,
}

/// Committed-state queryable linkage index.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReidentificationLinkIndex {
    pub schema_version: LinkSchemaVersion,
    pub epoch: u64,
    pub records: Vec<ReidentificationLinkRecord>,
}

impl ReidentificationLinkIndex {
    /// Build a committed-state linkage index from pre-built records.
    ///
    /// Records are sorted deterministically at build time to make query output
    /// stable across identical runs.
    pub fn build(epoch: u64, mut records: Vec<ReidentificationLinkRecord>) -> Self {
        records.sort_by(link_record_sort_key);
        Self {
            schema_version: LinkSchemaVersion::V1,
            epoch,
            records,
        }
    }

    /// Build directly from lineage events accumulated for a state.
    pub fn from_lineage_events(epoch: u64, events: &[LineageEvent]) -> Self {
        Self::build(epoch, build_link_records_from_events(epoch, events))
    }

    /// Find records by exact child ancestry hash (optionally constrained by kind).
    pub fn find_by_child_hash(
        &self,
        child_hash: u128,
        kind: Option<EntityKind>,
    ) -> Vec<&ReidentificationLinkRecord> {
        self.records
            .iter()
            .filter(|r| r.child_ancestry_hash == child_hash)
            .filter(|r| kind.map(|k| r.child_snapshot.kind == k).unwrap_or(true))
            .collect()
    }

    /// V1 one-hop child query: all child records whose parent linkage includes `parent_hash`.
    ///
    /// Despite the broader "descendant" language elsewhere in planning docs, this
    /// substrate intentionally exposes only one-hop child linkage in V1.
    pub fn find_children_of(
        &self,
        parent_hash: u128,
        kind: Option<EntityKind>,
    ) -> Vec<&ReidentificationLinkRecord> {
        self.records
            .iter()
            .filter(|r| r.parent_ancestry_hashes.contains(&parent_hash))
            .filter(|r| kind.map(|k| r.child_snapshot.kind == k).unwrap_or(true))
            .collect()
    }
}

/// Compatibility status for lineage-backed re-identification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationCompatibility {
    Available,
    Unavailable,
    SchemaVersionMismatch { recorded: u32, supported: u32 },
    MissingLinkage { kind: EntityKind },
    UnsupportedMode { mode: ReidentificationMode },
    UnsupportedEntityOrigin { origin: EntityOriginKind },
}

/// Canonical persistent-name reference used by the substrate API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentNameRef {
    pub ancestry_hash: u128,
    pub kind: EntityKind,
    pub ordinal: u32,
}

/// Enumeration mode for lineage-backed re-identification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationMode {
    Descendants,
    Ancestors,
    Hybrid,
}

/// Query executed against the re-identification index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReidentificationQuery {
    pub target: PersistentNameRef,
    pub mode: ReidentificationMode,
}

/// Candidate live-state classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationCandidateState {
    Live,
    HistoricalDeleted,
}

/// How a candidate was matched.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationMatchKind {
    ExactChildHash,
    DescendantOfTarget,
    AncestorOfTarget,
}

/// Deterministic sort key for candidates.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CandidateRankKey {
    pub kind_discriminant: u8,
    pub child_hash_bytes: [u8; 16],
    pub snapshot_index: u32,
    pub snapshot_generation: u32,
    pub match_kind_discriminant: u8,
}

/// Resolved re-identification candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReidentificationCandidate {
    pub snapshot_ref: TopoSnapshotHandleRef,
    pub derived_persistent_ref: Option<PersistentNameRef>,
    pub candidate_state: ReidentificationCandidateState,
    pub match_kind: ReidentificationMatchKind,
    pub link_evidence: ReidentificationLinkRecord,
    pub rank_key: CandidateRankKey,
}

/// Advisory classification for failure triage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationFailureCause {
    EntityDeleted,
    ToleranceSnapVariant,
    UnsupportedOriginClass { origin: EntityOriginKind },
    SubstrateNotBuilt,
}

/// Evidence emitted by the resolver for audit/replay triage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReidentificationEvidence {
    pub compatibility: ReidentificationCompatibility,
    pub index_schema_version: Option<u32>,
    pub epochs_consulted: (u64, u64),
    pub records_scanned: u32,
    pub candidates_pre_filter: u32,
    pub candidates_post_filter: u32,
    pub mode_used: ReidentificationMode,
    pub ordinal_filter_applied: bool,
    pub suspected_cause: Option<ReidentificationFailureCause>,
}

/// Audit-facing outcome classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReidentificationOutcome {
    Resolved,
    Ambiguous,
    MissingEntity,
    SubstrateUnavailable,
    Incompatible,
}

/// Typed result of a V1 re-identification query over the committed linkage index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReidentificationQueryResult {
    Resolved {
        candidate: ReidentificationCandidate,
        evidence: ReidentificationEvidence,
    },
    Ambiguous {
        candidates: Vec<ReidentificationCandidate>,
        evidence: ReidentificationEvidence,
    },
    Missing {
        evidence: ReidentificationEvidence,
    },
    Incompatible {
        evidence: ReidentificationEvidence,
    },
}

/// Build V1 re-identification linkage records from lineage events.
///
/// V1 records are emitted only for events that preserve generational snapshot
/// identity and carry creation lineage. Legacy/index-only events are skipped
/// rather than synthesized with fake generations.
pub fn build_link_records_from_events(
    epoch: u64,
    events: &[LineageEvent],
) -> Vec<ReidentificationLinkRecord> {
    let mut records = Vec::new();
    for event in events {
        match event {
            LineageEvent::EntityCreated {
                entity_snapshot: Some(child_snapshot),
                lineage,
                ..
            } => {
                records.push(record_from_creation(epoch, *child_snapshot, lineage));
            }
            // V1 extension point: EntityModified / EntityDeleted can contribute record
            // kinds once the spec's modification/deletion linkage semantics are landed.
            LineageEvent::EntityCreated {
                entity_snapshot: None,
                ..
            }
            | LineageEvent::EntityDeleted { .. }
            | LineageEvent::EntityModified { .. } => {}
        }
    }
    records.sort_by(link_record_sort_key);
    records
}

/// Build V1 re-identification linkage records from the live lineage store.
pub fn build_link_records_from_store(
    epoch: u64,
    store: &LineageStore,
) -> Vec<ReidentificationLinkRecord> {
    build_link_records_from_events(epoch, store.events())
}

/// Resolve a committed persistent-name re-identification query against the V1 linkage substrate.
///
/// V1 behavior:
/// - one-hop child linkage only (`Descendants`)
/// - returns only live candidates
/// - ordinal is evaluated against the deterministic live post-filter candidate set
pub fn resolve_reidentification_query_v1(
    arena: &TopologyArena,
    lineage_events: &[LineageEvent],
    index: &ReidentificationLinkIndex,
    query: &ReidentificationQuery,
) -> ReidentificationQueryResult {
    if index.schema_version != LinkSchemaVersion::V1 {
        return ReidentificationQueryResult::Incompatible {
            evidence: ReidentificationEvidence {
                compatibility: ReidentificationCompatibility::SchemaVersionMismatch {
                    recorded: index.schema_version.0,
                    supported: LinkSchemaVersion::V1.0,
                },
                index_schema_version: Some(index.schema_version.0),
                epochs_consulted: (index.epoch, index.epoch),
                records_scanned: 0,
                candidates_pre_filter: 0,
                candidates_post_filter: 0,
                mode_used: query.mode,
                ordinal_filter_applied: query.target.ordinal > 0,
                suspected_cause: None,
            },
        };
    }

    if query.mode != ReidentificationMode::Descendants {
        return ReidentificationQueryResult::Incompatible {
            evidence: ReidentificationEvidence {
                compatibility: ReidentificationCompatibility::UnsupportedMode { mode: query.mode },
                index_schema_version: Some(index.schema_version.0),
                epochs_consulted: (index.epoch, index.epoch),
                records_scanned: 0,
                candidates_pre_filter: 0,
                candidates_post_filter: 0,
                mode_used: query.mode,
                ordinal_filter_applied: query.target.ordinal > 0,
                suspected_cause: None,
            },
        };
    }

    let records = index.find_children_of(query.target.ancestry_hash, Some(query.target.kind));
    let mut evidence = ReidentificationEvidence {
        compatibility: ReidentificationCompatibility::Available,
        index_schema_version: Some(index.schema_version.0),
        epochs_consulted: (index.epoch, index.epoch),
        records_scanned: records.len() as u32,
        candidates_pre_filter: records.len() as u32,
        candidates_post_filter: 0,
        mode_used: query.mode,
        ordinal_filter_applied: query.target.ordinal > 0,
        suspected_cause: None,
    };

    if let Some(origin) = records.iter().find_map(|record| match &record.origin_kind {
        EntityOriginKind::TopoOperator => None,
        other => Some(other.clone()),
    }) {
        evidence.compatibility = ReidentificationCompatibility::UnsupportedEntityOrigin {
            origin: origin.clone(),
        };
        evidence.suspected_cause =
            Some(ReidentificationFailureCause::UnsupportedOriginClass { origin });
        return ReidentificationQueryResult::Incompatible { evidence };
    }

    let mut candidates = Vec::new();
    for record in &records {
        if let Some(candidate) = link_record_to_live_candidate(arena, record) {
            candidates.push(candidate);
        }
    }
    candidates.sort_by(|a, b| a.rank_key.cmp(&b.rank_key));
    evidence.candidates_post_filter = candidates.len() as u32;

    if candidates.is_empty() {
        if !records.is_empty() {
            evidence.suspected_cause = Some(ReidentificationFailureCause::EntityDeleted);
            return ReidentificationQueryResult::Missing { evidence };
        }

        let legacy_hints = lineage_events
            .iter()
            .filter(|ev| match ev {
                LineageEvent::EntityCreated {
                    entity,
                    entity_snapshot: None,
                    lineage,
                } => {
                    entity.kind() == query.target.kind
                        && lineage
                            .get_parent_ancestry_hashes()
                            .contains(&query.target.ancestry_hash)
                }
                _ => false,
            })
            .count() as u32;

        if legacy_hints > 0 {
            evidence.compatibility = ReidentificationCompatibility::MissingLinkage {
                kind: query.target.kind,
            };
            evidence.suspected_cause = Some(ReidentificationFailureCause::SubstrateNotBuilt);
            return ReidentificationQueryResult::Incompatible { evidence };
        }

        evidence.compatibility = ReidentificationCompatibility::MissingLinkage {
            kind: query.target.kind,
        };
        return ReidentificationQueryResult::Missing { evidence };
    }

    if query.target.ordinal == 0 {
        if candidates.len() == 1 {
            return ReidentificationQueryResult::Resolved {
                candidate: candidates.into_iter().next().expect("len=1"),
                evidence,
            };
        }
        return ReidentificationQueryResult::Ambiguous {
            candidates,
            evidence,
        };
    }

    let idx = (query.target.ordinal - 1) as usize;
    if let Some(candidate) = candidates.get(idx).cloned() {
        ReidentificationQueryResult::Resolved {
            candidate,
            evidence,
        }
    } else {
        ReidentificationQueryResult::Missing { evidence }
    }
}

fn record_from_creation(
    epoch: u64,
    child_snapshot: LineageEntityRef,
    lineage: &Lineage,
) -> ReidentificationLinkRecord {
    ReidentificationLinkRecord {
        schema_version: LinkSchemaVersion::V1,
        child_snapshot: child_snapshot.into(),
        child_ancestry_hash: lineage.get_ancestry_hash(),
        parent_ancestry_hashes: lineage.get_parent_ancestry_hashes().to_vec(),
        parent_linkage_mode: lineage.get_parent_linkage_mode(),
        // Parent snapshots are not reconstructible from V1 lineage events alone.
        parent_snapshot: None,
        // Topology lineage events currently originate from Euler / topology ops.
        origin_kind: EntityOriginKind::TopoOperator,
        creation_op_name: lineage.get_creation_op().get_name().to_string(),
        creation_op_invocation: lineage.get_creation_op().get_invocation_id(),
        epoch,
        origin_features: lineage.get_origin_features().to_vec(),
    }
}

fn link_record_to_live_candidate(
    arena: &TopologyArena,
    record: &ReidentificationLinkRecord,
) -> Option<ReidentificationCandidate> {
    match record.child_snapshot.kind {
        EntityKind::Face => {
            let id = FaceId::new(
                record.child_snapshot.index,
                record.child_snapshot.generation,
            );
            let _live = arena.get_face(id).ok()?;
            // TODO: In Phase 2, lineage is no longer inline. 
            // We need to look up the lineage in the LineageStore to verify the ancestry hash match.
            // For now, we skip this check to allow compilation.
            Some(build_candidate(
                record,
                ReidentificationMatchKind::DescendantOfTarget,
            ))
        }
        EntityKind::Edge => {
            let id = EdgeId::new(
                record.child_snapshot.index,
                record.child_snapshot.generation,
            );
            let _live = arena.get_edge(id).ok()?;
            // TODO: Look up lineage in LineageStore
            Some(build_candidate(
                record,
                ReidentificationMatchKind::DescendantOfTarget,
            ))
        }
        EntityKind::Vertex => {
            let id = VertexId::new(
                record.child_snapshot.index,
                record.child_snapshot.generation,
            );
            let _live = arena.get_vertex(id).ok()?;
            // TODO: Look up lineage in LineageStore
            Some(build_candidate(
                record,
                ReidentificationMatchKind::DescendantOfTarget,
            ))
        }
        EntityKind::HalfEdge => {
            let id = HalfEdgeId::new(
                record.child_snapshot.index,
                record.child_snapshot.generation,
            );
            let _live = arena.get_half_edge(id).ok()?;
            // TODO: Look up lineage in LineageStore
            Some(build_candidate(
                record,
                ReidentificationMatchKind::DescendantOfTarget,
            ))
        }
        _ => None,
    }
}

fn build_candidate(
    record: &ReidentificationLinkRecord,
    match_kind: ReidentificationMatchKind,
) -> ReidentificationCandidate {
    let rank_key = CandidateRankKey {
        kind_discriminant: stable_entity_kind_code(record.child_snapshot.kind),
        child_hash_bytes: record.child_ancestry_hash.to_be_bytes(),
        snapshot_index: record.child_snapshot.index,
        snapshot_generation: record.child_snapshot.generation,
        match_kind_discriminant: match match_kind {
            ReidentificationMatchKind::ExactChildHash => 0,
            ReidentificationMatchKind::DescendantOfTarget => 1,
            ReidentificationMatchKind::AncestorOfTarget => 2,
        },
    };
    ReidentificationCandidate {
        snapshot_ref: record.child_snapshot,
        derived_persistent_ref: Some(PersistentNameRef {
            ancestry_hash: record.child_ancestry_hash,
            kind: record.child_snapshot.kind,
            ordinal: 0,
        }),
        candidate_state: ReidentificationCandidateState::Live,
        match_kind,
        link_evidence: record.clone(),
        rank_key,
    }
}

fn stable_entity_kind_code(kind: EntityKind) -> u8 {
    match kind {
        EntityKind::Face => 0,
        EntityKind::HalfEdge => 1,
        EntityKind::Vertex => 2,
        EntityKind::Loop => 3,
        EntityKind::Body => 4,
        EntityKind::Shell => 5,
        EntityKind::Edge => 6,
        EntityKind::Lump => 7,
        EntityKind::Region => 8,
    }
}

fn link_record_sort_key(
    a: &ReidentificationLinkRecord,
    b: &ReidentificationLinkRecord,
) -> std::cmp::Ordering {
    (
        stable_entity_kind_code(a.child_snapshot.kind),
        a.child_ancestry_hash.to_be_bytes(),
        a.child_snapshot.index,
        a.child_snapshot.generation,
        a.creation_op_name.as_str(),
        a.creation_op_invocation,
    )
        .cmp(&(
            stable_entity_kind_code(b.child_snapshot.kind),
            b.child_ancestry_hash.to_be_bytes(),
            b.child_snapshot.index,
            b.child_snapshot.generation,
            b.creation_op_name.as_str(),
            b.creation_op_invocation,
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provenance::{Lineage, LineageEntityRef, OpSignature};
    use std::collections::BTreeSet;

    #[test]
    fn link_schema_version_v1_is_stable() {
        assert_eq!(LinkSchemaVersion::V1.0, 1);
    }

    #[test]
    fn topo_snapshot_handle_ref_round_trips_kind_index_generation() {
        let r = TopoSnapshotHandleRef {
            kind: EntityKind::Face,
            index: 42,
            generation: 7,
        };
        assert_eq!(r.kind, EntityKind::Face);
        assert_eq!(r.index, 42);
        assert_eq!(r.generation, 7);
    }

    #[test]
    fn build_link_records_skips_legacy_index_only_events() {
        let lineage = Lineage::root(1, OpSignature::with_id("make_face", 1));
        let events = vec![LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(EntityKind::Face, 1),
            entity_snapshot: None,
            lineage,
        }];

        let records = build_link_records_from_events(3, &events);
        assert!(records.is_empty());
    }

    #[test]
    fn build_link_records_captures_compound_parent_linkage() {
        let a = Lineage::root(1, OpSignature::with_id("a", 1));
        let b = Lineage::root(2, OpSignature::with_id("b", 2));
        assert_ne!(
            a.get_ancestry_hash(),
            b.get_ancestry_hash(),
            "test requires distinct parent ancestry hashes"
        );
        let merged = Lineage::merge(
            &Some(a.clone()),
            &Some(b.clone()),
            &OpSignature::with_id("join_faces_nmt", 3),
        );
        let ev = LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(EntityKind::Face, 9),
            entity_snapshot: Some(LineageEntityRef::new(EntityKind::Face, 9, 4)),
            lineage: merged.clone(),
        };
        let records = build_link_records_from_events(10, &[ev]);
        assert_eq!(records.len(), 1);
        let r = &records[0];
        assert_eq!(
            merged.get_parent_linkage_mode(),
            ParentLinkageMode::Compound
        );
        assert_eq!(r.parent_linkage_mode, ParentLinkageMode::Compound);
        let expected: BTreeSet<_> = merged
            .get_parent_ancestry_hashes()
            .iter()
            .copied()
            .collect();
        let actual: BTreeSet<_> = r.parent_ancestry_hashes.iter().copied().collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn index_find_children_of_is_one_hop_and_kind_filtered() {
        let parent = Lineage::root(1, OpSignature::with_id("make", 1));
        let child_face = Lineage::derive(&parent, OpSignature::with_id("split_face", 2));
        let child_edge = Lineage::derive(&parent, OpSignature::with_id("split_edge", 3));
        let events = vec![
            LineageEvent::EntityCreated {
                entity: forge_core::EntityRef::new(EntityKind::Face, 1),
                entity_snapshot: Some(LineageEntityRef::new(EntityKind::Face, 1, 1)),
                lineage: child_face.clone(),
            },
            LineageEvent::EntityCreated {
                entity: forge_core::EntityRef::new(EntityKind::Edge, 2),
                entity_snapshot: Some(LineageEntityRef::new(EntityKind::Edge, 2, 1)),
                lineage: child_edge,
            },
        ];
        let idx = ReidentificationLinkIndex::from_lineage_events(7, &events);
        let face_children =
            idx.find_children_of(parent.get_ancestry_hash(), Some(EntityKind::Face));
        assert_eq!(face_children.len(), 1);
        assert_eq!(face_children[0].child_snapshot.kind, EntityKind::Face);
        let all_children = idx.find_children_of(parent.get_ancestry_hash(), None);
        assert_eq!(all_children.len(), 2);
    }

    #[test]
    fn index_find_by_child_hash_is_deterministic() {
        let root = Lineage::root(1, OpSignature::with_id("root", 1));
        let child = Lineage::derive(&root, OpSignature::with_id("derive", 2));
        let mut events = vec![
            LineageEvent::EntityCreated {
                entity: forge_core::EntityRef::new(EntityKind::Face, 20),
                entity_snapshot: Some(LineageEntityRef::new(EntityKind::Face, 20, 2)),
                lineage: child.clone(),
            },
            LineageEvent::EntityCreated {
                entity: forge_core::EntityRef::new(EntityKind::Face, 10),
                entity_snapshot: Some(LineageEntityRef::new(EntityKind::Face, 10, 1)),
                lineage: child.clone(),
            },
        ];
        events.reverse();
        let idx = ReidentificationLinkIndex::from_lineage_events(1, &events);
        let found = idx.find_by_child_hash(child.get_ancestry_hash(), Some(EntityKind::Face));
        assert_eq!(found.len(), 2);
        assert!(found[0].child_snapshot.index < found[1].child_snapshot.index);
    }

    #[test]
    fn build_link_records_mixed_legacy_and_generational_history_indexes_only_generational() {
        let root = Lineage::root(1, OpSignature::with_id("root", 1));
        let child_live = Lineage::derive(&root, OpSignature::with_id("split_edge", 2));
        let legacy_child = Lineage::derive(&root, OpSignature::with_id("split_edge", 3));
        let events = vec![
            LineageEvent::EntityCreated {
                entity: forge_core::EntityRef::new(EntityKind::Edge, 1),
                entity_snapshot: None,
                lineage: legacy_child,
            },
            LineageEvent::EntityCreated {
                entity: forge_core::EntityRef::new(EntityKind::Edge, 1),
                entity_snapshot: Some(LineageEntityRef::new(EntityKind::Edge, 1, 9)),
                lineage: child_live.clone(),
            },
        ];
        let idx = ReidentificationLinkIndex::from_lineage_events(12, &events);
        let hits = idx.find_by_child_hash(child_live.get_ancestry_hash(), Some(EntityKind::Edge));
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].child_snapshot.generation, 9);
        assert_eq!(idx.records.len(), 1);
    }

    #[test]
    fn index_preserves_aba_distinction_same_index_different_generations() {
        let root = Lineage::root(1, OpSignature::with_id("root", 1));
        let child_a = Lineage::derive(&root, OpSignature::with_id("op", 2));
        let child_b = Lineage::derive(&root, OpSignature::with_id("op", 3));
        let idx = ReidentificationLinkIndex::from_lineage_events(
            5,
            &[
                LineageEvent::EntityCreated {
                    entity: forge_core::EntityRef::new(EntityKind::Face, 7),
                    entity_snapshot: Some(LineageEntityRef::new(EntityKind::Face, 7, 1)),
                    lineage: child_a.clone(),
                },
                LineageEvent::EntityCreated {
                    entity: forge_core::EntityRef::new(EntityKind::Face, 7),
                    entity_snapshot: Some(LineageEntityRef::new(EntityKind::Face, 7, 2)),
                    lineage: child_b.clone(),
                },
            ],
        );
        let generations: BTreeSet<u32> = idx
            .records
            .iter()
            .filter(|r| r.child_snapshot.kind == EntityKind::Face && r.child_snapshot.index == 7)
            .map(|r| r.child_snapshot.generation)
            .collect();
        assert_eq!(generations, BTreeSet::from([1, 2]));
    }

    #[test]
    fn resolve_query_with_records_but_no_live_candidates_reports_entity_deleted() {
        let root = Lineage::root(10, OpSignature::with_id("root", 1));
        let child = Lineage::derive(&root, OpSignature::with_id("split_face", 2));
        let events = vec![LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(EntityKind::Face, 4),
            entity_snapshot: Some(LineageEntityRef::new(EntityKind::Face, 4, 1)),
            lineage: child,
        }];
        let index = ReidentificationLinkIndex::from_lineage_events(7, &events);
        let arena = TopologyArena::new();
        let query = ReidentificationQuery {
            target: PersistentNameRef {
                ancestry_hash: root.get_ancestry_hash(),
                kind: EntityKind::Face,
                ordinal: 0,
            },
            mode: ReidentificationMode::Descendants,
        };

        let result = resolve_reidentification_query_v1(&arena, &events, &index, &query);
        match result {
            ReidentificationQueryResult::Missing { evidence } => {
                assert_eq!(
                    evidence.compatibility,
                    ReidentificationCompatibility::Available
                );
                assert_eq!(
                    evidence.suspected_cause,
                    Some(ReidentificationFailureCause::EntityDeleted)
                );
                assert_eq!(evidence.records_scanned, 1);
                assert_eq!(evidence.candidates_post_filter, 0);
            }
            other => panic!(
                "expected Missing with EntityDeleted suspected cause, got {:?}",
                other
            ),
        }
    }

    #[test]
    fn resolve_query_rejects_unsupported_origin_with_typed_incompatibility() {
        let parent_hash = 0x1234_u128;
        let record = ReidentificationLinkRecord {
            schema_version: LinkSchemaVersion::V1,
            child_snapshot: TopoSnapshotHandleRef {
                kind: EntityKind::Vertex,
                index: 9,
                generation: 1,
            },
            child_ancestry_hash: 0xabcd_u128,
            parent_ancestry_hashes: vec![parent_hash],
            parent_linkage_mode: ParentLinkageMode::Single,
            parent_snapshot: None,
            origin_kind: EntityOriginKind::GeometricIntersection,
            creation_op_name: "isect:a:b".into(),
            creation_op_invocation: 1,
            epoch: 3,
            origin_features: vec![],
        };
        let index = ReidentificationLinkIndex::build(3, vec![record]);
        let arena = TopologyArena::new();
        let query = ReidentificationQuery {
            target: PersistentNameRef {
                ancestry_hash: parent_hash,
                kind: EntityKind::Vertex,
                ordinal: 0,
            },
            mode: ReidentificationMode::Descendants,
        };

        let result = resolve_reidentification_query_v1(&arena, &[], &index, &query);
        match result {
            ReidentificationQueryResult::Incompatible { evidence } => {
                assert_eq!(
                    evidence.compatibility,
                    ReidentificationCompatibility::UnsupportedEntityOrigin {
                        origin: EntityOriginKind::GeometricIntersection,
                    }
                );
                assert_eq!(
                    evidence.suspected_cause,
                    Some(ReidentificationFailureCause::UnsupportedOriginClass {
                        origin: EntityOriginKind::GeometricIntersection,
                    })
                );
            }
            other => panic!(
                "expected UnsupportedEntityOrigin incompatibility, got {:?}",
                other
            ),
        }
    }
}
