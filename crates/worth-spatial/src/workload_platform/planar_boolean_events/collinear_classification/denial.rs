use super::counters::PlanarBooleanCollinearRelationCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCollinearRelationDenialKind {
    MissingPredicateBindingIdentity,
    UnsupportedDegenerateCollinearity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCollinearRelationDenial {
    kind: PlanarBooleanCollinearRelationDenialKind,
    predicate_binding_identity: String,
    segment_pair_identity: String,
    counters: PlanarBooleanCollinearRelationCounters,
    human_reason: String,
}

impl PlanarBooleanCollinearRelationDenial {
    pub(crate) fn new(
        kind: PlanarBooleanCollinearRelationDenialKind,
        predicate_binding_identity: impl Into<String>,
        segment_pair_identity: impl Into<String>,
        counters: PlanarBooleanCollinearRelationCounters,
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

    pub fn kind(&self) -> PlanarBooleanCollinearRelationDenialKind {
        self.kind
    }

    pub fn predicate_binding_identity(&self) -> &str {
        &self.predicate_binding_identity
    }

    pub fn segment_pair_identity(&self) -> &str {
        &self.segment_pair_identity
    }

    pub fn counters(&self) -> PlanarBooleanCollinearRelationCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
