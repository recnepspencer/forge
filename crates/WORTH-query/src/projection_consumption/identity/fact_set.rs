use worth_foundational::facade::{AspectValue, InternedString};

use super::core::compose_extraction_counters_digest;
use super::scope::{scope_encoder, seal};
use crate::runtime::{WorthQueryContinuityMutationFamily, WorthQueryContinuityOutcomeClass};
use crate::WorthQueryEvidenceTag;

use super::super::consumed::{
    ConsumedEffectContinuityFact, ConsumedEntityIdentityFact, ConsumedFieldValueFact,
    ConsumedMembershipFact, ConsumedRelationEndpointFact, ConsumedSourceReferenceFact,
    ConsumedTargetIdentityFact, ConsumedViewLocalIdentityFact, ProjectionFactExtractionCounters,
};
use super::super::contracts::ProjectionContractSupportPosture;
use super::super::eligibility::ProjectionConsumptionWarningKind;
use super::super::facts::ProjectionMaterializedFactPosture;
use super::super::source::ProjectionSourceFamily;

pub(crate) fn compose_consumed_projection_fact_set_digest(
    declaration_digest: &str,
    contract_digest: &str,
    source_family: ProjectionSourceFamily,
    source_identity: &str,
    support_posture: &ProjectionContractSupportPosture,
    materialized_fact_posture: Option<&ProjectionMaterializedFactPosture>,
    counters: &ProjectionFactExtractionCounters,
    entity_identities: &[ConsumedEntityIdentityFact],
    view_local_identities: &[ConsumedViewLocalIdentityFact],
    memberships: &[ConsumedMembershipFact],
    display_fields: &[ConsumedFieldValueFact],
    derived_scalar_fields: &[ConsumedFieldValueFact],
    target_identities: &[ConsumedTargetIdentityFact],
    source_references: &[ConsumedSourceReferenceFact],
    effect_continuity_facts: &[ConsumedEffectContinuityFact],
    relation_endpoints: &[ConsumedRelationEndpointFact],
) -> String {
    let mut encoder = scope_encoder("consumed_projection_fact_set_v1")
        .field_shape(
            WorthQueryEvidenceTag::new("declaration"),
            declaration_digest,
        )
        .field_shape(WorthQueryEvidenceTag::new("contract"), contract_digest)
        .field_shape(
            WorthQueryEvidenceTag::new("source_family"),
            source_family.as_str(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("source_identity"),
            source_identity,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("support_posture"),
            support_posture.as_str(),
        );
    if let Some(posture) = materialized_fact_posture {
        encoder = encoder.field_shape(
            WorthQueryEvidenceTag::new("materialized_fact_posture"),
            posture.posture_digest(),
        );
    }
    let entity_entries = entity_identities
        .iter()
        .map(compose_entity_identity_entry)
        .collect::<Vec<_>>();
    let view_local_entries = view_local_identities
        .iter()
        .map(compose_view_local_identity_entry)
        .collect::<Vec<_>>();
    let membership_entries = memberships
        .iter()
        .map(compose_membership_entry)
        .collect::<Vec<_>>();
    let display_field_entries = display_fields
        .iter()
        .map(|fact| compose_field_value_entry("display_field", fact))
        .collect::<Vec<_>>();
    let derived_scalar_entries = derived_scalar_fields
        .iter()
        .map(|fact| compose_field_value_entry("derived_scalar", fact))
        .collect::<Vec<_>>();
    let target_entries = target_identities
        .iter()
        .map(compose_target_identity_entry)
        .collect::<Vec<_>>();
    let source_reference_entries = source_references
        .iter()
        .map(compose_source_reference_entry)
        .collect::<Vec<_>>();
    let effect_entries = effect_continuity_facts
        .iter()
        .flat_map(compose_effect_continuity_entries)
        .collect::<Vec<_>>();
    let relation_entries = relation_endpoints
        .iter()
        .map(compose_relation_endpoint_entry)
        .collect::<Vec<_>>();
    seal(
        encoder
            .field_value_sequence(
                WorthQueryEvidenceTag::new("warning_kind"),
                support_posture
                    .warning_kinds()
                    .iter()
                    .map(|warning: &ProjectionConsumptionWarningKind| warning.as_str()),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("counters"),
                &compose_extraction_counters_digest(counters),
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("entity_identity"),
                entity_entries,
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("view_local_identity"),
                view_local_entries,
            )
            .field_value_sequence(WorthQueryEvidenceTag::new("membership"), membership_entries)
            .field_value_sequence(
                WorthQueryEvidenceTag::new("display_field"),
                display_field_entries,
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("derived_scalar"),
                derived_scalar_entries,
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("target_identity"),
                target_entries,
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("source_reference"),
                source_reference_entries,
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("effect_continuity"),
                effect_entries,
            )
            .field_value_sequence(
                WorthQueryEvidenceTag::new("relation_endpoint"),
                relation_entries,
            ),
    )
}

fn compose_entity_identity_entry(fact: &ConsumedEntityIdentityFact) -> String {
    let entity_identity = fact.entity_identity().evidence_identity();
    seal(
        scope_encoder("consumed_entity_identity_entry_v1")
            .field_shape(
                WorthQueryEvidenceTag::new("source_row"),
                fact.source_row_identity(),
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("entity_identity"),
                &entity_identity,
            ),
    )
}

fn compose_view_local_identity_entry(fact: &ConsumedViewLocalIdentityFact) -> String {
    seal(
        scope_encoder("consumed_view_local_identity_entry_v1")
            .field_shape(
                WorthQueryEvidenceTag::new("source_row"),
                fact.source_row_identity(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("view_local_identity"),
                fact.view_local_identity(),
            ),
    )
}

fn compose_membership_entry(fact: &ConsumedMembershipFact) -> String {
    seal(
        scope_encoder("consumed_membership_entry_v1")
            .field_shape(
                WorthQueryEvidenceTag::new("source_row"),
                fact.source_row_identity(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("grouping_aspect"),
                fact.native_grouping_aspect_key().as_str(),
            )
            .field_value(
                WorthQueryEvidenceTag::new("grouping_value"),
                native_aspect_value_text(fact.grouping_value()),
            ),
    )
}

fn compose_field_value_entry(family: &str, fact: &ConsumedFieldValueFact) -> String {
    seal(
        scope_encoder("consumed_field_value_entry_v1")
            .field_shape(WorthQueryEvidenceTag::new("family"), family)
            .field_shape(
                WorthQueryEvidenceTag::new("source_row"),
                fact.source_row_identity(),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("field_key"),
                fact.field_path().terminal_projection_for_boundary(),
            )
            .field_value(
                WorthQueryEvidenceTag::new("value"),
                native_aspect_value_text(fact.value()),
            ),
    )
}

fn compose_target_identity_entry(fact: &ConsumedTargetIdentityFact) -> String {
    let target_identity = fact.target_identity().evidence_identity();
    seal(
        scope_encoder("consumed_target_identity_entry_v1").field_evidence_identity(
            WorthQueryEvidenceTag::new("target_identity"),
            &target_identity,
        ),
    )
}

fn compose_source_reference_entry(fact: &ConsumedSourceReferenceFact) -> String {
    seal(
        scope_encoder("consumed_source_reference_entry_v1")
            .field_shape(WorthQueryEvidenceTag::new("label"), fact.label())
            .field_shape(WorthQueryEvidenceTag::new("identity"), fact.identity()),
    )
}

fn compose_effect_continuity_entries(fact: &ConsumedEffectContinuityFact) -> Vec<String> {
    let primary = seal(
        scope_encoder("consumed_effect_continuity_entry_v1")
            .field_shape(
                WorthQueryEvidenceTag::new("family"),
                continuity_family_label(fact.family()),
            )
            .field_shape(
                WorthQueryEvidenceTag::new("outcome_class"),
                continuity_outcome_label(fact.outcome_class()),
            )
            .field_evidence_identity(
                WorthQueryEvidenceTag::new("prior_authoritative_identity"),
                fact.prior_authoritative_identity().evidence_identity(),
            ),
    );
    let successors = fact
        .successor_authoritative_identities()
        .iter()
        .map(|identity| {
            seal(
                scope_encoder("consumed_effect_continuity_successor_entry_v1")
                    .field_evidence_identity(
                        WorthQueryEvidenceTag::new("successor_authoritative_identity"),
                        identity.evidence_identity(),
                    ),
            )
        })
        .collect::<Vec<_>>();
    std::iter::once(primary).chain(successors).collect()
}

fn compose_relation_endpoint_entry(fact: &ConsumedRelationEndpointFact) -> String {
    match fact {
        ConsumedRelationEndpointFact::MutationTarget {
            target_class,
            collection,
            entity_identity,
        } => {
            let mut encoder = scope_encoder("consumed_relation_endpoint_mutation_entry_v1")
                .field_shape(
                    WorthQueryEvidenceTag::new("target_class"),
                    target_class.as_str(),
                );
            if let Some(collection) = collection.as_deref() {
                encoder = encoder.field_shape(WorthQueryEvidenceTag::new("collection"), collection);
            }
            if let Some(entity_identity) = entity_identity {
                let evidence_identity = entity_identity.evidence_identity();
                encoder = encoder.field_evidence_identity(
                    WorthQueryEvidenceTag::new("entity_identity"),
                    &evidence_identity,
                );
            }
            seal(encoder)
        }
        ConsumedRelationEndpointFact::GroupedProjection {
            source_row_identity,
            grouping_aspect,
            grouping_value,
            ..
        } => seal(
            scope_encoder("consumed_relation_endpoint_grouped_entry_v1")
                .field_shape(
                    WorthQueryEvidenceTag::new("source_row"),
                    source_row_identity,
                )
                .field_shape(
                    WorthQueryEvidenceTag::new("grouping_aspect"),
                    grouping_aspect.as_str(),
                )
                .field_value(
                    WorthQueryEvidenceTag::new("grouping_value"),
                    native_aspect_value_text(grouping_value),
                ),
        ),
    }
}

fn continuity_family_label(family: WorthQueryContinuityMutationFamily) -> &'static str {
    family.as_str()
}

fn continuity_outcome_label(outcome: WorthQueryContinuityOutcomeClass) -> &'static str {
    outcome.as_str()
}

fn native_aspect_value_text(value: &AspectValue) -> String {
    match value {
        AspectValue::String(text) => interned_string_text(text),
        AspectValue::Null => "null".to_string(),
        AspectValue::Bool(value) => format!("bool:{value}"),
        AspectValue::Int8(value) => format!("i8:{value}"),
        AspectValue::Int16(value) => format!("i16:{value}"),
        AspectValue::Int32(value) => format!("i32:{value}"),
        AspectValue::Int64(value) => format!("i64:{value}"),
        AspectValue::UInt8(value) => format!("u8:{value}"),
        AspectValue::UInt16(value) => format!("u16:{value}"),
        AspectValue::UInt32(value) => format!("u32:{value}"),
        AspectValue::UInt64(value) => format!("u64:{value}"),
        AspectValue::Float32(value) => format!("f32-bits:{}", value.bits()),
        AspectValue::Float64(value) => format!("f64-bits:{}", value.bits()),
        AspectValue::Decimal(value) => format!("decimal:{}", value.as_str()),
        AspectValue::BigInt(value) => format!("bigint:{}", value.as_str()),
        AspectValue::Rational(value) => format!(
            "rational:{}/{}",
            value.numerator.as_str(),
            value.denominator.as_str()
        ),
        AspectValue::Bytes(value) => format!("bytes-ref:{}", value.0),
        AspectValue::Uuid(value) => value.iter().map(|byte| format!("{byte:02x}")).collect(),
        AspectValue::Date(value) => format!("date-days:{}", value.days_from_unix_epoch),
        AspectValue::Time(value) => format!("time-nanos:{}", value.nanos_since_midnight),
        AspectValue::Timestamp(value) => {
            format!("timestamp-micros:{}", value.micros_since_unix_epoch)
        }
        AspectValue::TimestampTz(value) => format!(
            "timestamp-tz:{}:{}",
            value.utc_micros_since_unix_epoch, value.offset_minutes
        ),
        AspectValue::EntityRef(value) => format!(
            "entity-ref:{}:{}:{}",
            value.partition_id.0, value.local_slot.0, value.generation.0
        ),
        AspectValue::ContentRef(value) => format!("content-ref:{}", value.0),
    }
}

fn interned_string_text(value: &InternedString) -> String {
    match value {
        InternedString::Raw(text) => text.clone(),
        InternedString::Symbol(symbol) => format!("symbol:{}", symbol.0),
    }
}
