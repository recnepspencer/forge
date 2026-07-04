use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::replay_undo_semantic_graph::{
    current_boolean_event_ledger_spatial_boundary, current_boolean_split_spatial_boundary,
    current_projection_receipt_spatial_boundary, CurrentReplayUndoSpatialBoundary,
};

use super::cutover::{
    WorthWorkloadOrdinaryConsumerCutoverError, WorthWorkloadOrdinaryConsumerCutoverErrorKind,
};
use crate::workload_composition::performance_trace::trace_scope;

#[derive(Clone)]
pub(crate) struct WorthWorkloadCurrentLookupConsumedRouteAuthority {
    left_boundary: CurrentReplayUndoSpatialBoundary,
    right_boundary: CurrentReplayUndoSpatialBoundary,
    route_authority_digest: String,
}

#[derive(Clone, Debug)]
pub(crate) struct WorthWorkloadCurrentCompletedSplitRouteAuthority {
    split_boundary: CurrentReplayUndoSpatialBoundary,
    lookup_route_authority: WorthWorkloadCurrentLookupConsumedRouteAuthority,
    route_authority_digest: String,
}

pub(crate) fn current_lookup_consumed_route_authority() -> Result<
    WorthWorkloadCurrentLookupConsumedRouteAuthority,
    WorthWorkloadOrdinaryConsumerCutoverError,
> {
    WorthWorkloadCurrentLookupConsumedRouteAuthority::current()
}

pub(crate) fn current_completed_split_route_authority() -> Result<
    WorthWorkloadCurrentCompletedSplitRouteAuthority,
    WorthWorkloadOrdinaryConsumerCutoverError,
> {
    WorthWorkloadCurrentCompletedSplitRouteAuthority::current()
}

impl WorthWorkloadCurrentLookupConsumedRouteAuthority {
    fn current() -> Result<Self, WorthWorkloadOrdinaryConsumerCutoverError> {
        trace_scope("current_lookup_consumed_route_authority", || {
            let left_boundary =
                current_boolean_event_ledger_spatial_boundary().map_err(current_route_error)?;
            let right_boundary =
                current_projection_receipt_spatial_boundary().map_err(current_route_error)?;
            let route_authority_digest = truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "worth-kernel:ordinary-consumer-lookup-route-authority:v2".to_string(),
                    format!(
                        "left-stage:{}",
                        left_boundary.workload_handoff().stage_receipt_identity()
                    ),
                    format!(
                        "left-lookup:{}",
                        left_boundary
                            .workload_handoff()
                            .lookup_execution_receipt_digest()
                    ),
                    format!(
                        "left-authority:{}",
                        left_boundary.authority().stage_index_identity()
                    ),
                    format!(
                        "right-stage:{}",
                        right_boundary.workload_handoff().stage_receipt_identity()
                    ),
                    format!(
                        "right-lookup:{}",
                        right_boundary
                            .workload_handoff()
                            .lookup_execution_receipt_digest()
                    ),
                    format!(
                        "right-authority:{}",
                        right_boundary.authority().stage_index_identity()
                    ),
                ],
            );
            Ok(Self {
                left_boundary,
                right_boundary,
                route_authority_digest,
            })
        })
    }

    pub(crate) fn left_boundary(&self) -> &CurrentReplayUndoSpatialBoundary {
        &self.left_boundary
    }

    pub(crate) fn right_boundary(&self) -> &CurrentReplayUndoSpatialBoundary {
        &self.right_boundary
    }

    pub(crate) fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }
}

impl WorthWorkloadCurrentCompletedSplitRouteAuthority {
    fn current() -> Result<Self, WorthWorkloadOrdinaryConsumerCutoverError> {
        trace_scope("current_completed_split_route_authority", || {
            let lookup_route_authority = current_lookup_consumed_route_authority()?;
            let split_boundary =
                current_boolean_split_spatial_boundary().map_err(current_route_error)?;
            let retained_replay_identity = split_boundary
                .retained_replay_receipt()
                .map(|receipt| receipt.identity().receipt_identity().to_string())
                .unwrap_or("not-required".to_string());
            let route_authority_digest = truth_digest_parts(
                TruthDigestScope::ArtifactIdentity,
                &[
                    "worth-kernel:ordinary-consumer-completed-split-route-authority:v1".to_string(),
                    format!(
                        "lookup-authority:{}",
                        lookup_route_authority.route_authority_digest()
                    ),
                    format!(
                        "split-stage:{}",
                        split_boundary.workload_handoff().stage_receipt_identity()
                    ),
                    format!(
                        "split-lookup:{}",
                        split_boundary
                            .workload_handoff()
                            .lookup_execution_receipt_digest()
                    ),
                    format!(
                        "split-authority:{}",
                        split_boundary.authority().stage_index_identity()
                    ),
                    format!("retained-replay:{retained_replay_identity}"),
                ],
            );
            Ok(Self {
                split_boundary,
                lookup_route_authority,
                route_authority_digest,
            })
        })
    }

    pub(crate) fn split_boundary(&self) -> &CurrentReplayUndoSpatialBoundary {
        &self.split_boundary
    }

    pub(crate) fn lookup_route_authority(
        &self,
    ) -> &WorthWorkloadCurrentLookupConsumedRouteAuthority {
        &self.lookup_route_authority
    }

    pub(crate) fn route_authority_digest(&self) -> &str {
        &self.route_authority_digest
    }
}

impl PartialEq for WorthWorkloadCurrentLookupConsumedRouteAuthority {
    fn eq(&self, other: &Self) -> bool {
        self.route_authority_digest == other.route_authority_digest
    }
}

impl Eq for WorthWorkloadCurrentLookupConsumedRouteAuthority {}

impl std::fmt::Debug for WorthWorkloadCurrentLookupConsumedRouteAuthority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorthWorkloadCurrentLookupConsumedRouteAuthority")
            .field("route_authority_digest", &self.route_authority_digest)
            .finish()
    }
}

fn current_route_error<E: std::fmt::Debug>(error: E) -> WorthWorkloadOrdinaryConsumerCutoverError {
    WorthWorkloadOrdinaryConsumerCutoverError::new(
        WorthWorkloadOrdinaryConsumerCutoverErrorKind::MissingCurrentProofChain,
        format!("phase 13 current ordinary route authority did not assemble: {error:?}"),
    )
}
