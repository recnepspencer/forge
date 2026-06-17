use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanPredicateBoundPair, PlanarBooleanSharedEndpointEvent,
};

use super::coordinate_fact::PlanarBooleanPointEventCoordinateFact;
use super::event_kind::PlanarBooleanPointEventKind;
use super::identity::{
    deduplicated_point_event_identity, point_event_identity, PointEventIdentityBasis,
};
use super::segment_parameter::PlanarBooleanPointEventSegmentParameterFact;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanPointEvent {
    event_identity: String,
    kind: PlanarBooleanPointEventKind,
    coordinate_fact: PlanarBooleanPointEventCoordinateFact,
    segment_pair_identity: String,
    segment_pair_identities: Vec<String>,
    left_segment_identity: String,
    right_segment_identity: String,
    left_carrier_identity: String,
    right_carrier_identity: String,
    participating_carrier_identities: Vec<String>,
    operand_a_parameter: PlanarBooleanPointEventSegmentParameterFact,
    operand_b_parameter: PlanarBooleanPointEventSegmentParameterFact,
    predicate_binding_identity: String,
    predicate_bound_pair_identity: String,
    segment_contract_fact_digest: String,
    endpoint_source_identities: Vec<String>,
    endpoint_projection_fact_digests: Vec<String>,
    predicate_receipt_identities: Vec<String>,
    shared_endpoint_event: Option<PlanarBooleanSharedEndpointEvent>,
}

impl PlanarBooleanPointEvent {
    pub(crate) fn new(
        kind: PlanarBooleanPointEventKind,
        coordinate_fact: PlanarBooleanPointEventCoordinateFact,
        operand_a_parameter: PlanarBooleanPointEventSegmentParameterFact,
        operand_b_parameter: PlanarBooleanPointEventSegmentParameterFact,
        bound_pair: &PlanarBooleanPredicateBoundPair,
    ) -> Self {
        let event_identity = point_event_identity(PointEventIdentityBasis {
            segment_pair_identity: bound_pair.segment_pair_identity(),
            left_segment_identity: bound_pair.left_segment_identity(),
            right_segment_identity: bound_pair.right_segment_identity(),
            left_carrier_identity: bound_pair.left_carrier_identity(),
            right_carrier_identity: bound_pair.right_carrier_identity(),
            local_frame_identity: bound_pair.local_frame_identity(),
            precision_basis_identity: bound_pair.precision_basis_identity(),
            coordinate_fact_identity: coordinate_fact.coordinate_fact_identity(),
            kind,
        });
        let left_carrier_identity = bound_pair.left_carrier_identity().to_string();
        let right_carrier_identity = bound_pair.right_carrier_identity().to_string();
        let endpoint_source_identities = bound_pair
            .segment_basis()
            .endpoint_source_identities()
            .map(str::to_string)
            .to_vec();
        let endpoint_projection_fact_digests = bound_pair
            .segment_basis()
            .endpoint_projection_fact_digests()
            .map(str::to_string)
            .to_vec();
        Self {
            event_identity,
            kind,
            coordinate_fact,
            segment_pair_identity: bound_pair.segment_pair_identity().to_string(),
            segment_pair_identities: vec![bound_pair.segment_pair_identity().to_string()],
            left_segment_identity: bound_pair.left_segment_identity().to_string(),
            right_segment_identity: bound_pair.right_segment_identity().to_string(),
            left_carrier_identity: left_carrier_identity.clone(),
            right_carrier_identity: right_carrier_identity.clone(),
            participating_carrier_identities: canonical_values(vec![
                left_carrier_identity,
                right_carrier_identity,
            ]),
            operand_a_parameter,
            operand_b_parameter,
            predicate_binding_identity: bound_pair.predicate_binding_identity().to_string(),
            predicate_bound_pair_identity: bound_pair.bound_pair_identity().to_string(),
            segment_contract_fact_digest: bound_pair.segment_contract_fact_digest().to_string(),
            endpoint_source_identities,
            endpoint_projection_fact_digests,
            predicate_receipt_identities: bound_pair
                .segment_basis()
                .orientation_fact_digests()
                .map(str::to_string)
                .to_vec(),
            shared_endpoint_event: None,
        }
    }

    pub(crate) fn new_shared_endpoint(
        coordinate_fact: PlanarBooleanPointEventCoordinateFact,
        operand_a_parameter: PlanarBooleanPointEventSegmentParameterFact,
        operand_b_parameter: PlanarBooleanPointEventSegmentParameterFact,
        bound_pair: &PlanarBooleanPredicateBoundPair,
        endpoint_indices: [usize; 2],
    ) -> Self {
        let mut event = Self::new(
            PlanarBooleanPointEventKind::SharedEndpoint,
            coordinate_fact,
            operand_a_parameter,
            operand_b_parameter,
            bound_pair,
        );
        let source_identities = endpoint_indices
            .iter()
            .map(|index| event.endpoint_source_identities[*index].clone())
            .collect::<Vec<_>>();
        let projection_digests = endpoint_indices
            .iter()
            .map(|index| event.endpoint_projection_fact_digests[*index].clone())
            .collect::<Vec<_>>();
        event.shared_endpoint_event = Some(PlanarBooleanSharedEndpointEvent::new(
            source_identities,
            projection_digests,
            event.participating_carrier_identities.clone(),
        ));
        event
    }

    pub(crate) fn merge_duplicate_report(&mut self, other: Self) {
        append_canonical(
            &mut self.segment_pair_identities,
            other.segment_pair_identities,
        );
        append_canonical(
            &mut self.participating_carrier_identities,
            other.participating_carrier_identities,
        );
        append_canonical(
            &mut self.endpoint_source_identities,
            other.endpoint_source_identities,
        );
        append_canonical(
            &mut self.endpoint_projection_fact_digests,
            other.endpoint_projection_fact_digests,
        );
        append_canonical(
            &mut self.predicate_receipt_identities,
            other.predicate_receipt_identities,
        );
        match (&mut self.shared_endpoint_event, other.shared_endpoint_event) {
            (Some(existing), Some(incoming)) => existing.merge_with(&incoming),
            (None, incoming) => self.shared_endpoint_event = incoming,
            _ => {}
        }
        self.canonicalize_deduplicated_identity();
    }

    pub(crate) fn canonicalize_deduplicated_identity(&mut self) {
        let endpoint_source_identities = self.identity_endpoint_source_identities();
        let endpoint_projection_fact_digests = self.identity_endpoint_projection_fact_digests();
        self.event_identity = deduplicated_point_event_identity(
            self.kind,
            self.coordinate_fact.coordinate_fact_identity(),
            &self.participating_carrier_identities,
            endpoint_source_identities,
            endpoint_projection_fact_digests,
        );
    }

    pub fn event_identity(&self) -> &str {
        &self.event_identity
    }

    pub fn kind(&self) -> PlanarBooleanPointEventKind {
        self.kind
    }

    pub fn coordinate_fact(&self) -> &PlanarBooleanPointEventCoordinateFact {
        &self.coordinate_fact
    }

    pub fn segment_pair_identity(&self) -> &str {
        &self.segment_pair_identity
    }

    pub fn segment_pair_identities(&self) -> &[String] {
        &self.segment_pair_identities
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

    pub fn participating_carrier_identities(&self) -> &[String] {
        &self.participating_carrier_identities
    }

    pub fn operand_a_parameter(&self) -> &PlanarBooleanPointEventSegmentParameterFact {
        &self.operand_a_parameter
    }

    pub fn operand_b_parameter(&self) -> &PlanarBooleanPointEventSegmentParameterFact {
        &self.operand_b_parameter
    }

    pub fn predicate_binding_identity(&self) -> &str {
        &self.predicate_binding_identity
    }

    pub fn predicate_bound_pair_identity(&self) -> &str {
        &self.predicate_bound_pair_identity
    }

    pub fn segment_contract_fact_digest(&self) -> &str {
        &self.segment_contract_fact_digest
    }

    pub fn endpoint_source_identities(&self) -> &[String] {
        &self.endpoint_source_identities
    }

    pub fn source_endpoint_identities(&self) -> &[String] {
        &self.endpoint_source_identities
    }

    pub fn endpoint_projection_fact_digests(&self) -> &[String] {
        &self.endpoint_projection_fact_digests
    }

    pub fn predicate_receipt_identities(&self) -> &[String] {
        &self.predicate_receipt_identities
    }

    pub fn shared_endpoint_event(&self) -> Option<&PlanarBooleanSharedEndpointEvent> {
        self.shared_endpoint_event.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn for_split_candidate_test(
        kind: PlanarBooleanPointEventKind,
        coordinate_fact: PlanarBooleanPointEventCoordinateFact,
        operand_a_parameter: PlanarBooleanPointEventSegmentParameterFact,
        operand_b_parameter: PlanarBooleanPointEventSegmentParameterFact,
        participating_carrier_identities: Vec<String>,
        shared_endpoint_source_identities: Vec<String>,
        shared_endpoint_projection_fact_digests: Vec<String>,
    ) -> Self {
        let participating_carrier_identities = canonical_values(participating_carrier_identities);
        let endpoint_source_identities = vec![
            "test-start-source-endpoint".to_string(),
            "test-end-source-endpoint".to_string(),
        ];
        let endpoint_projection_fact_digests = vec![
            "test-start-projected-endpoint".to_string(),
            "test-end-projected-endpoint".to_string(),
        ];
        let shared_endpoint_event = (!shared_endpoint_source_identities.is_empty()).then(|| {
            PlanarBooleanSharedEndpointEvent::new(
                shared_endpoint_source_identities,
                shared_endpoint_projection_fact_digests,
                participating_carrier_identities.clone(),
            )
        });
        let event_identity = deduplicated_point_event_identity(
            kind,
            coordinate_fact.coordinate_fact_identity(),
            &participating_carrier_identities,
            &endpoint_source_identities,
            &endpoint_projection_fact_digests,
        );
        Self {
            event_identity,
            kind,
            coordinate_fact,
            segment_pair_identity: "test-segment-pair".to_string(),
            segment_pair_identities: vec!["test-segment-pair".to_string()],
            left_segment_identity: operand_a_parameter.segment_identity().to_string(),
            right_segment_identity: operand_b_parameter.segment_identity().to_string(),
            left_carrier_identity: operand_a_parameter.carrier_identity().to_string(),
            right_carrier_identity: operand_b_parameter.carrier_identity().to_string(),
            participating_carrier_identities,
            operand_a_parameter,
            operand_b_parameter,
            predicate_binding_identity: "test-predicate-binding".to_string(),
            predicate_bound_pair_identity: "test-predicate-bound-pair".to_string(),
            segment_contract_fact_digest: "test-segment-contract-fact".to_string(),
            endpoint_source_identities,
            endpoint_projection_fact_digests,
            predicate_receipt_identities: vec!["test-predicate-receipt".to_string()],
            shared_endpoint_event,
        }
    }
}

impl PlanarBooleanPointEvent {
    fn identity_endpoint_source_identities(&self) -> &[String] {
        match &self.shared_endpoint_event {
            Some(shared_endpoint_event) => shared_endpoint_event.source_endpoint_identities(),
            None => &self.endpoint_source_identities,
        }
    }

    fn identity_endpoint_projection_fact_digests(&self) -> &[String] {
        match &self.shared_endpoint_event {
            Some(shared_endpoint_event) => shared_endpoint_event.endpoint_projection_fact_digests(),
            None => &self.endpoint_projection_fact_digests,
        }
    }
}

fn append_canonical(target: &mut Vec<String>, incoming: Vec<String>) {
    target.extend(incoming);
    *target = canonical_values(std::mem::take(target));
}

fn canonical_values(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}
