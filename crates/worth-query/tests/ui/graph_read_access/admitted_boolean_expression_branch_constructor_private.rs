use worth_query::facade::runtime::{WorthQueryAdmittedBooleanExpressionBranch, WorthQueryAdmittedBooleanExpressionBranchKind};

fn main() {
    let _ = WorthQueryAdmittedBooleanExpressionBranch {
        branch_kind: WorthQueryAdmittedBooleanExpressionBranchKind::ConjunctiveRoot,
        expression_path: "root".to_string(),
        predicate_leaves: Vec::new(),
    };
}
