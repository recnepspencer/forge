use worth_query_installation::facade::WorthQueryInstalledApplicationQuery;

use super::{
    WorthQueryApplicationQueryAdmissionDenial, WorthQueryApplicationQueryAdmissionDenialKind,
};

/// Admits only bounded one-shot root-selection shapes implemented by this lane.
///
/// Result relations and path-bound ordering are supported by the shared bounded
/// tree materializer after their exact requirements have been admitted.
/// Root enumeration must either remain the admitted scope or be a declared,
/// bounded union of scope-to-root paths. Broader enumeration remains unsupported.
pub(super) fn validate_one_shot_shape<Schema, Query, Parameters, QueryResult, Scope>(
    query: &WorthQueryInstalledApplicationQuery<Schema, Query, Parameters, QueryResult, Scope>,
) -> Result<(), WorthQueryApplicationQueryAdmissionDenial> {
    let graph = query.read_family_binding().planning_contract();
    let root = graph.root_entity();
    let exact_root_scope = root == query.scope_entity();
    let bounded_exact_selection = match graph.predicates() {
        [] => true,
        [predicate] => predicate.field().0 == root,
        _ => false,
    };
    let bounded_path_selection = !graph.root_paths().is_empty() && graph.predicates().is_empty();
    let declared_result = !graph.projections().is_empty() || !graph.relations().is_empty();
    if declared_result
        && ((exact_root_scope && graph.root_paths().is_empty() && bounded_exact_selection)
            || (!exact_root_scope && bounded_path_selection))
    {
        return Ok(());
    }
    Err(WorthQueryApplicationQueryAdmissionDenial::new(
        WorthQueryApplicationQueryAdmissionDenialKind::ExecutionShapeUnsupported,
        query.name(),
    ))
}
