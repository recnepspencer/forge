use std::collections::{BTreeMap, BTreeSet};

use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use serde_json::Value;

#[cfg(test)]
use super::{record_forbidden_fallback_seam_invocation, ForgeQueryForbiddenFallbackSeam};
use super::{
    ForgeQueryDerivedViewHandle, ForgeQueryLiveArtifactBinding, ForgeQueryLiveArtifactBundle,
    ForgeQueryLiveArtifactTarget, ForgeQueryLiveReadResult, ForgeQueryPatchBatch,
    ForgeQueryRuntimeError, ForgeQueryWorkspace,
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

    pub fn state_live_by_name(
        &self,
        view_name: &str,
    ) -> Result<super::ForgeQueryRuntimeStateSnapshot, ForgeQueryRuntimeError> {
        super::state::snapshot_live_view_name(&self.runtime, view_name)
    }

    pub fn read_live_by_name(
        &mut self,
        view_name: &str,
    ) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        let installation = self
            .runtime
            .inspect_live_view_name_installation(view_name)?
            .clone();
        let review = self.review_live_read_execution(super::ForgeQueryLiveView::<Value>::new(
            crate::memory_workspace::ForgeQueryLiveViewHandle::new(view_name),
            installation,
        ))?;
        let handoff = self.resolve_reviewed_admitted_live_read_execution_handoff(review)?;
        let binding = self.into_runtime_live_read_execution_binding(handoff)?;
        self.execute_bound_live_read_execution(binding)
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
            let result = self.runtime.execute_live_read_by_name(target.view_name())?;
            reads.insert(target.view_name().to_string(), result);
        }
        let snapshot_identity = live_bundle_snapshot_identity(&reads)?;
        Ok(ForgeQueryLiveArtifactBundle::new(snapshot_identity, reads))
    }

    pub fn observe<T>(&mut self, view: &super::ForgeQueryLiveView<T>) -> ForgeQueryPatchBatch {
        self.runtime.drain_patches(view)
    }

    pub fn materialize<T>(&self, view: &ForgeQueryDerivedViewHandle<T>) -> Vec<Value> {
        self.runtime.read_derived(view)
    }

    pub fn observe_computed(&mut self, view_name: &str) -> ForgeQueryPatchBatch {
        self.runtime.drain_derived_patches(view_name)
    }

    pub fn subscription_basis_digest_by_name(
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
    reads: &BTreeMap<String, ForgeQueryLiveReadResult>,
) -> Result<ForgeQuerySnapshotIdentity, ForgeQueryRuntimeError> {
    let snapshot_identities = reads
        .iter()
        .map(|(view_name, result)| {
            (
                view_name.clone(),
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
