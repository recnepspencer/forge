use std::time::Duration;

use axum::extract::{rejection::JsonRejection, State};
use axum::http::StatusCode;
use axum::Json;
use bank_domain::estate::{EstateAction, EstateCaseId, EstateDisbursement};
use bank_domain::model::{AccountId, BankPrincipalId, Money, USD};
use bank_domain::proposals::BankIdempotencyKey;

use super::super::protocol::{
    BankHttpDenial, BankHttpEstateDisbursementOutcome, BankHttpEstateDisbursementRequest,
    BankHttpRedoProgressionOutcome, BankHttpRedoProgressionRequest, BankHttpUndoProgressionOutcome,
    BankHttpUndoProgressionRequest,
};
use super::recovery_executor::{
    AdmittedBankHttpDisbursementRequest, AdmittedBankHttpRecoveryRequest,
    AdmittedBankHttpUndoProgressionRequest,
};
use super::recovery_routes::{malformed, valid_recovery_token};
use super::request_admission::UnadmittedBankHttpControls;
use super::routes::{response_status, BankHttpRouteState};

pub(super) async fn disburse(
    State(state): State<BankHttpRouteState>,
    request: Result<Json<BankHttpEstateDisbursementRequest>, JsonRejection>,
) -> (StatusCode, Json<BankHttpEstateDisbursementOutcome>) {
    let request = match request {
        Ok(Json(request)) => request,
        Err(_) => return response(disbursement_denied(None, malformed())),
    };
    let admitted = match admit_disbursement(request, state.maximum_deadline) {
        Ok(admitted) => admitted,
        Err(outcome) => return response(outcome),
    };
    response(state.recovery.disburse(admitted).await)
}

pub(super) async fn progress_undo(
    State(state): State<BankHttpRouteState>,
    request: Result<Json<BankHttpUndoProgressionRequest>, JsonRejection>,
) -> (StatusCode, Json<BankHttpUndoProgressionOutcome>) {
    let request = match request {
        Ok(Json(request)) => request,
        Err(_) => return undo_response(undo_denied(None, malformed())),
    };
    let admitted = match admit_undo(request, state.maximum_deadline) {
        Ok(admitted) => admitted,
        Err((request_id, denial)) => return undo_response(undo_denied(request_id, denial)),
    };
    undo_response(state.recovery.progress_undo(admitted).await)
}

pub(super) async fn progress_redo(
    State(state): State<BankHttpRouteState>,
    request: Result<Json<BankHttpRedoProgressionRequest>, JsonRejection>,
) -> (StatusCode, Json<BankHttpRedoProgressionOutcome>) {
    let request = match request {
        Ok(Json(request)) => request,
        Err(_) => return redo_response(redo_denied(None, malformed())),
    };
    let controls = match admit_controls(
        request.protocol,
        request.request_id,
        request.controls.deadline_milliseconds,
        state.maximum_deadline,
    ) {
        Ok(controls) => controls,
        Err((request_id, denial)) => return redo_response(redo_denied(request_id, denial)),
    };
    if !valid_recovery_token(&request.redo) {
        return redo_response(redo_denied(Some(controls.request_id), malformed()));
    }
    redo_response(
        state
            .recovery
            .progress_redo(AdmittedBankHttpRecoveryRequest {
                request_id: controls.request_id,
                credential: request.credential,
                token: request.redo,
                deadline: controls.deadline,
            })
            .await,
    )
}

fn admit_disbursement(
    request: BankHttpEstateDisbursementRequest,
    maximum_deadline: Duration,
) -> Result<AdmittedBankHttpDisbursementRequest, BankHttpEstateDisbursementOutcome> {
    let action = parse_disbursement(&request)
        .ok_or_else(|| disbursement_denied(Some(request.request_id.clone()), malformed()))?;
    let controls = admit_controls(
        request.protocol,
        request.request_id,
        request.controls.deadline_milliseconds,
        maximum_deadline,
    )
    .map_err(|(request_id, denial)| disbursement_denied(request_id, denial))?;
    let idempotency_key = BankIdempotencyKey::new(request.idempotency_key)
        .map_err(|_| disbursement_denied(Some(controls.request_id.clone()), malformed()))?;
    Ok(AdmittedBankHttpDisbursementRequest {
        request_id: controls.request_id,
        credential: request.credential,
        idempotency_key,
        action,
        deadline: controls.deadline,
    })
}

fn admit_undo(
    request: BankHttpUndoProgressionRequest,
    maximum_deadline: Duration,
) -> Result<AdmittedBankHttpUndoProgressionRequest, (Option<String>, BankHttpDenial)> {
    let controls = admit_controls(
        request.protocol,
        request.request_id,
        request.controls.deadline_milliseconds,
        maximum_deadline,
    )?;
    if !valid_recovery_token(&request.undo) {
        return Err((Some(controls.request_id), malformed()));
    }
    let idempotency_key = BankIdempotencyKey::new(request.idempotency_key)
        .map_err(|_| (Some(controls.request_id.clone()), malformed()))?;
    Ok(AdmittedBankHttpUndoProgressionRequest {
        request_id: controls.request_id,
        credential: request.credential,
        idempotency_key,
        token: request.undo,
        deadline: controls.deadline,
    })
}

fn admit_controls(
    protocol: super::super::protocol::BankHttpProtocolVersion,
    request_id: String,
    deadline_milliseconds: u64,
    maximum_deadline: Duration,
) -> Result<super::request_admission::AdmittedBankHttpControls, (Option<String>, BankHttpDenial)> {
    UnadmittedBankHttpControls {
        protocol,
        request_id,
        deadline_milliseconds,
    }
    .admit(maximum_deadline)
    .map_err(|rejected| (rejected.request_id, rejected.denial))
}

fn parse_disbursement(request: &BankHttpEstateDisbursementRequest) -> Option<EstateAction> {
    let amount = Money::<USD>::from_minor(request.amount_minor_units).ok()?;
    EstateDisbursement::new(
        EstateCaseId::parse_canonical_text(&request.estate)?,
        AccountId::parse_canonical_text(&request.source_account)?,
        AccountId::parse_canonical_text(&request.destination_account)?,
        BankPrincipalId::parse_canonical_text(&request.beneficiary)?,
        amount,
    )
    .ok()
    .map(EstateAction::DisburseEstate)
}

fn disbursement_denied(
    request_id: Option<String>,
    denial: BankHttpDenial,
) -> BankHttpEstateDisbursementOutcome {
    BankHttpEstateDisbursementOutcome::Denied { request_id, denial }
}

fn undo_denied(
    request_id: Option<String>,
    denial: BankHttpDenial,
) -> BankHttpUndoProgressionOutcome {
    BankHttpUndoProgressionOutcome::Denied { request_id, denial }
}

fn redo_denied(
    request_id: Option<String>,
    denial: BankHttpDenial,
) -> BankHttpRedoProgressionOutcome {
    BankHttpRedoProgressionOutcome::Denied { request_id, denial }
}

fn response(
    outcome: BankHttpEstateDisbursementOutcome,
) -> (StatusCode, Json<BankHttpEstateDisbursementOutcome>) {
    let status = match &outcome {
        BankHttpEstateDisbursementOutcome::Applied { .. } => StatusCode::OK,
        BankHttpEstateDisbursementOutcome::Denied { denial, .. } => response_status(denial.kind),
    };
    (status, Json(outcome))
}

fn undo_response(
    outcome: BankHttpUndoProgressionOutcome,
) -> (StatusCode, Json<BankHttpUndoProgressionOutcome>) {
    let status = match &outcome {
        BankHttpUndoProgressionOutcome::Applied { .. }
        | BankHttpUndoProgressionOutcome::Reconciled { .. } => StatusCode::OK,
        BankHttpUndoProgressionOutcome::Denied { denial, .. } => response_status(denial.kind),
    };
    (status, Json(outcome))
}

fn redo_response(
    outcome: BankHttpRedoProgressionOutcome,
) -> (StatusCode, Json<BankHttpRedoProgressionOutcome>) {
    let status = match &outcome {
        BankHttpRedoProgressionOutcome::Applied { .. } => StatusCode::OK,
        BankHttpRedoProgressionOutcome::Denied { denial, .. } => response_status(denial.kind),
    };
    (status, Json(outcome))
}
