use crate::identity::hash_parts;
use crate::runtime::computed::ForgeQueryDerivedViewHandle;
use crate::runtime::ForgeQueryRuntimeError;
#[cfg(test)]
use crate::runtime::{record_forbidden_fallback_seam_invocation, ForgeQueryForbiddenFallbackSeam};

use super::{ForgeQueryDerivedArtifactBinding, ForgeQueryRetainedScalarFactSet};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryRetainedScalarAlignmentFact {
    left_field_key: String,
    right_field_key: String,
    value: serde_json::Value,
}

impl ForgeQueryRetainedScalarAlignmentFact {
    pub fn left_field_key(&self) -> &str {
        &self.left_field_key
    }

    pub fn right_field_key(&self) -> &str {
        &self.right_field_key
    }

    pub fn value(&self) -> &serde_json::Value {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryRetainedScalarAlignment {
    artifact_name: String,
    binding_digest: String,
    left_view_name: String,
    right_view_name: String,
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

    pub fn left_view_name(&self) -> &str {
        &self.left_view_name
    }

    pub fn right_view_name(&self) -> &str {
        &self.right_view_name
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
    pub fn verify_scalar_alignment<V1, V2, I, S1, S2>(
        &self,
        left_view: &ForgeQueryDerivedViewHandle<V1>,
        right_view: &ForgeQueryDerivedViewHandle<V2>,
        field_pairs: I,
    ) -> Result<ForgeQueryRetainedScalarAlignment, ForgeQueryRuntimeError>
    where
        I: IntoIterator<Item = (S1, S2)>,
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        self.verify_scalar_alignment_by_name(left_view.name(), right_view.name(), field_pairs)
    }

    pub fn verify_scalar_alignment_by_name<I, S1, S2>(
        &self,
        left_view_name: &str,
        right_view_name: &str,
        field_pairs: I,
    ) -> Result<ForgeQueryRetainedScalarAlignment, ForgeQueryRuntimeError>
    where
        I: IntoIterator<Item = (S1, S2)>,
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        #[cfg(test)]
        record_forbidden_fallback_seam_invocation(
            ForgeQueryForbiddenFallbackSeam::VerifyScalarAlignment,
        );
        let normalized_pairs = field_pairs
            .into_iter()
            .map(|(left, right)| (left.as_ref().to_string(), right.as_ref().to_string()))
            .collect::<Vec<_>>();
        let left_facts = self.consume_scalar_fields_by_name(
            left_view_name,
            normalized_pairs.iter().map(|(left, _)| left.as_str()),
        )?;
        let right_facts = self.consume_scalar_fields_by_name(
            right_view_name,
            normalized_pairs.iter().map(|(_, right)| right.as_str()),
        )?;
        verify_scalar_alignment_between_fact_sets(
            self,
            left_view_name,
            right_view_name,
            &left_facts,
            &right_facts,
            &normalized_pairs,
        )
    }
}

fn verify_scalar_alignment_between_fact_sets(
    binding: &ForgeQueryDerivedArtifactBinding,
    left_view_name: &str,
    right_view_name: &str,
    left_facts: &ForgeQueryRetainedScalarFactSet,
    right_facts: &ForgeQueryRetainedScalarFactSet,
    field_pairs: &[(String, String)],
) -> Result<ForgeQueryRetainedScalarAlignment, ForgeQueryRuntimeError> {
    let aligned_facts = field_pairs
        .iter()
        .map(|(left_field_key, right_field_key)| {
            let left_value =
                left_facts
                    .field_value(left_field_key)
                    .ok_or_else(|| ForgeQueryRuntimeError::RetainedRowDecode {
                        view_name: left_view_name.to_string(),
                        stage: "retained-scalar-alignment",
                        message: format!(
                            "retained scalar fact set for `{left_view_name}` omitted `{left_field_key}`"
                        ),
                    })?;
            let right_value =
                right_facts
                    .field_value(right_field_key)
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
                left_field_key: left_field_key.clone(),
                right_field_key: right_field_key.clone(),
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
                    fact.left_field_key(),
                    fact.right_field_key(),
                    serde_json::to_string(fact.value())
                        .expect("aligned scalar fact value must encode")
                )
            }))
            .collect::<Vec<_>>(),
    );

    Ok(ForgeQueryRetainedScalarAlignment {
        artifact_name: binding.artifact_name().to_string(),
        binding_digest: binding.binding_for_reporting().to_string(),
        left_view_name: left_view_name.to_string(),
        right_view_name: right_view_name.to_string(),
        alignment_digest,
        aligned_facts,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use crate::runtime::surface::{
        ForgeQueryDerivedArtifactBinding, ForgeQueryDerivedMaterializationBundle,
        ForgeQueryDerivedMaterializationReceipt, ForgeQueryDerivedMaterializationResult,
        ForgeQueryDerivedMaterializationTarget,
    };

    fn binding(row: serde_json::Value) -> ForgeQueryDerivedArtifactBinding {
        let snapshot_identity =
            crate::memory_workspace::admit_external_snapshot_label("snapshot:test");
        let materialization = ForgeQueryDerivedMaterializationResult::new(
            vec![row],
            ForgeQueryDerivedMaterializationReceipt::test_only(
                "surface:test",
                snapshot_identity.clone(),
                "result:test",
            ),
        );
        let bundle = ForgeQueryDerivedMaterializationBundle::new(
            snapshot_identity,
            BTreeMap::from([("surface:test".to_string(), materialization)]),
        );
        ForgeQueryDerivedArtifactBinding::bind(
            bundle,
            "artifact:test",
            [ForgeQueryDerivedMaterializationTarget::new("surface:test")],
        )
        .expect("binding should build")
    }

    #[test]
    fn retained_scalar_alignment_verifies_mapped_fields() {
        let alignment = binding(json!({
            "authority_snapshot_id": 7,
            "nested": { "truth_basis_digest_hex": "basis:test" },
        }))
        .verify_scalar_alignment_by_name(
            "surface:test",
            "surface:test",
            [
                ("authority_snapshot_id", "authority_snapshot_id"),
                (
                    "nested.truth_basis_digest_hex",
                    "nested.truth_basis_digest_hex",
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
        let error = binding(json!({"left": 1, "right": 2}))
            .verify_scalar_alignment_by_name("surface:test", "surface:test", [("left", "right")])
            .expect_err("divergent scalar fields should fail");

        assert!(matches!(
            error,
            crate::runtime::ForgeQueryRuntimeError::RetainedRowDecode { .. }
        ));
    }
}
