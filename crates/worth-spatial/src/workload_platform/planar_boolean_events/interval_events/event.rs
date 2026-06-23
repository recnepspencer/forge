use crate::workload_platform::planar_boolean_events::PlanarBooleanCollinearRelation;

use super::event_kind::PlanarBooleanIntervalEventKind;
use super::identity::{interval_event_identity, IntervalEventIdentityBasis};
use super::normalized_interval::PlanarBooleanNormalizedInterval;
use super::source_interval::PlanarBooleanSourceInterval;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanIntervalEvent {
    event_identity: String,
    kind: PlanarBooleanIntervalEventKind,
    collinear_relation_identity: String,
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
    normalized_interval: PlanarBooleanNormalizedInterval,
    left_source_interval: PlanarBooleanSourceInterval,
    right_source_interval: PlanarBooleanSourceInterval,
}

impl PlanarBooleanIntervalEvent {
    pub(crate) fn new(
        kind: PlanarBooleanIntervalEventKind,
        relation: &PlanarBooleanCollinearRelation,
        normalized_interval: PlanarBooleanNormalizedInterval,
        left_source_interval: PlanarBooleanSourceInterval,
        right_source_interval: PlanarBooleanSourceInterval,
    ) -> Self {
        let event_identity = interval_event_identity(IntervalEventIdentityBasis {
            kind,
            collinear_relation_identity: relation.relation_identity(),
            segment_pair_identity: relation.segment_pair_identity(),
            left_segment_identity: relation.left_segment_identity(),
            right_segment_identity: relation.right_segment_identity(),
            left_carrier_identity: relation.left_carrier_identity(),
            right_carrier_identity: relation.right_carrier_identity(),
            normalized_interval_identity: normalized_interval.normalized_interval_identity(),
            left_source_interval_identity: left_source_interval.source_interval_identity(),
            right_source_interval_identity: right_source_interval.source_interval_identity(),
        });
        Self {
            event_identity,
            kind,
            collinear_relation_identity: relation.relation_identity().to_string(),
            reduced_pair_identity: relation.reduced_pair_identity().to_string(),
            predicate_binding_identity: relation.predicate_binding_identity().to_string(),
            predicate_bound_pair_identity: relation.predicate_bound_pair_identity().to_string(),
            segment_pair_identity: relation.segment_pair_identity().to_string(),
            left_segment_identity: relation.left_segment_identity().to_string(),
            right_segment_identity: relation.right_segment_identity().to_string(),
            left_carrier_identity: relation.left_carrier_identity().to_string(),
            right_carrier_identity: relation.right_carrier_identity().to_string(),
            segment_contract_fact_digest: relation.segment_contract_fact_digest().to_string(),
            local_frame_identity: relation.local_frame_identity().to_string(),
            precision_basis_identity: relation.precision_basis_identity().to_string(),
            normalized_interval,
            left_source_interval,
            right_source_interval,
        }
    }

    pub fn event_identity(&self) -> &str {
        &self.event_identity
    }

    pub fn kind(&self) -> PlanarBooleanIntervalEventKind {
        self.kind
    }

    pub fn collinear_relation_identity(&self) -> &str {
        &self.collinear_relation_identity
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

    pub fn normalized_interval(&self) -> &PlanarBooleanNormalizedInterval {
        &self.normalized_interval
    }

    pub fn left_source_interval(&self) -> &PlanarBooleanSourceInterval {
        &self.left_source_interval
    }

    pub fn right_source_interval(&self) -> &PlanarBooleanSourceInterval {
        &self.right_source_interval
    }
}
