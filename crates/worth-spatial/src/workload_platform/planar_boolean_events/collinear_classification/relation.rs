use crate::workload_platform::planar_boolean_events::PlanarBooleanPredicateBoundPair;

use super::identity::collinear_relation_identity;
use super::interval_basis::PlanarBooleanCollinearIntervalBasis;
use super::relation_kind::PlanarBooleanCollinearRelationKind;
use super::touch_point::PlanarBooleanCollinearTouchPoint;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCollinearRelation {
    relation_identity: String,
    kind: PlanarBooleanCollinearRelationKind,
    reduced_pair_identity: String,
    predicate_binding_identity: String,
    predicate_bound_pair_identity: String,
    segment_pair_identity: String,
    left_segment_identity: String,
    right_segment_identity: String,
    left_carrier_identity: String,
    right_carrier_identity: String,
    segment_contract_fact_digest: String,
    local_frame_identity: String,
    precision_basis_identity: String,
    interval_basis: Option<PlanarBooleanCollinearIntervalBasis>,
    touch_point: Option<PlanarBooleanCollinearTouchPoint>,
}

impl PlanarBooleanCollinearRelation {
    pub(crate) fn new(
        kind: PlanarBooleanCollinearRelationKind,
        bound_pair: &PlanarBooleanPredicateBoundPair,
        interval_basis: Option<PlanarBooleanCollinearIntervalBasis>,
        touch_point: Option<PlanarBooleanCollinearTouchPoint>,
    ) -> Self {
        let relation_identity = collinear_relation_identity(
            bound_pair.bound_pair_identity(),
            bound_pair.segment_contract_fact_digest(),
            kind,
            interval_basis
                .as_ref()
                .map(PlanarBooleanCollinearIntervalBasis::interval_basis_identity),
            touch_point
                .as_ref()
                .map(|point| point.coordinate_fact().coordinate_fact_identity()),
        );
        Self {
            relation_identity,
            kind,
            reduced_pair_identity: bound_pair.reduced_pair_identity().to_string(),
            predicate_binding_identity: bound_pair.predicate_binding_identity().to_string(),
            predicate_bound_pair_identity: bound_pair.bound_pair_identity().to_string(),
            segment_pair_identity: bound_pair.segment_pair_identity().to_string(),
            left_segment_identity: bound_pair.left_segment_identity().to_string(),
            right_segment_identity: bound_pair.right_segment_identity().to_string(),
            left_carrier_identity: bound_pair.left_carrier_identity().to_string(),
            right_carrier_identity: bound_pair.right_carrier_identity().to_string(),
            segment_contract_fact_digest: bound_pair.segment_contract_fact_digest().to_string(),
            local_frame_identity: bound_pair.local_frame_identity().to_string(),
            precision_basis_identity: bound_pair.precision_basis_identity().to_string(),
            interval_basis,
            touch_point,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_interval_event_test_parts(
        kind: PlanarBooleanCollinearRelationKind,
        interval_basis: Option<PlanarBooleanCollinearIntervalBasis>,
    ) -> Self {
        Self {
            relation_identity: "test-collinear-relation".to_string(),
            kind,
            reduced_pair_identity: "test-reduced-pair".to_string(),
            predicate_binding_identity: "test-predicate-binding".to_string(),
            predicate_bound_pair_identity: "test-predicate-bound-pair".to_string(),
            segment_pair_identity: "test-segment-pair".to_string(),
            left_segment_identity: "test-left-segment".to_string(),
            right_segment_identity: "test-right-segment".to_string(),
            left_carrier_identity: "test-left-carrier".to_string(),
            right_carrier_identity: "test-right-carrier".to_string(),
            segment_contract_fact_digest: "test-segment-contract-fact".to_string(),
            local_frame_identity: "test-local-frame".to_string(),
            precision_basis_identity: "test-precision-basis".to_string(),
            interval_basis,
            touch_point: None,
        }
    }

    pub fn relation_identity(&self) -> &str {
        &self.relation_identity
    }

    pub fn kind(&self) -> PlanarBooleanCollinearRelationKind {
        self.kind
    }

    pub fn reduced_pair_identity(&self) -> &str {
        &self.reduced_pair_identity
    }

    pub fn predicate_binding_identity(&self) -> &str {
        &self.predicate_binding_identity
    }

    pub fn predicate_bound_pair_identity(&self) -> &str {
        &self.predicate_bound_pair_identity
    }

    pub fn segment_pair_identity(&self) -> &str {
        &self.segment_pair_identity
    }

    pub fn left_segment_identity(&self) -> &str {
        &self.left_segment_identity
    }

    pub fn right_segment_identity(&self) -> &str {
        &self.right_segment_identity
    }

    pub fn left_carrier_identity(&self) -> &str {
        &self.left_carrier_identity
    }

    pub fn right_carrier_identity(&self) -> &str {
        &self.right_carrier_identity
    }

    pub fn segment_contract_fact_digest(&self) -> &str {
        &self.segment_contract_fact_digest
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }

    pub fn interval_basis(&self) -> Option<&PlanarBooleanCollinearIntervalBasis> {
        self.interval_basis.as_ref()
    }

    pub fn touch_point(&self) -> Option<&PlanarBooleanCollinearTouchPoint> {
        self.touch_point.as_ref()
    }
}
