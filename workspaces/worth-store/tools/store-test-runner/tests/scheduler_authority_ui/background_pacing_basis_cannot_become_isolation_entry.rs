use worth_store_io_scheduler::BackgroundPacingAdmissionBasis;
use worth_store_physical_isolation::PhysicalIsolationEntryAdmission;

fn background_pacing_basis_cannot_become_isolation_entry(
    basis: BackgroundPacingAdmissionBasis,
) -> PhysicalIsolationEntryAdmission {
    basis.into()
}

fn main() {}
