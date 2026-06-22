#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertifiedSignedArea2DPerformanceCounters {
    loop_edges_walked: usize,
    area_terms_evaluated: usize,
    precision_escalations: usize,
    local_scale_comparisons: usize,
    degeneracy_localization_breadth: usize,
    retained_basis_parts: usize,
}

impl CertifiedSignedArea2DPerformanceCounters {
    pub(crate) const fn certified(
        loop_edges_walked: usize,
        area_terms_evaluated: usize,
        precision_escalations: usize,
        local_scale_comparisons: usize,
        degeneracy_localization_breadth: usize,
        retained_basis_parts: usize,
    ) -> Self {
        Self {
            loop_edges_walked,
            area_terms_evaluated,
            precision_escalations,
            local_scale_comparisons,
            degeneracy_localization_breadth,
            retained_basis_parts,
        }
    }

    pub fn loop_edges_walked(self) -> usize {
        self.loop_edges_walked
    }

    pub fn area_terms_evaluated(self) -> usize {
        self.area_terms_evaluated
    }

    pub fn precision_escalations(self) -> usize {
        self.precision_escalations
    }

    pub fn local_scale_comparisons(self) -> usize {
        self.local_scale_comparisons
    }

    pub fn degeneracy_localization_breadth(self) -> usize {
        self.degeneracy_localization_breadth
    }

    pub fn retained_basis_parts(self) -> usize {
        self.retained_basis_parts
    }
}
