use std::collections::{BTreeMap, BTreeSet};

use crate::memory_workspace::WorthQuerySnapshotIdentity;

#[cfg(test)]
use super::{record_forbidden_fallback_seam_invocation, WorthQueryForbiddenFallbackSeam};
use super::{
    WorthQueryDerivedMaterializationResult, WorthQueryDerivedViewHandle,
    WorthQueryLiveArtifactBinding, WorthQueryLiveArtifactBundle, WorthQueryLiveArtifactTarget,
    WorthQueryLiveReadResult, WorthQueryPatchBatch, WorthQueryRuntimeError, WorthQueryWorkspace,
};

impl WorthQueryWorkspace {
    pub(crate) fn close_managed_live_view<T>(
        &mut self,
        view: &super::WorthQueryLiveView<T>,
        capability: &std::sync::Arc<super::WorthQueryManagedLiveWorkspaceCapability>,
    ) -> Result<crate::subscription::SubscriptionLifecycleCloseout, WorthQueryRuntimeError> {
        self.admit_managed_live_capability(capability, view.name())?;
        self.runtime.reap_abandoned_managed_live_resources()?;
        self.runtime.close_managed_live_view(view)
    }

    pub(crate) fn read_managed_live_view<T>(
        &mut self,
        view: &super::WorthQueryLiveView<T>,
        capability: &std::sync::Arc<super::WorthQueryManagedLiveWorkspaceCapability>,
    ) -> Result<WorthQueryLiveReadResult, WorthQueryRuntimeError> {
        self.admit_managed_live_capability(capability, view.name())?;
        self.runtime.reap_abandoned_managed_live_resources()?;
        self.read_live_result(view)
    }

    pub(crate) fn drain_managed_live_view<T>(
        &mut self,
        view: &super::WorthQueryLiveView<T>,
        capability: &std::sync::Arc<super::WorthQueryManagedLiveWorkspaceCapability>,
    ) -> Result<WorthQueryPatchBatch, WorthQueryRuntimeError> {
        self.admit_managed_live_capability(capability, view.name())?;
        self.runtime.reap_abandoned_managed_live_resources()?;
        let target = self.resolve_live_artifact_target(view.name())?;
        if target.subscription_installation() != Some(view.subscription_installation()) {
            return Err(WorthQueryRuntimeError::MissingLiveSubscription(
                view.name().to_string(),
            ));
        }
        Ok(self.runtime.drain_patches(view))
    }

    pub(crate) fn observe_managed_live_view<T>(
        &mut self,
        view: &super::WorthQueryLiveView<T>,
        capability: &std::sync::Arc<super::WorthQueryManagedLiveWorkspaceCapability>,
    ) -> Result<super::WorthQueryManagedLiveLifecycleObservation, WorthQueryRuntimeError> {
        self.admit_managed_live_capability(capability, view.name())?;
        self.runtime.observe_managed_live_view(view)
    }

    pub fn resolve_live_artifact_target(
        &self,
        view_name: &str,
    ) -> Result<WorthQueryLiveArtifactTarget, WorthQueryRuntimeError> {
        self.runtime.resolve_live_artifact_target(view_name)
    }

    pub fn read<T>(
        &mut self,
        view: &super::WorthQueryLiveView<T>,
    ) -> Vec<crate::memory_workspace::WorthQueryEntity> {
        self.read_live_result(view)
            .expect("live view declaration admitted before workspace.read execution")
            .rows()
            .to_vec()
    }

    pub fn read_live_result<T>(
        &mut self,
        view: &super::WorthQueryLiveView<T>,
    ) -> Result<WorthQueryLiveReadResult, WorthQueryRuntimeError> {
        self.read_live_intent(view).execute()
    }

    pub fn read_live_target(
        &mut self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Result<WorthQueryLiveReadResult, WorthQueryRuntimeError> {
        let resolved = self.resolve_live_artifact_target(target.terminal_view_name_projection())?;
        let installation = resolved
            .subscription_installation()
            .cloned()
            .ok_or_else(|| {
                WorthQueryRuntimeError::MissingLiveSubscription(
                    target.terminal_view_name_projection().to_string(),
                )
            })?;
        self.runtime
            .execute_live_read_for_installation(installation)
    }

    pub fn state_live<T>(
        &self,
        view: &super::WorthQueryLiveView<T>,
    ) -> Result<super::WorthQueryRuntimeStateSnapshot, WorthQueryRuntimeError> {
        self.state_live_by_name(view.name())
    }

    pub fn state_live_target(
        &self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Result<super::WorthQueryRuntimeStateSnapshot, WorthQueryRuntimeError> {
        let resolved = self.resolve_live_artifact_target(target.terminal_view_name_projection())?;
        self.state_live_by_name(resolved.terminal_view_name_projection())
    }

    pub(crate) fn state_live_by_name(
        &self,
        view_name: &str,
    ) -> Result<super::WorthQueryRuntimeStateSnapshot, WorthQueryRuntimeError> {
        super::state::snapshot_live_view_name(&self.runtime, view_name)
    }

    pub fn read_live_artifact_binding(
        &mut self,
        artifact_name: impl Into<String>,
        targets: impl IntoIterator<Item = WorthQueryLiveArtifactTarget>,
    ) -> Result<WorthQueryLiveArtifactBinding, WorthQueryRuntimeError> {
        #[cfg(test)]
        record_forbidden_fallback_seam_invocation(
            WorthQueryForbiddenFallbackSeam::ReadLiveArtifactBinding,
        );
        let retained_targets = targets.into_iter().collect::<Vec<_>>();
        self.read_live_artifact_bundle(retained_targets.clone())?
            .bind_live_artifact(artifact_name, retained_targets)
    }

    pub fn read_live_artifact_bundle(
        &mut self,
        targets: impl IntoIterator<Item = WorthQueryLiveArtifactTarget>,
    ) -> Result<WorthQueryLiveArtifactBundle, WorthQueryRuntimeError> {
        #[cfg(test)]
        record_forbidden_fallback_seam_invocation(
            WorthQueryForbiddenFallbackSeam::ReadLiveArtifactBundle,
        );
        let mut retained_targets = targets.into_iter().collect::<Vec<_>>();
        retained_targets.sort();
        retained_targets.dedup();

        let mut reads = BTreeMap::new();
        for target in retained_targets {
            let installation = target.subscription_installation().cloned().ok_or_else(|| {
                WorthQueryRuntimeError::MissingLiveView(
                    target.terminal_view_name_projection().to_string(),
                )
            })?;
            let result = self
                .runtime
                .execute_live_read_for_installation(installation)?;
            reads.insert(target, result);
        }
        let snapshot_identity = live_bundle_snapshot_identity(&reads)?;
        Ok(WorthQueryLiveArtifactBundle::new(snapshot_identity, reads))
    }

    pub fn observe<T>(&mut self, view: &super::WorthQueryLiveView<T>) -> WorthQueryPatchBatch {
        self.runtime.drain_patches(view)
    }

    pub fn materialize_result<T>(
        &self,
        view: &WorthQueryDerivedViewHandle<T>,
    ) -> Result<WorthQueryDerivedMaterializationResult, WorthQueryRuntimeError> {
        self.runtime.read_derived_result(view)
    }

    pub fn observe_computed<T>(
        &mut self,
        view: &WorthQueryDerivedViewHandle<T>,
    ) -> WorthQueryPatchBatch {
        self.runtime.drain_derived_patches(view)
    }

    pub fn subscription_basis_digest<T>(
        &self,
        view: &super::WorthQueryLiveView<T>,
    ) -> Result<String, WorthQueryRuntimeError> {
        self.subscription_basis_digest_by_name(view.name())
    }

    pub fn subscription_basis_digest_for_target(
        &self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Result<String, WorthQueryRuntimeError> {
        let resolved = self.resolve_live_artifact_target(target.terminal_view_name_projection())?;
        self.subscription_basis_digest_by_name(resolved.terminal_view_name_projection())
    }

    pub(crate) fn subscription_basis_digest_by_name(
        &self,
        view_name: &str,
    ) -> Result<String, WorthQueryRuntimeError> {
        Ok(self
            .runtime
            .inspect_live_view_name_installation(view_name)?
            .basis_binding_projection()
            .label()
            .to_string())
    }
}

fn live_bundle_snapshot_identity(
    reads: &BTreeMap<WorthQueryLiveArtifactTarget, WorthQueryLiveReadResult>,
) -> Result<WorthQuerySnapshotIdentity, WorthQueryRuntimeError> {
    let snapshot_identities = reads
        .iter()
        .map(|(target, result)| {
            (
                target.terminal_view_name_projection().to_string(),
                result.receipt().snapshot_identity().clone(),
            )
        })
        .collect::<Vec<_>>();
    let distinct_snapshot_identities = snapshot_identities
        .iter()
        .map(|(_, snapshot_identity)| {
            snapshot_identity
                .evidence_identity()
                .terminal_projection_for_reporting()
                .to_string()
        })
        .collect::<BTreeSet<_>>();
    match snapshot_identities.as_slice() {
        [] => Ok(WorthQuerySnapshotIdentity::empty_relational_state()),
        [(_, snapshot_identity)] if distinct_snapshot_identities.len() == 1 => {
            Ok(snapshot_identity.clone())
        }
        _ if distinct_snapshot_identities.len() == 1 => Ok(snapshot_identities[0].1.clone()),
        _ => Err(WorthQueryRuntimeError::ReadCompositionDenied(
            super::WorthQueryReadDenial::new(
                super::WorthQueryReadDenialKind::ExecutionDenied,
                format!(
                    "live artifact bundle materialized multiple snapshot identities: {}",
                    snapshot_identities
                        .iter()
                        .map(|(view_name, snapshot_identity)| {
                            format!(
                                "{view_name}:{}",
                                snapshot_identity
                                    .evidence_identity()
                                    .terminal_projection_for_reporting()
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            ),
        )),
    }
}
