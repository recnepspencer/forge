use axum::extract::{rejection::JsonRejection, State};
use axum::http::StatusCode;
use axum::response::sse::{KeepAlive, Sse};
use axum::response::{IntoResponse, Response};
use axum::Json;
use tokio_stream::StreamExt;

use super::super::protocol::{
    BankHttpAccountActivityStreamRequest, BankHttpDenial, BankHttpDenialKind, BankHttpNextAction,
};
use super::live_executor::AdmittedAccountActivityStreamRequest;
use super::request_admission::UnadmittedBankHttpRequestBasis;
use super::routes::BankHttpRouteState;

pub(super) async fn account_activity_stream(
    State(state): State<BankHttpRouteState>,
    request: Result<Json<BankHttpAccountActivityStreamRequest>, JsonRejection>,
) -> Response {
    let request = match request {
        Ok(Json(request)) => request,
        Err(_) => return denial_response(malformed()),
    };
    let admitted = match admit_request(request, state.maximum_deadline) {
        Ok(request) => request,
        Err(denial) => return denial_response(denial),
    };
    let receiver = match state.live.open(admitted) {
        Ok(receiver) => receiver,
        Err(denial) => return denial_response(denial),
    };
    let stream = receiver.map(|event| {
        axum::response::sse::Event::default()
            .event("bank_account_activity")
            .json_data(event)
    });
    Sse::new(stream)
        .keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(10)))
        .into_response()
}

fn admit_request(
    request: BankHttpAccountActivityStreamRequest,
    maximum_deadline: std::time::Duration,
) -> Result<AdmittedAccountActivityStreamRequest, BankHttpDenial> {
    let basis = UnadmittedBankHttpRequestBasis {
        protocol: request.protocol,
        request_id: request.request_id,
        account: request.account,
        deadline_milliseconds: request.controls.deadline_milliseconds,
    }
    .admit(maximum_deadline)
    .map_err(|rejected| rejected.denial)?;
    if request.source_buffer_capacity == 0 {
        return Err(malformed());
    }
    Ok(AdmittedAccountActivityStreamRequest {
        request_id: basis.request_id,
        credential: request.credential,
        controls: request.controls,
        account: basis.account,
        source_buffer_capacity: request.source_buffer_capacity,
        deadline: basis.deadline,
    })
}

fn denial_response(denial: BankHttpDenial) -> Response {
    let status = match denial.kind {
        BankHttpDenialKind::Saturated | BankHttpDenialKind::ResourceExhausted => {
            StatusCode::TOO_MANY_REQUESTS
        }
        BankHttpDenialKind::Unauthenticated => StatusCode::UNAUTHORIZED,
        BankHttpDenialKind::PermissionDenied => StatusCode::FORBIDDEN,
        BankHttpDenialKind::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::BAD_REQUEST,
    };
    (status, Json(denial)).into_response()
}

const fn malformed() -> BankHttpDenial {
    BankHttpDenial::new(
        BankHttpDenialKind::MalformedRequest,
        BankHttpNextAction::CorrectRequest,
    )
}
