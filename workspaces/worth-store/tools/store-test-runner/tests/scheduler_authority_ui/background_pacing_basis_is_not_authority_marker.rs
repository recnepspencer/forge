use worth_proof::AuthorityMarker;
use worth_store_io_scheduler::BackgroundPacingAdmissionBasis;

fn require_authority<T: AuthorityMarker>(_: T) {}

fn background_pacing_basis_is_not_authority(basis: BackgroundPacingAdmissionBasis) {
    require_authority(basis);
}

fn main() {}
