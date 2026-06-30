#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupQueryConsumerKitCounters {
    binding_row_count: usize,
    support_pinning_binding_row_count: usize,
    support_row_count: usize,
    query_residue_row_count: usize,
    boundary_audit_finding_count: usize,
}

impl EvidenceLookupQueryConsumerKitCounters {
    pub(crate) fn new(
        binding_row_count: usize,
        support_pinning_binding_row_count: usize,
        support_row_count: usize,
        query_residue_row_count: usize,
        boundary_audit_finding_count: usize,
    ) -> Self {
        Self {
            binding_row_count,
            support_pinning_binding_row_count,
            support_row_count,
            query_residue_row_count,
            boundary_audit_finding_count,
        }
    }

    pub const fn binding_row_count(&self) -> usize {
        self.binding_row_count
    }

    pub const fn support_pinning_binding_row_count(&self) -> usize {
        self.support_pinning_binding_row_count
    }

    pub const fn support_row_count(&self) -> usize {
        self.support_row_count
    }

    pub const fn query_residue_row_count(&self) -> usize {
        self.query_residue_row_count
    }

    pub const fn boundary_audit_finding_count(&self) -> usize {
        self.boundary_audit_finding_count
    }
}
