use crate::planar_contracts::polygon_winding_2d::{
    CertifiedLoopWindingSummary, CertifiedPolygonWinding2DBasis, ProjectedLoopVertexSnapshot,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WindingSegmentContactCandidateIndex {
    rows: Vec<WindingSegmentContactCandidateRow>,
    counters: WindingSegmentContactCandidateIndexCounters,
}

impl WindingSegmentContactCandidateIndex {
    pub(crate) fn from_basis(basis: &CertifiedPolygonWinding2DBasis) -> Self {
        let mut builder = WindingSegmentContactCandidateIndexBuilder::default();
        for (loop_index, loop_summary) in basis.loop_summaries().iter().enumerate() {
            builder.push_self_contacts(loop_index, loop_summary);
        }
        let loops = basis.loop_summaries();
        for (candidate_index, candidate) in loops.iter().enumerate().skip(1) {
            builder.push_cross_loop_contacts(0, &loops[0], candidate_index, candidate);
        }
        builder.finish()
    }

    pub(crate) fn rows(&self) -> &[WindingSegmentContactCandidateRow] {
        &self.rows
    }

    pub(crate) fn counters(&self) -> WindingSegmentContactCandidateIndexCounters {
        self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WindingSegmentContactCandidateRow {
    first_loop_index: usize,
    first_loop_identity: String,
    first_edge: usize,
    second_loop_index: usize,
    second_loop_identity: String,
    second_edge: usize,
}

impl WindingSegmentContactCandidateRow {
    fn new(
        first_loop: &CertifiedLoopWindingSummary,
        first_loop_index: usize,
        first_edge: usize,
        second_loop: &CertifiedLoopWindingSummary,
        second_loop_index: usize,
        second_edge: usize,
    ) -> Self {
        Self {
            first_loop_index,
            first_loop_identity: first_loop.loop_identity().to_string(),
            first_edge,
            second_loop_index,
            second_loop_identity: second_loop.loop_identity().to_string(),
            second_edge,
        }
    }

    pub(crate) fn first_loop_index(&self) -> usize {
        self.first_loop_index
    }

    pub(crate) fn first_loop_identity(&self) -> &str {
        &self.first_loop_identity
    }

    pub(crate) fn first_edge(&self) -> usize {
        self.first_edge
    }

    pub(crate) fn second_loop_index(&self) -> usize {
        self.second_loop_index
    }

    pub(crate) fn second_loop_identity(&self) -> &str {
        &self.second_loop_identity
    }

    pub(crate) fn second_edge(&self) -> usize {
        self.second_edge
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindingSegmentContactCandidateIndexCounters {
    possible_pairs: usize,
    candidate_pairs: usize,
    culled_pairs: usize,
    adjacent_self_pairs_skipped: usize,
    fallback_used: bool,
}

impl WindingSegmentContactCandidateIndexCounters {
    pub(crate) const fn new(
        possible_pairs: usize,
        candidate_pairs: usize,
        culled_pairs: usize,
        adjacent_self_pairs_skipped: usize,
        fallback_used: bool,
    ) -> Self {
        Self {
            possible_pairs,
            candidate_pairs,
            culled_pairs,
            adjacent_self_pairs_skipped,
            fallback_used,
        }
    }

    pub fn possible_pairs(self) -> usize {
        self.possible_pairs
    }

    pub fn candidate_pairs(self) -> usize {
        self.candidate_pairs
    }

    pub fn culled_pairs(self) -> usize {
        self.culled_pairs
    }

    pub fn adjacent_self_pairs_skipped(self) -> usize {
        self.adjacent_self_pairs_skipped
    }

    pub fn fallback_used(self) -> bool {
        self.fallback_used
    }
}

#[derive(Default)]
struct WindingSegmentContactCandidateIndexBuilder {
    rows: Vec<WindingSegmentContactCandidateRow>,
    possible_pairs: usize,
    culled_pairs: usize,
    adjacent_self_pairs_skipped: usize,
}

impl WindingSegmentContactCandidateIndexBuilder {
    fn push_self_contacts(
        &mut self,
        loop_index: usize,
        loop_summary: &CertifiedLoopWindingSummary,
    ) {
        let edge_count = loop_summary.canonical_vertices().len();
        for first in 0..edge_count {
            for second in first + 1..edge_count {
                if edges_are_adjacent(first, second, edge_count) {
                    self.adjacent_self_pairs_skipped += 1;
                    continue;
                }
                self.push_if_envelopes_overlap(
                    LoopEdgeRef::new(loop_index, loop_summary, first),
                    LoopEdgeRef::new(loop_index, loop_summary, second),
                );
            }
        }
    }

    fn push_cross_loop_contacts(
        &mut self,
        primary_index: usize,
        primary: &CertifiedLoopWindingSummary,
        candidate_index: usize,
        candidate: &CertifiedLoopWindingSummary,
    ) {
        for first in 0..primary.canonical_vertices().len() {
            for second in 0..candidate.canonical_vertices().len() {
                self.push_if_envelopes_overlap(
                    LoopEdgeRef::new(primary_index, primary, first),
                    LoopEdgeRef::new(candidate_index, candidate, second),
                );
            }
        }
    }

    fn push_if_envelopes_overlap(&mut self, first: LoopEdgeRef<'_>, second: LoopEdgeRef<'_>) {
        self.possible_pairs += 1;
        if edge_envelope(first.loop_summary, first.edge)
            .overlaps(edge_envelope(second.loop_summary, second.edge))
        {
            self.rows.push(WindingSegmentContactCandidateRow::new(
                first.loop_summary,
                first.loop_index,
                first.edge,
                second.loop_summary,
                second.loop_index,
                second.edge,
            ));
        } else {
            self.culled_pairs += 1;
        }
    }

    fn finish(self) -> WindingSegmentContactCandidateIndex {
        let counters = WindingSegmentContactCandidateIndexCounters::new(
            self.possible_pairs,
            self.rows.len(),
            self.culled_pairs,
            self.adjacent_self_pairs_skipped,
            false,
        );
        WindingSegmentContactCandidateIndex {
            rows: self.rows,
            counters,
        }
    }
}

#[derive(Clone, Copy)]
struct LoopEdgeRef<'a> {
    loop_index: usize,
    loop_summary: &'a CertifiedLoopWindingSummary,
    edge: usize,
}

impl<'a> LoopEdgeRef<'a> {
    fn new(loop_index: usize, loop_summary: &'a CertifiedLoopWindingSummary, edge: usize) -> Self {
        Self {
            loop_index,
            loop_summary,
            edge,
        }
    }
}

#[derive(Clone, Copy)]
struct SegmentEnvelope {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

impl SegmentEnvelope {
    fn from_vertices(
        left: &ProjectedLoopVertexSnapshot,
        right: &ProjectedLoopVertexSnapshot,
    ) -> Self {
        Self {
            min_x: left.point_2d[0].min(right.point_2d[0]),
            max_x: left.point_2d[0].max(right.point_2d[0]),
            min_y: left.point_2d[1].min(right.point_2d[1]),
            max_y: left.point_2d[1].max(right.point_2d[1]),
        }
    }

    fn overlaps(self, other: Self) -> bool {
        self.min_x <= other.max_x
            && other.min_x <= self.max_x
            && self.min_y <= other.max_y
            && other.min_y <= self.max_y
    }
}

fn edge_envelope(loop_summary: &CertifiedLoopWindingSummary, edge: usize) -> SegmentEnvelope {
    let vertices = loop_summary.canonical_vertices();
    SegmentEnvelope::from_vertices(vertices[edge], vertices[(edge + 1) % vertices.len()])
}

fn edges_are_adjacent(first: usize, second: usize, edge_count: usize) -> bool {
    first == second || first + 1 == second || (first == 0 && second + 1 == edge_count)
}
