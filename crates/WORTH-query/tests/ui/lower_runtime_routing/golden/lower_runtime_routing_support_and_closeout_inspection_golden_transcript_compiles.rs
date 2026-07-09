use worth_query::facade::{
    worth_query_lower_runtime_closeout_registry, worth_query_lower_runtime_support_matrix,
    inspect_lower_runtime_closeout, WorthQueryLowerRuntimeSeamKey,
};

fn main() {
    let support = worth_query_lower_runtime_support_matrix();
    let closeout = worth_query_lower_runtime_closeout_registry();

    let support_row = support
        .support_for(WorthQueryLowerRuntimeSeamKey::StoreBackedRouteParityNeighbor)
        .expect("deferred store-backed neighbor should stay support-visible");
    let deferred = closeout
        .rows()
        .iter()
        .find(|row| row.seam_key() == WorthQueryLowerRuntimeSeamKey::StoreBackedRouteParityNeighbor)
        .expect("deferred store-backed neighbor should stay closeout-visible");
    let inspection = inspect_lower_runtime_closeout(deferred);

    let _ = support_row.posture().as_str();
    let _ = support_row.authority_owner().as_str();
    let _ = support_row.route_kind().as_str();
    let _ = inspection.headline();
    let _ = inspection.detail();
    let _ = inspection.inspection_digest();
}
