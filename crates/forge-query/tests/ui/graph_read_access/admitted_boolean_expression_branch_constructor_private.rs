use forge_query::facade::runtime::{
    ForgeQueryAdmittedBooleanExpressionBranch, ForgeQueryAdmittedBooleanExpressionBranchKind,
};

fn main() {
    let _ = ForgeQueryAdmittedBooleanExpressionBranch {
        branch_kind: ForgeQueryAdmittedBooleanExpressionBranchKind::ConjunctiveRoot,
        expression_path: "root".to_string(),
        predicate_leaves: Vec::new(),
    };
}
