mod admitted_counters;
mod amplification_receipt;
mod attempt_cost;
mod counter_snapshot;
#[cfg(test)]
pub(crate) mod counter_tests;
pub(crate) mod counter_witness;
mod denial;
mod executed_evidence;
mod facade;
mod freshness;
mod lowering_request;
mod observed_counters;
mod outcomes;
mod path_kind;
mod performance_receipt;
mod planned_vs_observed;
#[cfg(test)]
pub(crate) mod progression_tests;
mod ready_plan;
#[cfg(test)]
pub(crate) mod tests_support;
mod violation;

pub use admitted_counters::S8AdmittedExecutedCounters;
pub use amplification_receipt::S8AccessPathAmplificationReceipt;
pub use attempt_cost::S8AccessAttemptCostReceipt;
pub use counter_snapshot::S8AccessPathCounterSnapshot;
pub use counter_witness::S8ExecutedCounterWitness;
pub use denial::{S8AccessLoweringDeferred, S8AccessLoweringDenied};
pub use executed_evidence::S8ExecutedAccessReceipt;
pub use facade::access_lowering;
pub(crate) use facade::AccessLoweringFacade;
pub use freshness::{S8ExecutionReadmissionWitness, S8ExecutionRebindWitness};
pub use lowered_plan::{
    S8AccessLoweringBasis, S8LoweredAccessPayload, S8LoweredAccessReceipt,
    S8RebindRequiredAccessReceipt, S8StaleLoweredAccessReceipt,
};
pub use observed_counters::S8ObservedAccessPathCounters;
pub use outcomes::{
    S8AccessLoweringOutcome, S8AccessLoweringView, S8ExecutedCounterAdmissionOutcome,
    S8ExecutedCounterAdmissionView, S8ExecutedEvidenceOutcome, S8ExecutedEvidenceView,
    S8ExecutionReadinessOutcome, S8ExecutionReadinessView, S8StaleReadmissionOutcome,
    S8StaleReadmissionView,
};
pub use path_kind::S8AccessPathKind;
pub use performance_receipt::S8StoreLayoutPerformanceReceipt;
pub use planned_vs_observed::S8PlannedVsObservedCounterReceipt;
pub use ready_plan::S8ExecutionReadyAccessReceipt;
pub use violation::{S8CostEnvelopeViolationOutcome, S8ObservedCounterMetric};

mod lowered_plan;
