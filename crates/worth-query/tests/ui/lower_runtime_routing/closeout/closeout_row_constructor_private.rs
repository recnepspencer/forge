use worth_query::facade::runtime::{WorthQueryLowerRuntimeAuthorityOwner, WorthQueryLowerRuntimeRouteKind, WorthQueryLowerRuntimeSeamKey};
use worth_query::facade::certification::{WorthQueryLowerRuntimeCloseoutPosture, WorthQueryLowerRuntimeCloseoutRow};

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
