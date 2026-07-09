use std::collections::BTreeMap;

use crate::identity::hash_parts;
use crate::runtime::computed::WorthQueryDerivedViewHandle;
use crate::runtime::WorthQueryRuntimeError;
use worth_foundational::facade::AspectValue;

use super::retained_scalar_values::retained_scalar_value_digest_text;
use super::{
    WorthQueryDerivedArtifactBinding, WorthQueryDerivedMaterializationResult,
    WorthQueryDerivedMaterializationTarget, WorthQueryRetainedFieldPath,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryRetainedScalarFieldFact {
    field_path: WorthQueryRetainedFieldPath,
    value: AspectValue,
}

impl WorthQueryRetainedScalarFieldFact {
    pub fn field_path(&self) -> &WorthQueryRetainedFieldPath {
        &self.field_path
    }

    pub fn value(&self) -> &AspectValue {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
struct WorthQueryRetainedScalarRow {
    fields: BTreeMap<WorthQueryRetainedFieldPath, AspectValue>,
}

impl WorthQueryRetainedScalarRow {
    fn admit_paths_from_materialization(
        target: &WorthQueryDerivedMaterializationTarget,
        materialization: &WorthQueryDerivedMaterializationResult,
        field_paths: Vec<WorthQueryRetainedFieldPath>,
    ) -> Result<Self, WorthQueryRuntimeError> {
        let view_name = target.view_name();
        let rows = materialization.retained_rows();
        let [row] = rows else {
            return Err(WorthQueryRuntimeError::RetainedRowDecode {
                view_name: view_name.to_string(),
                stage: "retained-scalar-row-admission",
                message: format!(
                    "expected exactly one retained row for scalar extraction, but found {}",
                    rows.len()
                ),
            });
        };
        let fields = field_paths
            .iter()
            .map(|field_path| {
                let terminal_field_key = field_path.terminal_projection_for_boundary();
                let value = row.field_value_at(field_path).ok_or_else(|| {
                    WorthQueryRuntimeError::RetainedRowDecode {
                        view_name: view_name.to_string(),
                        stage: "retained-scalar-row-admission",
                        message: format!(
                            "retained row for `{view_name}` did not carry declared scalar field `{terminal_field_key}`"
                        ),
                    }
                })?;
                Ok((field_path.clone(), value.clone()))
            })
            .collect::<Result<BTreeMap<_, _>, WorthQueryRuntimeError>>()?;

        Ok(Self { fields })
    }

    fn into_facts(self) -> Vec<WorthQueryRetainedScalarFieldFact> {
        self.fields
            .into_iter()
            .map(|(field_path, value)| WorthQueryRetainedScalarFieldFact { field_path, value })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryRetainedScalarFactSet {
    artifact_name: String,
    binding_digest: String,
    target: WorthQueryDerivedMaterializationTarget,
    source_result_digest: String,
    fact_set_digest: String,
    facts: Vec<WorthQueryRetainedScalarFieldFact>,
}

impl WorthQueryRetainedScalarFactSet {
    pub fn artifact_name(&self) -> &str {
        &self.artifact_name
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub fn target(&self) -> &WorthQueryDerivedMaterializationTarget {
        &self.target
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

    pub fn facts(&self) -> &[WorthQueryRetainedScalarFieldFact] {
        &self.facts
    }

    pub fn field_value_at(&self, field_path: &WorthQueryRetainedFieldPath) -> Option<&AspectValue> {
        self.facts
            .iter()
            .find(|fact| fact.field_path() == field_path)
            .map(WorthQueryRetainedScalarFieldFact::value)
    }
}

impl WorthQueryDerivedArtifactBinding {
    pub fn consume_scalar_fields<V, I>(
        &self,
        view: &WorthQueryDerivedViewHandle<V>,
        field_paths: I,
    ) -> Result<WorthQueryRetainedScalarFactSet, WorthQueryRuntimeError>
    where
        I: IntoIterator<Item = WorthQueryRetainedFieldPath>,
    {
        self.consume_scalar_field_paths_for_target(
            &WorthQueryDerivedMaterializationTarget::from(view),
            field_paths.into_iter().collect(),
        )
    }

    pub(in crate::runtime) fn consume_scalar_field_paths_for_target(
        &self,
        target: &WorthQueryDerivedMaterializationTarget,
        field_paths: Vec<WorthQueryRetainedFieldPath>,
    ) -> Result<WorthQueryRetainedScalarFactSet, WorthQueryRuntimeError> {
        let materialization = self.materialization_for_target(target)?;
        consume_scalar_field_paths_from_materialization(
            self,
            target.clone(),
            materialization,
            field_paths,
        )
    }
}

fn retained_scalar_fact_set_from_facts(
    binding: &WorthQueryDerivedArtifactBinding,
    target: WorthQueryDerivedMaterializationTarget,
    materialization: &WorthQueryDerivedMaterializationResult,
    facts: Vec<WorthQueryRetainedScalarFieldFact>,
) -> Result<WorthQueryRetainedScalarFactSet, WorthQueryRuntimeError> {
    let view_name = target.view_name();
    let fact_set_digest = hash_parts(
        &std::iter::once("worth_query_retained_scalar_fact_set_v1".to_string())
            .chain(std::iter::once(format!(
                "artifact:{}",
                binding.artifact_name()
            )))
            .chain(std::iter::once(format!(
                "binding:{}",
                binding.binding_for_reporting()
            )))
            .chain(std::iter::once(format!("view:{view_name}")))
            .chain(std::iter::once(format!(
                "result:{}",
                materialization.receipt().result_digest()
            )))
            .chain(facts.iter().map(|fact| {
                format!(
                    "field:{}:{}",
                    fact.field_path().terminal_projection_for_boundary(),
                    retained_scalar_value_digest_text(fact.value())
                )
            }))
            .collect::<Vec<_>>(),
    );

    Ok(WorthQueryRetainedScalarFactSet {
        artifact_name: binding.artifact_name().to_string(),
        binding_digest: binding.binding_for_reporting().to_string(),
        target,
        source_result_digest: materialization.receipt().result_digest().to_string(),
        fact_set_digest,
        facts,
    })
}

fn consume_scalar_field_paths_from_materialization(
    binding: &WorthQueryDerivedArtifactBinding,
    target: WorthQueryDerivedMaterializationTarget,
    materialization: &WorthQueryDerivedMaterializationResult,
    mut field_paths: Vec<WorthQueryRetainedFieldPath>,
) -> Result<WorthQueryRetainedScalarFactSet, WorthQueryRuntimeError> {
    field_paths.sort();
    field_paths.dedup();
    let facts = WorthQueryRetainedScalarRow::admit_paths_from_materialization(
        &target,
        materialization,
        field_paths,
    )?
    .into_facts();

    retained_scalar_fact_set_from_facts(binding, target, materialization, facts)
}

#[cfg(test)]
#[path = "retained_scalar_facts_tests.rs"]
mod retained_scalar_facts_tests;
