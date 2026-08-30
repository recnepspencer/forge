use worth_relational::facade::branch::RelationalForkSourceDescriptor;
use worth_relational::facade::history::BranchId;
use worth_relational::facade::runtime::RelationalRuntime;

fn main() {
    let runtime: &RelationalRuntime = todo!();
    let descriptor: RelationalForkSourceDescriptor = todo!();
    let _ = runtime.fork_branch(BranchId("storm".to_owned()), descriptor);
}
