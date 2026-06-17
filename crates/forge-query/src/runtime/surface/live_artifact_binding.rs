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
    target_view_names: Vec<String>,
}

impl ForgeQueryLiveArtifactBinding {
    pub(in crate::runtime) fn bind(
        bundle: ForgeQueryLiveArtifactBundle,
        artifact_name: impl Into<String>,
        required_targets: impl IntoIterator<Item = ForgeQueryLiveArtifactTarget>,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        let artifact_name = artifact_name.into();
        let mut target_view_names = required_targets
            .into_iter()
            .map(|target| target.view_name().to_string())
            .collect::<Vec<_>>();
        target_view_names.sort();
        target_view_names.dedup();

        let bundle_view_names = bundle.target_view_names().collect::<Vec<_>>();
        if bundle.target_count() != target_view_names.len()
            || !target_view_names
                .iter()
                .all(|view_name| bundle.includes_view_name(view_name))
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
            target_view_names,
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
        self.target_view_names.len()
    }

    pub fn target_view_names(&self) -> impl Iterator<Item = &str> {
        self.target_view_names.iter().map(String::as_str)
    }

    pub fn read<T>(
        &self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<&ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.bundle.read(view)
    }

    pub fn read_by_name(
        &self,
        view_name: &str,
    ) -> Result<&ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.bundle.read_by_name(view_name)
    }
}
