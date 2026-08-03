mod allocation_pressure;
mod cancellation;
mod checkpoint;
pub(super) mod configuration;
mod generation_fencing;
mod producer;
mod protocol;
mod read_pressure;
pub(crate) mod schedule;
mod serving;
mod speculative_pressure;
mod work_reconciliation;
mod workload;
mod writeback_pressure;

pub(super) fn produce(
    invocation: super::arguments::BoundedResidencyProducerInvocation,
) -> Result<(), String> {
    producer::run(invocation)
}

pub(super) fn serve(
    invocation: super::arguments::BoundedResidencyServingInvocation,
) -> Result<(), String> {
    serving::run(invocation)
}
