use std::collections::BTreeMap;

use crate::identity::hash_parts;
use crate::runtime::surface::live_artifact_binding::ForgeQueryLiveArtifactBinding;
use crate::runtime::ForgeQueryLiveView;
use crate::runtime::ForgeQueryRuntimeError;
#[cfg(test)]
use crate::runtime::{record_forbidden_fallback_seam_invocation, ForgeQueryForbiddenFallbackSeam};

use super::ForgeQueryLiveReadResult;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ForgeQueryLiveArtifactTarget {
    view_name: String,
}

impl ForgeQueryLiveArtifactTarget {
    pub fn new(view_name: impl Into<String>) -> Self {
        Self {
            view_name: view_name.into(),
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }
}

impl<T> From<&ForgeQueryLiveView<T>> for ForgeQueryLiveArtifactTarget {
    fn from(value: &ForgeQueryLiveView<T>) -> Self {
        Self::new(value.name())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryLiveArtifactBundle {
    snapshot_token: String,
    bundle_digest: String,
    reads: BTreeMap<String, ForgeQueryLiveReadResult>,
}

impl ForgeQueryLiveArtifactBundle {
    pub(in crate::runtime) fn new(
        snapshot_token: impl Into<String>,
        reads: BTreeMap<String, ForgeQueryLiveReadResult>,
    ) -> Self {
        let snapshot_token = snapshot_token.into();
        let bundle_digest = hash_parts(
            &std::iter::once("forge_query_live_artifact_bundle_v1".to_string())
                .chain(std::iter::once(format!("snapshot:{snapshot_token}")))
                .chain(reads.iter().map(|(view_name, result)| {
                    format!("{view_name}:{}", result.receipt().result_digest())
                }))
                .collect::<Vec<_>>(),
        );
        Self {
            snapshot_token,
            bundle_digest,
            reads,
        }
    }

    pub fn snapshot_token(&self) -> &str {
        &self.snapshot_token
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }

    pub fn target_count(&self) -> usize {
        self.reads.len()
    }

    pub fn target_view_names(&self) -> impl Iterator<Item = &str> {
        self.reads.keys().map(String::as_str)
    }

    pub fn includes_view_name(&self, view_name: &str) -> bool {
        self.reads.contains_key(view_name)
    }

    pub fn read<T>(
        &self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<&ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.read_by_name(view.name())
    }

    pub fn read_by_name(
        &self,
        view_name: &str,
    ) -> Result<&ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.reads.get(view_name).ok_or_else(|| {
            ForgeQueryRuntimeError::ReadCompositionDenied(
                crate::runtime::ForgeQueryReadDenial::new(
                    crate::runtime::ForgeQueryReadDenialKind::ExecutionDenied,
                    format!(
                        "live artifact bundle did not retain requested live view `{view_name}`"
                    ),
                ),
            )
        })
    }

    pub fn bind_live_artifact(
        self,
        artifact_name: impl Into<String>,
        required_targets: impl IntoIterator<Item = ForgeQueryLiveArtifactTarget>,
    ) -> Result<ForgeQueryLiveArtifactBinding, ForgeQueryRuntimeError> {
        #[cfg(test)]
        record_forbidden_fallback_seam_invocation(
            ForgeQueryForbiddenFallbackSeam::BindLiveArtifact,
        );
        ForgeQueryLiveArtifactBinding::bind(self, artifact_name, required_targets)
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        snapshot_token: impl Into<String>,
        reads: BTreeMap<String, ForgeQueryLiveReadResult>,
    ) -> Self {
        Self::new(snapshot_token, reads)
    }
}
