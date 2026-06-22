use super::{
    ForgeQueryAdmittedGraphReadRelationDirection, ForgeQueryGraphReadAccessShapeDigest,
    ForgeQueryGraphReadFanoutPosture, ForgeQueryGraphReadLifecycleClass,
    ForgeQueryGraphReadOperationResolution, ForgeQueryGraphReadOrderingPosture,
    ForgeQueryGraphReadPredicateFamily, ForgeQueryGraphReadRelationshipProofBindingPosture,
    ForgeQueryGraphReadResultPressure, ForgeQueryGraphReadRootPosture,
    ForgeQueryGraphReadTraversalOperator,
};
use crate::runtime::ForgeQueryReadScopeClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessShape {
    digest: ForgeQueryGraphReadAccessShapeDigest,
    operation_resolution: ForgeQueryGraphReadOperationResolution,
    root_posture: ForgeQueryGraphReadRootPosture,
    scope_class: ForgeQueryReadScopeClass,
    relation_directions: Vec<ForgeQueryAdmittedGraphReadRelationDirection>,
    traversal_operators: Vec<ForgeQueryGraphReadTraversalOperator>,
    max_depth: usize,
    fanout_posture: ForgeQueryGraphReadFanoutPosture,
    predicate_family: ForgeQueryGraphReadPredicateFamily,
    ordering_posture: ForgeQueryGraphReadOrderingPosture,
    result_pressure: ForgeQueryGraphReadResultPressure,
    lifecycle_class: ForgeQueryGraphReadLifecycleClass,
}

impl ForgeQueryGraphReadAccessShape {
    pub fn digest(&self) -> &ForgeQueryGraphReadAccessShapeDigest {
        &self.digest
    }

    pub fn operation_resolution(&self) -> &ForgeQueryGraphReadOperationResolution {
        &self.operation_resolution
    }

    pub fn root_posture(&self) -> &ForgeQueryGraphReadRootPosture {
        &self.root_posture
    }

    pub fn scope_class(&self) -> &ForgeQueryReadScopeClass {
        &self.scope_class
    }

    pub fn relation_directions(&self) -> &[ForgeQueryAdmittedGraphReadRelationDirection] {
        &self.relation_directions
    }

    pub fn traversal_operators(&self) -> &[ForgeQueryGraphReadTraversalOperator] {
        &self.traversal_operators
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    pub fn fanout_posture(&self) -> &ForgeQueryGraphReadFanoutPosture {
        &self.fanout_posture
    }

    pub fn predicate_family(&self) -> &ForgeQueryGraphReadPredicateFamily {
        &self.predicate_family
    }

    pub fn ordering_posture(&self) -> &ForgeQueryGraphReadOrderingPosture {
        &self.ordering_posture
    }

    pub fn result_pressure(&self) -> &ForgeQueryGraphReadResultPressure {
        &self.result_pressure
    }

    pub fn lifecycle_class(&self) -> &ForgeQueryGraphReadLifecycleClass {
        &self.lifecycle_class
    }

    pub fn relationship_proof_posture(
        &self,
    ) -> &ForgeQueryGraphReadRelationshipProofBindingPosture {
        self.operation_resolution
            .policy_tenant_proof_binding()
            .relationship_proof_posture()
    }

    pub(crate) fn new(
        operation_resolution: ForgeQueryGraphReadOperationResolution,
        root_posture: ForgeQueryGraphReadRootPosture,
        scope_class: ForgeQueryReadScopeClass,
        relation_directions: Vec<ForgeQueryAdmittedGraphReadRelationDirection>,
        traversal_operators: Vec<ForgeQueryGraphReadTraversalOperator>,
        max_depth: usize,
        fanout_posture: ForgeQueryGraphReadFanoutPosture,
        predicate_family: ForgeQueryGraphReadPredicateFamily,
        ordering_posture: ForgeQueryGraphReadOrderingPosture,
        result_pressure: ForgeQueryGraphReadResultPressure,
        lifecycle_class: ForgeQueryGraphReadLifecycleClass,
    ) -> Self {
        let mut parts = operation_resolution.digest_parts();
        parts.push(format!("root_posture:{}", root_posture.as_str()));
        parts.push(format!("scope_class:{}", scope_class.as_str()));
        parts.extend(
            relation_directions
                .iter()
                .map(|direction| format!("relation_direction:{}", direction.as_str())),
        );
        parts.extend(
            traversal_operators
                .iter()
                .map(|operator| format!("traversal_operator:{}", operator.as_str())),
        );
        parts.push(format!("max_depth:{max_depth}"));
        parts.push(format!("fanout:{}", fanout_posture.as_str()));
        parts.push(format!("predicate_family:{}", predicate_family.as_str()));
        parts.push(format!("ordering:{}", ordering_posture.as_str()));
        parts.push(format!("result_pressure:{}", result_pressure.as_str()));
        parts.push(format!("lifecycle:{}", lifecycle_class.as_str()));
        let digest = ForgeQueryGraphReadAccessShapeDigest::from_parts(&parts);
        Self {
            digest,
            operation_resolution,
            root_posture,
            scope_class,
            relation_directions,
            traversal_operators,
            max_depth,
            fanout_posture,
            predicate_family,
            ordering_posture,
            result_pressure,
            lifecycle_class,
        }
    }
}
