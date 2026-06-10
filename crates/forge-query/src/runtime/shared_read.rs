use std::collections::BTreeMap;

use crate::authorized_projection::AuthorizedProjectionArtifact;
use crate::canonicalization::CanonicalResultShapeArtifact;
use crate::projection_consumption::{
    ProjectMaterializedFacts, ProjectionFactConsumptionAttempt, ProjectionFactConsumptionPathError,
};

use super::{
    ForgeQueryDerivedArtifactBinding, ForgeQueryDerivedMaterializationBundle,
    ForgeQueryDerivedMaterializationReceipt, ForgeQueryDerivedMaterializationResult,
    ForgeQueryDerivedMaterializationTarget, ForgeQueryDerivedViewHandle, ForgeQueryRuntime,
    ForgeQuerySharedReadGenerationLease,
    ForgeQueryRuntimeAsyncResultState, ForgeQueryRuntimeAsyncResultStateKind,
    ForgeQueryRuntimeError,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryPublishedProjectionConsumption {
    Current(ProjectionFactConsumptionAttempt),
    ResultState(ForgeQueryRuntimeAsyncResultState),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPublishedProjectionInspection {
    published: bool,
    artifact_binding_digest: Option<String>,
    snapshot_token: String,
    async_result_state: Option<ForgeQueryRuntimeAsyncResultState>,
}

impl ForgeQueryPublishedProjectionInspection {
    pub fn published(&self) -> bool {
        self.published
    }

    pub fn artifact_binding_digest(&self) -> Option<&str> {
        self.artifact_binding_digest.as_deref()
    }

    pub fn snapshot_token(&self) -> &str {
        &self.snapshot_token
    }

    pub fn async_result_state(&self) -> Option<&ForgeQueryRuntimeAsyncResultState> {
        self.async_result_state.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryPublishedDerivedArtifactHandle {
    lease: ForgeQuerySharedReadGenerationLease,
    view_name: String,
    published_binding: Option<ForgeQueryDerivedArtifactBinding>,
    async_result_state: Option<ForgeQueryRuntimeAsyncResultState>,
}

impl ForgeQueryPublishedDerivedArtifactHandle {
    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn snapshot_token(&self) -> &str {
        self.lease.generation().snapshot_token()
    }

    pub fn published_binding(&self) -> Option<&ForgeQueryDerivedArtifactBinding> {
        self.published_binding.as_ref()
    }

    pub fn async_result_state(&self) -> Option<&ForgeQueryRuntimeAsyncResultState> {
        self.async_result_state.as_ref()
    }

    pub fn inspect_projection_consumption(&self) -> ForgeQueryPublishedProjectionInspection {
        ForgeQueryPublishedProjectionInspection {
            published: self.published_binding.is_some(),
            artifact_binding_digest: self
                .published_binding
                .as_ref()
                .map(|binding| binding.binding_digest().to_string()),
            snapshot_token: self.snapshot_token().to_string(),
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
                binding.consume_projection_facts(
                    result_shape.digest().as_str(),
                    authorized_projection,
                    requested,
                )?,
            )),
            None => Ok(ForgeQueryPublishedProjectionConsumption::ResultState(
                self.async_result_state
                    .clone()
                    .expect("unpublished shared-read artifact must retain typed async posture"),
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQuerySharedReadContext {
    lease: ForgeQuerySharedReadGenerationLease,
}

impl ForgeQuerySharedReadContext {
    pub fn snapshot_token(&self) -> &str {
        self.lease.generation().snapshot_token()
    }

    pub fn published_derived_artifact<T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> Result<ForgeQueryPublishedDerivedArtifactHandle, ForgeQueryRuntimeError> {
        if !self.lease.is_generation_live() {
            return Err(super::forge_query_shared_read_stale_basis_error(
                self.snapshot_token().to_string(),
            ));
        }
        let Some(derived_view) = self.lease.snapshot().derived_views().get(view.name()) else {
            return Err(ForgeQueryRuntimeError::MissingDerivedView(
                view.name().to_string(),
            ));
        };
        if !derived_view.published {
            return Ok(ForgeQueryPublishedDerivedArtifactHandle {
                lease: self.lease.clone(),
                view_name: view.name().to_string(),
                published_binding: None,
                async_result_state: Some(ForgeQueryRuntimeAsyncResultState::new(
                    ForgeQueryRuntimeAsyncResultStateKind::Pending,
                    format!("shared-read:unpublished:{}", view.name()),
                    self.snapshot_token().to_string(),
                    shared_read_generation_digest(self.snapshot_token()),
                )),
            });
        }

        let binding = bind_shared_read_artifact(
            self.snapshot_token(),
            view.name(),
            derived_view.materialization.clone(),
        )?;
        Ok(ForgeQueryPublishedDerivedArtifactHandle {
            lease: self.lease.clone(),
            view_name: view.name().to_string(),
            published_binding: Some(binding),
            async_result_state: derived_view.async_result_state(self.snapshot_token(), view.name()),
        })
    }

    #[allow(dead_code)]
    pub(in crate::runtime) fn from_runtime(runtime: &ForgeQueryRuntime) -> Self {
        runtime
            .mint_shared_read_context()
            .expect("runtime-owned shared read context should mint")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(in crate::runtime) struct SharedReadDerivedViewState {
    pub(in crate::runtime) published: bool,
    pub(in crate::runtime) materialization: ForgeQueryDerivedMaterializationResult,
    pub(in crate::runtime) pending_patch_count: usize,
    pub(in crate::runtime) pending_refresh_fallback_count: usize,
}

impl SharedReadDerivedViewState {
    fn async_result_state(
        &self,
        snapshot_token: &str,
        view_name: &str,
    ) -> Option<ForgeQueryRuntimeAsyncResultState> {
        if self.pending_patch_count == 0 {
            return None;
        }

        let kind = if self.pending_refresh_fallback_count > 0 {
            ForgeQueryRuntimeAsyncResultStateKind::Revalidating
        } else {
            ForgeQueryRuntimeAsyncResultStateKind::Stale
        };

        Some(ForgeQueryRuntimeAsyncResultState::new(
            kind,
            format!("shared-read:republishing:{view_name}:{}", kind.as_str()),
            snapshot_token.to_string(),
            shared_read_generation_digest(snapshot_token),
        ))
    }
}

impl ForgeQueryRuntime {
    pub(in crate::runtime) fn capture_shared_read_generation(&mut self, snapshot_token: &str) {
        let derived_views = self
            .derived_views
            .iter()
            .map(|(view_name, runtime_view)| {
                let evidence = super::ForgeQueryComputedInspectionEvidence::from_runtime(runtime_view);
                let receipt = ForgeQueryDerivedMaterializationReceipt::from_evidence(
                    &evidence,
                    snapshot_token.to_string(),
                );
                let materialization = ForgeQueryDerivedMaterializationResult::new(
                    runtime_view.materialization.rows().to_vec(),
                    receipt,
                );
                (
                    view_name.clone(),
                    SharedReadDerivedViewState {
                        published: runtime_view.materialization.is_published(),
                        materialization,
                        pending_patch_count: evidence.pending_patch_count(),
                        pending_refresh_fallback_count: evidence.pending_refresh_fallback_count(),
                    },
                )
            })
            .collect::<BTreeMap<_, _>>();
        self.shared_read_pins
            .capture_committed_snapshot(snapshot_token.to_string(), derived_views);
    }

    pub(in crate::runtime) fn mint_shared_read_context(
        &self,
    ) -> Result<ForgeQuerySharedReadContext, ForgeQueryRuntimeError> {
        let snapshot_token = self.backend.snapshot_token();
        if !self.shared_read_pins.has_current_generation() {
            let derived_views = self
                .derived_views
                .iter()
                .map(|(view_name, runtime_view)| {
                    let evidence =
                        super::ForgeQueryComputedInspectionEvidence::from_runtime(runtime_view);
                    let receipt = ForgeQueryDerivedMaterializationReceipt::from_evidence(
                        &evidence,
                        snapshot_token.clone(),
                    );
                    let materialization = ForgeQueryDerivedMaterializationResult::new(
                        runtime_view.materialization.rows().to_vec(),
                        receipt,
                    );
                    (
                        view_name.clone(),
                        SharedReadDerivedViewState {
                            published: runtime_view.materialization.is_published(),
                            materialization,
                            pending_patch_count: evidence.pending_patch_count(),
                            pending_refresh_fallback_count: evidence.pending_refresh_fallback_count(),
                        },
                    )
                })
                .collect::<BTreeMap<_, _>>();
            self.shared_read_pins
                .capture_committed_snapshot(snapshot_token, derived_views);
        }
        let lease = self
            .shared_read_pins
            .pin_current_generation()
            .ok_or_else(|| ForgeQueryRuntimeError::Workspace(crate::memory_workspace::ForgeQueryWorkspaceError::new(
                "shared read context requires a committed generation",
            )))?;
        Ok(ForgeQuerySharedReadContext { lease })
    }

    #[cfg(test)]
    pub(in crate::runtime) fn shared_read_counters(
        &self,
    ) -> super::ForgeQuerySharedReadCounters {
        self.shared_read_pins.counters()
    }

    #[cfg(test)]
    pub(crate) fn force_retire_shared_read_snapshot_for_tests(
        &self,
        snapshot_token: &str,
    ) {
        self.shared_read_pins
            .force_retire_snapshot_token(snapshot_token);
    }
}

fn bind_shared_read_artifact(
    snapshot_token: &str,
    view_name: &str,
    materialization: ForgeQueryDerivedMaterializationResult,
) -> Result<ForgeQueryDerivedArtifactBinding, ForgeQueryRuntimeError> {
    let bundle = ForgeQueryDerivedMaterializationBundle::new(
        snapshot_token.to_string(),
        BTreeMap::from([(view_name.to_string(), materialization)]),
    );
    bundle.bind_retained_artifact(
        format!("shared-read:{view_name}"),
        [ForgeQueryDerivedMaterializationTarget::new(view_name)],
    )
}

fn shared_read_generation_digest(snapshot_token: &str) -> String {
    format!("shared-read-generation:{snapshot_token}")
}
