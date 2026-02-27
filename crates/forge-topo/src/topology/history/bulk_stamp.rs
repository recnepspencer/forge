//! Bulk lineage generation for complete topology arenas.
//!
//! DOMAIN: System-wide provenance (Phase 3 causality).

use crate::arena::TopologyArena;
use crate::topology::history::lineage::{Lineage, LineageEvent, OpSignature};
use forge_core::{EntityKind, EntityRef};

/// Record lineage events for all entities in the result topology.
pub fn record_result_lineage(arena: &TopologyArena, seq: u64) -> Vec<LineageEvent> {
    let op = OpSignature::with_id("assemble_result", seq);
    let mut events: Vec<LineageEvent> = Vec::new();

    for (fid, _) in arena.iter_faces() {
        events.push(LineageEvent::EntityCreated {
            entity: EntityRef::new(EntityKind::Face, fid.index()),
            entity_snapshot: None,
            lineage: Lineage::root(fid.index() as u64, op.clone()),
        });
    }

    for (he_id, _) in arena.iter_half_edges() {
        events.push(LineageEvent::EntityCreated {
            entity: EntityRef::new(EntityKind::HalfEdge, he_id.index()),
            entity_snapshot: None,
            lineage: Lineage::root(he_id.index() as u64, op.clone()),
        });
    }

    for (vid, _) in arena.iter_vertices() {
        events.push(LineageEvent::EntityCreated {
            entity: EntityRef::new(EntityKind::Vertex, vid.index()),
            entity_snapshot: None,
            lineage: Lineage::root(vid.index() as u64, op.clone()),
        });
    }

    events
}
