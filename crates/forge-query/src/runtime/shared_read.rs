use std::sync::Arc;

use crate::authorized_projection::AuthorizedProjectionArtifact;
use crate::canonicalization::CanonicalResultShapeArtifact;
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::projection_consumption::{
    ProjectMaterializedFacts, ProjectionFactConsumptionAttempt, ProjectionFactConsumptionPathError,
};

use super::published_artifacts::{
    ForgeQueryPublishedArtifactDiagnostics, ForgeQueryPublishedArtifactEntry,
    ForgeQueryPublishedArtifactRegistry, ForgeQueryPublishedArtifactResolution,
};
use super::{
    ForgeQueryDerivedArtifactBinding, ForgeQueryDerivedMaterializationTarget,
    ForgeQueryDerivedViewHandle, ForgeQueryRuntime, ForgeQueryRuntimeAsyncResultState,
    ForgeQueryRuntimeError, ForgeQuerySharedReadGenerationLease,
    ForgeQuerySharedReadPinningDiagnostics,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryPublishedProjectionConsumption {
    Current(ProjectionFactConsumptionAttempt),
    ResultState(ForgeQueryRuntimeAsyncResultState),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPublishedProjectionInspection {
    published: bool,
    artifact_binding_identity: Option<ForgeQueryEvidenceIdentity>,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    snapshot_evidence_identity: ForgeQueryEvidenceIdentity,
    async_result_state: Option<ForgeQueryRuntimeAsyncResultState>,
}

impl ForgeQueryPublishedProjectionInspection {
    pub fn published(&self) -> bool {
        self.published
    }

    pub fn artifact_binding_identity(&self) -> Option<&ForgeQueryEvidenceIdentity> {
        self.artifact_binding_identity.as_ref()
    }

    pub fn artifact_binding_for_reporting(&self) -> Option<&str> {
        self.artifact_binding_identity
            .as_ref()
            .map(ForgeQueryEvidenceIdentity::as_str)
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn snapshot_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.snapshot_evidence_identity
    }

    pub fn async_result_state(&self) -> Option<&ForgeQueryRuntimeAsyncResultState> {
        self.async_result_state.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryPublishedDerivedArtifactHandle {
    lease: ForgeQuerySharedReadGenerationLease,
    view_name: String,
    published_binding: Option<Arc<ForgeQueryDerivedArtifactBinding>>,
    async_result_state: Option<ForgeQueryRuntimeAsyncResultState>,
}

impl ForgeQueryPublishedDerivedArtifactHandle {
    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        self.lease.generation().snapshot_identity()
    }

    pub fn published_binding(&self) -> Option<&ForgeQueryDerivedArtifactBinding> {
        self.published_binding.as_deref()
    }

    pub fn async_result_state(&self) -> Option<&ForgeQueryRuntimeAsyncResultState> {
        self.async_result_state.as_ref()
    }

    pub fn inspect_projection_consumption(&self) -> ForgeQueryPublishedProjectionInspection {
        ForgeQueryPublishedProjectionInspection {
            published: self.published_binding.is_some(),
            artifact_binding_identity: self
                .published_binding
                .as_ref()
                .map(|binding| binding.binding_identity().clone()),
            snapshot_identity: self.snapshot_identity().clone(),
            snapshot_evidence_identity: self.snapshot_identity().evidence_identity(),
            async_result_state: self.async_result_state.clone(),
        }
    }

    pub fn consume_projection_facts(
        &self,
        result_shape: &CanonicalResultShapeArtifact,
        authorized_projection: &AuthorizedProjectionArtifact,
        requested: ProjectMaterializedFacts,
    ) -> Result<ForgeQueryPublishedProjectionConsumption, ProjectionFactConsumptionPathError> {
        match &self.published_binding {
            Some(binding) => Ok(ForgeQueryPublishedProjectionConsumption::Current(
                binding.consume_projection_facts(result_shape, authorized_projection, requested)?,
            )),
            None => Ok(ForgeQueryPublishedProjectionConsumption::ResultState(
                self.async_result_state
                    .clone()
                    .expect("unpublished shared-read artifact must retain typed async posture"),
            )),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySharedReadBasisInspection {
    generation_ordinal: u64,
    snapshot_identity: ForgeQuerySnapshotIdentity,
    snapshot_evidence_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQuerySharedReadBasisInspection {
    pub(in crate::runtime) fn from_lease(lease: &ForgeQuerySharedReadGenerationLease) -> Self {
        let generation = lease.generation();
        Self {
            generation_ordinal: generation.ordinal(),
            snapshot_identity: generation.snapshot_identity().clone(),
            snapshot_evidence_identity: generation.snapshot_identity().evidence_identity(),
        }
    }

    pub fn generation_ordinal(&self) -> u64 {
        self.generation_ordinal
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn snapshot_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.snapshot_evidence_identity
    }
}

#[derive(Clone, Debug)]
pub struct ForgeQuerySharedReadContext {
    lease: ForgeQuerySharedReadGenerationLease,
    published_artifacts: ForgeQueryPublishedArtifactRegistry,
}

impl ForgeQuerySharedReadContext {
    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        self.lease.generation().snapshot_identity()
    }

    pub fn inspect_basis(&self) -> ForgeQuerySharedReadBasisInspection {
        ForgeQuerySharedReadBasisInspection::from_lease(&self.lease)
    }

    pub fn published_derived_artifact<T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> Result<ForgeQueryPublishedDerivedArtifactHandle, ForgeQueryRuntimeError> {
        if !self.lease.is_generation_live() {
            return Err(super::forge_query_shared_read_stale_basis_error(
                self.snapshot_identity().clone(),
            ));
        }
        let target = ForgeQueryDerivedMaterializationTarget::from(view);
        match self
            .published_artifacts
            .resolve(self.lease.generation(), &target)
        {
            ForgeQueryPublishedArtifactResolution::Published {
                binding,
                async_result_state,
            } => Ok(ForgeQueryPublishedDerivedArtifactHandle {
                lease: self.lease.clone(),
                view_name: target.terminal_view_name_projection().to_string(),
                published_binding: Some(binding),
                async_result_state,
            }),
            ForgeQueryPublishedArtifactResolution::Unpublished { async_result_state } => {
                Ok(ForgeQueryPublishedDerivedArtifactHandle {
                    lease: self.lease.clone(),
                    view_name: target.terminal_view_name_projection().to_string(),
                    published_binding: None,
                    async_result_state: Some(async_result_state),
                })
            }
            ForgeQueryPublishedArtifactResolution::MissingGeneration => Err(
                super::forge_query_shared_read_stale_basis_error(self.snapshot_identity().clone()),
            ),
            ForgeQueryPublishedArtifactResolution::MissingView => Err(
                ForgeQueryRuntimeError::MissingDerivedView(view.name().to_string()),
            ),
        }
    }

    #[allow(dead_code)]
    pub(in crate::runtime) fn from_runtime(runtime: &ForgeQueryRuntime) -> Self {
        runtime
            .mint_shared_read_context()
            .expect("runtime-owned shared read context should mint")
    }
}

impl ForgeQueryRuntime {
    pub(in crate::runtime) fn capture_shared_read_generation(
        &mut self,
        snapshot_identity: ForgeQuerySnapshotIdentity,
    ) {
        self.publish_shared_read_generation(snapshot_identity);
    }

    pub(in crate::runtime) fn mint_shared_read_context(
        &self,
    ) -> Result<ForgeQuerySharedReadContext, ForgeQueryRuntimeError> {
        let snapshot_identity = self.current_snapshot_identity();
        if !self.shared_read_pins.has_current_generation() {
            self.capture_shared_read_generation_for_current_snapshot(snapshot_identity);
        }
        let lease = self
            .shared_read_pins
            .pin_current_generation()
            .ok_or_else(|| {
                ForgeQueryRuntimeError::Workspace(
                    crate::memory_workspace::ForgeQueryWorkspaceError::new(
                        "shared read context requires a committed generation",
                    ),
                )
            })?;
        Ok(ForgeQuerySharedReadContext {
            lease,
            published_artifacts: self.published_artifacts.clone(),
        })
    }

    #[allow(dead_code)]
    pub fn shared_read_counters(&self) -> super::ForgeQuerySharedReadCounters {
        self.shared_read_pins
            .counters()
            .with_published_artifacts(self.published_artifacts.counters().snapshot())
    }

    pub fn shared_read_pinning_diagnostics(&self) -> ForgeQuerySharedReadPinningDiagnostics {
        self.shared_read_pins.diagnostics()
    }

    pub fn published_artifact_diagnostics(&self) -> ForgeQueryPublishedArtifactDiagnostics {
        self.published_artifacts.diagnostics()
    }

    pub fn invalidate_shared_read_snapshot_for_certification(
        &self,
        snapshot_identity: &ForgeQuerySnapshotIdentity,
    ) {
        self.shared_read_pins
            .force_retire_snapshot_identity(snapshot_identity);
    }

    pub fn record_shared_read_hot_path_lock_for_certification(&self) {
        self.shared_read_pins
            .record_committed_read_hot_path_lock_for_certification();
    }
}

impl ForgeQueryRuntime {
    fn capture_shared_read_generation_for_current_snapshot(
        &self,
        snapshot_identity: ForgeQuerySnapshotIdentity,
    ) {
        self.publish_shared_read_generation(snapshot_identity);
    }

    fn publish_shared_read_generation(&self, snapshot_identity: ForgeQuerySnapshotIdentity) {
        let generation = self
            .shared_read_pins
            .capture_committed_snapshot(snapshot_identity.clone());
        let entries = self
            .derived_views
            .iter()
            .map(|(target, runtime_view)| {
                let entry = ForgeQueryPublishedArtifactEntry::from_runtime_view(
                    &snapshot_identity,
                    target.terminal_view_name_projection(),
                    runtime_view,
                )
                .expect("published artifact entry should package runtime view");
                (target.clone(), entry)
            })
            .collect();
        self.published_artifacts
            .publish_generation(&generation, entries);
        self.published_artifacts
            .retain_generations(&self.shared_read_pins.retained_generation_ordinals());
    }
}
