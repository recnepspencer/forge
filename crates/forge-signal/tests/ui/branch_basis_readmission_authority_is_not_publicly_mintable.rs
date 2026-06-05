use forge_proof::AuthorityWitness;
use forge_signal::facade::SignalBranchBasisReadmissionAuthority;

fn main() {
    let _authority = AuthorityWitness::from_authority_marker(
        SignalBranchBasisReadmissionAuthority::new(),
    );
}
