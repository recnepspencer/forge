use std::sync::Arc;

use axum::extract::{rejection::JsonRejection, State};
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};

use crate::protocol::{
    BankUserNodeDenial, BankUserNodeDenialKind, BankUserNodeElevationApprovalOutcome,
    BankUserNodeElevationApprovalRequest, BankUserNodeElevationRequest,
    BankUserNodeElevationRequestOutcome, BankUserNodeElevationRevocationOutcome,
    BankUserNodeElevationRevocationRequest, BankUserNodeMandatoryReviewOutcome,
    BankUserNodeMandatoryReviewRequest,
};

use super::super::UserNodeState;
use super::node_denial_status;

pub(super) fn router() -> Router<UserNodeState> {
    Router::new()
        .route("/v1/estate/elevation/request", post(request_elevation))
        .route("/v1/estate/elevation/approve", post(approve_elevation))
        .route("/v1/estate/elevation/revoke", post(revoke_elevation))
        .route("/v1/estate/elevation/review", post(complete_review))
}

async fn request_elevation(
    State(state): State<UserNodeState>,
    request: Result<Json<BankUserNodeElevationRequest>, JsonRejection>,
) -> (StatusCode, Json<BankUserNodeElevationRequestOutcome>) {
    let Ok(Json(request)) = request else {
        return request_response(BankUserNodeElevationRequestOutcome::Denied {
            denial: malformed(),
        });
    };
    let Ok(_permit) = Arc::clone(&state.requests).try_acquire_owned() else {
        return request_response(BankUserNodeElevationRequestOutcome::Denied {
            denial: saturated(),
        });
    };
    request_response(state.session.request_elevation(request).await)
}

async fn approve_elevation(
    State(state): State<UserNodeState>,
    request: Result<Json<BankUserNodeElevationApprovalRequest>, JsonRejection>,
) -> (StatusCode, Json<BankUserNodeElevationApprovalOutcome>) {
    let Ok(Json(request)) = request else {
        return approval_response(BankUserNodeElevationApprovalOutcome::Denied {
            denial: malformed(),
        });
    };
    let Ok(_permit) = Arc::clone(&state.requests).try_acquire_owned() else {
        return approval_response(BankUserNodeElevationApprovalOutcome::Denied {
            denial: saturated(),
        });
    };
    approval_response(state.session.approve_elevation(request).await)
}

async fn revoke_elevation(
    State(state): State<UserNodeState>,
    request: Result<Json<BankUserNodeElevationRevocationRequest>, JsonRejection>,
) -> (StatusCode, Json<BankUserNodeElevationRevocationOutcome>) {
    let Ok(Json(request)) = request else {
        return revocation_response(BankUserNodeElevationRevocationOutcome::Denied {
            denial: malformed(),
        });
    };
    let Ok(_permit) = Arc::clone(&state.requests).try_acquire_owned() else {
        return revocation_response(BankUserNodeElevationRevocationOutcome::Denied {
            denial: saturated(),
        });
    };
    revocation_response(state.session.revoke_elevation(request).await)
}

async fn complete_review(
    State(state): State<UserNodeState>,
    request: Result<Json<BankUserNodeMandatoryReviewRequest>, JsonRejection>,
) -> (StatusCode, Json<BankUserNodeMandatoryReviewOutcome>) {
    let Ok(Json(request)) = request else {
        return review_response(BankUserNodeMandatoryReviewOutcome::Denied {
            denial: malformed(),
        });
    };
    let Ok(_permit) = Arc::clone(&state.requests).try_acquire_owned() else {
        return review_response(BankUserNodeMandatoryReviewOutcome::Denied {
            denial: saturated(),
        });
    };
    review_response(state.session.complete_mandatory_review(request).await)
}

fn request_response(
    outcome: BankUserNodeElevationRequestOutcome,
) -> (StatusCode, Json<BankUserNodeElevationRequestOutcome>) {
    let status = match &outcome {
        BankUserNodeElevationRequestOutcome::Forwarded { .. } => StatusCode::OK,
        BankUserNodeElevationRequestOutcome::Denied { denial } => node_denial_status(*denial),
    };
    (status, Json(outcome))
}

fn approval_response(
    outcome: BankUserNodeElevationApprovalOutcome,
) -> (StatusCode, Json<BankUserNodeElevationApprovalOutcome>) {
    let status = match &outcome {
        BankUserNodeElevationApprovalOutcome::Forwarded { .. } => StatusCode::OK,
        BankUserNodeElevationApprovalOutcome::Denied { denial } => node_denial_status(*denial),
    };
    (status, Json(outcome))
}

fn revocation_response(
    outcome: BankUserNodeElevationRevocationOutcome,
) -> (StatusCode, Json<BankUserNodeElevationRevocationOutcome>) {
    let status = match &outcome {
        BankUserNodeElevationRevocationOutcome::Forwarded { .. } => StatusCode::OK,
        BankUserNodeElevationRevocationOutcome::Denied { denial } => node_denial_status(*denial),
    };
    (status, Json(outcome))
}

fn review_response(
    outcome: BankUserNodeMandatoryReviewOutcome,
) -> (StatusCode, Json<BankUserNodeMandatoryReviewOutcome>) {
    let status = match &outcome {
        BankUserNodeMandatoryReviewOutcome::Forwarded { .. } => StatusCode::OK,
        BankUserNodeMandatoryReviewOutcome::Denied { denial } => node_denial_status(*denial),
    };
    (status, Json(outcome))
}

fn malformed() -> BankUserNodeDenial {
    BankUserNodeDenial::new(BankUserNodeDenialKind::MalformedRequest)
}

fn saturated() -> BankUserNodeDenial {
    BankUserNodeDenial::new(BankUserNodeDenialKind::RequestSaturated)
}
