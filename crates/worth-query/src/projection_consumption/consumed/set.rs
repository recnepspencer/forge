use super::facts::{
    ConsumedEffectContinuityFact, ConsumedEntityIdentityFact, ConsumedMembershipFact,
    ConsumedRelationEndpointFact, ConsumedSourceReferenceFact, ConsumedTargetIdentityFact,
    ConsumedViewLocalIdentityFact,
};
use super::field_value_fact::ConsumedFieldValueFact;
use crate::projection_consumption::identity::{
    compose_consumed_projection_fact_set_digest, compose_extraction_counters_digest,
};
use crate::projection_consumption::receipt::ProjectionConsumptionReceipt;
use crate::projection_consumption::ProjectionMaterializedFactPosture;

use super::super::contracts::ProjectionContractSupportPosture;
use super::super::source::{ProjectionSourceFamily, ProjectionSourceIdentity};

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
        compose_extraction_counters_digest(self)
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
    source_identity: ProjectionSourceIdentity,
    support_posture: ProjectionContractSupportPosture,
    materialized_fact_posture: Option<ProjectionMaterializedFactPosture>,
    counters: ProjectionFactExtractionCounters,
    fact_set_digest: String,
    entity_identities: Vec<ConsumedEntityIdentityFact>,
    view_local_identities: Vec<ConsumedViewLocalIdentityFact>,
    memberships: Vec<ConsumedMembershipFact>,
    display_fields: Vec<ConsumedFieldValueFact>,
    derived_fields: Vec<ConsumedFieldValueFact>,
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
        self.source_identity.as_str()
    }

    pub fn source_identity_handle(&self) -> &ProjectionSourceIdentity {
        &self.source_identity
    }

    pub fn support_posture(&self) -> &ProjectionContractSupportPosture {
        &self.support_posture
    }

    pub fn counters(&self) -> &ProjectionFactExtractionCounters {
        &self.counters
    }

    pub fn materialized_fact_posture(&self) -> Option<&ProjectionMaterializedFactPosture> {
        self.materialized_fact_posture.as_ref()
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

    pub fn derived_fields(&self) -> &[ConsumedFieldValueFact] {
        &self.derived_fields
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
        source_identity: ProjectionSourceIdentity,
        support_posture: ProjectionContractSupportPosture,
        materialized_fact_posture: Option<ProjectionMaterializedFactPosture>,
        counters: ProjectionFactExtractionCounters,
        entity_identities: Vec<ConsumedEntityIdentityFact>,
        view_local_identities: Vec<ConsumedViewLocalIdentityFact>,
        memberships: Vec<ConsumedMembershipFact>,
        display_fields: Vec<ConsumedFieldValueFact>,
        derived_fields: Vec<ConsumedFieldValueFact>,
        target_identities: Vec<ConsumedTargetIdentityFact>,
        source_references: Vec<ConsumedSourceReferenceFact>,
        effect_continuity_facts: Vec<ConsumedEffectContinuityFact>,
        relation_endpoints: Vec<ConsumedRelationEndpointFact>,
    ) -> Self {
        let declaration_digest = declaration_digest.into();
        let contract_digest = contract_digest.into();
        let fact_set_digest = compose_consumed_projection_fact_set_digest(
            &declaration_digest,
            &contract_digest,
            source_family,
            source_identity.as_str(),
            &support_posture,
            materialized_fact_posture.as_ref(),
            &counters,
            &entity_identities,
            &view_local_identities,
            &memberships,
            &display_fields,
            &derived_fields,
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
            materialized_fact_posture,
            counters,
            fact_set_digest,
            entity_identities,
            view_local_identities,
            memberships,
            display_fields,
            derived_fields,
            target_identities,
            source_references,
            effect_continuity_facts,
            relation_endpoints,
        }
    }
}
