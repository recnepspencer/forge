use worth_query::facade::runtime::{WorthQueryLowerRuntimeAuthorityOwner, WorthQueryLowerRuntimeCapabilityRequest, WorthQueryLowerRuntimeRouteKind, WorthQueryLowerRuntimeSeamKey};

fn main() {
    let _ = WorthQueryLowerRuntimeCapabilityRequest::new(
        WorthQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        WorthQueryLowerRuntimeRouteKind::RoutePlanning,
        WorthQueryLowerRuntimeAuthorityOwner::Query,
        "worthd-capability",
        todo!(),
    );
}
