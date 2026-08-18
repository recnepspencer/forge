use worth_relational::facade::branch::AdmittedRelationalForkSourceBasis;
use worth_relational::facade::history::BranchId;
use worth_relational::facade::runtime::RelationalRuntime;

fn main() {
    let mut runtime = unimplemented_runtime();
    let basis = unimplemented_basis();
    let _ = runtime.fork_branch(BranchId("first".to_owned()), basis);
    let _ = runtime.fork_branch(BranchId("second".to_owned()), basis);
}

fn unimplemented_runtime() -> RelationalRuntime {
    panic!("compile-fail fixture")
}

fn unimplemented_basis() -> AdmittedRelationalForkSourceBasis {
    panic!("compile-fail fixture")
}
