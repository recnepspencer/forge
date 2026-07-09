use crate::facade::schema::{
    HistoricalInterpretationSensitivity, ProposedSchemaTransition, SchemaDiffAtom,
    SchemaDiffDetail, SchemaElementKind, SchemaElementRef, SchemaId, SchemaPublicationImpact,
    SchemaStratum, SchemaSubscriberImpact, SchemaVersionId,
};
use crate::tests::support::{aspect_key, field_key, KindId};
use worth_foundational::facade::{AspectValue, InternedString};

pub(super) fn schema_transition_for_subscriber_impact(
    target_schema_version_id: SchemaVersionId,
    subscriber_impact: SchemaSubscriberImpact,
) -> ProposedSchemaTransition {
    ProposedSchemaTransition {
        source_schema_id: SchemaId("test".to_string()),
        source_schema_version_id: SchemaVersionId(target_schema_version_id.0 - 1),
        target_schema_id: SchemaId("test".to_string()),
        target_schema_version_id,
        diff_atoms: vec![SchemaDiffAtom::new(
            SchemaElementRef::new(
                SchemaElementKind::Field,
                SchemaId("test".to_string()),
                target_schema_version_id,
                Some(KindId(1)),
                "tag",
            ),
            vec![
                SchemaStratum::StructuralShape,
                SchemaStratum::PublicationContract,
            ],
            SchemaPublicationImpact::ObservableSurfaceChanged,
            subscriber_impact,
            HistoricalInterpretationSensitivity::NotSensitive,
            SchemaDiffDetail::AddedField {
                field: field_key("tag"),
                required: false,
                default_expression: Some("null".into()),
            },
        )
        .with_boundary_visibility_proof(match subscriber_impact {
            SchemaSubscriberImpact::ConsumableSurfaceChanged => {
                crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable
            }
            SchemaSubscriberImpact::ContractUpgradeRequired => {
                crate::schema::data::SubscriberBoundaryVisibility::VisibleRequiresContractUptake
            }
            _ => crate::schema::data::SubscriberBoundaryVisibility::NotVisible,
        })],
    }
}

pub(super) fn authoritative_patch_surface_contains(
    record: &crate::facade::publication::PublishedAuthoritativeRecordPatch,
    needle: &str,
) -> bool {
    let name_key = aspect_key("name");
    record
        .authoritative_patch
        .scalar_set_for(&name_key)
        .is_some_and(|value| aspect_value_contains(value, needle))
        || record
            .authoritative_patch
            .struct_set_for(&name_key)
            .is_some_and(|struct_value| {
                struct_value
                    .fields()
                    .any(|(_, value)| aspect_value_contains(value, needle))
            })
        || record
            .authoritative_patch
            .field_sets_for(&name_key)
            .any(|field_set| aspect_value_contains(&field_set.value, needle))
}

fn aspect_value_contains(value: &AspectValue, needle: &str) -> bool {
    matches!(
        value,
        AspectValue::String(InternedString::Raw(text)) if text.contains(needle)
    )
}
