#![allow(unreachable_code)]

use forge_query::facade::runtime::{
    ForgeQueryBooleanPredicateTopology, ForgeQueryBooleanSelectivityAdmissionPosture,
    ForgeQueryBooleanSelectivityShape, ForgeQueryPredicateAnchorPosture,
    ForgeQueryTraversalPredicateOrderingPosture,
};

fn main() {
    let _ = ForgeQueryBooleanSelectivityShape {
        digest: todo!(),
        read_graph_digest: String::new(),
        access_shape_digest: String::new(),
        boolean_topology: ForgeQueryBooleanPredicateTopology::None,
        anchor_posture: ForgeQueryPredicateAnchorPosture::NoPredicateAnchor,
        traversal_predicate_ordering_posture: ForgeQueryTraversalPredicateOrderingPosture::NoPredicate,
        admission_posture: ForgeQueryBooleanSelectivityAdmissionPosture::InlineEligible,
        branches: Vec::new(),
        predicate_rows: Vec::new(),
        counters: todo!(),
    };
}
