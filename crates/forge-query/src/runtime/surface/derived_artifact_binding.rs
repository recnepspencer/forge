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

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryDerivedArtifactBinding {
    artifact_name: String,
    binding_identity: ForgeQueryEvidenceIdentity,
    bundle: ForgeQueryDerivedMaterializationBundle,
    targets: Vec<ForgeQueryDerivedMaterializationTarget>,
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
            targets,
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
        self.targets.len()
    }

    pub fn targets(&self) -> &[ForgeQueryDerivedMaterializationTarget] {
        &self.targets
    }

    pub fn terminal_target_view_names_projection(&self) -> impl Iterator<Item = &str> {
        self.targets.iter().map(|target| target.view_name())
    }

    pub fn materialization<T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> Result<&ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
        self.bundle.materialization(view)
    }

    pub(crate) fn materialization_for_target(
        &self,
        target: &ForgeQueryDerivedMaterializationTarget,
    ) -> Result<&ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
        self.bundle.materialization_by_name(target.view_name())
    }

    pub fn into_bundle(self) -> ForgeQueryDerivedMaterializationBundle {
        self.bundle
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use forge_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey, InternedString};

    use super::ForgeQueryDerivedArtifactBinding;

    use crate::runtime::computed::ForgeQueryDerivedViewHandle;
    use crate::runtime::surface::{
        ForgeQueryDerivedMaterializationBundle, ForgeQueryDerivedMaterializationReceipt,
        ForgeQueryDerivedMaterializationResult, ForgeQueryDerivedMaterializationTarget,
        ForgeQueryRetainedFieldPath, ForgeQueryRetainedMaterializedRow,
    };
    use crate::runtime::ForgeQueryNativeRow;

    fn scalar_row(field: &str, value: AspectValue) -> ForgeQueryRetainedMaterializedRow {
        ForgeQueryRetainedMaterializedRow::from_scalar_values(BTreeMap::from([(
            retained_field_path(field).expect("field path admits"),
            value,
        )]))
        .expect("retained row should build")
    }

    fn retained_row(view_name: &str, value: &str) -> ForgeQueryDerivedMaterializationResult {
        ForgeQueryDerivedMaterializationResult::from_retained_rows(
            vec![scalar_row(
                "value",
                AspectValue::String(InternedString::Raw(value.to_string())),
            )],
            ForgeQueryDerivedMaterializationReceipt::test_only(
                view_name,
                crate::memory_workspace::admit_external_snapshot_label("snapshot-test"),
                &format!("{view_name}-digest"),
            ),
        )
    }

    fn binding() -> (
        ForgeQueryDerivedArtifactBinding,
        ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
        ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
        ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
    ) {
        let first = ForgeQueryDerivedViewHandle::new("derived.first");
        let second = ForgeQueryDerivedViewHandle::new("derived.second");
        let third = ForgeQueryDerivedViewHandle::new("derived.third");
        let bundle = ForgeQueryDerivedMaterializationBundle::new(
            crate::memory_workspace::admit_external_snapshot_label("snapshot-test"),
            BTreeMap::from([
                (
                    first.name().to_string(),
                    retained_row(first.name(), "first"),
                ),
                (
                    second.name().to_string(),
                    retained_row(second.name(), "second"),
                ),
                (
                    third.name().to_string(),
                    retained_row(third.name(), "third"),
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

    fn retained_string_value(
        binding: &ForgeQueryDerivedArtifactBinding,
        view: &ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
    ) -> String {
        let value_path = retained_field_path("value").expect("value path admits");
        let value = binding
            .materialization(view)
            .expect("bound materialization should exist")
            .single_retained_row()
            .expect("bound materialization should carry one row")
            .field_value_at(&value_path)
            .expect("bound materialization should carry value field");
        let AspectValue::String(InternedString::Raw(value)) = value else {
            panic!("expected retained string value, got {value:?}");
        };
        value.clone()
    }

    #[test]
    fn row_pair_uses_bound_retained_artifact_identity() {
        let (binding, first, second, _) = binding();

        assert_eq!(retained_string_value(&binding, &first), "first");
        assert_eq!(retained_string_value(&binding, &second), "second");
    }

    #[test]
    fn row_triple_uses_bound_retained_artifact_identity() {
        let (binding, first, second, third) = binding();

        assert_eq!(retained_string_value(&binding, &first), "first");
        assert_eq!(retained_string_value(&binding, &second), "second");
        assert_eq!(retained_string_value(&binding, &third), "third");
    }

    fn retained_field_path(path: &str) -> Result<ForgeQueryRetainedFieldPath, String> {
        let fields = path
            .split('.')
            .map(|segment| {
                FieldKey::new(segment.to_string())
                    .ok_or_else(|| format!("`{path}` is not a retained scalar field path"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let path = CanonicalFieldPath::new(fields)
            .ok_or_else(|| format!("`{path}` is not a retained scalar field path"))?;
        Ok(ForgeQueryRetainedFieldPath::from_canonical_field_path(path))
    }
}
