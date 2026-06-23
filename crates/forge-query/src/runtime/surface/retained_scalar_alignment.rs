use crate::identity::hash_parts;
use crate::runtime::computed::ForgeQueryDerivedViewHandle;
use crate::runtime::ForgeQueryRuntimeError;
use forge_foundational::facade::AspectValue;

use super::retained_scalar_values::retained_scalar_value_digest_text;
use super::{
    ForgeQueryDerivedArtifactBinding, ForgeQueryDerivedMaterializationTarget,
    ForgeQueryRetainedFieldPath, ForgeQueryRetainedScalarFactSet,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryRetainedScalarAlignmentFact {
    left_field_path: ForgeQueryRetainedFieldPath,
    right_field_path: ForgeQueryRetainedFieldPath,
    value: AspectValue,
}

impl ForgeQueryRetainedScalarAlignmentFact {
    pub fn left_field_path(&self) -> &ForgeQueryRetainedFieldPath {
        &self.left_field_path
    }

    pub fn right_field_path(&self) -> &ForgeQueryRetainedFieldPath {
        &self.right_field_path
    }

    pub fn value(&self) -> &AspectValue {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryRetainedScalarAlignment {
    artifact_name: String,
    binding_digest: String,
    left_target: ForgeQueryDerivedMaterializationTarget,
    right_target: ForgeQueryDerivedMaterializationTarget,
    alignment_digest: String,
    aligned_facts: Vec<ForgeQueryRetainedScalarAlignmentFact>,
}

impl ForgeQueryRetainedScalarAlignment {
    pub fn artifact_name(&self) -> &str {
        &self.artifact_name
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn left_target(&self) -> &ForgeQueryDerivedMaterializationTarget {
        &self.left_target
    }

    pub fn right_target(&self) -> &ForgeQueryDerivedMaterializationTarget {
        &self.right_target
    }

    pub fn alignment_digest(&self) -> &str {
        &self.alignment_digest
    }

    pub fn aligned_field_count(&self) -> usize {
        self.aligned_facts.len()
    }

    pub fn aligned_facts(&self) -> &[ForgeQueryRetainedScalarAlignmentFact] {
        &self.aligned_facts
    }
}

impl ForgeQueryDerivedArtifactBinding {
    pub fn verify_scalar_alignment<V1, V2, I>(
        &self,
        left_view: &ForgeQueryDerivedViewHandle<V1>,
        right_view: &ForgeQueryDerivedViewHandle<V2>,
        field_pairs: I,
    ) -> Result<ForgeQueryRetainedScalarAlignment, ForgeQueryRuntimeError>
    where
        I: IntoIterator<Item = (ForgeQueryRetainedFieldPath, ForgeQueryRetainedFieldPath)>,
    {
        let normalized_pairs = field_pairs.into_iter().collect::<Vec<_>>();
        let left_target = ForgeQueryDerivedMaterializationTarget::from(left_view);
        let right_target = ForgeQueryDerivedMaterializationTarget::from(right_view);
        let left_facts = self.consume_scalar_field_paths_for_target(
            &left_target,
            normalized_pairs
                .iter()
                .map(|(left_path, _)| left_path.clone())
                .collect(),
        )?;
        let right_facts = self.consume_scalar_field_paths_for_target(
            &right_target,
            normalized_pairs
                .iter()
                .map(|(_, right_path)| right_path.clone())
                .collect(),
        )?;
        verify_scalar_alignment_between_fact_sets(
            self,
            left_target,
            right_target,
            &left_facts,
            &right_facts,
            &normalized_pairs,
        )
    }
}

fn verify_scalar_alignment_between_fact_sets(
    binding: &ForgeQueryDerivedArtifactBinding,
    left_target: ForgeQueryDerivedMaterializationTarget,
    right_target: ForgeQueryDerivedMaterializationTarget,
    left_facts: &ForgeQueryRetainedScalarFactSet,
    right_facts: &ForgeQueryRetainedScalarFactSet,
    field_pairs: &[(ForgeQueryRetainedFieldPath, ForgeQueryRetainedFieldPath)],
) -> Result<ForgeQueryRetainedScalarAlignment, ForgeQueryRuntimeError> {
    let left_view_name = left_target.view_name();
    let right_view_name = right_target.view_name();
    let aligned_facts = field_pairs
        .iter()
        .map(|(left_field_path, right_field_path)| {
            let left_field_key = left_field_path.terminal_projection_for_boundary();
            let right_field_key = right_field_path.terminal_projection_for_boundary();
            let left_value =
                left_facts
                    .field_value_at(left_field_path)
                    .ok_or_else(|| ForgeQueryRuntimeError::RetainedRowDecode {
                        view_name: left_view_name.to_string(),
                        stage: "retained-scalar-alignment",
                        message: format!(
                            "retained scalar fact set for `{left_view_name}` omitted `{left_field_key}`"
                        ),
                    })?;
            let right_value =
                right_facts
                    .field_value_at(right_field_path)
                    .ok_or_else(|| ForgeQueryRuntimeError::RetainedRowDecode {
                        view_name: right_view_name.to_string(),
                        stage: "retained-scalar-alignment",
                        message: format!(
                            "retained scalar fact set for `{right_view_name}` omitted `{right_field_key}`"
                        ),
                    })?;
            if left_value != right_value {
                return Err(ForgeQueryRuntimeError::RetainedRowDecode {
                    view_name: binding.artifact_name().to_string(),
                    stage: "retained-scalar-alignment",
                    message: format!(
                        "retained scalar alignment diverged between `{left_view_name}.{left_field_key}` and `{right_view_name}.{right_field_key}`"
                    ),
                });
            }
            Ok(ForgeQueryRetainedScalarAlignmentFact {
                left_field_path: left_field_path.clone(),
                right_field_path: right_field_path.clone(),
                value: left_value.clone(),
            })
        })
        .collect::<Result<Vec<_>, ForgeQueryRuntimeError>>()?;

    let alignment_digest = hash_parts(
        &std::iter::once("forge_query_retained_scalar_alignment_v1".to_string())
            .chain(std::iter::once(format!(
                "artifact:{}",
                binding.artifact_name()
            )))
            .chain(std::iter::once(format!(
                "binding:{}",
                binding.binding_for_reporting()
            )))
            .chain(std::iter::once(format!("left:{left_view_name}")))
            .chain(std::iter::once(format!("right:{right_view_name}")))
            .chain(aligned_facts.iter().map(|fact| {
                format!(
                    "pair:{}={}:{}",
                    fact.left_field_path().terminal_projection_for_boundary(),
                    fact.right_field_path().terminal_projection_for_boundary(),
                    retained_scalar_value_digest_text(fact.value())
                )
            }))
            .collect::<Vec<_>>(),
    );

    Ok(ForgeQueryRetainedScalarAlignment {
        artifact_name: binding.artifact_name().to_string(),
        binding_digest: binding.binding_for_reporting().to_string(),
        left_target,
        right_target,
        alignment_digest,
        aligned_facts,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use forge_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};

    use crate::runtime::surface::{
        ForgeQueryDerivedArtifactBinding, ForgeQueryDerivedMaterializationBundle,
        ForgeQueryDerivedMaterializationReceipt, ForgeQueryDerivedMaterializationResult,
        ForgeQueryDerivedMaterializationTarget, ForgeQueryRetainedFieldPath,
        ForgeQueryRetainedMaterializedRow,
    };
    use crate::runtime::ForgeQueryDerivedViewHandle;

    fn retained_row(
        fields: impl IntoIterator<Item = (&'static str, AspectValue)>,
    ) -> ForgeQueryRetainedMaterializedRow {
        let fields = fields
            .into_iter()
            .map(|(path, value)| {
                (
                    retained_field_path(path).expect("retained field path admits"),
                    value,
                )
            })
            .collect::<BTreeMap<_, _>>();
        ForgeQueryRetainedMaterializedRow::from_scalar_values(fields)
            .expect("retained row should build")
    }

    fn binding(row: ForgeQueryRetainedMaterializedRow) -> ForgeQueryDerivedArtifactBinding {
        let snapshot_identity =
            crate::memory_workspace::admit_external_snapshot_label("snapshot:test");
        let materialization = ForgeQueryDerivedMaterializationResult::from_retained_rows(
            vec![row],
            ForgeQueryDerivedMaterializationReceipt::test_only(
                "surface:test",
                snapshot_identity.clone(),
                "result:test",
            ),
        );
        let target = ForgeQueryDerivedMaterializationTarget::new("surface:test");
        let bundle = ForgeQueryDerivedMaterializationBundle::new(
            snapshot_identity,
            BTreeMap::from([(target.clone(), materialization)]),
        );
        ForgeQueryDerivedArtifactBinding::bind(bundle, "artifact:test", [target])
            .expect("binding should build")
    }

    fn view_handle() -> ForgeQueryDerivedViewHandle<crate::runtime::ForgeQueryNativeRow> {
        ForgeQueryDerivedViewHandle::new("surface:test")
    }

    #[test]
    fn retained_scalar_alignment_verifies_mapped_fields() {
        let left_view = view_handle();
        let right_view = view_handle();
        let alignment = binding(retained_row([
            ("authority_snapshot_id", AspectValue::Int64(7)),
            (
                "nested.truth_basis_digest_hex",
                crate::runtime::ForgeQueryAdmittedAspectValue::native_string_value("basis:test"),
            ),
        ]))
        .verify_scalar_alignment(
            &left_view,
            &right_view,
            [
                (
                    retained_field_path("authority_snapshot_id").expect("left path admits"),
                    retained_field_path("authority_snapshot_id").expect("right path admits"),
                ),
                (
                    retained_field_path("nested.truth_basis_digest_hex").expect("left path admits"),
                    retained_field_path("nested.truth_basis_digest_hex")
                        .expect("right path admits"),
                ),
            ],
        )
        .expect("alignment should succeed");

        assert_eq!(alignment.artifact_name(), "artifact:test");
        assert_eq!(alignment.aligned_field_count(), 2);
        assert!(!alignment.alignment_digest().is_empty());
    }

    #[test]
    fn retained_scalar_alignment_rejects_divergent_value() {
        let left_view = view_handle();
        let right_view = view_handle();
        let error = binding(retained_row([
            ("left", AspectValue::Int64(1)),
            ("right", AspectValue::Int64(2)),
        ]))
        .verify_scalar_alignment(
            &left_view,
            &right_view,
            [(
                retained_field_path("left").expect("left path admits"),
                retained_field_path("right").expect("right path admits"),
            )],
        )
        .expect_err("divergent scalar fields should fail");

        assert!(matches!(
            error,
            crate::runtime::ForgeQueryRuntimeError::RetainedRowDecode { .. }
        ));
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
