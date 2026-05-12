use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkerBoundaryEnvelopeFamily {
    TransactionSubmission,
    TransactionResult,
    HostCapabilityIngress,
    BrowserHistoryIngress,
    HostEffectEgress,
    OutputDelivery,
    ObservationDelivery,
    DiagnosticsHistoryRead,
    LifecycleControl,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerBoundaryEnvelopeSummary {
    pub label: &'static str,
    pub direction: &'static str,
    pub carries_causality: bool,
    pub requires_worker_readmission: bool,
}

pub(in crate::runtime::worker_bridge) fn worker_boundary_envelope_families(
) -> Vec<WorkerBoundaryEnvelopeSummary> {
    [
        WorkerBoundaryEnvelopeFamily::TransactionSubmission,
        WorkerBoundaryEnvelopeFamily::TransactionResult,
        WorkerBoundaryEnvelopeFamily::HostCapabilityIngress,
        WorkerBoundaryEnvelopeFamily::BrowserHistoryIngress,
        WorkerBoundaryEnvelopeFamily::HostEffectEgress,
        WorkerBoundaryEnvelopeFamily::OutputDelivery,
        WorkerBoundaryEnvelopeFamily::ObservationDelivery,
        WorkerBoundaryEnvelopeFamily::DiagnosticsHistoryRead,
        WorkerBoundaryEnvelopeFamily::LifecycleControl,
    ]
    .into_iter()
    .map(WorkerBoundaryEnvelopeFamily::summary)
    .collect()
}

impl WorkerBoundaryEnvelopeFamily {
    fn summary(self) -> WorkerBoundaryEnvelopeSummary {
        WorkerBoundaryEnvelopeSummary {
            label: self.label(),
            direction: self.direction(),
            carries_causality: true,
            requires_worker_readmission: true,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::TransactionSubmission => "transactionSubmission",
            Self::TransactionResult => "transactionResult",
            Self::HostCapabilityIngress => "hostCapabilityIngress",
            Self::BrowserHistoryIngress => "browserHistoryIngress",
            Self::HostEffectEgress => "hostEffectEgress",
            Self::OutputDelivery => "outputDelivery",
            Self::ObservationDelivery => "observationDelivery",
            Self::DiagnosticsHistoryRead => "diagnosticsHistoryRead",
            Self::LifecycleControl => "lifecycleControl",
        }
    }

    fn direction(self) -> &'static str {
        match self {
            Self::TransactionSubmission
            | Self::HostCapabilityIngress
            | Self::BrowserHistoryIngress => "mainThreadToWorker",
            Self::TransactionResult
            | Self::HostEffectEgress
            | Self::OutputDelivery
            | Self::ObservationDelivery => "workerToMainThread",
            Self::DiagnosticsHistoryRead | Self::LifecycleControl => "bidirectional",
        }
    }
}
