use forge_foundational::{
    foundational_branch_candidate, BoundaryEpoch, BoundaryHandle, EquivalenceBasisId,
    FoundationalBranchCandidateId, FoundationalBranchForkBasis, FoundationalBranchId,
    FoundationalBranchObservationBasis,
};

fn main() {
    let candidate = foundational_branch_candidate()
        .on_branch(FoundationalBranchId::new("feature/geometry").unwrap())
        .with_candidate_id(FoundationalBranchCandidateId::new(BoundaryHandle::new(7)))
        .from_fork_basis(FoundationalBranchForkBasis::new(
            FoundationalBranchId::new("main").unwrap(),
            BoundaryEpoch::new(4),
        ))
        .under_observation_basis(FoundationalBranchObservationBasis::new(
            EquivalenceBasisId::new(31),
            BoundaryEpoch::new(5),
        ))
        .stage("mesh-update")
        .unwrap();

    let _ = candidate.commit_with(());
}
