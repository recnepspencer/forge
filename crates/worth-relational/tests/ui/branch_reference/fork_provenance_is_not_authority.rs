use worth_relational::facade::branch::{RelationalBranchObservation, RelationalForkOutcome};
use worth_relational::facade::history::BranchId;
use worth_relational::facade::runtime::RelationalRuntime;

fn main() {
    let mut runtime: RelationalRuntime = todo!();
    let outcome: RelationalForkOutcome = todo!();
    let provenance: &RelationalBranchObservation = outcome.fork_provenance();
    let _ = runtime.fork_branch(BranchId("target".to_owned()), provenance);
}
