use forge_query::facade::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeCloseoutPosture,
    ForgeQueryLowerRuntimeCloseoutRow, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeSeamKey,
};

fn main() {
    let _ = ForgeQueryLowerRuntimeCloseoutRow::new(
        ForgeQueryLowerRuntimeSeamKey::StoreBackedRouteParityNeighbor,
        "Store-backed route parity",
        ForgeQueryLowerRuntimeCloseoutPosture::DeferredNeighbor,
        ForgeQueryLowerRuntimeAuthorityOwner::Store,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        "later store-backed route parity milestone",
        "close it later",
        "deferred-store-route-parity",
    );
}
