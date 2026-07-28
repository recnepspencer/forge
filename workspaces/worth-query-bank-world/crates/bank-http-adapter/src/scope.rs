use std::future::Future;

use worth_query_host::facade::admission::authenticated_principal::{
    WorthQueryRequestInterruption, WorthQueryRequestScope,
};

pub(crate) async fn await_in_scope<F>(
    scope: &WorthQueryRequestScope,
    future: F,
) -> Result<F::Output, WorthQueryRequestInterruption>
where
    F: Future,
{
    if let Some(interruption) = scope.interruption() {
        return Err(interruption);
    }
    let remaining = scope
        .remaining()
        .ok_or(WorthQueryRequestInterruption::DeadlineExceeded)?;
    tokio::select! {
        biased;
        _ = scope.cancellation().cancelled() => {
            Err(WorthQueryRequestInterruption::Cancelled)
        }
        _ = tokio::time::sleep(remaining) => {
            Err(WorthQueryRequestInterruption::DeadlineExceeded)
        }
        output = future => Ok(output),
    }
}
