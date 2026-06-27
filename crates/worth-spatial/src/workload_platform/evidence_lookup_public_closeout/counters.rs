#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupPublicCloseoutCounters {
    family_stage_row_count: usize,
    receipt_proof_row_count: usize,
    non_ordinary_residue_row_count: usize,
    query_surface_row_count: usize,
    query_consumer_binding_row_count: usize,
    query_support_pin_row_count: usize,
    spatial_deletion_row_count: usize,
    spatial_deletion_residue_row_count: usize,
    query_residue_row_count: usize,
    firewall_forbidden_row_count: usize,
    firewall_exception_row_count: usize,
}

impl EvidenceLookupPublicCloseoutCounters {
    pub(crate) fn new(
        family_stage_row_count: usize,
        receipt_proof_row_count: usize,
        non_ordinary_residue_row_count: usize,
        query_surface_row_count: usize,
        query_consumer_binding_row_count: usize,
        query_support_pin_row_count: usize,
        spatial_deletion_row_count: usize,
        spatial_deletion_residue_row_count: usize,
        query_residue_row_count: usize,
        firewall_forbidden_row_count: usize,
        firewall_exception_row_count: usize,
    ) -> Self {
        Self {
            family_stage_row_count,
            receipt_proof_row_count,
            non_ordinary_residue_row_count,
            query_surface_row_count,
            query_consumer_binding_row_count,
            query_support_pin_row_count,
            spatial_deletion_row_count,
            spatial_deletion_residue_row_count,
            query_residue_row_count,
            firewall_forbidden_row_count,
            firewall_exception_row_count,
        }
    }

    pub const fn family_stage_row_count(&self) -> usize {
        self.family_stage_row_count
    }

    pub const fn receipt_proof_row_count(&self) -> usize {
        self.receipt_proof_row_count
    }

    pub const fn non_ordinary_residue_row_count(&self) -> usize {
        self.non_ordinary_residue_row_count
    }

    pub const fn query_surface_row_count(&self) -> usize {
        self.query_surface_row_count
    }

    pub const fn query_consumer_binding_row_count(&self) -> usize {
        self.query_consumer_binding_row_count
    }

    pub const fn query_support_pin_row_count(&self) -> usize {
        self.query_support_pin_row_count
    }

    pub const fn spatial_deletion_row_count(&self) -> usize {
        self.spatial_deletion_row_count
    }

    pub const fn spatial_deletion_residue_row_count(&self) -> usize {
        self.spatial_deletion_residue_row_count
    }

    pub const fn query_residue_row_count(&self) -> usize {
        self.query_residue_row_count
    }

    pub const fn firewall_forbidden_row_count(&self) -> usize {
        self.firewall_forbidden_row_count
    }

    pub const fn firewall_exception_row_count(&self) -> usize {
        self.firewall_exception_row_count
    }
}
