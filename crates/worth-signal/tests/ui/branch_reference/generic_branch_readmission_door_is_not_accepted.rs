use worth_proof::{AuthorityMarker, AuthorityWitness};
use worth_signal::facade::branch::{
    admit_signal_branch_observation, SignalBranchBasisAuthority, SignalBranchObservation,
};

fn generic_readmission_door<Auth: AuthorityMarker>(
    observation: SignalBranchObservation,
    authority: AuthorityWitness<Auth>,
) {
    let _authority: SignalBranchBasisAuthority = authority;
    let _ = admit_signal_branch_observation(observation, _authority);
}

fn main() {}
