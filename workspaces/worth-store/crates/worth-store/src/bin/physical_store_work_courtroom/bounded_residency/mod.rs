use std::path::Path;

mod allocation_pressure;
mod cancellation;
mod configuration;
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

pub(super) fn validate_configuration(path: &Path) -> Result<(), String> {
    configuration::BoundedResidencyConfiguration::read(path).map(|_| ())
}

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
