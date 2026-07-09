#![allow(unreachable_code)]

use worth_query::facade::runtime::{
    WorthQueryBooleanPredicateTopology, WorthQueryBooleanSelectivityAdmissionPosture,
    WorthQueryBooleanSelectivityShape, WorthQueryPredicateAnchorPosture,
    WorthQueryTraversalPredicateOrderingPosture,
};

fn main() {
    let _ = WorthQueryBooleanSelectivityShape {
        digest: todo!(),
        read_graph_digest: String::new(),
        access_shape_digest: String::new(),
        boolean_topology: WorthQueryBooleanPredicateTopology::None,
        anchor_posture: WorthQueryPredicateAnchorPosture::NoPredicateAnchor,
        traversal_predicate_ordering_posture: WorthQueryTraversalPredicateOrderingPosture::NoPredicate,
        admission_posture: WorthQueryBooleanSelectivityAdmissionPosture::InlineEligible,
        branches: Vec::new(),
        predicate_rows: Vec::new(),
        counters: todo!(),
    };
}
