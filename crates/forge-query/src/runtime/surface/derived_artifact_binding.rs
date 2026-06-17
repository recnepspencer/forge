use serde::de::DeserializeOwned;

use super::{
    ForgeQueryDerivedMaterializationBundle, ForgeQueryDerivedMaterializationResult,
    ForgeQueryDerivedMaterializationTarget,
};
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::runtime::computed::ForgeQueryDerivedViewHandle;
use crate::runtime::ForgeQueryRuntimeError;
#[cfg(test)]
use crate::runtime::{record_forbidden_fallback_seam_invocation, ForgeQueryForbiddenFallbackSeam};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedArtifactBinding {
    artifact_name: String,
    binding_identity: ForgeQueryEvidenceIdentity,
    bundle: ForgeQueryDerivedMaterializationBundle,
    target_view_names: Vec<String>,
}

impl ForgeQueryDerivedArtifactBinding {
    pub(in crate::runtime) fn bind(
        bundle: ForgeQueryDerivedMaterializationBundle,
        artifact_name: impl Into<String>,
        required_targets: impl IntoIterator<Item = ForgeQueryDerivedMaterializationTarget>,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        let artifact_name = artifact_name.into();
        Self::bind_inner(bundle, artifact_name, required_targets)
    }

    pub(in crate::runtime) fn bind_with_identity(
        bundle: ForgeQueryDerivedMaterializationBundle,
        artifact_identity: ForgeQueryEvidenceIdentity,
        required_targets: impl IntoIterator<Item = ForgeQueryDerivedMaterializationTarget>,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        Self::bind_inner(
            bundle,
            artifact_identity.as_str().to_string(),
            required_targets,
        )
    }

    fn bind_inner(
        bundle: ForgeQueryDerivedMaterializationBundle,
        artifact_name: String,
        required_targets: impl IntoIterator<Item = ForgeQueryDerivedMaterializationTarget>,
    ) -> Result<Self, ForgeQueryRuntimeError> {
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
            return Err(ForgeQueryRuntimeError::RetainedRowDecode {
                view_name: artifact_name.clone(),
                stage: "derived-artifact-binding",
                message: format!(
                    "retained artifact binding `{artifact_name}` requires exact target set {:?}, but bundle carried {:?}",
                    target_view_names, bundle_view_names
                ),
            });
        }

        let binding_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::SharedReadGeneration)
                .field_shape(
                    ForgeQueryEvidenceTag::new("identity_family"),
                    "forge_query_derived_artifact_binding_v1",
                )
                .field_shape(ForgeQueryEvidenceTag::new("artifact"), &artifact_name)
                .field_shape(ForgeQueryEvidenceTag::new("bundle"), bundle.bundle_digest())
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("target"),
                    target_view_names.iter().map(String::as_str),
                )
                .seal();

        Ok(Self {
            artifact_name,
            binding_identity,
            bundle,
            target_view_names,
        })
    }

    pub fn artifact_name(&self) -> &str {
        &self.artifact_name
    }

    pub fn binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.binding_identity
    }

    pub fn binding_for_reporting(&self) -> &str {
        self.binding_identity.as_str()
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

    pub fn materialization<T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> Result<&ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
        self.bundle.materialization(view)
    }

    pub fn materialization_by_name(
        &self,
        view_name: &str,
    ) -> Result<&ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
        self.bundle.materialization_by_name(view_name)
    }

    pub fn decode_single_row<V, T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<V>,
    ) -> Result<T, ForgeQueryRuntimeError>
    where
        T: DeserializeOwned,
    {
        self.bundle.decode_single_row(view)
    }

    pub fn decode_row_pair<V1, T1, V2, T2>(
        &self,
        first: &ForgeQueryDerivedViewHandle<V1>,
        second: &ForgeQueryDerivedViewHandle<V2>,
    ) -> Result<(T1, T2), ForgeQueryRuntimeError>
    where
        T1: DeserializeOwned,
        T2: DeserializeOwned,
    {
        #[cfg(test)]
        record_forbidden_fallback_seam_invocation(ForgeQueryForbiddenFallbackSeam::DecodeRowPair);
        Ok((
            self.decode_single_row(first)?,
            self.decode_single_row(second)?,
        ))
    }

    pub fn decode_row_triple<V1, T1, V2, T2, V3, T3>(
        &self,
        first: &ForgeQueryDerivedViewHandle<V1>,
        second: &ForgeQueryDerivedViewHandle<V2>,
        third: &ForgeQueryDerivedViewHandle<V3>,
    ) -> Result<(T1, T2, T3), ForgeQueryRuntimeError>
    where
        T1: DeserializeOwned,
        T2: DeserializeOwned,
        T3: DeserializeOwned,
    {
        #[cfg(test)]
        record_forbidden_fallback_seam_invocation(ForgeQueryForbiddenFallbackSeam::DecodeRowTriple);
        Ok((
            self.decode_single_row(first)?,
            self.decode_single_row(second)?,
            self.decode_single_row(third)?,
        ))
    }

    pub fn into_bundle(self) -> ForgeQueryDerivedMaterializationBundle {
        self.bundle
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::ForgeQueryDerivedArtifactBinding;

    use crate::runtime::computed::ForgeQueryDerivedViewHandle;
    use crate::runtime::surface::{
        ForgeQueryDerivedMaterializationBundle, ForgeQueryDerivedMaterializationReceipt,
        ForgeQueryDerivedMaterializationResult, ForgeQueryDerivedMaterializationTarget,
    };

    fn retained_row(
        view_name: &str,
        value: serde_json::Value,
    ) -> ForgeQueryDerivedMaterializationResult {
        ForgeQueryDerivedMaterializationResult::new(
            vec![value],
            ForgeQueryDerivedMaterializationReceipt::test_only(
                view_name,
                crate::memory_workspace::admit_external_snapshot_label("snapshot-test"),
                &format!("{view_name}-digest"),
            ),
        )
    }

    fn binding() -> (
        ForgeQueryDerivedArtifactBinding,
        ForgeQueryDerivedViewHandle<serde_json::Value>,
        ForgeQueryDerivedViewHandle<serde_json::Value>,
        ForgeQueryDerivedViewHandle<serde_json::Value>,
    ) {
        let first = ForgeQueryDerivedViewHandle::new("derived.first");
        let second = ForgeQueryDerivedViewHandle::new("derived.second");
        let third = ForgeQueryDerivedViewHandle::new("derived.third");
        let bundle = ForgeQueryDerivedMaterializationBundle::new(
            crate::memory_workspace::admit_external_snapshot_label("snapshot-test"),
            BTreeMap::from([
                (
                    first.name().to_string(),
                    retained_row(first.name(), json!({"value": "first"})),
                ),
                (
                    second.name().to_string(),
                    retained_row(second.name(), json!({"value": "second"})),
                ),
                (
                    third.name().to_string(),
                    retained_row(third.name(), json!({"value": "third"})),
                ),
            ]),
        );
        let binding = ForgeQueryDerivedArtifactBinding::bind(
            bundle,
            "test.binding",
            [
                ForgeQueryDerivedMaterializationTarget::from(&first),
                ForgeQueryDerivedMaterializationTarget::from(&second),
                ForgeQueryDerivedMaterializationTarget::from(&third),
            ],
        )
        .expect("binding should succeed");
        (binding, first, second, third)
    }

    #[test]
    fn decode_row_pair_uses_bound_retained_artifact_identity() {
        let (binding, first, second, _) = binding();
        let rows: (serde_json::Value, serde_json::Value) = binding
            .decode_row_pair(&first, &second)
            .expect("pair decode should succeed");

        assert_eq!(rows.0["value"], "first");
        assert_eq!(rows.1["value"], "second");
    }

    #[test]
    fn decode_row_triple_uses_bound_retained_artifact_identity() {
        let (binding, first, second, third) = binding();
        let rows: (serde_json::Value, serde_json::Value, serde_json::Value) = binding
            .decode_row_triple(&first, &second, &third)
            .expect("triple decode should succeed");

        assert_eq!(rows.0["value"], "first");
        assert_eq!(rows.1["value"], "second");
        assert_eq!(rows.2["value"], "third");
    }
}
