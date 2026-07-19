use super::{
    WorthQueryAdmittedGraphReadRelationDirection, WorthQueryGraphReadAccessShapeDigest,
    WorthQueryGraphReadFanoutPosture, WorthQueryGraphReadLifecycleClass,
    WorthQueryGraphReadOperationResolution, WorthQueryGraphReadOrderingPosture,
    WorthQueryGraphReadPredicateFamily, WorthQueryGraphReadRelationshipProofBindingPosture,
    WorthQueryGraphReadResultPressure, WorthQueryGraphReadRootPosture,
    WorthQueryGraphReadTraversalOperator,
};
use crate::runtime::WorthQueryReadScopeClass;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadAccessShape {
    digest: WorthQueryGraphReadAccessShapeDigest,
    operation_resolution: WorthQueryGraphReadOperationResolution,
    root_posture: WorthQueryGraphReadRootPosture,
    scope_class: WorthQueryReadScopeClass,
    relation_directions: Vec<WorthQueryAdmittedGraphReadRelationDirection>,
    traversal_operators: Vec<WorthQueryGraphReadTraversalOperator>,
    max_depth: usize,
    fanout_posture: WorthQueryGraphReadFanoutPosture,
    predicate_family: WorthQueryGraphReadPredicateFamily,
    ordering_posture: WorthQueryGraphReadOrderingPosture,
    result_pressure: WorthQueryGraphReadResultPressure,
    lifecycle_class: WorthQueryGraphReadLifecycleClass,
}

impl WorthQueryGraphReadAccessShape {
    pub fn digest(&self) -> &WorthQueryGraphReadAccessShapeDigest {
        &self.digest
    }

    pub fn operation_resolution(&self) -> &WorthQueryGraphReadOperationResolution {
        &self.operation_resolution
    }

    pub fn root_posture(&self) -> &WorthQueryGraphReadRootPosture {
        &self.root_posture
    }

    pub fn scope_class(&self) -> &WorthQueryReadScopeClass {
        &self.scope_class
    }

    pub fn relation_directions(&self) -> &[WorthQueryAdmittedGraphReadRelationDirection] {
        &self.relation_directions
    }

    pub fn traversal_operators(&self) -> &[WorthQueryGraphReadTraversalOperator] {
        &self.traversal_operators
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    pub fn fanout_posture(&self) -> &WorthQueryGraphReadFanoutPosture {
        &self.fanout_posture
    }

    pub fn predicate_family(&self) -> &WorthQueryGraphReadPredicateFamily {
        &self.predicate_family
    }

    pub fn ordering_posture(&self) -> &WorthQueryGraphReadOrderingPosture {
        &self.ordering_posture
    }

    pub fn result_pressure(&self) -> &WorthQueryGraphReadResultPressure {
        &self.result_pressure
    }

    pub fn lifecycle_class(&self) -> &WorthQueryGraphReadLifecycleClass {
        &self.lifecycle_class
    }

    pub fn relationship_proof_posture(
        &self,
    ) -> &WorthQueryGraphReadRelationshipProofBindingPosture {
        self.operation_resolution
            .policy_tenant_proof_binding()
            .relationship_proof_posture()
    }

    pub(crate) fn new(
        operation_resolution: WorthQueryGraphReadOperationResolution,
        root_posture: WorthQueryGraphReadRootPosture,
        scope_class: WorthQueryReadScopeClass,
        relation_directions: Vec<WorthQueryAdmittedGraphReadRelationDirection>,
        traversal_operators: Vec<WorthQueryGraphReadTraversalOperator>,
        max_depth: usize,
        fanout_posture: WorthQueryGraphReadFanoutPosture,
        predicate_family: WorthQueryGraphReadPredicateFamily,
        ordering_posture: WorthQueryGraphReadOrderingPosture,
        result_pressure: WorthQueryGraphReadResultPressure,
        lifecycle_class: WorthQueryGraphReadLifecycleClass,
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
        let digest = WorthQueryGraphReadAccessShapeDigest::from_parts(&parts);
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
