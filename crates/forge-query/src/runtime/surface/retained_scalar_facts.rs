use crate::identity::hash_parts;
use crate::runtime::computed::ForgeQueryDerivedViewHandle;
use crate::runtime::ForgeQueryRuntimeError;
#[cfg(test)]
use crate::runtime::{record_forbidden_fallback_seam_invocation, ForgeQueryForbiddenFallbackSeam};

use super::{ForgeQueryDerivedArtifactBinding, ForgeQueryDerivedMaterializationResult};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryRetainedScalarFieldFact {
    field_key: String,
    value: serde_json::Value,
}

impl ForgeQueryRetainedScalarFieldFact {
    pub fn field_key(&self) -> &str {
        &self.field_key
    }

    pub fn value(&self) -> &serde_json::Value {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryRetainedScalarFactSet {
    artifact_name: String,
    binding_digest: String,
    view_name: String,
    source_result_digest: String,
    fact_set_digest: String,
    facts: Vec<ForgeQueryRetainedScalarFieldFact>,
}

impl ForgeQueryRetainedScalarFactSet {
    pub fn artifact_name(&self) -> &str {
        &self.artifact_name
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn view_name(&self) -> &str {
        &self.view_name
    }

    pub fn source_result_digest(&self) -> &str {
        &self.source_result_digest
    }

    pub fn fact_set_digest(&self) -> &str {
        &self.fact_set_digest
    }

    pub fn field_count(&self) -> usize {
        self.facts.len()
    }

    pub fn facts(&self) -> &[ForgeQueryRetainedScalarFieldFact] {
        &self.facts
    }

    pub fn field_value(&self, field_key: &str) -> Option<&serde_json::Value> {
        self.facts
            .iter()
            .find(|fact| fact.field_key() == field_key)
            .map(ForgeQueryRetainedScalarFieldFact::value)
    }
}

impl ForgeQueryDerivedArtifactBinding {
    pub fn consume_scalar_fields<V, I, S>(
        &self,
        view: &ForgeQueryDerivedViewHandle<V>,
        field_keys: I,
    ) -> Result<ForgeQueryRetainedScalarFactSet, ForgeQueryRuntimeError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        self.consume_scalar_fields_by_name(view.name(), field_keys)
    }

    pub fn consume_scalar_fields_by_name<I, S>(
        &self,
        view_name: &str,
        field_keys: I,
    ) -> Result<ForgeQueryRetainedScalarFactSet, ForgeQueryRuntimeError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        #[cfg(test)]
        record_forbidden_fallback_seam_invocation(
            ForgeQueryForbiddenFallbackSeam::ConsumeScalarFields,
        );
        let materialization = self.materialization_by_name(view_name)?;
        consume_scalar_fields_from_materialization(self, view_name, materialization, field_keys)
    }
}

fn consume_scalar_fields_from_materialization<I, S>(
    binding: &ForgeQueryDerivedArtifactBinding,
    view_name: &str,
    materialization: &ForgeQueryDerivedMaterializationResult,
    field_keys: I,
) -> Result<ForgeQueryRetainedScalarFactSet, ForgeQueryRuntimeError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let [row] = materialization.rows() else {
        return Err(ForgeQueryRuntimeError::RetainedRowDecode {
            view_name: view_name.to_string(),
            stage: "retained-scalar-facts",
            message: format!(
                "expected exactly one retained row for scalar extraction, but found {}",
                materialization.rows().len()
            ),
        });
    };

    let mut normalized_field_keys = field_keys
        .into_iter()
        .map(|field_key| field_key.as_ref().to_string())
        .collect::<Vec<_>>();
    normalized_field_keys.sort();
    normalized_field_keys.dedup();

    let facts = normalized_field_keys
        .iter()
        .map(|field_key| {
            let value = nested_value(row, field_key).ok_or_else(|| {
                ForgeQueryRuntimeError::RetainedRowDecode {
                    view_name: view_name.to_string(),
                    stage: "retained-scalar-facts",
                    message: format!(
                        "retained row for `{view_name}` did not carry declared scalar field `{field_key}`"
                    ),
                }
            })?;
            Ok(ForgeQueryRetainedScalarFieldFact {
                field_key: field_key.clone(),
                value: value.clone(),
            })
        })
        .collect::<Result<Vec<_>, ForgeQueryRuntimeError>>()?;

    let fact_set_digest = hash_parts(
        &std::iter::once("forge_query_retained_scalar_fact_set_v1".to_string())
            .chain(std::iter::once(format!(
                "artifact:{}",
                binding.artifact_name()
            )))
            .chain(std::iter::once(format!(
                "binding:{}",
                binding.binding_digest()
            )))
            .chain(std::iter::once(format!("view:{view_name}")))
            .chain(std::iter::once(format!(
                "result:{}",
                materialization.receipt().result_digest()
            )))
            .chain(facts.iter().map(|fact| {
                format!(
                    "field:{}:{}",
                    fact.field_key(),
                    serde_json::to_string(fact.value()).expect("scalar fact value must encode")
                )
            }))
            .collect::<Vec<_>>(),
    );

    Ok(ForgeQueryRetainedScalarFactSet {
        artifact_name: binding.artifact_name().to_string(),
        binding_digest: binding.binding_digest().to_string(),
        view_name: view_name.to_string(),
        source_result_digest: materialization.receipt().result_digest().to_string(),
        fact_set_digest,
        facts,
    })
}

fn nested_value<'a>(row: &'a serde_json::Value, field_key: &str) -> Option<&'a serde_json::Value> {
    let mut current = row;
    for segment in field_key.split('.') {
        current = current.get(segment)?;
    }
    Some(current)
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

    fn binding() -> ForgeQueryDerivedArtifactBinding {
        let materialization = ForgeQueryDerivedMaterializationResult::new(
            vec![json!({
                "authority_snapshot_id": 7,
                "nested": { "truth_basis_digest_hex": "basis:test" },
            })],
            ForgeQueryDerivedMaterializationReceipt::test_only(
                "surface:test",
                "snapshot:test",
                "result:test",
            ),
        );
        let bundle = ForgeQueryDerivedMaterializationBundle::new(
            "snapshot:test",
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
    fn retained_scalar_fact_set_reads_nested_fields() {
        let facts = binding()
            .consume_scalar_fields_by_name(
                "surface:test",
                ["nested.truth_basis_digest_hex", "authority_snapshot_id"],
            )
            .expect("scalar facts should extract");

        assert_eq!(facts.artifact_name(), "artifact:test");
        assert_eq!(facts.view_name(), "surface:test");
        assert_eq!(facts.field_count(), 2);
        assert_eq!(facts.field_value("authority_snapshot_id"), Some(&json!(7)));
        assert_eq!(
            facts.field_value("nested.truth_basis_digest_hex"),
            Some(&json!("basis:test"))
        );
        assert!(!facts.fact_set_digest().is_empty());
    }

    #[test]
    fn retained_scalar_fact_set_rejects_missing_field() {
        let error = binding()
            .consume_scalar_fields_by_name("surface:test", ["missing.field"])
            .expect_err("missing field should fail");

        assert!(matches!(
            error,
            crate::runtime::ForgeQueryRuntimeError::RetainedRowDecode { .. }
        ));
    }
}
