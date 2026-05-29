use serde_json::Value;

use super::super::consumed::{
    ConsumedEntityIdentityFact, ConsumedFieldValueFact, ConsumedProjectionFactSet,
    ConsumedSourceReferenceFact, ConsumedViewLocalIdentityFact, ProjectionFactExtractionCounters,
};
use super::super::contracts::MaterializedProjectionContract;
use super::super::facts::ProjectionFactKind;
use super::super::source::ProjectionSourceFamily;
use crate::projection_consumption::ProjectionFactExtractionError;
use crate::query_context::QueryContextExecutionArtifact;

pub(super) fn extract_query_context_facts(
    contract: &MaterializedProjectionContract,
    execution: &QueryContextExecutionArtifact,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    super::ensure_contract_family(contract, ProjectionSourceFamily::QueryContextExecution)?;
    super::ensure_source_identity(
        contract.source_identity(),
        &query_context_source_identity(execution),
    )?;

    let extracts_entity_identity = contract
        .fact_families()
        .iter()
        .any(|fact| fact.kind() == ProjectionFactKind::EntityIdentity);
    let extracts_view_local_identity = contract
        .fact_families()
        .iter()
        .any(|fact| fact.kind() == ProjectionFactKind::ViewLocalIdentity);
    let extracts_row_value_fields = contract.fact_families().iter().any(|fact| {
        matches!(
            fact.kind(),
            ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField
        )
    });
    let extracts_source_references = contract
        .fact_families()
        .iter()
        .any(|fact| fact.kind() == ProjectionFactKind::SourceReference);

    let mut entity_identities = Vec::new();
    let mut view_local_identities = Vec::new();
    let mut display_fields = Vec::new();
    let mut derived_scalar_fields = Vec::new();

    for (index, row) in execution.rows().iter().enumerate() {
        let row_identity = query_context_row_identity(execution, index);
        let row_value = Value::String(row.clone());
        for fact_family in contract.fact_families() {
            match fact_family.kind() {
                ProjectionFactKind::EntityIdentity => {
                    entity_identities.push(ConsumedEntityIdentityFact::new(
                        row_identity.clone(),
                        row_identity.clone(),
                    ));
                }
                ProjectionFactKind::ViewLocalIdentity => {
                    view_local_identities.push(ConsumedViewLocalIdentityFact::new(
                        row_identity.clone(),
                        row_identity.clone(),
                    ));
                }
                ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField => {
                    let field_key = fact_family.field_key().expect("field key required");
                    let fact = ConsumedFieldValueFact::new(
                        row_identity.clone(),
                        field_key,
                        row_value.clone(),
                    );
                    if fact_family.kind() == ProjectionFactKind::DisplayField {
                        display_fields.push(fact);
                    } else {
                        derived_scalar_fields.push(fact);
                    }
                }
                ProjectionFactKind::TargetIdentity
                | ProjectionFactKind::EffectContinuity
                | ProjectionFactKind::Membership
                | ProjectionFactKind::RelationEndpoint
                | ProjectionFactKind::SourceReference => {}
            }
        }
    }

    let source_references = if extracts_source_references {
        query_context_source_references(execution)
    } else {
        Vec::new()
    };
    if extracts_source_references
        && !source_reference_inventory_matches(contract, &source_references)
    {
        return Err(
            ProjectionFactExtractionError::SourceReferenceEvidenceMismatch {
                expected_count: contract.source_reference_identities().len(),
                actual_count: source_references.len(),
            },
        );
    }

    let row_identity_surface_count =
        usize::from(extracts_entity_identity || extracts_view_local_identity);
    let row_value_surface_count = usize::from(extracts_row_value_fields);
    let source_row_width_consumed =
        execution.rows().len() * (row_identity_surface_count + row_value_surface_count);
    let source_evidence_lookup_width = source_references.len();
    let extracted_fact_count = entity_identities.len()
        + view_local_identities.len()
        + display_fields.len()
        + derived_scalar_fields.len()
        + source_references.len();

    Ok(ConsumedProjectionFactSet::new(
        contract.declaration_digest(),
        contract.contract_digest(),
        contract.source_family(),
        contract.source_identity(),
        contract.support_posture().clone(),
        ProjectionFactExtractionCounters::new(
            contract.fact_families().len(),
            contract.fact_families().len(),
            extracted_fact_count,
            source_row_width_consumed,
            source_evidence_lookup_width,
        ),
        entity_identities,
        view_local_identities,
        Vec::new(),
        display_fields,
        derived_scalar_fields,
        Vec::new(),
        source_references,
        Vec::new(),
        Vec::new(),
    ))
}

fn query_context_row_identity(execution: &QueryContextExecutionArtifact, index: usize) -> String {
    format!("query-context:{}:{index}", execution.family().as_str())
}

fn query_context_source_identity(execution: &QueryContextExecutionArtifact) -> String {
    execution
        .materialization_path_identity()
        .unwrap_or_else(|| execution.family().as_str())
        .to_string()
}

fn query_context_source_references(
    execution: &QueryContextExecutionArtifact,
) -> Vec<ConsumedSourceReferenceFact> {
    let mut references = Vec::new();
    if let Some(materialization_path_identity) = execution.materialization_path_identity() {
        references.push(ConsumedSourceReferenceFact::new(
            "query_context_materialization_path",
            materialization_path_identity,
        ));
    }
    if let Some(preview_provenance_identity) = execution.preview_provenance_identity() {
        references.push(ConsumedSourceReferenceFact::new(
            "query_context_preview_provenance",
            preview_provenance_identity,
        ));
    }
    references
}

fn source_reference_inventory_matches(
    contract: &MaterializedProjectionContract,
    actual: &[ConsumedSourceReferenceFact],
) -> bool {
    contract.source_reference_identities().len() == actual.len()
        && contract
            .source_reference_identities()
            .iter()
            .zip(actual.iter())
            .all(|(expected, actual)| {
                expected.label() == actual.label() && expected.identity() == actual.identity()
            })
}
