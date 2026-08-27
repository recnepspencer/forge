use crate::branch::{validate_signal_branch_name, ValidatedSignalBranchName};
use worth_proof::TransitionOutcome;

use super::{SignalBranchBasisArtifact, SignalBranchBasisDenial, SignalBranchForkDenial};

pub(super) fn validate_fork_branch_name(
    branch_name: &str,
) -> Result<ValidatedSignalBranchName, SignalBranchForkDenial> {
    validate_signal_branch_name(branch_name.to_owned())
        .map_err(|_| SignalBranchForkDenial::InvalidBranchIdentity)
}

pub(super) fn expect_fork_branch_basis(
    outcome: TransitionOutcome<SignalBranchBasisArtifact, SignalBranchBasisDenial>,
) -> SignalBranchBasisArtifact {
    match outcome {
        TransitionOutcome::Success(basis) => basis,
        other => panic!("validated branch fork basis must succeed, got {other:?}"),
    }
}
