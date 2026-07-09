use worth_query::facade::runtime::{
    WorthQueryBooleanSelectivityBranch, WorthQueryBooleanSelectivityBranchKind,
    WorthQueryPredicateAnchorPosture, WorthQueryTraversalPredicateOrderingPosture,
};

fn main() {
    let _ = WorthQueryBooleanSelectivityBranch {
        branch_kind: WorthQueryBooleanSelectivityBranchKind::ConjunctiveRoot,
        expression_path: "root".to_string(),
        anchor_posture: WorthQueryPredicateAnchorPosture::NoPredicateAnchor,
        traversal_predicate_ordering_posture: WorthQueryTraversalPredicateOrderingPosture::NoPredicate,
        predicate_rows: Vec::new(),
    };
}
