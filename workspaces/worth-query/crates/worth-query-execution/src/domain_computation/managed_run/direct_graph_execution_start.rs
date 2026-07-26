use std::sync::Arc;

use crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor;

use super::managed_graph_execution::WorthQueryManagedGraphExecution;
use super::{
    WorthQueryActiveDirectGraphExecution, WorthQueryManagedGraphCallRequest,
    WorthQueryRunningDirectRun,
};
use crate::domain_computation::WorthQueryGraphProviderCall;

struct WorthQueryBoundDirectGraphStart {
    running: WorthQueryRunningDirectRun,
    anchor: Arc<WorthQueryGraphProviderAnchor>,
    call: WorthQueryGraphProviderCall,
}

struct WorthQueryReadyDirectGraphStart {
    bound: WorthQueryBoundDirectGraphStart,
    contract: super::step_contract_admission::WorthQueryAdmittedManagedStepContract,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDirectGraphExecutionStartFailureKind {
    GraphCallBinding,
    MissingInstalledProvider,
    ProviderSupportMismatch,
    ProviderStart,
    ProviderStartPanicked,
    ProviderStartContractDenied,
    ProviderStartMemoryLeaked,
    ProviderStartReleaseRecoveryRequired,
    InvalidStepContract,
    StepContract(super::WorthQueryManagedStepContractDenialKind),
}

pub struct WorthQueryDirectGraphExecutionStartFailure {
    kind: WorthQueryDirectGraphExecutionStartFailureKind,
    detail: Arc<str>,
    running: WorthQueryRunningDirectRun,
    provider_retained_bytes: u64,
    provider_retained_allocation_count: u64,
    provider_execution_release:
        Option<crate::domain_computation::WorthQueryProviderExecutionReleaseEvidence>,
}

impl WorthQueryDirectGraphExecutionStartFailure {
    pub const fn kind(&self) -> WorthQueryDirectGraphExecutionStartFailureKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn into_running(self) -> WorthQueryRunningDirectRun {
        self.running
    }

    pub const fn provider_retained_bytes(&self) -> u64 {
        self.provider_retained_bytes
    }

    pub const fn provider_retained_allocation_count(&self) -> u64 {
        self.provider_retained_allocation_count
    }

    pub const fn provider_execution_release(
        &self,
    ) -> Option<&crate::domain_computation::WorthQueryProviderExecutionReleaseEvidence> {
        self.provider_execution_release.as_ref()
    }
}

impl std::fmt::Debug for WorthQueryDirectGraphExecutionStartFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryDirectGraphExecutionStartFailure")
            .field("kind", &self.kind)
            .field("detail", &self.detail)
            .field("run_identity", &self.running.identity())
            .finish()
    }
}

pub(super) fn begin(
    running: WorthQueryRunningDirectRun,
    graph_authority: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
    request: WorthQueryManagedGraphCallRequest,
) -> Result<WorthQueryActiveDirectGraphExecution, WorthQueryDirectGraphExecutionStartFailure> {
    WorthQueryBoundDirectGraphStart::bind(running, graph_authority, request)?
        .validate_contract()?
        .start_provider()
}

impl WorthQueryBoundDirectGraphStart {
    fn bind(
        running: WorthQueryRunningDirectRun,
        graph_authority: &worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority,
        request: WorthQueryManagedGraphCallRequest,
    ) -> Result<Self, WorthQueryDirectGraphExecutionStartFailure> {
        let Some(anchor) =
            graph_authority.retain_provider_anchor::<WorthQueryGraphProviderAnchor>()
        else {
            return Err(start_failure(
                WorthQueryDirectGraphExecutionStartFailureKind::MissingInstalledProvider,
                "installed graph authority does not retain the exact execution-owned provider anchor",
                running,
            ));
        };
        if running.graph_resource_support(graph_authority.role()) != Some(anchor.resource_support())
        {
            return Err(start_failure(
                WorthQueryDirectGraphExecutionStartFailureKind::ProviderSupportMismatch,
                "admitted graph support was not minted from the installed provider's exact support authority",
                running,
            ));
        }
        let call = match running.mint_graph_provider_call(graph_authority, request) {
            Ok(call) => call,
            Err(denial) => {
                return Err(start_failure(
                    WorthQueryDirectGraphExecutionStartFailureKind::GraphCallBinding,
                    format!("{denial:?}"),
                    running,
                ))
            }
        };
        Ok(Self {
            running,
            anchor,
            call,
        })
    }

    fn validate_contract(
        self,
    ) -> Result<WorthQueryReadyDirectGraphStart, WorthQueryDirectGraphExecutionStartFailure> {
        let contract = match self.call.resource_envelope().bounded_step_contract() {
            Ok(contract) => contract,
            Err(detail) => {
                return Err(start_failure(
                    WorthQueryDirectGraphExecutionStartFailureKind::InvalidStepContract,
                    detail,
                    self.running,
                ))
            }
        };
        let contract = match super::step_contract_admission::admit_managed_step_contract(
            contract,
            self.running.bridge_basis().step_contract(),
        ) {
            Ok(contract) => contract,
            Err(denial) => return Err(step_contract_failure(denial, self.running)),
        };
        Ok(WorthQueryReadyDirectGraphStart {
            bound: self,
            contract,
        })
    }
}

impl WorthQueryReadyDirectGraphStart {
    fn start_provider(
        mut self,
    ) -> Result<WorthQueryActiveDirectGraphExecution, WorthQueryDirectGraphExecutionStartFailure>
    {
        self.bound.running.provider_work_mut().begin_step_call();
        let started = match super::provider_start::start_managed_provider(
            &self.bound.anchor,
            &self.bound.call,
            self.contract.installed().retained_bytes_ceiling(),
        ) {
            Ok(started) => started,
            Err(failure) => {
                if let Some(release) = &failure.provider_execution_release {
                    self.bound
                        .running
                        .provider_work_mut()
                        .record_provider_execution_release(release);
                }
                self.bound.running.provider_work_mut().abandon();
                return Err(provider_start_failure(
                    failure,
                    self.bound.running,
                ));
            }
        };
        Ok(WorthQueryActiveDirectGraphExecution::new(
            self.bound.running,
            WorthQueryManagedGraphExecution::new(
                self.bound.call,
                started.execution,
                self.bound.anchor,
                self.contract,
                None,
                started.memory,
            ),
        ))
    }
}

fn start_failure(
    kind: WorthQueryDirectGraphExecutionStartFailureKind,
    detail: impl Into<Arc<str>>,
    running: WorthQueryRunningDirectRun,
) -> WorthQueryDirectGraphExecutionStartFailure {
    WorthQueryDirectGraphExecutionStartFailure {
        kind,
        detail: detail.into(),
        running,
        provider_retained_bytes: 0,
        provider_retained_allocation_count: 0,
        provider_execution_release: None,
    }
}

fn provider_start_failure(
    failure: super::provider_start::WorthQueryManagedProviderStartFailure,
    running: WorthQueryRunningDirectRun,
) -> WorthQueryDirectGraphExecutionStartFailure {
    use super::provider_start::WorthQueryManagedProviderStartFailureKind as Kind;
    let kind = match failure.kind {
        Kind::Rejected => WorthQueryDirectGraphExecutionStartFailureKind::ProviderStart,
        Kind::Panicked => WorthQueryDirectGraphExecutionStartFailureKind::ProviderStartPanicked,
        Kind::ContractDenied => {
            WorthQueryDirectGraphExecutionStartFailureKind::ProviderStartContractDenied
        }
        Kind::MemoryLeaked => {
            WorthQueryDirectGraphExecutionStartFailureKind::ProviderStartMemoryLeaked
        }
        Kind::ProviderExecutionReleaseRecoveryRequired => {
            WorthQueryDirectGraphExecutionStartFailureKind::ProviderStartReleaseRecoveryRequired
        }
    };
    WorthQueryDirectGraphExecutionStartFailure {
        kind,
        detail: failure.detail,
        running,
        provider_retained_bytes: failure.memory.retained_bytes(),
        provider_retained_allocation_count: failure.memory.retained_allocation_count(),
        provider_execution_release: failure.provider_execution_release,
    }
}

fn step_contract_failure(
    denial: super::step_contract_admission::WorthQueryManagedStepContractDenial,
    running: WorthQueryRunningDirectRun,
) -> WorthQueryDirectGraphExecutionStartFailure {
    WorthQueryDirectGraphExecutionStartFailure {
        kind: WorthQueryDirectGraphExecutionStartFailureKind::StepContract(denial.kind()),
        detail: Arc::from(denial.detail()),
        running,
        provider_retained_bytes: 0,
        provider_retained_allocation_count: 0,
        provider_execution_release: None,
    }
}
