use super::ordinary_topology_undo_support::{
    undo_scope_support_for_loop_successor, OrdinaryTraversalViewsUndoScopeSupport,
};

pub(crate) fn alternate_ordinary_traversal_views_undo_scope_support(
) -> OrdinaryTraversalViewsUndoScopeSupport {
    undo_scope_support_for_loop_successor(21, 12, 13)
}
