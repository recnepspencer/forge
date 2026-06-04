use crate::error::{BridgeRouteError, BridgeRouteErrorKind};
use crate::facade::{BridgeRouteRequest, RuntimeBridge};
use crate::routing::{BridgeMappingContext, IngestedBridgePatch};

pub(crate) fn ingest_committed_patch(
    runtime: &RuntimeBridge,
    request: BridgeRouteRequest,
) -> Result<IngestedBridgePatch, BridgeRouteError> {
    let envelope = runtime
        .committed_patch_source
        .load_committed_patch(request.into_committed_patch_request())
        .map_err(|error| {
            BridgeRouteError::new(
                BridgeRouteErrorKind::UnsupportedTruthPatchScope,
                format!("Relational bridge source failed to load committed patch: {error}"),
            )
        })?;
    Ok(IngestedBridgePatch::new(
        envelope,
        BridgeMappingContext::default(),
        crate::routing::scope::RouteScope::begin(),
    ))
}
