use super::super::consumed::{
    ConsumedFieldValueFact, ConsumedNativeValue, ConsumedProjectionFactSet,
    ConsumedSourceReferenceFact, ConsumedViewLocalIdentityFact, ProjectionFactExtractionCounters,
};
use super::super::contracts::MaterializedProjectionContract;
use super::super::facts::ProjectionFactKind;
use super::super::identity::{
    compose_retained_binding_row_identity, compose_scoped_row_source_identity,
};
use super::super::source::ProjectionSourceFamily;
use crate::projection_consumption::ProjectionFactExtractionError;
use crate::runtime::{WorthQueryDerivedArtifactBinding, WorthQueryDerivedMaterializationTarget};
use crate::runtime::{WorthQueryRetainedFieldPath, WorthQueryRetainedValueView};
use std::collections::BTreeSet;

pub(super) fn extract_retained_binding_facts(
    contract: &MaterializedProjectionContract,
    binding: &WorthQueryDerivedArtifactBinding,
) -> Result<ConsumedProjectionFactSet, ProjectionFactExtractionError> {
    super::ensure_contract_family(
        contract,
        ProjectionSourceFamily::RetainedDerivedArtifactBinding,
    )?;
    super::ensure_source_identity(contract.source_identity(), binding.binding_for_reporting())?;

    let extracts_view_local_identity = contract
        .fact_families()
        .iter()
        .any(|fact| fact.kind() == ProjectionFactKind::ViewLocalIdentity);
    let requested_field_keys = contract
        .fact_families()
        .iter()
        .filter_map(|fact| match fact.kind() {
            ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedField => fact
                .field_path()
                .map(|field_path| field_path.terminal_projection_for_boundary().to_string()),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let extracts_source_references = contract
        .fact_families()
        .iter()
        .any(|fact| fact.kind() == ProjectionFactKind::SourceReference);

    let mut view_local_identities = Vec::new();
    let mut display_fields = Vec::new();
    let mut derived_fields = Vec::new();
    let mut row_count = 0;

    for target in binding.targets() {
        let materialization = binding
            .materialization_for_target(target)
            .expect("binding targets should resolve");
        let view_name = target.view_name();
        for index in 0..materialization.retained_row_count() {
            row_count += 1;
            let row_identity =
                retained_binding_row_identity(binding.binding_for_reporting(), view_name, index);
            for fact_family in contract.fact_families() {
                match fact_family.kind() {
                    ProjectionFactKind::ViewLocalIdentity => {
                        view_local_identities.push(ConsumedViewLocalIdentityFact::new(
                            row_identity.as_str(),
                            row_identity.as_str(),
                        ));
                    }
                    ProjectionFactKind::DisplayField | ProjectionFactKind::DerivedField => {
                        let consumed_field_path = fact_family
                            .field_path()
                            .expect("field path required")
                            .clone();
                        let field_key = consumed_field_path.terminal_projection_for_boundary();
                        let field_path = WorthQueryRetainedFieldPath::from_canonical_field_path(
                            consumed_field_path.canonical_field_path().clone(),
                        );
                        let value = materialization
                            .retained_native_value_at_path(index, &field_path)
                            .ok_or_else(|| {
                                ProjectionFactExtractionError::MissingDeclaredFieldEvidence {
                                    source_family: contract.source_family(),
                                    source_identity: compose_scoped_row_source_identity(
                                        contract.source_identity(),
                                        row_identity.as_str(),
                                    ),
                                    field_key: field_key.to_string(),
                                    fact_kind: fact_family.kind(),
                                }
                            })?;
                        let value = match value {
                            WorthQueryRetainedValueView::Scalar(value) => {
                                ConsumedNativeValue::scalar(value.clone())
                            }
                            WorthQueryRetainedValueView::Struct(value) => {
                                ConsumedNativeValue::struct_value(value.clone())
                            }
                        };
                        let fact = ConsumedFieldValueFact::new_native(
                            contract,
                            row_identity.as_str(),
                            consumed_field_path,
                            value,
                        );
                        if fact_family.kind() == ProjectionFactKind::DisplayField {
                            display_fields.push(fact);
                        } else {
                            derived_fields.push(fact);
                        }
                    }
                    ProjectionFactKind::EntityIdentity
                    | ProjectionFactKind::TargetIdentity
                    | ProjectionFactKind::SourceReference
                    | ProjectionFactKind::EffectContinuity
                    | ProjectionFactKind::Membership
                    | ProjectionFactKind::RelationEndpoint => {}
                }
            }
        }
    }

    let source_references = if extracts_source_references {
        binding_target_source_references("retained_target_view", binding.targets())
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

    let row_width_per_row = requested_field_keys.len() + usize::from(extracts_view_local_identity);
    let extracted_fact_count = view_local_identities.len()
        + display_fields.len()
        + derived_fields.len()
        + source_references.len();

    Ok(ConsumedProjectionFactSet::new(
        contract.declaration_digest(),
        contract.contract_digest(),
        contract.source_family(),
        contract.source_identity_handle().clone(),
        contract.support_posture().clone(),
        contract.materialized_fact_posture().cloned(),
        ProjectionFactExtractionCounters::new(
            contract.fact_families().len(),
            contract.fact_families().len(),
            extracted_fact_count,
            row_count * row_width_per_row,
            source_references.len(),
        ),
        Vec::new(),
        view_local_identities,
        Vec::new(),
        display_fields,
        derived_fields,
        Vec::new(),
        source_references,
        Vec::new(),
        Vec::new(),
    ))
}

fn retained_binding_row_identity(binding_digest: &str, view_name: &str, index: usize) -> String {
    compose_retained_binding_row_identity(binding_digest, view_name, index)
}

fn binding_target_source_references<'a>(
    label: &'static str,
    targets: impl IntoIterator<Item = &'a WorthQueryDerivedMaterializationTarget>,
) -> Vec<ConsumedSourceReferenceFact> {
    targets
        .into_iter()
        .map(|target| ConsumedSourceReferenceFact::new(label, target.view_name()))
        .collect()
}
