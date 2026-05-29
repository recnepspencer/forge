mod edited_query_traversal;
mod edited_query_traversal_types;

pub use edited_query_traversal_types::{
    MilestoneThreeEditedTopologyQueryTraversalRow, MilestoneThreeEditedTopologyQueryTraversalView,
};

pub(in crate::certification::topology_operator_closeout) use edited_query_traversal::{
    certify_milestone_three_edited_query_traversal_impl, ensure_edited_query_traversal_rows,
    required_edited_query_traversal_views,
};




