mod mutation_query_traversal;
mod mutation_query_traversal_types;

pub use mutation_query_traversal_types::{
    MilestoneThreeMutationTopologyQueryTraversalRow,
    MilestoneThreeMutationTopologyQueryTraversalView,
};

pub(in crate::certification::topology_operator_closeout) use mutation_query_traversal::{
    certify_milestone_three_mutation_query_traversal_impl, ensure_mutation_query_traversal_rows,
    required_mutation_query_traversal_views,
};
