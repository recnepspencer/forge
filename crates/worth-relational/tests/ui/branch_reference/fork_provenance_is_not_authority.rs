use worth_relational::facade::branch::{
    RelationalBranchReferenceObservation, RelationalForkOutcome,
};
use worth_relational::facade::history::BranchId;
use worth_relational::facade::runtime::RelationalRuntime;

fn main() {
    let mut runtime: RelationalRuntime = todo!();
    let outcome: RelationalForkOutcome = todo!();
    let provenance: &RelationalBranchReferenceObservation = outcome.fork_provenance();
    let _ = runtime.fork_branch(BranchId("target".to_owned()), provenance);
}
