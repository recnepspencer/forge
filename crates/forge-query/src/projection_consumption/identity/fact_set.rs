use serde_json::Value;

use super::core::compose_extraction_counters_digest;
use super::scope::{compose_sequence_digest, scope_encoder, seal};
use crate::runtime::{ForgeQueryContinuityMutationFamily, ForgeQueryContinuityOutcomeClass};
use crate::ForgeQueryEvidenceTag;

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
            ForgeQueryEvidenceTag::new("declaration"),
            declaration_digest,
        )
        .field_shape(ForgeQueryEvidenceTag::new("contract"), contract_digest)
        .field_shape(
            ForgeQueryEvidenceTag::new("source_family"),
            source_family.as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_identity"),
            source_identity,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("support_posture"),
            support_posture.as_str(),
        );
    if let Some(posture) = materialized_fact_posture {
        encoder = encoder.field_shape(
            ForgeQueryEvidenceTag::new("materialized_fact_posture"),
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
                ForgeQueryEvidenceTag::new("warning_kind"),
                support_posture
                    .warning_kinds()
                    .iter()
                    .map(|warning: &ProjectionConsumptionWarningKind| warning.as_str()),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("counters"),
                &compose_extraction_counters_digest(counters),
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("entity_identity"),
                entity_entries,
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("view_local_identity"),
                view_local_entries,
            )
            .field_value_sequence(ForgeQueryEvidenceTag::new("membership"), membership_entries)
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("display_field"),
                display_field_entries,
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("derived_scalar"),
                derived_scalar_entries,
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("target_identity"),
                target_entries,
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("source_reference"),
                source_reference_entries,
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("effect_continuity"),
                effect_entries,
            )
            .field_value_sequence(
                ForgeQueryEvidenceTag::new("relation_endpoint"),
                relation_entries,
            ),
    )
}

fn compose_entity_identity_entry(fact: &ConsumedEntityIdentityFact) -> String {
    let entity_identity = fact.entity_identity().evidence_identity();
    seal(
        scope_encoder("consumed_entity_identity_entry_v1")
            .field_shape(
                ForgeQueryEvidenceTag::new("source_row"),
                fact.source_row_identity(),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("entity_identity"),
                &entity_identity,
            ),
    )
}

fn compose_view_local_identity_entry(fact: &ConsumedViewLocalIdentityFact) -> String {
    seal(
        scope_encoder("consumed_view_local_identity_entry_v1")
            .field_shape(
                ForgeQueryEvidenceTag::new("source_row"),
                fact.source_row_identity(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("view_local_identity"),
                fact.view_local_identity(),
            ),
    )
}

fn compose_membership_entry(fact: &ConsumedMembershipFact) -> String {
    seal(
        scope_encoder("consumed_membership_entry_v1")
            .field_shape(
                ForgeQueryEvidenceTag::new("source_row"),
                fact.source_row_identity(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("grouping_aspect"),
                fact.native_grouping_aspect_key().as_str(),
            )
            .field_value(
                ForgeQueryEvidenceTag::new("grouping_value"),
                json_value_text(fact.grouping_value()),
            ),
    )
}

fn compose_field_value_entry(family: &str, fact: &ConsumedFieldValueFact) -> String {
    seal(
        scope_encoder("consumed_field_value_entry_v1")
            .field_shape(ForgeQueryEvidenceTag::new("family"), family)
            .field_shape(
                ForgeQueryEvidenceTag::new("source_row"),
                fact.source_row_identity(),
            )
            .field_shape(ForgeQueryEvidenceTag::new("field_key"), fact.field_key())
            .field_value(
                ForgeQueryEvidenceTag::new("value"),
                json_value_text(fact.value()),
            ),
    )
}

fn compose_target_identity_entry(fact: &ConsumedTargetIdentityFact) -> String {
    let target_identity = fact.target_identity().evidence_identity();
    seal(
        scope_encoder("consumed_target_identity_entry_v1").field_evidence_identity(
            ForgeQueryEvidenceTag::new("target_identity"),
            &target_identity,
        ),
    )
}

fn compose_source_reference_entry(fact: &ConsumedSourceReferenceFact) -> String {
    seal(
        scope_encoder("consumed_source_reference_entry_v1")
            .field_shape(ForgeQueryEvidenceTag::new("label"), fact.label())
            .field_shape(ForgeQueryEvidenceTag::new("identity"), fact.identity()),
    )
}

fn compose_effect_continuity_entries(fact: &ConsumedEffectContinuityFact) -> Vec<String> {
    let primary = seal(
        scope_encoder("consumed_effect_continuity_entry_v1")
            .field_shape(
                ForgeQueryEvidenceTag::new("family"),
                continuity_family_label(fact.family()),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("outcome_class"),
                continuity_outcome_label(fact.outcome_class()),
            )
            .field_evidence_identity(
                ForgeQueryEvidenceTag::new("prior_authoritative_identity"),
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
                        ForgeQueryEvidenceTag::new("successor_authoritative_identity"),
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
                    ForgeQueryEvidenceTag::new("target_class"),
                    target_class.as_str(),
                );
            if let Some(collection) = collection.as_deref() {
                encoder = encoder.field_shape(ForgeQueryEvidenceTag::new("collection"), collection);
            }
            if let Some(entity_identity) = entity_identity {
                let evidence_identity = entity_identity.evidence_identity();
                encoder = encoder.field_evidence_identity(
                    ForgeQueryEvidenceTag::new("entity_identity"),
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
                    ForgeQueryEvidenceTag::new("source_row"),
                    source_row_identity,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("grouping_aspect"),
                    grouping_aspect.as_str(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("grouping_value"),
                    json_value_text(grouping_value),
                ),
        ),
    }
}

fn continuity_family_label(family: ForgeQueryContinuityMutationFamily) -> &'static str {
    family.as_str()
}

fn continuity_outcome_label(outcome: ForgeQueryContinuityOutcomeClass) -> &'static str {
    outcome.as_str()
}

fn json_value_text(value: &Value) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| value.to_string())
}

pub(crate) fn compose_json_canonical_digest(value: &Value) -> String {
    compose_sequence_digest(
        "projection_consumption_json_canonical_v1",
        "json",
        [json_value_text(value)],
    )
}
