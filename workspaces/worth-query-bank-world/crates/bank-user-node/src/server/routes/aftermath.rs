use std::sync::Arc;

use axum::extract::{rejection::JsonRejection, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};

use crate::protocol::{
    BankUserNodeDenial, BankUserNodeDenialKind, BankUserNodeEstateDisbursementOutcome,
    BankUserNodeEstateDisbursementRequest, BankUserNodeRedoProgressionOutcome,
    BankUserNodeRedoProgressionRequest, BankUserNodeUndoProgressionOutcome,
    BankUserNodeUndoProgressionRequest,
};

use super::super::UserNodeState;
use super::node_denial_status;

pub(super) fn router() -> Router<UserNodeState> {
    Router::new()
        .route("/v1/estate/disburse", post(disburse_estate))
        .route("/v1/recovery/progress-undo", post(progress_undo))
        .route("/v1/recovery/progress-redo", post(progress_redo))
}

async fn disburse_estate(
    State(state): State<UserNodeState>,
    request: Result<Json<BankUserNodeEstateDisbursementRequest>, JsonRejection>,
) -> (StatusCode, Json<BankUserNodeEstateDisbursementOutcome>) {
    let Ok(Json(request)) = request else {
        return response(BankUserNodeEstateDisbursementOutcome::Denied {
            denial: malformed(),
        });
    };
    let Ok(_permit) = Arc::clone(&state.requests).try_acquire_owned() else {
        return response(BankUserNodeEstateDisbursementOutcome::Denied {
            denial: saturated(),
        });
    };
    response(state.session.disburse_estate(request).await)
}

async fn progress_undo(
    State(state): State<UserNodeState>,
    request: Result<Json<BankUserNodeUndoProgressionRequest>, JsonRejection>,
) -> (StatusCode, Json<BankUserNodeUndoProgressionOutcome>) {
    let Ok(Json(request)) = request else {
        return undo_response(BankUserNodeUndoProgressionOutcome::Denied {
            denial: malformed(),
        });
    };
    let Ok(_permit) = Arc::clone(&state.requests).try_acquire_owned() else {
        return undo_response(BankUserNodeUndoProgressionOutcome::Denied {
            denial: saturated(),
        });
    };
    undo_response(state.session.progress_undo(request).await)
}

async fn progress_redo(
    State(state): State<UserNodeState>,
    request: Result<Json<BankUserNodeRedoProgressionRequest>, JsonRejection>,
) -> (StatusCode, Json<BankUserNodeRedoProgressionOutcome>) {
    let Ok(Json(request)) = request else {
        return redo_response(BankUserNodeRedoProgressionOutcome::Denied {
            denial: malformed(),
        });
    };
    let Ok(_permit) = Arc::clone(&state.requests).try_acquire_owned() else {
        return redo_response(BankUserNodeRedoProgressionOutcome::Denied {
            denial: saturated(),
        });
    };
    redo_response(state.session.progress_redo(request).await)
}

fn response(
    outcome: BankUserNodeEstateDisbursementOutcome,
) -> (StatusCode, Json<BankUserNodeEstateDisbursementOutcome>) {
    let status = match &outcome {
        BankUserNodeEstateDisbursementOutcome::Forwarded { .. } => StatusCode::OK,
        BankUserNodeEstateDisbursementOutcome::Denied { denial } => node_denial_status(*denial),
    };
    (status, Json(outcome))
}

fn undo_response(
    outcome: BankUserNodeUndoProgressionOutcome,
) -> (StatusCode, Json<BankUserNodeUndoProgressionOutcome>) {
    let status = match &outcome {
        BankUserNodeUndoProgressionOutcome::Forwarded { .. } => StatusCode::OK,
        BankUserNodeUndoProgressionOutcome::Denied { denial } => node_denial_status(*denial),
    };
    (status, Json(outcome))
}

fn redo_response(
    outcome: BankUserNodeRedoProgressionOutcome,
) -> (StatusCode, Json<BankUserNodeRedoProgressionOutcome>) {
    let status = match &outcome {
        BankUserNodeRedoProgressionOutcome::Forwarded { .. } => StatusCode::OK,
        BankUserNodeRedoProgressionOutcome::Denied { denial } => node_denial_status(*denial),
    };
    (status, Json(outcome))
}

fn malformed() -> BankUserNodeDenial {
    BankUserNodeDenial::new(BankUserNodeDenialKind::MalformedRequest)
}

fn saturated() -> BankUserNodeDenial {
    BankUserNodeDenial::new(BankUserNodeDenialKind::RequestSaturated)
}
