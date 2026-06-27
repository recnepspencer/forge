use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupReplacementPhase;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupFamilySourceInventoryPressure {
    replacement_phase: EvidenceLookupReplacementPhase,
    migrate_row_count: usize,
    source_inventory_digest: String,
}

impl EvidenceLookupFamilySourceInventoryPressure {
    pub(crate) fn phase_two_family_catalog(
        migrate_row_count: usize,
        source_inventory_digest: impl Into<String>,
    ) -> Self {
        Self {
            replacement_phase: EvidenceLookupReplacementPhase::PhaseTwoFamilyCatalog,
            migrate_row_count,
            source_inventory_digest: source_inventory_digest.into(),
        }
    }

    pub const fn replacement_phase(&self) -> EvidenceLookupReplacementPhase {
        self.replacement_phase
    }

    pub const fn migrate_row_count(&self) -> usize {
        self.migrate_row_count
    }

    pub fn source_inventory_digest(&self) -> &str {
        &self.source_inventory_digest
    }

    pub fn pressure_digest_basis(&self) -> String {
        format!(
            "{:?}:{}:{}",
            self.replacement_phase, self.migrate_row_count, self.source_inventory_digest
        )
    }
}
