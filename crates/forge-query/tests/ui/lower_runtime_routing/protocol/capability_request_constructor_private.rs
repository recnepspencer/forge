use forge_query::facade::runtime::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeCapabilityRequest,
    ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeSeamKey,
};

fn main() {
    let _ = ForgeQueryLowerRuntimeCapabilityRequest::new(
        ForgeQueryLowerRuntimeSeamKey::WriteAuthorityBackendExecution,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::Query,
        "forged-capability",
        "forged-subject",
    );
}
