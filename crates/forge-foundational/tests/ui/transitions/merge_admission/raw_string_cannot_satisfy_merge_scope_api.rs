use forge_foundational::{foundational_merge, FoundationalSelectedNodeLocus};

fn requires_scope(_: forge_foundational::FoundationalMergeScope) {}

fn main() {
    requires_scope("gear");

    let source = forge_foundational::foundational_branch_candidate()
        .on_branch(forge_foundational::FoundationalBranchId::new("feature").unwrap())
        .with_candidate_id(forge_foundational::FoundationalBranchCandidateId::new(
            forge_foundational::BoundaryHandle::new(1),
        ))
        .from_fork_basis(forge_foundational::FoundationalBranchForkBasis::new(
            forge_foundational::FoundationalBranchId::new("main").unwrap(),
            forge_foundational::BoundaryEpoch::new(1),
        ))
        .under_observation_basis(forge_foundational::FoundationalBranchObservationBasis::new(
            forge_foundational::EquivalenceBasisId::new(1),
            forge_foundational::BoundaryEpoch::new(1),
        ))
        .stage(())
        .unwrap()
        .staged();

    let _ = foundational_merge(source).with_scope(FoundationalSelectedNodeLocus::new("gear").unwrap());
}
