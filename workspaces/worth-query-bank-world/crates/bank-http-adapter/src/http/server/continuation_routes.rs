use std::time::Duration;

use axum::extract::{rejection::JsonRejection, State};
use axum::http::StatusCode;
use axum::Json;

use super::super::protocol::{
    BankHttpAccountActivityPageOutcome, BankHttpAccountActivityPageRequest,
    BankHttpAccountActivityResumeRequest, BankHttpDenial, BankHttpDenialKind, BankHttpNextAction,
};
use super::continuation_executor::{AdmittedPageRequest, AdmittedResumeRequest};
use super::request_admission::{RejectedBankHttpRequest, UnadmittedBankHttpRequestBasis};
use super::routes::{response_status, BankHttpRouteState};

pub(super) async fn page(
    State(state): State<BankHttpRouteState>,
    request: Result<Json<BankHttpAccountActivityPageRequest>, JsonRejection>,
) -> (StatusCode, Json<BankHttpAccountActivityPageOutcome>) {
    let request = match request {
        Ok(Json(request)) => request,
        Err(_) => return response(denied(None, malformed())),
    };
    let admitted = match admit_page(request, state.maximum_deadline) {
        Ok(request) => request,
        Err(rejected) => return response(denied(rejected.request_id, rejected.denial)),
    };
    response(state.continuations.page(admitted).await)
}

pub(super) async fn resume(
    State(state): State<BankHttpRouteState>,
    request: Result<Json<BankHttpAccountActivityResumeRequest>, JsonRejection>,
) -> (StatusCode, Json<BankHttpAccountActivityPageOutcome>) {
    let request = match request {
        Ok(Json(request)) => request,
        Err(_) => return response(denied(None, malformed())),
    };
    if !super::continuation_registry::BankHttpContinuationRegistry::recognizes_token(
        &request.continuation,
    ) {
        return response(denied(Some(request.request_id), malformed()));
    }
    let continuation = request.continuation;
    let page = match admit_page(
        BankHttpAccountActivityPageRequest {
            protocol: request.protocol,
            request_id: request.request_id,
            credential: request.credential,
            controls: request.controls,
            account: request.account,
        },
        state.maximum_deadline,
    ) {
        Ok(request) => request,
        Err(rejected) => return response(denied(rejected.request_id, rejected.denial)),
    };
    response(
        state
            .continuations
            .resume(AdmittedResumeRequest { page, continuation })
            .await,
    )
}

fn admit_page(
    request: BankHttpAccountActivityPageRequest,
    maximum_deadline: Duration,
) -> Result<AdmittedPageRequest, RejectedBankHttpRequest> {
    let basis = UnadmittedBankHttpRequestBasis {
        protocol: request.protocol,
        request_id: request.request_id,
        account: request.account,
        deadline_milliseconds: request.controls.deadline_milliseconds,
    }
    .admit(maximum_deadline)?;
    Ok(AdmittedPageRequest {
        request_id: basis.request_id,
        credential: request.credential,
        controls: request.controls,
        account: basis.account,
        deadline: basis.deadline,
    })
}

fn response(
    outcome: BankHttpAccountActivityPageOutcome,
) -> (StatusCode, Json<BankHttpAccountActivityPageOutcome>) {
    let status = match &outcome {
        BankHttpAccountActivityPageOutcome::Delivered { .. } => StatusCode::OK,
        BankHttpAccountActivityPageOutcome::Denied { denial, .. } => response_status(denial.kind),
    };
    (status, Json(outcome))
}

fn denied(
    request_id: Option<String>,
    denial: BankHttpDenial,
) -> BankHttpAccountActivityPageOutcome {
    BankHttpAccountActivityPageOutcome::Denied { request_id, denial }
}

const fn malformed() -> BankHttpDenial {
    BankHttpDenial::new(
        BankHttpDenialKind::MalformedRequest,
        BankHttpNextAction::CorrectRequest,
    )
}
