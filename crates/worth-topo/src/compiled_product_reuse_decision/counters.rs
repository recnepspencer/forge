#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TopologyDerivedReuseDecisionCounters {
    compared_basis_dimension_count: usize,
    compared_derived_surface_digest_count: usize,
}

impl TopologyDerivedReuseDecisionCounters {
    pub(crate) fn new(
        compared_basis_dimension_count: usize,
        compared_derived_surface_digest_count: usize,
    ) -> Self {
        Self {
            compared_basis_dimension_count,
            compared_derived_surface_digest_count,
        }
    }

    pub const fn compared_basis_dimension_count(&self) -> usize {
        self.compared_basis_dimension_count
    }

    pub const fn compared_derived_surface_digest_count(&self) -> usize {
        self.compared_derived_surface_digest_count
    }
}
