use std::collections::{BTreeMap, BTreeSet};

use crate::memory_workspace::ForgeQuerySnapshotIdentity;

#[cfg(test)]
use super::{record_forbidden_fallback_seam_invocation, ForgeQueryForbiddenFallbackSeam};
use super::{
    ForgeQueryDerivedMaterializationResult, ForgeQueryDerivedViewHandle,
    ForgeQueryLiveArtifactBinding, ForgeQueryLiveArtifactBundle, ForgeQueryLiveArtifactTarget,
    ForgeQueryLiveReadResult, ForgeQueryPatchBatch, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};

impl ForgeQueryWorkspace {
    pub fn read<T>(
        &mut self,
        view: &super::ForgeQueryLiveView<T>,
    ) -> Vec<crate::memory_workspace::ForgeQueryEntity> {
        self.read_live_result(view)
            .expect("live view declaration admitted before workspace.read execution")
            .rows()
            .to_vec()
    }

    pub fn read_live_result<T>(
        &mut self,
        view: &super::ForgeQueryLiveView<T>,
    ) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.read_live_intent(view).execute()
    }

    pub fn state_live<T>(
        &self,
        view: &super::ForgeQueryLiveView<T>,
    ) -> Result<super::ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        self.state_live_by_name(view.name())
    }

    pub(crate) fn state_live_by_name(
        &self,
        view_name: &str,
    ) -> Result<super::ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        super::state::snapshot_live_view_name(&self.runtime, view_name)
    }

    pub fn read_live_artifact_binding(
        &mut self,
        artifact_name: impl Into<String>,
        targets: impl IntoIterator<Item = ForgeQueryLiveArtifactTarget>,
    ) -> Result<ForgeQueryLiveArtifactBinding, ForgeQueryRuntimeError> {
        #[cfg(test)]
        record_forbidden_fallback_seam_invocation(
            ForgeQueryForbiddenFallbackSeam::ReadLiveArtifactBinding,
        );
        let retained_targets = targets.into_iter().collect::<Vec<_>>();
        self.read_live_artifact_bundle(retained_targets.clone())?
            .bind_live_artifact(artifact_name, retained_targets)
    }

    pub fn read_live_artifact_bundle(
        &mut self,
        targets: impl IntoIterator<Item = ForgeQueryLiveArtifactTarget>,
    ) -> Result<ForgeQueryLiveArtifactBundle, ForgeQueryRuntimeError> {
        #[cfg(test)]
        record_forbidden_fallback_seam_invocation(
            ForgeQueryForbiddenFallbackSeam::ReadLiveArtifactBundle,
        );
        let mut retained_targets = targets.into_iter().collect::<Vec<_>>();
        retained_targets.sort();
        retained_targets.dedup();

        let mut reads = BTreeMap::new();
        for target in retained_targets {
            let installation = target.subscription_installation().cloned().ok_or_else(|| {
                ForgeQueryRuntimeError::MissingLiveView(
                    target.terminal_view_name_projection().to_string(),
                )
            })?;
            let result = self
                .runtime
                .execute_live_read_for_installation(installation)?;
            reads.insert(target, result);
        }
        let snapshot_identity = live_bundle_snapshot_identity(&reads)?;
        Ok(ForgeQueryLiveArtifactBundle::new(snapshot_identity, reads))
    }

    pub fn observe<T>(&mut self, view: &super::ForgeQueryLiveView<T>) -> ForgeQueryPatchBatch {
        self.runtime.drain_patches(view)
    }

    pub fn materialize_result<T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> Result<ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
        self.runtime.read_derived_result(view)
    }

    pub fn observe_computed<T>(
        &mut self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> ForgeQueryPatchBatch {
        self.runtime.drain_derived_patches(view)
    }

    pub fn subscription_basis_digest<T>(
        &self,
        view: &super::ForgeQueryLiveView<T>,
    ) -> Result<String, ForgeQueryRuntimeError> {
        self.subscription_basis_digest_by_name(view.name())
    }

    pub(crate) fn subscription_basis_digest_by_name(
        &self,
        view_name: &str,
    ) -> Result<String, ForgeQueryRuntimeError> {
        Ok(self
            .runtime
            .inspect_live_view_name_installation(view_name)?
            .basis_binding_projection()
            .label()
            .to_string())
    }
}

fn live_bundle_snapshot_identity(
    reads: &BTreeMap<ForgeQueryLiveArtifactTarget, ForgeQueryLiveReadResult>,
) -> Result<ForgeQuerySnapshotIdentity, ForgeQueryRuntimeError> {
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
        [] => Ok(ForgeQuerySnapshotIdentity::empty_relational_state()),
        [(_, snapshot_identity)] if distinct_snapshot_identities.len() == 1 => {
            Ok(snapshot_identity.clone())
        }
        _ if distinct_snapshot_identities.len() == 1 => Ok(snapshot_identities[0].1.clone()),
        _ => Err(ForgeQueryRuntimeError::ReadCompositionDenied(
            super::ForgeQueryReadDenial::new(
                super::ForgeQueryReadDenialKind::ExecutionDenied,
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
