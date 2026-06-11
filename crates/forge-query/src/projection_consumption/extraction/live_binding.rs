use super::super::consumed::{
    ConsumedEntityIdentityFact, ConsumedFieldValueFact, ConsumedProjectionFactSet,
    ConsumedSourceReferenceFact, ConsumedViewLocalIdentityFact, ProjectionFactExtractionCounters,
};
use super::super::contracts::MaterializedProjectionContract;
use super::super::facts::ProjectionFactKind;
use super::super::source::ProjectionSourceFamily;
use crate::projection_consumption::ProjectionFactExtractionError;
use crate::runtime::ForgeQueryLiveArtifactBinding;
use std::collections::BTreeSet;

pub(super) fn extract_live_binding_facts(
    contract: &MaterializedProjectionContract,
    binding: &ForgeQueryLiveArtifactBinding,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    super::ensure_contract_family(contract, ProjectionSourceFamily::LiveArtifactBinding)?;
    super::ensure_source_identity(contract.source_identity(), binding.binding_digest())?;

    let extracts_entity_identity = contract
        .fact_families()
        .iter()
        .any(|fact| fact.kind() == ProjectionFactKind::EntityIdentity);
    let extracts_view_local_identity = contract
        .fact_families()
        .iter()
        .any(|fact| fact.kind() == ProjectionFactKind::ViewLocalIdentity);
    let extracts_source_references = contract
        .fact_families()
        .iter()
        .any(|fact| fact.kind() == ProjectionFactKind::SourceReference);
    let requested_field_keys = contract
        .fact_families()
        .iter()
        .filter_map(|fact| match fact.kind() {
            ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField => {
                fact.field_key().map(str::to_string)
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();

    let mut entity_identities = Vec::new();
    let mut view_local_identities = Vec::new();
    let mut display_fields = Vec::new();
    let mut derived_scalar_fields = Vec::new();
    let mut row_count = 0;

    for view_name in binding.target_view_names() {
        let read = binding
            .read_by_name(view_name)
            .expect("binding target names should resolve");
        for (index, row) in read.rows().iter().enumerate() {
            row_count += 1;
            let row_identity =
                live_binding_row_identity(binding.binding_digest(), view_name, index);
            for fact_family in contract.fact_families() {
                match fact_family.kind() {
                    ProjectionFactKind::EntityIdentity => {
                        entity_identities.push(ConsumedEntityIdentityFact::new(
                            row_identity.as_str(),
                            row.identity(),
                        ));
                    }
                    ProjectionFactKind::ViewLocalIdentity => {
                        view_local_identities.push(ConsumedViewLocalIdentityFact::new(
                            row_identity.as_str(),
                            row_identity.as_str(),
                        ));
                    }
                    ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedScalarField => {
                        let field_key = fact_family.field_key().expect("field key required");
                        let value = row.external_row_path(field_key).ok_or_else(|| {
                            ProjectionFactExtractionError::MissingDeclaredFieldEvidence {
                                source_family: contract.source_family(),
                                source_identity: format!(
                                    "{}::{}",
                                    contract.source_identity(),
                                    row_identity
                                ),
                                field_key: field_key.to_string(),
                                fact_kind: fact_family.kind(),
                            }
                        })?;
                        let fact = ConsumedFieldValueFact::new(
                            row_identity.as_str(),
                            field_key,
                            value.clone(),
                        );
                        if fact_family.kind() == ProjectionFactKind::DisplayField {
                            display_fields.push(fact);
                        } else {
                            derived_scalar_fields.push(fact);
                        }
                    }
                    ProjectionFactKind::TargetIdentity
                    | ProjectionFactKind::SourceReference
                    | ProjectionFactKind::EffectContinuity
                    | ProjectionFactKind::Membership
                    | ProjectionFactKind::RelationEndpoint => {}
                }
            }
        }
    }

    let source_references = if extracts_source_references {
        binding_target_source_references("live_target_view", binding.target_view_names())
    } else {
        Vec::new()
    };
    if extracts_source_references
        && !super::source_reference_inventory_matches(
            contract.source_reference_identities(),
            &source_references,
        )
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
    let row_width_per_row = requested_field_keys.len() + row_identity_surface_count;
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
        contract.materialized_fact_posture().cloned(),
        ProjectionFactExtractionCounters::new(
            contract.fact_families().len(),
            contract.fact_families().len(),
            extracted_fact_count,
            row_count * row_width_per_row,
            source_references.len(),
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

fn live_binding_row_identity(binding_digest: &str, view_name: &str, index: usize) -> String {
    format!("live-binding:{binding_digest}:{view_name}:{index}")
}

fn binding_target_source_references<'a>(
    label: &'static str,
    view_names: impl Iterator<Item = &'a str>,
) -> Vec<ConsumedSourceReferenceFact> {
    view_names
        .map(|view_name| ConsumedSourceReferenceFact::new(label, view_name))
        .collect()
}
