use crate::identity::hash_parts;
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::runtime::WorthQueryLiveView;
use crate::runtime::WorthQueryRuntimeError;

use super::{WorthQueryLiveArtifactBundle, WorthQueryLiveArtifactTarget, WorthQueryLiveReadResult};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryLiveArtifactBinding {
    artifact_name: String,
    binding_digest: String,
    bundle: WorthQueryLiveArtifactBundle,
    targets: Vec<WorthQueryLiveArtifactTarget>,
}

impl WorthQueryLiveArtifactBinding {
    pub(in crate::runtime) fn bind(
        bundle: WorthQueryLiveArtifactBundle,
        artifact_name: impl Into<String>,
        required_targets: impl IntoIterator<Item = WorthQueryLiveArtifactTarget>,
    ) -> Result<Self, WorthQueryRuntimeError> {
        let artifact_name = artifact_name.into();
        let mut targets = required_targets.into_iter().collect::<Vec<_>>();
        targets.sort();
        targets.dedup();

        if bundle.target_count() != targets.len()
            || !targets.iter().all(|target| bundle.includes_target(target))
        {
            let target_view_names = terminal_target_view_names(&targets);
            let bundle_view_names = bundle
                .terminal_target_view_names_projection()
                .collect::<Vec<_>>();
            return Err(WorthQueryRuntimeError::ReadCompositionDenied(
                crate::runtime::WorthQueryReadDenial::new(
                    crate::runtime::WorthQueryReadDenialKind::ExecutionDenied,
                    format!(
                        "live artifact binding `{artifact_name}` requires exact target set {:?}, but bundle carried {:?}",
                        target_view_names, bundle_view_names
                    ),
                ),
            ));
        }

        let binding_digest = hash_parts(
            &std::iter::once("worth_query_live_artifact_binding_v1".to_string())
                .chain(std::iter::once(format!("artifact:{artifact_name}")))
                .chain(std::iter::once(format!(
                    "bundle:{}",
                    bundle.bundle_digest()
                )))
                .chain(
                    targets
                        .iter()
                        .map(|target| format!("target:{}", target.terminal_view_name_projection())),
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

    pub fn snapshot_identity(&self) -> &WorthQuerySnapshotIdentity {
        self.bundle.snapshot_identity()
    }

    pub fn target_count(&self) -> usize {
        self.targets.len()
    }

    pub fn targets(&self) -> &[WorthQueryLiveArtifactTarget] {
        &self.targets
    }

    pub fn terminal_target_view_names_projection(&self) -> impl Iterator<Item = &str> {
        self.targets.iter().map(|target| target.view_name())
    }

    pub fn read<T>(
        &self,
        view: &WorthQueryLiveView<T>,
    ) -> Result<&WorthQueryLiveReadResult, WorthQueryRuntimeError> {
        self.bundle.read(view)
    }

    pub(crate) fn read_for_target(
        &self,
        target: &WorthQueryLiveArtifactTarget,
    ) -> Result<&WorthQueryLiveReadResult, WorthQueryRuntimeError> {
        self.bundle.read_for_target(target)
    }
}

fn terminal_target_view_names(targets: &[WorthQueryLiveArtifactTarget]) -> Vec<String> {
    targets
        .iter()
        .map(|target| target.terminal_view_name_projection().to_string())
        .collect()
}
