use forge_query::facade::runtime::{
    ForgeQueryBooleanSelectivityBranch, ForgeQueryBooleanSelectivityBranchKind,
    ForgeQueryPredicateAnchorPosture, ForgeQueryTraversalPredicateOrderingPosture,
};

fn main() {
    let _ = ForgeQueryBooleanSelectivityBranch {
        branch_kind: ForgeQueryBooleanSelectivityBranchKind::ConjunctiveRoot,
        expression_path: "root".to_string(),
        anchor_posture: ForgeQueryPredicateAnchorPosture::NoPredicateAnchor,
        traversal_predicate_ordering_posture: ForgeQueryTraversalPredicateOrderingPosture::NoPredicate,
        predicate_rows: Vec::new(),
    };
}
