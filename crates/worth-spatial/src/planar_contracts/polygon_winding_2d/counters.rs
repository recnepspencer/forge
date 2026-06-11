#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertifiedPolygonWinding2DPerformanceCounters {
    loop_edges_walked: usize,
    projected_vertices_consumed: usize,
    segment_contacts_classified: usize,
    winding_predicates_consumed: usize,
    winding_tie_breaks_used: usize,
    basis_digest_part_count: usize,
}

impl CertifiedPolygonWinding2DPerformanceCounters {
    pub(crate) const fn certified(
        loop_edges_walked: usize,
        projected_vertices_consumed: usize,
        segment_contacts_classified: usize,
        winding_predicates_consumed: usize,
        winding_tie_breaks_used: usize,
        basis_digest_part_count: usize,
    ) -> Self {
        Self {
            loop_edges_walked,
            projected_vertices_consumed,
            segment_contacts_classified,
            winding_predicates_consumed,
            winding_tie_breaks_used,
            basis_digest_part_count,
        }
    }

    pub fn loop_edges_walked(&self) -> usize {
        self.loop_edges_walked
    }

    pub fn projected_vertices_consumed(&self) -> usize {
        self.projected_vertices_consumed
    }

    pub fn segment_contacts_classified(&self) -> usize {
        self.segment_contacts_classified
    }

    pub fn winding_predicates_consumed(&self) -> usize {
        self.winding_predicates_consumed
    }

    pub fn winding_tie_breaks_used(&self) -> usize {
        self.winding_tie_breaks_used
    }

    pub fn basis_digest_part_count(&self) -> usize {
        self.basis_digest_part_count
    }
}
