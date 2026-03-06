//! Shared topology walk kernels.
//!
//! DOMAIN: Canonical pointer walks over loop, radial, and vertex-disk connectivity.

use std::collections::{BTreeSet, VecDeque};

use crate::b_rep::TopologyArena;
use crate::handles::{HalfEdgeId, VertexId};
use forge_core::{KernelError, TopologyError};

/// Iterator over a face/loop cycle by following `next`.
pub struct LoopWalkIter<'a> {
    arena: &'a TopologyArena,
    start: HalfEdgeId,
    current: Option<HalfEdgeId>,
    steps: usize,
    bound: usize,
}

impl<'a> LoopWalkIter<'a> {
    /// Create a loop walker from a halfedge seed.
    pub fn new(arena: &'a TopologyArena, start_he: HalfEdgeId) -> Result<Self, KernelError> {
        arena.get_half_edge(start_he)?;
        Ok(Self {
            arena,
            start: start_he,
            current: Some(start_he),
            steps: 0,
            bound: arena.half_edge_count().max(1),
        })
    }
}

impl<'a> Iterator for LoopWalkIter<'a> {
    type Item = Result<HalfEdgeId, KernelError>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        if self.steps > self.bound {
            self.current = None;
            return Some(Err(KernelError::TopologyViolation {
                err: TopologyError::LoopCorruption {
                    walk_kind: "walk_loop_iter".to_string(),
                    seed_index: self.start.index(),
                    last_visited_index: current.index(),
                    steps_taken: self.steps,
                    entity_bound: self.bound,
                },
                context: None,
            }));
        }

        let next = match self.arena.get_half_edge(current) {
            Ok(he) => he.next(),
            Err(err) => {
                self.current = None;
                return Some(Err(err));
            }
        };

        self.steps += 1;
        if next == self.start {
            self.current = None;
        } else {
            self.current = Some(next);
        }
        Some(Ok(current))
    }
}

/// Iterator over an edge radial ring by following `radial_next`.
pub struct RadialWalkIter<'a> {
    arena: &'a TopologyArena,
    start: HalfEdgeId,
    current: Option<HalfEdgeId>,
    steps: usize,
    bound: usize,
}

impl<'a> RadialWalkIter<'a> {
    /// Create a radial walker from a halfedge seed.
    pub fn new(arena: &'a TopologyArena, start_he: HalfEdgeId) -> Result<Self, KernelError> {
        arena.get_half_edge(start_he)?;
        Ok(Self {
            arena,
            start: start_he,
            current: Some(start_he),
            steps: 0,
            bound: arena.half_edge_count().max(1),
        })
    }
}

impl<'a> Iterator for RadialWalkIter<'a> {
    type Item = Result<HalfEdgeId, KernelError>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        if self.steps > self.bound {
            self.current = None;
            return Some(Err(KernelError::TopologyViolation {
                err: TopologyError::LoopCorruption {
                    walk_kind: "walk_radial_iter".to_string(),
                    seed_index: self.start.index(),
                    last_visited_index: current.index(),
                    steps_taken: self.steps,
                    entity_bound: self.bound,
                },
                context: None,
            }));
        }

        let next = match self.arena.get_half_edge(current) {
            Ok(he) => he.radial_next(),
            Err(err) => {
                self.current = None;
                return Some(Err(err));
            }
        };

        self.steps += 1;
        if next == self.start {
            self.current = None;
        } else {
            self.current = Some(next);
        }
        Some(Ok(current))
    }
}

/// Iterator over a single vertex-disk component seeded by an outgoing halfedge.
pub struct DiskWalkIter<'a> {
    arena: &'a TopologyArena,
    vertex: VertexId,
    outgoing_set: BTreeSet<HalfEdgeId>,
    queued: BTreeSet<HalfEdgeId>,
    queue: VecDeque<HalfEdgeId>,
}

impl<'a> DiskWalkIter<'a> {
    /// Build a disk walk iterator for the component containing `seed_he`.
    pub fn new(
        arena: &'a TopologyArena,
        vertex: VertexId,
        seed_he: HalfEdgeId,
    ) -> Result<Self, KernelError> {
        let seed_data = arena.get_half_edge(seed_he)?;
        if seed_data.origin() != vertex {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "seed halfedge {} does not originate at vertex {}",
                    seed_he.index(),
                    vertex.index()
                ),
                context: None,
            });
        }

        let mut outgoing_set = BTreeSet::new();
        for (he_id, he_data) in arena.iter_half_edges() {
            if he_data.origin() == vertex {
                outgoing_set.insert(he_id);
            }
        }

        let mut queued = BTreeSet::new();
        let mut queue = VecDeque::new();
        queued.insert(seed_he);
        queue.push_back(seed_he);

        Ok(Self {
            arena,
            vertex,
            outgoing_set,
            queued,
            queue,
        })
    }
}

impl<'a> Iterator for DiskWalkIter<'a> {
    type Item = Result<HalfEdgeId, KernelError>;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.queue.pop_front()?;

        let current_data = match self.arena.get_half_edge(current) {
            Ok(d) => d,
            Err(err) => return Some(Err(err)),
        };
        if current_data.origin() != self.vertex {
            return Some(Ok(current));
        }

        let enqueue_candidate = |candidate: HalfEdgeId,
                                 outgoing_set: &BTreeSet<HalfEdgeId>,
                                 queued: &mut BTreeSet<HalfEdgeId>,
                                 queue: &mut VecDeque<HalfEdgeId>| {
            if outgoing_set.contains(&candidate) && queued.insert(candidate) {
                queue.push_back(candidate);
            }
        };

        let radial_iter = match walk_radial_iter(self.arena, current) {
            Ok(iter) => iter,
            Err(err) => return Some(Err(err)),
        };
        for radial_res in radial_iter {
            match radial_res {
                Ok(r) => {
                    enqueue_candidate(r, &self.outgoing_set, &mut self.queued, &mut self.queue);
                    match self.arena.get_half_edge(r) {
                        Ok(r_data) => {
                            enqueue_candidate(
                                r_data.next(),
                                &self.outgoing_set,
                                &mut self.queued,
                                &mut self.queue,
                            );
                        }
                        Err(err) => return Some(Err(err)),
                    }
                }
                Err(err) => return Some(Err(err)),
            }
        }

        let incoming = current_data.prev();
        let radial_iter = match walk_radial_iter(self.arena, incoming) {
            Ok(iter) => iter,
            Err(err) => return Some(Err(err)),
        };
        for radial_res in radial_iter {
            match radial_res {
                Ok(r) => {
                    enqueue_candidate(r, &self.outgoing_set, &mut self.queued, &mut self.queue);
                    match self.arena.get_half_edge(r) {
                        Ok(r_data) => {
                            enqueue_candidate(
                                r_data.next(),
                                &self.outgoing_set,
                                &mut self.queued,
                                &mut self.queue,
                            );
                        }
                        Err(err) => return Some(Err(err)),
                    }
                }
                Err(err) => return Some(Err(err)),
            }
        }

        Some(Ok(current))
    }
}

/// Walk a loop starting from `start_he`, following `next` pointers.
pub fn walk_loop_iter(
    arena: &TopologyArena,
    start_he: HalfEdgeId,
) -> Result<LoopWalkIter<'_>, KernelError> {
    LoopWalkIter::new(arena, start_he)
}

/// Walk a radial ring starting from `start_he`, following `radial_next`.
pub fn walk_radial_iter(
    arena: &TopologyArena,
    start_he: HalfEdgeId,
) -> Result<RadialWalkIter<'_>, KernelError> {
    RadialWalkIter::new(arena, start_he)
}

/// Walk one vertex-disk component starting from `seed_he`.
pub fn walk_vertex_disk_iter(
    arena: &TopologyArena,
    vertex: VertexId,
    seed_he: HalfEdgeId,
) -> Result<DiskWalkIter<'_>, KernelError> {
    DiskWalkIter::new(arena, vertex, seed_he)
}

/// Collect full loop walk into a vector.
pub fn collect_loop(
    arena: &TopologyArena,
    start_he: HalfEdgeId,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    walk_loop_iter(arena, start_he)?.collect()
}

/// Collect full radial walk into a vector.
pub fn collect_radial(
    arena: &TopologyArena,
    start_he: HalfEdgeId,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    walk_radial_iter(arena, start_he)?.collect()
}

/// Collect one full vertex-disk component into a vector.
pub fn collect_vertex_disk(
    arena: &TopologyArena,
    vertex: VertexId,
    seed_he: HalfEdgeId,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    walk_vertex_disk_iter(arena, vertex, seed_he)?.collect()
}
