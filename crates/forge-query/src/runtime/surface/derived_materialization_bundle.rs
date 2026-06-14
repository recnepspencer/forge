use std::collections::BTreeMap;

use serde::de::DeserializeOwned;

use super::derived_artifact_binding::ForgeQueryDerivedArtifactBinding;
use super::derived_materialization_result::ForgeQueryDerivedMaterializationResult;
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::runtime::computed::ForgeQueryDerivedViewHandle;
use crate::runtime::ForgeQueryRuntimeError;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ForgeQueryDerivedMaterializationTarget {
    view_name: String,
}

impl ForgeQueryDerivedMaterializationTarget {
    pub fn new(view_name: impl Into<String>) -> Self {
        Self {
            view_name: view_name.into(),
        }
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }
}

impl<T> From<&ForgeQueryDerivedViewHandle<T>> for ForgeQueryDerivedMaterializationTarget {
    fn from(value: &ForgeQueryDerivedViewHandle<T>) -> Self {
        Self::new(value.name())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedMaterializationBundle {
    snapshot_identity: ForgeQuerySnapshotIdentity,
    snapshot_evidence_identity: ForgeQueryEvidenceIdentity,
    bundle_digest: String,
    materializations: BTreeMap<String, ForgeQueryDerivedMaterializationResult>,
}

impl ForgeQueryDerivedMaterializationBundle {
    pub(in crate::runtime) fn new(
        snapshot_identity: ForgeQuerySnapshotIdentity,
        materializations: BTreeMap<String, ForgeQueryDerivedMaterializationResult>,
    ) -> Self {
        let snapshot_evidence_identity = snapshot_identity.evidence_identity();
        let bundle_digest = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::DerivedMaterializationBundle,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("snapshot_identity"),
            &snapshot_evidence_identity,
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("materialization_result"),
            materializations.iter().map(|(view_name, result)| {
                format!("{view_name}:{}", result.receipt().result_digest())
            }),
        )
        .seal()
        .to_string();
        Self {
            snapshot_identity,
            snapshot_evidence_identity,
            bundle_digest,
            materializations,
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
        self.materializations.len()
    }

    pub fn target_view_names(&self) -> impl Iterator<Item = &str> {
        self.materializations.keys().map(String::as_str)
    }

    pub fn includes_view_name(&self, view_name: &str) -> bool {
        self.materializations.contains_key(view_name)
    }

    pub fn materialization<T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> Result<&ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
        self.materialization_by_name(view.name())
    }

    pub fn materialization_by_name(
        &self,
        view_name: &str,
    ) -> Result<&ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
        self.materializations.get(view_name).ok_or_else(|| {
            ForgeQueryRuntimeError::RetainedRowDecode {
                view_name: view_name.to_string(),
                stage: "derived-materialization-bundle",
                message: "bundle did not retain the requested derived surface".to_string(),
            }
        })
    }

    pub fn decode_single_row<V, T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<V>,
    ) -> Result<T, ForgeQueryRuntimeError>
    where
        T: DeserializeOwned,
    {
        self.materialization(view)?.decode_single_row()
    }

    pub fn bind_retained_artifact(
        self,
        artifact_name: impl Into<String>,
        required_targets: impl IntoIterator<Item = ForgeQueryDerivedMaterializationTarget>,
    ) -> Result<ForgeQueryDerivedArtifactBinding, ForgeQueryRuntimeError> {
        ForgeQueryDerivedArtifactBinding::bind(self, artifact_name, required_targets)
    }

    pub fn bind_retained_artifact_identity(
        self,
        artifact_identity: ForgeQueryEvidenceIdentity,
        required_targets: impl IntoIterator<Item = ForgeQueryDerivedMaterializationTarget>,
    ) -> Result<ForgeQueryDerivedArtifactBinding, ForgeQueryRuntimeError> {
        ForgeQueryDerivedArtifactBinding::bind_with_identity(
            self,
            artifact_identity,
            required_targets,
        )
    }

    #[cfg(test)]
    pub(crate) fn test_only(
        snapshot_identity: ForgeQuerySnapshotIdentity,
        materializations: BTreeMap<String, ForgeQueryDerivedMaterializationResult>,
    ) -> Self {
        Self::new(snapshot_identity, materializations)
    }
}
