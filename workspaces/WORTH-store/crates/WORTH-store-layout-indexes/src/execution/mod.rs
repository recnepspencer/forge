mod admitted_counters;
mod amplification_receipt;
mod attempt_cost;
mod counter_snapshot;
pub(crate) mod counter_witness;
mod denial;
mod executed_evidence;
mod facade;
mod freshness;
mod lowering_request;
mod observed_counters;
mod path_kind;
mod performance_receipt;
mod planned_vs_observed;
mod violation;
mod ready_plan;
#[cfg(test)]
mod counter_tests;
#[cfg(test)]
mod progression_tests;
#[cfg(test)]
mod tests_support;

pub use counter_snapshot::S8AccessPathCounterSnapshot;
pub use attempt_cost::S8AccessAttemptCostReceipt;
pub use admitted_counters::S8AdmittedExecutedCounters;
pub use denial::{S8AccessLoweringDeferred, S8AccessLoweringDenied};
pub use amplification_receipt::S8AccessPathAmplificationReceipt;
pub use counter_witness::S8ExecutedCounterWitness;
pub use executed_evidence::S8ExecutedAccessReceipt;
pub use facade::{access_lowering, S8AccessLoweringOutcome};
pub use freshness::{S8ExecutionReadmissionWitness, S8ExecutionRebindWitness};
pub use observed_counters::S8ObservedAccessPathCounters;
pub use lowered_plan::{
    S8AccessLoweringBasis, S8LoweredAccessPayload, S8LoweredAccessReceipt,
    S8RebindRequiredAccessReceipt, S8StaleLoweredAccessReceipt,
};
pub use path_kind::S8AccessPathKind;
pub use performance_receipt::S8StoreLayoutPerformanceReceipt;
pub use planned_vs_observed::S8PlannedVsObservedCounterReceipt;
pub use violation::{S8CostEnvelopeViolationOutcome, S8ObservedCounterMetric};
pub use ready_plan::S8ExecutionReadyAccessReceipt;

mod lowered_plan;
