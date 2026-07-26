use std::sync::Arc;

use worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority;

use super::core::WorthQueryConvergenceEpochCore;
use super::{WorthQueryConvergenceDomainProvider, WorthQueryConvergenceEpochCounters};
use crate::domain_computation::{
    WorthQueryAdmittedDirectRun, WorthQueryAdmittedWorkflowRun, WorthQueryRunningDirectRun,
    WorthQueryRunningWorkflowRun, WorthQueryWorkflowRunStartRejection,
};

pub struct WorthQueryAdmittedDirectConvergenceEpoch {
    core: WorthQueryConvergenceEpochCore,
    managed_run: WorthQueryAdmittedDirectRun,
    graph: WorthQueryInstalledGraphParticipationAuthority,
    provider: Arc<dyn WorthQueryConvergenceDomainProvider>,
}

impl WorthQueryAdmittedDirectConvergenceEpoch {
    pub(super) fn new(
        core: WorthQueryConvergenceEpochCore,
        managed_run: WorthQueryAdmittedDirectRun,
        graph: WorthQueryInstalledGraphParticipationAuthority,
        provider: Arc<dyn WorthQueryConvergenceDomainProvider>,
    ) -> Self {
        Self {
            core,
            managed_run,
            graph,
            provider,
        }
    }

    pub fn identity(&self) -> &str {
        self.core.identity()
    }

    pub fn counters(&self) -> &WorthQueryConvergenceEpochCounters {
        self.core.counters()
    }

    pub fn start(self) -> WorthQueryIteratingDirectConvergenceEpoch {
        WorthQueryIteratingDirectConvergenceEpoch {
            core: self.core,
            managed_run: self.managed_run.start(),
            graph: self.graph,
            provider: self.provider,
        }
    }
}

pub struct WorthQueryIteratingDirectConvergenceEpoch {
    pub(super) core: WorthQueryConvergenceEpochCore,
    pub(super) managed_run: WorthQueryRunningDirectRun,
    pub(super) graph: WorthQueryInstalledGraphParticipationAuthority,
    pub(super) provider: Arc<dyn WorthQueryConvergenceDomainProvider>,
}

impl WorthQueryIteratingDirectConvergenceEpoch {
    pub fn identity(&self) -> &str {
        self.core.identity()
    }

    pub fn logical_run_identity(&self) -> &str {
        self.core.logical_run_identity()
    }

    pub fn counters(&self) -> &WorthQueryConvergenceEpochCounters {
        self.core.counters()
    }
}

pub struct WorthQueryAdmittedWorkflowConvergenceEpoch {
    core: WorthQueryConvergenceEpochCore,
    managed_run: WorthQueryAdmittedWorkflowRun,
    graph: WorthQueryInstalledGraphParticipationAuthority,
    provider: Arc<dyn WorthQueryConvergenceDomainProvider>,
}

impl WorthQueryAdmittedWorkflowConvergenceEpoch {
    pub(super) fn new(
        core: WorthQueryConvergenceEpochCore,
        managed_run: WorthQueryAdmittedWorkflowRun,
        graph: WorthQueryInstalledGraphParticipationAuthority,
        provider: Arc<dyn WorthQueryConvergenceDomainProvider>,
    ) -> Self {
        Self {
            core,
            managed_run,
            graph,
            provider,
        }
    }

    pub fn identity(&self) -> &str {
        self.core.identity()
    }

    pub fn counters(&self) -> &WorthQueryConvergenceEpochCounters {
        self.core.counters()
    }

    pub fn start(
        self,
    ) -> Result<
        WorthQueryIteratingWorkflowConvergenceEpoch,
        WorthQueryWorkflowConvergenceStartRejection,
    > {
        match self.managed_run.start() {
            Ok(managed_run) => Ok(WorthQueryIteratingWorkflowConvergenceEpoch {
                core: self.core,
                managed_run,
                graph: self.graph,
                provider: self.provider,
            }),
            Err(rejection) => Err(WorthQueryWorkflowConvergenceStartRejection {
                core: self.core,
                rejection,
                graph: self.graph,
                provider: self.provider,
            }),
        }
    }
}

pub struct WorthQueryWorkflowConvergenceStartRejection {
    core: WorthQueryConvergenceEpochCore,
    rejection: WorthQueryWorkflowRunStartRejection,
    graph: WorthQueryInstalledGraphParticipationAuthority,
    provider: Arc<dyn WorthQueryConvergenceDomainProvider>,
}

impl WorthQueryWorkflowConvergenceStartRejection {
    pub fn managed_run_rejection(&self) -> &WorthQueryWorkflowRunStartRejection {
        &self.rejection
    }

    pub fn into_admitted(self) -> WorthQueryAdmittedWorkflowConvergenceEpoch {
        WorthQueryAdmittedWorkflowConvergenceEpoch {
            core: self.core,
            managed_run: self.rejection.into_admitted(),
            graph: self.graph,
            provider: self.provider,
        }
    }
}

pub struct WorthQueryIteratingWorkflowConvergenceEpoch {
    pub(super) core: WorthQueryConvergenceEpochCore,
    pub(super) managed_run: WorthQueryRunningWorkflowRun,
    pub(super) graph: WorthQueryInstalledGraphParticipationAuthority,
    pub(super) provider: Arc<dyn WorthQueryConvergenceDomainProvider>,
}

impl WorthQueryIteratingWorkflowConvergenceEpoch {
    pub fn identity(&self) -> &str {
        self.core.identity()
    }

    pub fn logical_run_identity(&self) -> &str {
        self.core.logical_run_identity()
    }

    pub fn counters(&self) -> &WorthQueryConvergenceEpochCounters {
        self.core.counters()
    }
}
