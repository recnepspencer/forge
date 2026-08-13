use crate::publication::cdc::data::{
    SubscriberResumeRequest, SubscriberStreamBatch, SubscriberStreamFailure,
};
use crate::publication::cdc::execution::execute_subscriber_stream;
use crate::publication::cdc::planning::plan_subscriber_recovery;
use crate::runtime::RelationalRuntime;

pub(crate) fn read_subscriber_stream(
    runtime: &RelationalRuntime,
    request: SubscriberResumeRequest,
) -> Result<SubscriberStreamBatch, SubscriberStreamFailure> {
    let (plan, diagnostics) = plan_subscriber_recovery(runtime, request)?;
    execute_subscriber_stream(runtime, plan, diagnostics)
}
