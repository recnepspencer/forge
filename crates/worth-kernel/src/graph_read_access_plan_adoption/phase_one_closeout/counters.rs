use super::super::execution_folklore_inventory::WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventory;
use super::super::seed_admission::WorthGraphReadAccessPlanAdoptionAdmittedSeed;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionPhaseOneCounters {
    read_family_identity_count: usize,
    requirement_row_evidence_count: usize,
    admission_capability_gap_count: usize,
    carried_requirement_derivation_gap_count: usize,
    execution_folklore_row_count: usize,
    migrate_row_count: usize,
    delete_row_count: usize,
    capped_residue_row_count: usize,
    query_gap_row_count: usize,
}

impl WorthGraphReadAccessPlanAdoptionPhaseOneCounters {
    pub(crate) fn from_parts(
        seed: &WorthGraphReadAccessPlanAdoptionAdmittedSeed,
        inventory: &WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventory,
    ) -> Self {
        Self {
            read_family_identity_count: seed.seed().read_family_identities().len(),
            requirement_row_evidence_count: seed.seed().requirement_row_evidence().len(),
            admission_capability_gap_count: seed.seed().admission_capability_gaps().len(),
            carried_requirement_derivation_gap_count: seed
                .seed()
                .carried_requirement_derivation_gaps()
                .len(),
            execution_folklore_row_count: inventory.counters().row_count(),
            migrate_row_count: inventory.counters().migrate_count(),
            delete_row_count: inventory.counters().delete_count(),
            capped_residue_row_count: inventory.counters().cap_count(),
            query_gap_row_count: inventory.counters().query_gap_count(),
        }
    }

    pub const fn read_family_identity_count(&self) -> usize {
        self.read_family_identity_count
    }

    pub const fn requirement_row_evidence_count(&self) -> usize {
        self.requirement_row_evidence_count
    }

    pub const fn admission_capability_gap_count(&self) -> usize {
        self.admission_capability_gap_count
    }

    pub const fn carried_requirement_derivation_gap_count(&self) -> usize {
        self.carried_requirement_derivation_gap_count
    }

    pub const fn execution_folklore_row_count(&self) -> usize {
        self.execution_folklore_row_count
    }

    pub const fn migrate_row_count(&self) -> usize {
        self.migrate_row_count
    }

    pub const fn delete_row_count(&self) -> usize {
        self.delete_row_count
    }

    pub const fn capped_residue_row_count(&self) -> usize {
        self.capped_residue_row_count
    }

    pub const fn query_gap_row_count(&self) -> usize {
        self.query_gap_row_count
    }
}
