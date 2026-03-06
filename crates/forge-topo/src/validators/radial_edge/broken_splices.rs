//! Broken radial splice detection validator.
//!
//! INVARIANT: An edge's radial ring must contain ALL halfedges that reference
//! that edge. No disjoint sub-rings sharing an EdgeId.

use crate::b_rep::TopologyArena;
use crate::queries::walk::walk_radial_iter;
use forge_core::KernelError;
use std::collections::{BTreeMap, BTreeSet};

use super::vf;

pub(crate) fn validate_no_broken_radial_splices(arena: &TopologyArena) -> Result<(), KernelError> {
    let mut edge_he_counts: BTreeMap<u32, usize> = BTreeMap::new();
    for (_he_id, he_data) in arena.iter_half_edges() {
        *edge_he_counts.entry(he_data.edge().index()).or_default() += 1;
    }

    let mut checked_edges = BTreeSet::new();
    for (edge_id, edge_data) in arena.iter_edges() {
        if !checked_edges.insert(edge_id.index()) {
            continue;
        }
        let rep = edge_data.half_edge();
        let mut ring_count = 0usize;
        for he_result in walk_radial_iter(arena, rep)? {
            let _ = he_result?;
            ring_count += 1;
        }

        let expected = edge_he_counts.get(&edge_id.index()).copied().unwrap_or(0);
        if ring_count != expected {
            return Err(vf(
                "no_broken_radial_splices",
                format!(
                "Edge {} has {} HEs referencing it but radial ring from rep HE {} only reaches {} \
                 (disjoint sub-rings detected)",
                edge_id.index(), expected, rep.index(), ring_count
            ),
            ));
        }
    }
    Ok(())
}
