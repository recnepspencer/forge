use worth_signal::facade::adapters::RuntimeTelemetry;

use super::{
    counters::UiNativePhysicalSignalCounters, identity::UiNativePhysicalSignalRuntimeIdentity,
    wake_delivery::UiNativePhysicalSignalWakeDelivery,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativePhysicalRecoveryPosture {
    None,
    Required { active_requests: usize },
    Resolved { total_resolutions: u64 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativePhysicalSignalObservation {
    pub(crate) runtime: UiNativePhysicalSignalRuntimeIdentity,
    pub(crate) active_requests: usize,
    pub(crate) pending_wakes: usize,
    pub(crate) counters: UiNativePhysicalSignalCounters,
    pub(crate) recovery: UiNativePhysicalRecoveryPosture,
    pub(crate) runtime_owned: bool,
    pub(crate) accepting_admissions: bool,
    pub(crate) signal_request_admissions: u64,
    pub(crate) signal_completion_commits: u64,
    pub(crate) signal_evaluation_calls: u64,
    pub(crate) signal_nodes_evaluated: u64,
    pub(crate) signal_in_flight_requests: u64,
    pub(crate) signal_performed_transitions: u64,
    pub(crate) signal_performed_nodes: u64,
    pub(crate) performed_request_sequence: Option<u64>,
    pub(crate) performed_fact_revision: Option<u64>,
    pub(crate) performed_read_scopes: u8,
}

pub(super) struct UiNativePhysicalSignalObservationInput<'owner> {
    pub(super) runtime: UiNativePhysicalSignalRuntimeIdentity,
    pub(super) active_requests: usize,
    pub(super) wake: &'owner UiNativePhysicalSignalWakeDelivery,
    pub(super) counters: UiNativePhysicalSignalCounters,
    pub(super) runtime_owned: bool,
    pub(super) accepting_admissions: bool,
    pub(super) active_recoveries: usize,
    pub(super) telemetry: RuntimeTelemetry,
    pub(super) performed_transitions: u64,
    pub(super) performed_nodes: u64,
    pub(super) last_performed: Option<super::worker_graph::UiNativePhysicalSignalPerformed>,
}

impl UiNativePhysicalSignalObservation {
    pub(super) fn from_parts(input: UiNativePhysicalSignalObservationInput<'_>) -> Self {
        let recovery = if input.active_recoveries != 0 {
            UiNativePhysicalRecoveryPosture::Required {
                active_requests: input.active_recoveries,
            }
        } else if input.counters.recovery_resolutions != 0 {
            UiNativePhysicalRecoveryPosture::Resolved {
                total_resolutions: input.counters.recovery_resolutions,
            }
        } else {
            UiNativePhysicalRecoveryPosture::None
        };
        Self {
            runtime: input.runtime,
            active_requests: input.active_requests,
            pending_wakes: input.wake.pending(),
            counters: input.counters,
            recovery,
            runtime_owned: input.runtime_owned,
            accepting_admissions: input.accepting_admissions,
            signal_request_admissions: input.telemetry.resource.resource_request_admission_count,
            signal_completion_commits: input.telemetry.resource.resource_completion_commit_count,
            signal_evaluation_calls: input.telemetry.evaluation.evaluation_calls,
            signal_nodes_evaluated: input.telemetry.evaluation.nodes_evaluated,
            signal_in_flight_requests: input.telemetry.resource.resource_in_flight_request_count,
            signal_performed_transitions: input.performed_transitions,
            signal_performed_nodes: input.performed_nodes,
            performed_request_sequence: input
                .last_performed
                .map(|performed| performed.work().sequence()),
            performed_fact_revision: input
                .last_performed
                .map(super::worker_graph::UiNativePhysicalSignalPerformed::fact_revision),
            performed_read_scopes: input.last_performed.map_or(
                0,
                super::worker_graph::UiNativePhysicalSignalPerformed::read_scopes,
            ),
        }
    }
}
