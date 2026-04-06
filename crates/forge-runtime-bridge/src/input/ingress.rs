use crate::error::{BridgeRouteError, BridgeRouteErrorKind};
use crate::facade::{BridgeRouteRequest, RuntimeBridge};
use crate::routing::{BridgeMappingContext, IngestedBridgePatch};

use super::{normalization, validation};

pub(crate) fn ingest_committed_patch(
    runtime: &RuntimeBridge,
    request: BridgeRouteRequest,
) -> Result<IngestedBridgePatch, BridgeRouteError> {
    let raw_envelope = runtime
        .committed_patch_source
        .load_committed_patch(request.into_committed_patch_request())
        .map_err(|error| {
            BridgeRouteError::new(
                BridgeRouteErrorKind::UnsupportedTruthPatchScope,
                format!("Relational bridge source failed to load committed patch: {error}"),
            )
        })?;
    let normalized = normalization::normalize_raw_envelope(raw_envelope);
    let envelope = validation::validate_normalized_envelope(normalized)?;
    Ok(IngestedBridgePatch::new(
        envelope,
        BridgeMappingContext::default(),
        crate::routing::scope::RouteScope::begin(),
    ))
}
