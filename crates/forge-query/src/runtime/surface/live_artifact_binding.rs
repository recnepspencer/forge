use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::runtime::ForgeQueryLiveView;
use crate::runtime::ForgeQueryRuntimeError;

use super::{ForgeQueryLiveArtifactBundle, ForgeQueryLiveArtifactTarget, ForgeQueryLiveReadResult};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryLiveArtifactBinding {
    artifact_name: String,
    binding_digest: String,
    bundle: ForgeQueryLiveArtifactBundle,
    targets: Vec<ForgeQueryLiveArtifactTarget>,
}

impl ForgeQueryLiveArtifactBinding {
    pub(in crate::runtime) fn bind(
        bundle: ForgeQueryLiveArtifactBundle,
        artifact_name: impl Into<String>,
        required_targets: impl IntoIterator<Item = ForgeQueryLiveArtifactTarget>,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        let artifact_name = artifact_name.into();
        let mut targets = required_targets.into_iter().collect::<Vec<_>>();
        targets.sort();
        targets.dedup();
        let target_view_names = targets
            .iter()
            .map(|target| target.view_name().to_string())
            .collect::<Vec<_>>();

        let bundle_view_names = bundle
            .terminal_target_view_names_projection()
            .collect::<Vec<_>>();
        if bundle.target_count() != target_view_names.len()
            || !targets.iter().all(|target| bundle.includes_target(target))
        {
            return Err(ForgeQueryRuntimeError::ReadCompositionDenied(
                crate::runtime::ForgeQueryReadDenial::new(
                    crate::runtime::ForgeQueryReadDenialKind::ExecutionDenied,
                    format!(
                        "live artifact binding `{artifact_name}` requires exact target set {:?}, but bundle carried {:?}",
                        target_view_names, bundle_view_names
                    ),
                ),
            ));
        }

        let binding_digest = hash_parts(
            &std::iter::once("forge_query_live_artifact_binding_v1".to_string())
                .chain(std::iter::once(format!("artifact:{artifact_name}")))
                .chain(std::iter::once(format!(
                    "bundle:{}",
                    bundle.bundle_digest()
                )))
                .chain(
                    target_view_names
                        .iter()
                        .map(|view_name| format!("target:{view_name}")),
                )
                .collect::<Vec<_>>(),
        );

        Ok(Self {
            artifact_name,
            binding_digest,
            bundle,
            targets,
        })
    }

    pub fn artifact_name(&self) -> &str {
        &self.artifact_name
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        self.bundle.snapshot_identity()
    }

    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    pub fn targets(&self) -> &[ForgeQueryLiveArtifactTarget] {
        &self.targets
    }

    pub fn terminal_target_view_names_projection(&self) -> impl Iterator<Item = &str> {
        self.targets.iter().map(|target| target.view_name())
    }

    pub fn read<T>(
        &self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<&ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.bundle.read(view)
    }

    pub(crate) fn read_for_target(
        &self,
        target: &ForgeQueryLiveArtifactTarget,
    ) -> Result<&ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.bundle.read_by_name(target.view_name())
    }
}
