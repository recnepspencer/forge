use super::managed_graph_execution::WorthQueryManagedGraphExecution;
use crate::domain_computation::provider_session::graph_provider::bounded_step::WorthQueryGraphProviderMemorySnapshot;
use crate::domain_computation::WorthQueryProviderExecutionReleaseEvidence;

pub(super) struct WorthQueryManagedProviderExecutionRelease {
    evidence: WorthQueryProviderExecutionReleaseEvidence,
    memory: WorthQueryGraphProviderMemorySnapshot,
}

impl WorthQueryManagedGraphExecution {
    pub(super) fn release_provider_execution(self) -> WorthQueryManagedProviderExecutionRelease {
        let memory = self.memory.clone();
        let evidence = self.execution.release();
        WorthQueryManagedProviderExecutionRelease {
            evidence,
            memory: memory.snapshot(),
        }
    }
}

impl WorthQueryManagedProviderExecutionRelease {
    pub(super) const fn evidence(&self) -> &WorthQueryProviderExecutionReleaseEvidence {
        &self.evidence
    }

    pub(super) const fn memory(&self) -> WorthQueryGraphProviderMemorySnapshot {
        self.memory
    }
}
