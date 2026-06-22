use std::cmp::Ordering;
use std::collections::BTreeMap;

use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::runtime::surface::live_artifact_binding::ForgeQueryLiveArtifactBinding;
use crate::runtime::ForgeQueryLiveView;
#[cfg(test)]
use crate::runtime::{record_forbidden_fallback_seam_invocation, ForgeQueryForbiddenFallbackSeam};
use crate::runtime::{ForgeQueryRuntimeError, ForgeQueryRuntimeLiveSubscriptionInstallation};

use super::ForgeQueryLiveReadResult;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveArtifactTarget {
    view_name: String,
    installation: Option<ForgeQueryRuntimeLiveSubscriptionInstallation>,
}

impl Ord for ForgeQueryLiveArtifactTarget {
    fn cmp(&self, other: &Self) -> Ordering {
        self.view_name.cmp(&other.view_name)
    }
}

impl PartialOrd for ForgeQueryLiveArtifactTarget {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl ForgeQueryLiveArtifactTarget {
    pub(crate) fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn terminal_view_name_projection(&self) -> &str {
        self.view_name()
    }

    pub(in crate::runtime) fn from_view_name(view_name: impl Into<String>) -> Self {
        Self {
            view_name: view_name.into(),
            installation: None,
        }
    }

    pub(in crate::runtime) fn subscription_installation(
        &self,
    ) -> Option<&ForgeQueryRuntimeLiveSubscriptionInstallation> {
        self.installation.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn test_only(view_name: impl Into<String>) -> Self {
        Self::from_view_name(view_name)
    }
}

impl<T> From<&ForgeQueryLiveView<T>> for ForgeQueryLiveArtifactTarget {
    fn from(value: &ForgeQueryLiveView<T>) -> Self {
        Self {
            view_name: value.name().to_string(),
            installation: Some(value.subscription_installation().clone()),
        }
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
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("read_result"),
                    reads.iter().map(|(view_name, result)| {
                        format!("{view_name}:{}", result.receipt().result_digest())
                    }),
                )
                .seal()
                .terminal_projection_for_reporting()
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

    pub fn targets(&self) -> impl Iterator<Item = ForgeQueryLiveArtifactTarget> + '_ {
        self.reads
            .keys()
            .map(|view_name| ForgeQueryLiveArtifactTarget::from_view_name(view_name.clone()))
    }

    pub fn terminal_target_view_names_projection(&self) -> impl Iterator<Item = &str> {
        self.reads.keys().map(String::as_str)
    }

    pub fn includes_target(&self, target: &ForgeQueryLiveArtifactTarget) -> bool {
        self.reads.contains_key(target.view_name())
    }

    pub fn read<T>(
        &self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<&ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.read_by_name(view.name())
    }

    pub(crate) fn read_by_name(
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
