use super::{
    ForgeQueryGraphReadAccessComplexityContract, ForgeQueryGraphReadAccessInvalidationBasis,
    ForgeQueryGraphReadAccessMemoryEstimateBasis, ForgeQueryGraphReadAccessRebuildBasis,
    ForgeQueryGraphReadAccessRequirementKind, ForgeQueryGraphReadOrderingFieldAuthority,
    ForgeQueryGraphReadPredicateFieldAuthority, ForgeQueryGraphReadRelationAuthority,
};
use crate::runtime::{
    ForgeQueryAdmittedGraphReadRelationDirection, ForgeQueryGraphReadFanoutPosture,
    ForgeQueryGraphReadLifecycleClass, ForgeQueryGraphReadOrderingPosture,
    ForgeQueryGraphReadPredicateFamily, ForgeQueryGraphReadResultPressure,
    ForgeQueryGraphReadTraversalOperator,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessRequirementRow {
    kind: ForgeQueryGraphReadAccessRequirementKind,
    rebuild_basis: ForgeQueryGraphReadAccessRebuildBasis,
    relation_name: Option<String>,
    relation_authority: Option<ForgeQueryGraphReadRelationAuthority>,
    relation_direction: Option<ForgeQueryAdmittedGraphReadRelationDirection>,
    relation_depth: Option<usize>,
    fanout_posture: Option<ForgeQueryGraphReadFanoutPosture>,
    predicate_family: Option<ForgeQueryGraphReadPredicateFamily>,
    predicate_field_authorities: Vec<ForgeQueryGraphReadPredicateFieldAuthority>,
    ordering_posture: Option<ForgeQueryGraphReadOrderingPosture>,
    ordering_field_authorities: Vec<ForgeQueryGraphReadOrderingFieldAuthority>,
    traversal_operator: Option<ForgeQueryGraphReadTraversalOperator>,
    lifecycle_class: Option<ForgeQueryGraphReadLifecycleClass>,
    result_pressure: Option<ForgeQueryGraphReadResultPressure>,
    invalidation_basis: ForgeQueryGraphReadAccessInvalidationBasis,
    complexity_contract: ForgeQueryGraphReadAccessComplexityContract,
    memory_estimate_basis: ForgeQueryGraphReadAccessMemoryEstimateBasis,
}

impl ForgeQueryGraphReadAccessRequirementRow {
    pub fn kind(&self) -> &ForgeQueryGraphReadAccessRequirementKind {
        &self.kind
    }

    pub fn rebuild_basis(&self) -> &ForgeQueryGraphReadAccessRebuildBasis {
        &self.rebuild_basis
    }

    pub fn relation_name(&self) -> Option<&str> {
        self.relation_name.as_deref()
    }

    pub fn relation_authority(&self) -> Option<&ForgeQueryGraphReadRelationAuthority> {
        self.relation_authority.as_ref()
    }

    pub fn relation_direction(&self) -> Option<&ForgeQueryAdmittedGraphReadRelationDirection> {
        self.relation_direction.as_ref()
    }

    pub fn relation_depth(&self) -> Option<usize> {
        self.relation_depth
    }

    pub fn fanout_posture(&self) -> Option<&ForgeQueryGraphReadFanoutPosture> {
        self.fanout_posture.as_ref()
    }

    pub fn predicate_family(&self) -> Option<&ForgeQueryGraphReadPredicateFamily> {
        self.predicate_family.as_ref()
    }

    pub fn predicate_field_authorities(&self) -> &[ForgeQueryGraphReadPredicateFieldAuthority] {
        &self.predicate_field_authorities
    }

    pub fn ordering_posture(&self) -> Option<&ForgeQueryGraphReadOrderingPosture> {
        self.ordering_posture.as_ref()
    }

    pub fn ordering_field_authorities(&self) -> &[ForgeQueryGraphReadOrderingFieldAuthority] {
        &self.ordering_field_authorities
    }

    pub fn traversal_operator(&self) -> Option<&ForgeQueryGraphReadTraversalOperator> {
        self.traversal_operator.as_ref()
    }

    pub fn lifecycle_class(&self) -> Option<&ForgeQueryGraphReadLifecycleClass> {
        self.lifecycle_class.as_ref()
    }

    pub fn result_pressure(&self) -> Option<&ForgeQueryGraphReadResultPressure> {
        self.result_pressure.as_ref()
    }

    pub fn invalidation_basis(&self) -> &ForgeQueryGraphReadAccessInvalidationBasis {
        &self.invalidation_basis
    }

    pub fn complexity_contract(&self) -> &ForgeQueryGraphReadAccessComplexityContract {
        &self.complexity_contract
    }

    pub fn memory_estimate_basis(&self) -> &ForgeQueryGraphReadAccessMemoryEstimateBasis {
        &self.memory_estimate_basis
    }

    pub fn semantic_slot_key(&self) -> String {
        format!(
            "slot:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.kind.as_str(),
            self.relation_direction
                .as_ref()
                .map(|direction| direction.as_str())
                .unwrap_or("none"),
            self.relation_depth
                .map(|depth| depth.to_string())
                .unwrap_or_else(|| "none".to_string()),
            self.fanout_posture
                .as_ref()
                .map(|posture| posture.as_str())
                .unwrap_or("none"),
            self.predicate_family
                .as_ref()
                .map(|family| family.as_str())
                .unwrap_or("none"),
            self.ordering_posture
                .as_ref()
                .map(|posture| posture.as_str())
                .unwrap_or("none"),
            self.traversal_operator
                .as_ref()
                .map(|operator| operator.as_str())
                .unwrap_or("none"),
            self.lifecycle_class
                .as_ref()
                .map(|lifecycle| lifecycle.as_str())
                .unwrap_or("none"),
            self.result_pressure
                .as_ref()
                .map(|pressure| pressure.as_str())
                .unwrap_or("none")
        )
    }

    pub fn digest_part(&self) -> String {
        format!(
            "requirement:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.kind.as_str(),
            self.rebuild_basis.as_str(),
            self.relation_name.as_deref().unwrap_or("none"),
            self.relation_authority
                .as_ref()
                .map(|authority| authority.digest_part())
                .unwrap_or_else(|| "none".to_string()),
            self.relation_direction
                .as_ref()
                .map(|direction| direction.as_str())
                .unwrap_or("none"),
            self.relation_depth
                .map(|depth| depth.to_string())
                .unwrap_or_else(|| "none".to_string()),
            self.fanout_posture
                .as_ref()
                .map(|posture| posture.as_str())
                .unwrap_or("none"),
            self.predicate_family
                .as_ref()
                .map(|family| family.as_str())
                .unwrap_or("none"),
            self.predicate_field_authorities
                .iter()
                .map(|authority| authority.digest_part())
                .collect::<Vec<_>>()
                .join(","),
            self.ordering_posture
                .as_ref()
                .map(|posture| posture.as_str())
                .unwrap_or("none"),
            self.ordering_field_authorities
                .iter()
                .map(|authority| authority.digest_part())
                .collect::<Vec<_>>()
                .join(","),
            self.traversal_operator
                .as_ref()
                .map(|operator| operator.as_str())
                .unwrap_or("none"),
            self.lifecycle_class
                .as_ref()
                .map(|lifecycle| lifecycle.as_str())
                .unwrap_or("none"),
            self.result_pressure
                .as_ref()
                .map(|pressure| pressure.as_str())
                .unwrap_or("none"),
            self.invalidation_basis.as_str(),
            self.complexity_contract.as_str(),
            self.memory_estimate_basis.as_str()
        )
    }

    pub(crate) fn new(
        kind: ForgeQueryGraphReadAccessRequirementKind,
        rebuild_basis: ForgeQueryGraphReadAccessRebuildBasis,
        invalidation_basis: ForgeQueryGraphReadAccessInvalidationBasis,
        complexity_contract: ForgeQueryGraphReadAccessComplexityContract,
        memory_estimate_basis: ForgeQueryGraphReadAccessMemoryEstimateBasis,
    ) -> Self {
        Self {
            kind,
            rebuild_basis,
            relation_name: None,
            relation_authority: None,
            relation_direction: None,
            relation_depth: None,
            fanout_posture: None,
            predicate_family: None,
            predicate_field_authorities: Vec::new(),
            ordering_posture: None,
            ordering_field_authorities: Vec::new(),
            traversal_operator: None,
            lifecycle_class: None,
            result_pressure: None,
            invalidation_basis,
            complexity_contract,
            memory_estimate_basis,
        }
    }

    pub(crate) fn with_relation(
        mut self,
        relation_name: impl Into<String>,
        relation_authority: ForgeQueryGraphReadRelationAuthority,
        relation_direction: ForgeQueryAdmittedGraphReadRelationDirection,
        relation_depth: usize,
    ) -> Self {
        self.relation_name = Some(relation_name.into());
        self.relation_authority = Some(relation_authority);
        self.relation_direction = Some(relation_direction);
        self.relation_depth = Some(relation_depth);
        self
    }

    pub(crate) fn with_fanout_posture(
        mut self,
        fanout_posture: ForgeQueryGraphReadFanoutPosture,
    ) -> Self {
        self.fanout_posture = Some(fanout_posture);
        self
    }

    pub(crate) fn with_predicate_family(
        mut self,
        predicate_family: ForgeQueryGraphReadPredicateFamily,
    ) -> Self {
        self.predicate_family = Some(predicate_family);
        self
    }

    pub(crate) fn with_predicate_field_authorities(
        mut self,
        mut predicate_field_authorities: Vec<ForgeQueryGraphReadPredicateFieldAuthority>,
    ) -> Self {
        predicate_field_authorities.sort_by_key(|authority| authority.digest_part());
        predicate_field_authorities.dedup();
        self.predicate_field_authorities = predicate_field_authorities;
        self
    }

    pub(crate) fn with_ordering_posture(
        mut self,
        ordering_posture: ForgeQueryGraphReadOrderingPosture,
    ) -> Self {
        self.ordering_posture = Some(ordering_posture);
        self
    }

    pub(crate) fn with_ordering_field_authorities(
        mut self,
        mut ordering_field_authorities: Vec<ForgeQueryGraphReadOrderingFieldAuthority>,
    ) -> Self {
        ordering_field_authorities.sort_by_key(|authority| authority.digest_part());
        ordering_field_authorities.dedup();
        self.ordering_field_authorities = ordering_field_authorities;
        self
    }

    pub(crate) fn with_traversal_operator(
        mut self,
        traversal_operator: ForgeQueryGraphReadTraversalOperator,
    ) -> Self {
        self.traversal_operator = Some(traversal_operator);
        self
    }

    pub(crate) fn with_lifecycle_class(
        mut self,
        lifecycle_class: ForgeQueryGraphReadLifecycleClass,
    ) -> Self {
        self.lifecycle_class = Some(lifecycle_class);
        self
    }

    pub(crate) fn with_result_pressure(
        mut self,
        result_pressure: ForgeQueryGraphReadResultPressure,
    ) -> Self {
        self.result_pressure = Some(result_pressure);
        self
    }
}
