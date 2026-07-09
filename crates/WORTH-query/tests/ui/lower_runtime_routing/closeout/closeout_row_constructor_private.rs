use worth_query::facade::{
    WorthQueryLowerRuntimeAuthorityOwner, WorthQueryLowerRuntimeCloseoutPosture,
    WorthQueryLowerRuntimeCloseoutRow, WorthQueryLowerRuntimeRouteKind,
    WorthQueryLowerRuntimeSeamKey,
};

fn main() {
    let _ = WorthQueryLowerRuntimeCloseoutRow::new(
        WorthQueryLowerRuntimeSeamKey::StoreBackedRouteParityNeighbor,
        "Store-backed route parity",
        WorthQueryLowerRuntimeCloseoutPosture::DeferredNeighbor,
        WorthQueryLowerRuntimeAuthorityOwner::Store,
        WorthQueryLowerRuntimeRouteKind::RoutePlanning,
        "later store-backed route parity milestone",
        "close it later",
        "deferred-store-route-parity",
    );
}
