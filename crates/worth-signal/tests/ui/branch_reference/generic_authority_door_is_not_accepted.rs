use worth_proof::{AuthorityMarker, AuthorityWitness};
use worth_signal::facade::branch::{
    admit_signal_branch_observation, SignalBranchObservation,
};

fn generic_door<Auth: AuthorityMarker>(
    reference: SignalBranchObservation,
    authority: AuthorityWitness<Auth>,
) {
    let _ = admit_signal_branch_observation(reference, authority);
}

fn main() {}
