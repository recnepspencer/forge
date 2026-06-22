#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEventLedgerDenialKind {
    MissingReducedPairIdentity,
    MissingEventExtractionRequestIdentity,
    MissingSegmentCarrierSetIdentity,
    MissingSegmentPairEnumerationIdentity,
    MissingPredicateBindingIdentity,
    MissingPointEventExtractionIdentity,
    MissingCollinearRelationReceiptIdentity,
    MissingIntervalEventExtractionIdentity,
    MismatchedSegmentPairEnumeration,
    MismatchedPredicateBindingForPointEvents,
    MismatchedPredicateBindingForCollinearRelations,
    MismatchedCollinearRelationReceiptForIntervals,
    MismatchedReducedPairForPredicateBinding,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEventLedgerDenial {
    kind: PlanarBooleanEventLedgerDenialKind,
    evidence_identity: String,
    human_reason: String,
}

impl PlanarBooleanEventLedgerDenial {
    pub(crate) fn new(
        kind: PlanarBooleanEventLedgerDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanEventLedgerDenialKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
