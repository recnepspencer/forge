use crate::identity::hash_parts;

use super::facts::{
    ConsumedEffectContinuityFact, ConsumedEntityIdentityFact, ConsumedFieldValueFact,
    ConsumedMembershipFact, ConsumedRelationEndpointFact, ConsumedSourceReferenceFact,
    ConsumedTargetIdentityFact, ConsumedViewLocalIdentityFact,
};
use crate::projection_consumption::receipt::ProjectionConsumptionReceipt;
use crate::runtime::ForgeQueryMutationTargetClass;
use serde_json::Value;

use super::super::contracts::ProjectionContractSupportPosture;
use super::super::source::ProjectionSourceFamily;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProjectionFactExtractionCounters {
    declared_fact_family_count: usize,
    admitted_fact_family_count: usize,
    extracted_fact_count: usize,
    source_row_width_consumed: usize,
    source_evidence_lookup_width: usize,
    authority_reopen_count: usize,
}

impl ProjectionFactExtractionCounters {
    pub fn declared_fact_family_count(&self) -> usize {
        self.declared_fact_family_count
    }

    pub fn admitted_fact_family_count(&self) -> usize {
        self.admitted_fact_family_count
    }

    pub fn extracted_fact_count(&self) -> usize {
        self.extracted_fact_count
    }

    pub fn source_row_width_consumed(&self) -> usize {
        self.source_row_width_consumed
    }

    pub fn source_evidence_lookup_width(&self) -> usize {
        self.source_evidence_lookup_width
    }

    pub fn authority_reopen_count(&self) -> usize {
        self.authority_reopen_count
    }

    pub(crate) fn digest(&self) -> String {
        hash_parts(&[
            "projection_fact_extraction_counters_v1".to_string(),
            format!("declared:{}", self.declared_fact_family_count),
            format!("admitted:{}", self.admitted_fact_family_count),
            format!("extracted:{}", self.extracted_fact_count),
            format!("row_width:{}", self.source_row_width_consumed),
            format!("evidence_width:{}", self.source_evidence_lookup_width),
            format!("authority_reopen:{}", self.authority_reopen_count),
        ])
    }

    pub(crate) fn new(
        declared_fact_family_count: usize,
        admitted_fact_family_count: usize,
        extracted_fact_count: usize,
        source_row_width_consumed: usize,
        source_evidence_lookup_width: usize,
    ) -> Self {
        Self {
            declared_fact_family_count,
            admitted_fact_family_count,
            extracted_fact_count,
            source_row_width_consumed,
            source_evidence_lookup_width,
            authority_reopen_count: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConsumedProjectionFactSet {
    declaration_digest: String,
    contract_digest: String,
    source_family: ProjectionSourceFamily,
    source_identity: String,
    support_posture: ProjectionContractSupportPosture,
    counters: ProjectionFactExtractionCounters,
    fact_set_digest: String,
    entity_identities: Vec<ConsumedEntityIdentityFact>,
    view_local_identities: Vec<ConsumedViewLocalIdentityFact>,
    memberships: Vec<ConsumedMembershipFact>,
    display_fields: Vec<ConsumedFieldValueFact>,
    derived_scalar_fields: Vec<ConsumedFieldValueFact>,
    target_identities: Vec<ConsumedTargetIdentityFact>,
    source_references: Vec<ConsumedSourceReferenceFact>,
    effect_continuity_facts: Vec<ConsumedEffectContinuityFact>,
    relation_endpoints: Vec<ConsumedRelationEndpointFact>,
}

impl ConsumedProjectionFactSet {
    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn source_family(&self) -> ProjectionSourceFamily {
        self.source_family
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn support_posture(&self) -> &ProjectionContractSupportPosture {
        &self.support_posture
    }

    pub fn counters(&self) -> &ProjectionFactExtractionCounters {
        &self.counters
    }

    pub fn fact_set_digest(&self) -> &str {
        &self.fact_set_digest
    }

    pub fn entity_identities(&self) -> &[ConsumedEntityIdentityFact] {
        &self.entity_identities
    }

    pub fn view_local_identities(&self) -> &[ConsumedViewLocalIdentityFact] {
        &self.view_local_identities
    }

    pub fn memberships(&self) -> &[ConsumedMembershipFact] {
        &self.memberships
    }

    pub fn display_fields(&self) -> &[ConsumedFieldValueFact] {
        &self.display_fields
    }

    pub fn derived_scalar_fields(&self) -> &[ConsumedFieldValueFact] {
        &self.derived_scalar_fields
    }

    pub fn target_identities(&self) -> &[ConsumedTargetIdentityFact] {
        &self.target_identities
    }

    pub fn source_references(&self) -> &[ConsumedSourceReferenceFact] {
        &self.source_references
    }

    pub fn effect_continuity_facts(&self) -> &[ConsumedEffectContinuityFact] {
        &self.effect_continuity_facts
    }

    pub fn relation_endpoints(&self) -> &[ConsumedRelationEndpointFact] {
        &self.relation_endpoints
    }

    pub fn issue_receipt(&self) -> ProjectionConsumptionReceipt {
        ProjectionConsumptionReceipt::from_fact_set(self)
    }

    pub(crate) fn new(
        declaration_digest: impl Into<String>,
        contract_digest: impl Into<String>,
        source_family: ProjectionSourceFamily,
        source_identity: impl Into<String>,
        support_posture: ProjectionContractSupportPosture,
        counters: ProjectionFactExtractionCounters,
        entity_identities: Vec<ConsumedEntityIdentityFact>,
        view_local_identities: Vec<ConsumedViewLocalIdentityFact>,
        memberships: Vec<ConsumedMembershipFact>,
        display_fields: Vec<ConsumedFieldValueFact>,
        derived_scalar_fields: Vec<ConsumedFieldValueFact>,
        target_identities: Vec<ConsumedTargetIdentityFact>,
        source_references: Vec<ConsumedSourceReferenceFact>,
        effect_continuity_facts: Vec<ConsumedEffectContinuityFact>,
        relation_endpoints: Vec<ConsumedRelationEndpointFact>,
    ) -> Self {
        let declaration_digest = declaration_digest.into();
        let contract_digest = contract_digest.into();
        let source_identity = source_identity.into();
        let fact_set_digest = fact_set_digest(
            &declaration_digest,
            &contract_digest,
            source_family,
            &source_identity,
            &support_posture,
            &counters,
            &entity_identities,
            &view_local_identities,
            &memberships,
            &display_fields,
            &derived_scalar_fields,
            &target_identities,
            &source_references,
            &effect_continuity_facts,
            &relation_endpoints,
        );
        Self {
            declaration_digest,
            contract_digest,
            source_family,
            source_identity,
            support_posture,
            counters,
            fact_set_digest,
            entity_identities,
            view_local_identities,
            memberships,
            display_fields,
            derived_scalar_fields,
            target_identities,
            source_references,
            effect_continuity_facts,
            relation_endpoints,
        }
    }
}

fn fact_set_digest(
    declaration_digest: &str,
    contract_digest: &str,
    source_family: ProjectionSourceFamily,
    source_identity: &str,
    support_posture: &ProjectionContractSupportPosture,
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
    hash_parts(
        &std::iter::once("consumed_projection_fact_set_v1".to_string())
            .chain(std::iter::once(format!("declaration:{declaration_digest}")))
            .chain(std::iter::once(format!("contract:{contract_digest}")))
            .chain(std::iter::once(format!(
                "source_family:{}",
                source_family.as_str()
            )))
            .chain(std::iter::once(format!(
                "source_identity:{source_identity}"
            )))
            .chain(std::iter::once(format!(
                "support_posture:{}",
                support_posture.as_str()
            )))
            .chain(
                support_posture
                    .warning_kinds()
                    .iter()
                    .map(|warning| format!("warning:{}", warning.as_str())),
            )
            .chain(std::iter::once(format!("counters:{}", counters.digest())))
            .chain(entity_identities.iter().map(|fact| {
                format!(
                    "entity_identity:{}:{}",
                    fact.source_row_identity(),
                    fact.entity_identity()
                )
            }))
            .chain(view_local_identities.iter().map(|fact| {
                format!(
                    "view_local_identity:{}:{}",
                    fact.source_row_identity(),
                    fact.view_local_identity()
                )
            }))
            .chain(memberships.iter().map(|fact| {
                format!(
                    "membership:{}:{}:{}",
                    fact.source_row_identity(),
                    fact.grouping_aspect(),
                    json_value_digest(fact.grouping_value())
                )
            }))
            .chain(display_fields.iter().map(field_digest("display_field")))
            .chain(
                derived_scalar_fields
                    .iter()
                    .map(field_digest("derived_scalar")),
            )
            .chain(
                target_identities
                    .iter()
                    .map(|fact| format!("target_identity:{}", fact.target_identity())),
            )
            .chain(
                source_references
                    .iter()
                    .map(|fact| format!("source_reference:{}:{}", fact.label(), fact.identity())),
            )
            .chain(effect_continuity_facts.iter().map(|fact| {
                format!(
                    "effect_continuity:{}:{}:{}",
                    fact.family() as u8,
                    fact.outcome_class() as u8,
                    fact.prior_authoritative_identity()
                )
            }))
            .chain(relation_endpoints.iter().map(relation_endpoint_digest))
            .collect::<Vec<_>>(),
    )
}

fn field_digest<'a>(family: &'static str) -> impl Fn(&'a ConsumedFieldValueFact) -> String + 'a {
    move |fact| {
        format!(
            "{family}:{}:{}:{}",
            fact.source_row_identity(),
            fact.field_key(),
            json_value_digest(fact.value())
        )
    }
}

fn relation_endpoint_digest(fact: &ConsumedRelationEndpointFact) -> String {
    match fact {
        ConsumedRelationEndpointFact::MutationTarget {
            target_class,
            collection,
            entity_identity,
        } => format!(
            "relation_endpoint:mutation:{:?}:{}:{}",
            target_class,
            collection.as_deref().unwrap_or("none"),
            entity_identity.as_deref().unwrap_or("none")
        ),
        ConsumedRelationEndpointFact::GroupedProjection {
            source_row_identity,
            grouping_aspect,
            grouping_value,
            ..
        } => format!(
            "relation_endpoint:grouped:{}:{}:{}",
            source_row_identity,
            grouping_aspect,
            json_value_digest(grouping_value)
        ),
    }
}

fn json_value_digest(value: &Value) -> String {
    value.to_string()
}

#[allow(dead_code)]
fn _keep_target_class_used(
    target_class: ForgeQueryMutationTargetClass,
) -> ForgeQueryMutationTargetClass {
    target_class
}
