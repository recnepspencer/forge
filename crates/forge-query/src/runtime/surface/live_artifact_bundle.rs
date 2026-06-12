use std::collections::BTreeMap;

use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
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
    snapshot_identity: ForgeQuerySnapshotIdentity,
    snapshot_evidence_identity: ForgeQueryEvidenceIdentity,
    bundle_digest: String,
    reads: BTreeMap<String, ForgeQueryLiveReadResult>,
}

impl ForgeQueryLiveArtifactBundle {
    pub(in crate::runtime) fn new(
        snapshot_identity: ForgeQuerySnapshotIdentity,
        reads: BTreeMap<String, ForgeQueryLiveReadResult>,
    ) -> Self {
        let snapshot_evidence_identity = snapshot_identity.evidence_identity();
        let bundle_digest =
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::LiveArtifactBundle)
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("snapshot_identity"),
                    &snapshot_evidence_identity,
                )
                .field_identity_sequence(
                    ForgeQueryEvidenceTag::new("read_result"),
                    reads.iter().map(|(view_name, result)| {
                        format!("{view_name}:{}", result.receipt().result_digest())
                    }),
                )
                .seal()
                .to_string();
        Self {
            snapshot_identity,
            snapshot_evidence_identity,
            bundle_digest,
            reads,
        }
    }

    pub fn snapshot_identity(&self) -> &ForgeQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub fn snapshot_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.snapshot_evidence_identity
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
        snapshot_identity: ForgeQuerySnapshotIdentity,
        reads: BTreeMap<String, ForgeQueryLiveReadResult>,
    ) -> Self {
        Self::new(snapshot_identity, reads)
    }
}
