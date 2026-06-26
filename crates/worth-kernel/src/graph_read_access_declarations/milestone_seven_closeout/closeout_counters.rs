#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationCloseoutCounters {
    declaration_catalog_record_count: usize,
    read_family_identity_count: usize,
    requirement_evidence_row_count: usize,
    admission_capability_gap_count: usize,
    carried_requirement_derivation_gap_count: usize,
    deletion_ledger_row_count: usize,
    capped_residue_row_count: usize,
    source_firewall_scanned_region_count: usize,
    execution_receipt_count: usize,
    access_plan_consumption_count: usize,
}

impl WorthGraphReadAccessDeclarationCloseoutCounters {
    pub(crate) const fn new(
        declaration_catalog_record_count: usize,
        read_family_identity_count: usize,
        requirement_evidence_row_count: usize,
        admission_capability_gap_count: usize,
        carried_requirement_derivation_gap_count: usize,
        deletion_ledger_row_count: usize,
        capped_residue_row_count: usize,
        source_firewall_scanned_region_count: usize,
    ) -> Self {
        Self {
            declaration_catalog_record_count,
            read_family_identity_count,
            requirement_evidence_row_count,
            admission_capability_gap_count,
            carried_requirement_derivation_gap_count,
            deletion_ledger_row_count,
            capped_residue_row_count,
            source_firewall_scanned_region_count,
            execution_receipt_count: 0,
            access_plan_consumption_count: 0,
        }
    }

    pub const fn declaration_catalog_record_count(&self) -> usize {
        self.declaration_catalog_record_count
    }

    pub const fn read_family_identity_count(&self) -> usize {
        self.read_family_identity_count
    }

    pub const fn requirement_evidence_row_count(&self) -> usize {
        self.requirement_evidence_row_count
    }

    pub const fn admission_capability_gap_count(&self) -> usize {
        self.admission_capability_gap_count
    }

    pub const fn carried_requirement_derivation_gap_count(&self) -> usize {
        self.carried_requirement_derivation_gap_count
    }

    pub const fn deletion_ledger_row_count(&self) -> usize {
        self.deletion_ledger_row_count
    }

    pub const fn capped_residue_row_count(&self) -> usize {
        self.capped_residue_row_count
    }

    pub const fn source_firewall_scanned_region_count(&self) -> usize {
        self.source_firewall_scanned_region_count
    }

    pub const fn execution_receipt_count(&self) -> usize {
        self.execution_receipt_count
    }

    pub const fn access_plan_consumption_count(&self) -> usize {
        self.access_plan_consumption_count
    }
}
