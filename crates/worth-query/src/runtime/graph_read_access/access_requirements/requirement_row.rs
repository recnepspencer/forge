use super::{
    WorthQueryGraphReadAccessComplexityContract, WorthQueryGraphReadAccessInvalidationBasis,
    WorthQueryGraphReadAccessMemoryEstimateBasis, WorthQueryGraphReadAccessRebuildBasis,
    WorthQueryGraphReadAccessRequirementKind, WorthQueryGraphReadOrderingFieldAuthority,
    WorthQueryGraphReadPredicateFieldAuthority, WorthQueryGraphReadRelationAuthority,
};
use crate::runtime::{
    WorthQueryAdmittedGraphReadRelationDirection, WorthQueryGraphReadFanoutPosture,
    WorthQueryGraphReadLifecycleClass, WorthQueryGraphReadOperationCapabilityRequirement,
    WorthQueryGraphReadOrderingPosture, WorthQueryGraphReadPredicateFamily,
    WorthQueryGraphReadResultPressure, WorthQueryGraphReadTraversalOperator,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadAccessRequirementRow {
    kind: WorthQueryGraphReadAccessRequirementKind,
    rebuild_basis: WorthQueryGraphReadAccessRebuildBasis,
    relation_name: Option<String>,
    relation_authority: Option<WorthQueryGraphReadRelationAuthority>,
    relation_direction: Option<WorthQueryAdmittedGraphReadRelationDirection>,
    relation_depth: Option<usize>,
    fanout_posture: Option<WorthQueryGraphReadFanoutPosture>,
    predicate_family: Option<WorthQueryGraphReadPredicateFamily>,
    predicate_field_authorities: Vec<WorthQueryGraphReadPredicateFieldAuthority>,
    ordering_posture: Option<WorthQueryGraphReadOrderingPosture>,
    ordering_field_authorities: Vec<WorthQueryGraphReadOrderingFieldAuthority>,
    traversal_operator: Option<WorthQueryGraphReadTraversalOperator>,
    lifecycle_class: Option<WorthQueryGraphReadLifecycleClass>,
    result_pressure: Option<WorthQueryGraphReadResultPressure>,
    operation_capability_requirement: Option<WorthQueryGraphReadOperationCapabilityRequirement>,
    invalidation_basis: WorthQueryGraphReadAccessInvalidationBasis,
    complexity_contract: WorthQueryGraphReadAccessComplexityContract,
    memory_estimate_basis: WorthQueryGraphReadAccessMemoryEstimateBasis,
}

impl WorthQueryGraphReadAccessRequirementRow {
    pub fn kind(&self) -> &WorthQueryGraphReadAccessRequirementKind {
        &self.kind
    }

    pub fn rebuild_basis(&self) -> &WorthQueryGraphReadAccessRebuildBasis {
        &self.rebuild_basis
    }

    pub fn relation_name(&self) -> Option<&str> {
        self.relation_name.as_deref()
    }

    pub fn relation_authority(&self) -> Option<&WorthQueryGraphReadRelationAuthority> {
        self.relation_authority.as_ref()
    }

    pub fn relation_direction(&self) -> Option<&WorthQueryAdmittedGraphReadRelationDirection> {
        self.relation_direction.as_ref()
    }

    pub fn relation_depth(&self) -> Option<usize> {
        self.relation_depth
    }

    pub fn fanout_posture(&self) -> Option<&WorthQueryGraphReadFanoutPosture> {
        self.fanout_posture.as_ref()
    }

    pub fn predicate_family(&self) -> Option<&WorthQueryGraphReadPredicateFamily> {
        self.predicate_family.as_ref()
    }

    pub fn predicate_field_authorities(&self) -> &[WorthQueryGraphReadPredicateFieldAuthority] {
        &self.predicate_field_authorities
    }

    pub fn ordering_posture(&self) -> Option<&WorthQueryGraphReadOrderingPosture> {
        self.ordering_posture.as_ref()
    }

    pub fn ordering_field_authorities(&self) -> &[WorthQueryGraphReadOrderingFieldAuthority] {
        &self.ordering_field_authorities
    }

    pub fn traversal_operator(&self) -> Option<&WorthQueryGraphReadTraversalOperator> {
        self.traversal_operator.as_ref()
    }

    pub fn lifecycle_class(&self) -> Option<&WorthQueryGraphReadLifecycleClass> {
        self.lifecycle_class.as_ref()
    }

    pub fn result_pressure(&self) -> Option<&WorthQueryGraphReadResultPressure> {
        self.result_pressure.as_ref()
    }

    pub fn operation_capability_requirement(
        &self,
    ) -> Option<&WorthQueryGraphReadOperationCapabilityRequirement> {
        self.operation_capability_requirement.as_ref()
    }

    pub fn invalidation_basis(&self) -> &WorthQueryGraphReadAccessInvalidationBasis {
        &self.invalidation_basis
    }

    pub fn complexity_contract(&self) -> &WorthQueryGraphReadAccessComplexityContract {
        &self.complexity_contract
    }

    pub fn memory_estimate_basis(&self) -> &WorthQueryGraphReadAccessMemoryEstimateBasis {
        &self.memory_estimate_basis
    }

    pub fn semantic_slot_key(&self) -> String {
        format!(
            "slot:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
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
                .unwrap_or("none"),
            self.operation_capability_requirement
                .as_ref()
                .map(|requirement| requirement.digest_part())
                .unwrap_or_else(|| "none".to_string())
        )
    }

    pub fn digest_part(&self) -> String {
        format!(
            "requirement:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
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
            self.operation_capability_requirement
                .as_ref()
                .map(|requirement| requirement.digest_part())
                .unwrap_or_else(|| "none".to_string()),
            self.invalidation_basis.as_str(),
            self.complexity_contract.as_str(),
            self.memory_estimate_basis.as_str()
        )
    }

    pub(crate) fn new(
        kind: WorthQueryGraphReadAccessRequirementKind,
        rebuild_basis: WorthQueryGraphReadAccessRebuildBasis,
        invalidation_basis: WorthQueryGraphReadAccessInvalidationBasis,
        complexity_contract: WorthQueryGraphReadAccessComplexityContract,
        memory_estimate_basis: WorthQueryGraphReadAccessMemoryEstimateBasis,
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
            operation_capability_requirement: None,
            invalidation_basis,
            complexity_contract,
            memory_estimate_basis,
        }
    }

    pub(crate) fn with_relation(
        mut self,
        relation_name: impl Into<String>,
        relation_authority: WorthQueryGraphReadRelationAuthority,
        relation_direction: WorthQueryAdmittedGraphReadRelationDirection,
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
        fanout_posture: WorthQueryGraphReadFanoutPosture,
    ) -> Self {
        self.fanout_posture = Some(fanout_posture);
        self
    }

    pub(crate) fn with_predicate_family(
        mut self,
        predicate_family: WorthQueryGraphReadPredicateFamily,
    ) -> Self {
        self.predicate_family = Some(predicate_family);
        self
    }

    pub(crate) fn with_predicate_field_authorities(
        mut self,
        mut predicate_field_authorities: Vec<WorthQueryGraphReadPredicateFieldAuthority>,
    ) -> Self {
        predicate_field_authorities.sort_by_key(|authority| authority.digest_part());
        predicate_field_authorities.dedup();
        self.predicate_field_authorities = predicate_field_authorities;
        self
    }

    pub(crate) fn with_ordering_posture(
        mut self,
        ordering_posture: WorthQueryGraphReadOrderingPosture,
    ) -> Self {
        self.ordering_posture = Some(ordering_posture);
        self
    }

    pub(crate) fn with_ordering_field_authorities(
        mut self,
        mut ordering_field_authorities: Vec<WorthQueryGraphReadOrderingFieldAuthority>,
    ) -> Self {
        ordering_field_authorities.sort_by_key(|authority| authority.digest_part());
        ordering_field_authorities.dedup();
        self.ordering_field_authorities = ordering_field_authorities;
        self
    }

    pub(crate) fn with_traversal_operator(
        mut self,
        traversal_operator: WorthQueryGraphReadTraversalOperator,
    ) -> Self {
        self.traversal_operator = Some(traversal_operator);
        self
    }

    pub(crate) fn with_lifecycle_class(
        mut self,
        lifecycle_class: WorthQueryGraphReadLifecycleClass,
    ) -> Self {
        self.lifecycle_class = Some(lifecycle_class);
        self
    }

    pub(crate) fn with_result_pressure(
        mut self,
        result_pressure: WorthQueryGraphReadResultPressure,
    ) -> Self {
        self.result_pressure = Some(result_pressure);
        self
    }

    pub(crate) fn with_operation_capability_requirement(
        mut self,
        operation_capability_requirement: WorthQueryGraphReadOperationCapabilityRequirement,
    ) -> Self {
        self.operation_capability_requirement = Some(operation_capability_requirement);
        self
    }
}
