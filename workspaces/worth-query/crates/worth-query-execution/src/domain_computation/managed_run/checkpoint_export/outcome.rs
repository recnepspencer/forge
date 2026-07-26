use std::sync::Arc;

use crate::domain_computation::{
    WorthQueryProviderCheckpointEvidence, WorthQueryWorkflowArtifactRegistryEvidence,
};

use super::super::{
    WorthQueryDirectYieldCleanupOutcome, WorthQueryWorkflowYieldCleanupOutcome,
    WorthQueryYieldedDirectRun, WorthQueryYieldedWorkflowRun,
};
use super::WorthQueryCheckpointExportHandoff;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCheckpointExportRecoveryKind {
    ProviderExportPanicked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryCheckpointExportRecoveryPosture {
    TerminalCleanupRequired,
}

#[must_use = "checkpoint export outcomes retain yielded-run or cleanup authority"]
pub enum WorthQueryDirectCheckpointExportOutcome {
    Exported(WorthQueryDirectCheckpointExported),
    Failed(WorthQueryDirectCheckpointExportFailed),
    RecoveryRequired(WorthQueryDirectCheckpointExportRecoveryRequired),
}

pub struct WorthQueryDirectCheckpointExported {
    pub(super) handoff: WorthQueryCheckpointExportHandoff,
    pub(super) yielded: WorthQueryYieldedDirectRun,
}

pub struct WorthQueryDirectCheckpointExportFailed {
    pub(super) detail: Arc<str>,
    pub(super) yielded: WorthQueryYieldedDirectRun,
}

pub struct WorthQueryDirectCheckpointExportRecoveryRequired {
    pub(super) kind: WorthQueryCheckpointExportRecoveryKind,
    pub(super) detail: Arc<str>,
    pub(super) yielded: WorthQueryYieldedDirectRun,
}

#[must_use = "checkpoint export outcomes retain yielded-run or cleanup authority"]
pub enum WorthQueryWorkflowCheckpointExportOutcome {
    Exported(WorthQueryWorkflowCheckpointExported),
    Failed(WorthQueryWorkflowCheckpointExportFailed),
    RecoveryRequired(WorthQueryWorkflowCheckpointExportRecoveryRequired),
}

pub struct WorthQueryWorkflowCheckpointExported {
    pub(super) handoff: WorthQueryCheckpointExportHandoff,
    pub(super) yielded: WorthQueryYieldedWorkflowRun,
}

pub struct WorthQueryWorkflowCheckpointExportFailed {
    pub(super) detail: Arc<str>,
    pub(super) yielded: WorthQueryYieldedWorkflowRun,
}

pub struct WorthQueryWorkflowCheckpointExportRecoveryRequired {
    pub(super) kind: WorthQueryCheckpointExportRecoveryKind,
    pub(super) detail: Arc<str>,
    pub(super) yielded: WorthQueryYieldedWorkflowRun,
}

impl WorthQueryDirectCheckpointExported {
    pub fn handoff(&self) -> &WorthQueryCheckpointExportHandoff {
        &self.handoff
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryCheckpointExportHandoff,
        WorthQueryYieldedDirectRun,
    ) {
        (self.handoff, self.yielded)
    }
}

impl WorthQueryDirectCheckpointExportFailed {
    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn into_yielded(self) -> WorthQueryYieldedDirectRun {
        self.yielded
    }
}

impl WorthQueryDirectCheckpointExportRecoveryRequired {
    pub const fn kind(&self) -> WorthQueryCheckpointExportRecoveryKind {
        self.kind
    }

    pub const fn posture(&self) -> WorthQueryCheckpointExportRecoveryPosture {
        WorthQueryCheckpointExportRecoveryPosture::TerminalCleanupRequired
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn checkpoint(&self) -> &WorthQueryProviderCheckpointEvidence {
        self.yielded.checkpoint()
    }

    pub fn cleanup(self) -> WorthQueryDirectYieldCleanupOutcome {
        self.yielded.cleanup()
    }
}

impl WorthQueryWorkflowCheckpointExported {
    pub fn handoff(&self) -> &WorthQueryCheckpointExportHandoff {
        &self.handoff
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryCheckpointExportHandoff,
        WorthQueryYieldedWorkflowRun,
    ) {
        (self.handoff, self.yielded)
    }
}

impl WorthQueryWorkflowCheckpointExportFailed {
    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn into_yielded(self) -> WorthQueryYieldedWorkflowRun {
        self.yielded
    }
}

impl WorthQueryWorkflowCheckpointExportRecoveryRequired {
    pub const fn kind(&self) -> WorthQueryCheckpointExportRecoveryKind {
        self.kind
    }

    pub const fn posture(&self) -> WorthQueryCheckpointExportRecoveryPosture {
        WorthQueryCheckpointExportRecoveryPosture::TerminalCleanupRequired
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn checkpoint(&self) -> &WorthQueryProviderCheckpointEvidence {
        self.yielded.checkpoint()
    }

    pub const fn artifact_evidence(&self) -> WorthQueryWorkflowArtifactRegistryEvidence {
        self.yielded.artifact_evidence()
    }

    pub fn cleanup(self) -> WorthQueryWorkflowYieldCleanupOutcome {
        self.yielded.cleanup()
    }
}
