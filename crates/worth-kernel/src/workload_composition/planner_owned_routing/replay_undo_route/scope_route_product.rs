use std::sync::OnceLock;

use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::replay_family_catalog::current_spatial_replay_family_catalog;
use worth_spatial::facade::replay_undo_semantic_graph::{
    admit_prepared_spatial_replay_semantic_graph_input, current_boolean_split_spatial_boundary,
    lower_spatial_replay_scope_product_from_admitted_input,
    lower_spatial_undo_scope_product_from_boolean_event_ledger_request,
    prepare_spatial_replay_semantic_graph_request, BooleanEventLedgerRollbackRequest,
    SpatialReplaySemanticGraphPreparationRequest,
};

use crate::replay_undo_transaction_boundary::{
    assemble_replay_undo_transaction_boundary_input, ReplayUndoTransactionBoundaryAssemblyRequest,
    ReplayUndoTransactionBoundaryInput, ReplayUndoTransactionBoundarySupportSource,
};
use crate::workload_composition::performance_trace::trace_scope;
use crate::workload_composition::planner_owned_routing::{
    PlannerOwnedRoutingError, PlannerOwnedRoutingErrorKind,
};

#[derive(Clone)]
pub(crate) struct ReplayUndoPlannerScopeRouteProduct {
    transaction_boundary_input: ReplayUndoTransactionBoundaryInput,
    replay_scope_identity_digest: String,
    undo_scope_identity_digest: String,
    stage_index_identity_digest: String,
    lookup_receipt_identity_digest: String,
    product_identity: String,
}

pub(crate) fn current_replay_undo_planner_scope_route_product(
) -> Result<ReplayUndoPlannerScopeRouteProduct, PlannerOwnedRoutingError> {
    static CACHE: OnceLock<ReplayUndoPlannerScopeRouteProduct> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }
    let product = trace_scope("current_replay_undo_planner_scope_route_product", || {
        let split_boundary = trace_scope("current_boolean_split_spatial_boundary", || {
            current_boolean_split_spatial_boundary().map_err(current_route_error)
        })?;
        let topology_boundary = trace_scope(
            "current_replay_undo_topology_ordinary_undo_scope_boundary",
            || {
                topology::replay_undo_semantic_graph::current_replay_undo_topology_ordinary_undo_scope_boundary()
                    .map_err(current_route_error)
            },
        )?;
        let topology_undo_scope = topology_boundary.lower_undo_scope_product();
        let retained_replay = split_boundary.retained_replay_receipt().ok_or_else(|| {
            PlannerOwnedRoutingError::new(
                PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
                "planner-owned replay/undo route requires retained replay authority on the current split boundary",
            )
        })?;
        let replay_request = prepare_spatial_replay_semantic_graph_request(
            SpatialReplaySemanticGraphPreparationRequest::new(
                split_boundary.replay_family_identity(),
                split_boundary.authority(),
                split_boundary.execution_receipt(),
                split_boundary.workload_handoff(),
            )
            .with_retained_replay_receipt(retained_replay),
        )
        .map_err(current_route_error)?;
        let admitted_replay = trace_scope("current_spatial_replay_family_catalog", || {
            admit_prepared_spatial_replay_semantic_graph_input(
                &current_spatial_replay_family_catalog(),
                &replay_request,
            )
            .map_err(current_route_error)
        })?;
        let replay_scope = lower_spatial_replay_scope_product_from_admitted_input(&admitted_replay)
            .map_err(current_route_error)?;
        let undo_scope = lower_spatial_undo_scope_product_from_boolean_event_ledger_request(
            BooleanEventLedgerRollbackRequest::new(
                split_boundary.authority(),
                split_boundary.execution_receipt(),
                split_boundary.stage_index_product(),
                split_boundary.workload_handoff(),
            ),
        )
        .map_err(current_route_error)?;
        let transaction_boundary_input = assemble_replay_undo_transaction_boundary_input(
            ReplayUndoTransactionBoundaryAssemblyRequest::new(
                &topology_undo_scope,
                &replay_scope,
                &undo_scope,
                ReplayUndoTransactionBoundarySupportSource::Ordinary,
            ),
        )
        .map_err(current_route_error)?;
        let replay_scope_identity_digest = replay_scope.scope_identity().digest().to_string();
        let undo_scope_identity_digest = undo_scope.scope_identity().digest().to_string();
        let stage_index_identity_digest = split_boundary
            .authority()
            .stage_index_identity()
            .to_string();
        let lookup_receipt_identity_digest = split_boundary
            .workload_handoff()
            .lookup_execution_receipt_digest()
            .to_string();
        let product_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-kernel:planner-owned-replay-undo-scope-route-product:v1".to_string(),
                format!(
                    "transaction-boundary-input:{}",
                    transaction_boundary_input.touched_digest()
                ),
                format!("stage:{stage_index_identity_digest}"),
                format!("lookup:{lookup_receipt_identity_digest}"),
                format!("replay-scope:{replay_scope_identity_digest}"),
                format!("undo-scope:{undo_scope_identity_digest}"),
            ],
        );

        Ok(ReplayUndoPlannerScopeRouteProduct {
            transaction_boundary_input,
            replay_scope_identity_digest,
            undo_scope_identity_digest,
            stage_index_identity_digest,
            lookup_receipt_identity_digest,
            product_identity,
        })
    })?;
    let _ = CACHE.set(product.clone());
    Ok(product)
}

impl ReplayUndoPlannerScopeRouteProduct {
    pub(crate) fn transaction_boundary_input(&self) -> &ReplayUndoTransactionBoundaryInput {
        &self.transaction_boundary_input
    }

    pub(crate) fn replay_scope_identity_digest(&self) -> &str {
        &self.replay_scope_identity_digest
    }

    pub(crate) fn undo_scope_identity_digest(&self) -> &str {
        &self.undo_scope_identity_digest
    }

    pub(crate) fn stage_index_identity_digest(&self) -> &str {
        &self.stage_index_identity_digest
    }

    pub(crate) fn lookup_receipt_identity_digest(&self) -> &str {
        &self.lookup_receipt_identity_digest
    }

    pub(crate) fn product_identity(&self) -> &str {
        &self.product_identity
    }
}

fn current_route_error<E: std::fmt::Debug>(error: E) -> PlannerOwnedRoutingError {
    PlannerOwnedRoutingError::new(
        PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
        format!("planner-owned replay/undo scope route product did not assemble: {error:?}"),
    )
}
