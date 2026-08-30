use worth_relational::facade::branch::AdmittedRelationalForkSourceBasis;
use worth_relational::facade::history::BranchId;
use worth_relational::facade::runtime::RelationalRuntime;
use worth_proof::{AuthorityMarker, AuthorityWitness};

fn generic_door<Auth: AuthorityMarker>(
    runtime: &RelationalRuntime,
    target: BranchId,
    authority: AuthorityWitness<Auth>,
) {
    let _basis: AdmittedRelationalForkSourceBasis = authority;
    let _ = runtime.fork_branch(target, _basis);
}

fn main() {}
