use super::ForgeQueryBatchWriteComponentInspection;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{ForgeQueryAspectTouch, ForgeQueryGraphCompositionResolutionMap};

pub(super) fn component_artifact_identities(
    components: &[ForgeQueryBatchWriteComponentInspection],
) -> Vec<ForgeQueryEvidenceIdentity> {
    components
        .iter()
        .enumerate()
        .map(|(index, component)| component_artifact_identity(index, component))
        .collect()
}

pub(super) fn graph_resolution_identities(
    resolution_map: &ForgeQueryGraphCompositionResolutionMap,
) -> Vec<ForgeQueryEvidenceIdentity> {
    resolution_map
        .entries()
        .iter()
        .map(|entry| {
            let aspect_digest = entry
                .aspect_touch()
                .map(ForgeQueryAspectTouch::admitted_touch_digest_part);
            forge_query_evidence_identity(ForgeQueryEvidenceScope::BatchWriteReceiptGraphResolution)
                .field_usize(
                    ForgeQueryEvidenceTag::new("component_index"),
                    entry.component_index(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("aspect_path"),
                    aspect_digest.as_deref(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("symbol_identity"),
                    entry.symbol().evidence_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("resolved_entity_identity"),
                    &entry.resolved_entity_identity().evidence_identity(),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("target_collection"),
                    entry
                        .target_collection()
                        .map(|collection| collection.evidence_identity()),
                )
                .seal()
        })
        .collect()
}

fn component_artifact_identity(
    index: usize,
    component: &ForgeQueryBatchWriteComponentInspection,
) -> ForgeQueryEvidenceIdentity {
    let symbolic_aspect_resolution_identities = symbolic_aspect_resolution_identities(component);
    let declared_aspect_operation_identities = declared_aspect_operation_identities(component);
    let target_declared_entity_identity = component
        .target_evidence()
        .declared()
        .entity_identity()
        .map(|identity| identity.evidence_identity());
    let target_resolved_entity_identity = component
        .target_evidence()
        .resolved()
        .entity_identity()
        .map(|identity| identity.evidence_identity());
    let existing_truth_resolved_target_identity = component
        .existing_truth_binding_evidence()
        .map(|evidence| evidence.resolved_target_identity().evidence_identity());
    let entity_identities = component
        .entity_identities()
        .iter()
        .map(|identity| identity.evidence_identity())
        .collect::<Vec<_>>();
    forge_query_evidence_identity(ForgeQueryEvidenceScope::BatchWriteReceiptComponent)
        .field_usize(ForgeQueryEvidenceTag::new("index"), index)
        .field_shape(ForgeQueryEvidenceTag::new("family"), component.family())
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("commit_identity"),
            &component.commit_identity().evidence_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_declared_class"),
            component
                .target_evidence()
                .declared()
                .target_class()
                .as_str(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("target_declared_collection"),
            component
                .target_evidence()
                .declared()
                .collection()
                .map(|collection| collection.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("target_declared_entity_identity"),
            target_declared_entity_identity.as_ref(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("target_resolved_class"),
            component
                .target_evidence()
                .resolved()
                .target_class()
                .as_str(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("target_resolved_collection"),
            component
                .target_evidence()
                .resolved()
                .collection()
                .map(|collection| collection.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("target_resolved_entity_identity"),
            target_resolved_entity_identity.as_ref(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("existing_assertion_verification_digest"),
            component
                .existing_truth_assertion_evidence()
                .map(|evidence| evidence.verification_evidence_identity()),
        )
        .optional_shape(
            ForgeQueryEvidenceTag::new("existing_truth_family"),
            component
                .existing_truth_binding_evidence()
                .map(|evidence| evidence.family().as_str()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("existing_truth_authoritative_identity"),
            component
                .existing_truth_binding_evidence()
                .map(|evidence| evidence.authoritative_identity().evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("existing_truth_resolved_target_identity"),
            existing_truth_resolved_target_identity.as_ref(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("existing_truth_target_collection"),
            component
                .existing_truth_binding_evidence()
                .and_then(|evidence| evidence.target_collection())
                .map(|collection| collection.evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("existing_truth_binding_digest"),
            component
                .existing_truth_binding_evidence()
                .map(|evidence| evidence.binding_digest().evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("symbolic_target_symbol"),
            component
                .symbolic_target_reference_evidence()
                .map(|evidence| evidence.symbol().evidence_identity()),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("symbolic_aspect_resolution_identity"),
            symbolic_aspect_resolution_identities.iter(),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("causality_digest"),
            component
                .causality_evidence()
                .map(|evidence| evidence.causality_digest().evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("continuity_lineage_digest"),
            component
                .continuity_mutation_evidence()
                .map(|evidence| evidence.lineage_digest().evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("continuity_resolution_digest"),
            component
                .continuity_mutation_evidence()
                .map(|evidence| evidence.continuity_resolution_digest().evidence_identity()),
        )
        .optional_evidence_identity(
            ForgeQueryEvidenceTag::new("provenance_execution_record_digest"),
            component
                .provenance_evidence()
                .map(|evidence| evidence.execution_record_digest().evidence_identity()),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("collection"),
            evidence_value_identities("component-collection", component.collections()).iter(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("entity_identity"),
            entity_identities.iter(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("declared_aspect_operation"),
            declared_aspect_operation_identities.iter(),
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("touched_aspect"),
            evidence_touch_identities(
                "component-touched-aspect",
                component.admitted_touched_aspects(),
            )
            .iter(),
        )
        .seal()
}

fn evidence_value_identities(
    role: &'static str,
    values: &[String],
) -> Vec<ForgeQueryEvidenceIdentity> {
    values
        .iter()
        .map(|value| {
            forge_query_evidence_identity(
                ForgeQueryEvidenceScope::BatchWriteReceiptInspectionArtifact,
            )
            .field_shape(ForgeQueryEvidenceTag::new("role"), role)
            .field_value(ForgeQueryEvidenceTag::new("value"), value)
            .seal()
        })
        .collect()
}

fn evidence_touch_identities(
    role: &'static str,
    touches: &[ForgeQueryAspectTouch],
) -> Vec<ForgeQueryEvidenceIdentity> {
    touches
        .iter()
        .map(|touch| {
            forge_query_evidence_identity(
                ForgeQueryEvidenceScope::BatchWriteReceiptInspectionArtifact,
            )
            .field_shape(ForgeQueryEvidenceTag::new("role"), role)
            .field_value(
                ForgeQueryEvidenceTag::new("value"),
                touch.admitted_touch_digest_part(),
            )
            .seal()
        })
        .collect()
}

fn symbolic_aspect_resolution_identities(
    component: &ForgeQueryBatchWriteComponentInspection,
) -> Vec<ForgeQueryEvidenceIdentity> {
    component
        .symbolic_aspect_resolution_evidence()
        .iter()
        .map(|evidence| {
            forge_query_evidence_identity(
                ForgeQueryEvidenceScope::BatchWriteReceiptSymbolicAspectResolution,
            )
            .field_value(
                ForgeQueryEvidenceTag::new("aspect_path"),
                evidence.aspect_touch().admitted_touch_digest_part(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("symbol_identity"),
                evidence.symbol().evidence_identity(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("resolved_entity_identity"),
                &evidence.resolved_entity_identity().evidence_identity(),
            )
            .optional_evidence_identity(
                ForgeQueryEvidenceTag::new("target_collection"),
                evidence
                    .target_collection()
                    .map(|collection| collection.evidence_identity()),
            )
            .seal()
        })
        .collect()
}

fn declared_aspect_operation_identities(
    component: &ForgeQueryBatchWriteComponentInspection,
) -> Vec<ForgeQueryEvidenceIdentity> {
    component
        .declared_aspect_operations()
        .iter()
        .map(|operation| {
            forge_query_evidence_identity(
                ForgeQueryEvidenceScope::WriteReceiptDeclaredAspectOperation,
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("kind"),
                operation.kind().as_str(),
            )
            .field_value(
                ForgeQueryEvidenceTag::new("aspect_path"),
                operation.aspect_touch().admitted_touch_digest_part(),
            )
            .seal()
        })
        .collect()
}
