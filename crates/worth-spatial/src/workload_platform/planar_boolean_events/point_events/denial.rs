use crate::workload_platform::planar_boolean_events::PlanarBooleanPointEventExtractionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanPointEventExtractionDenialKind {
    MissingPredicateBindingIdentity,
    AmbiguousPredicateRelation,
    DegenerateSegmentParameterBasis,
    MissingInteriorEndpointWitness,
    NonFinitePointEventCoordinate,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanPointEventExtractionDenial {
    kind: PlanarBooleanPointEventExtractionDenialKind,
    predicate_binding_identity: String,
    segment_pair_identity: String,
    counters: PlanarBooleanPointEventExtractionCounters,
    human_reason: String,
}

impl PlanarBooleanPointEventExtractionDenial {
    pub(crate) fn new(
        kind: PlanarBooleanPointEventExtractionDenialKind,
        predicate_binding_identity: impl Into<String>,
        segment_pair_identity: impl Into<String>,
        counters: PlanarBooleanPointEventExtractionCounters,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            predicate_binding_identity: predicate_binding_identity.into(),
            segment_pair_identity: segment_pair_identity.into(),
            counters,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanPointEventExtractionDenialKind {
        self.kind
    }

    pub fn predicate_binding_identity(&self) -> &str {
        &self.predicate_binding_identity
    }

    pub fn segment_pair_identity(&self) -> &str {
        &self.segment_pair_identity
    }

    pub fn counters(&self) -> PlanarBooleanPointEventExtractionCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
