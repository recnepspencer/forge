use super::counters::PlanarBooleanIntervalEventExtractionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanIntervalEventExtractionDenialKind {
    MissingCollinearRelationReceiptIdentity,
    MissingIntervalBasisForIntervalRelation,
    CollapsedIntervalAfterNormalization,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanIntervalEventExtractionDenial {
    kind: PlanarBooleanIntervalEventExtractionDenialKind,
    collinear_relation_receipt_identity: String,
    collinear_relation_identity: String,
    segment_pair_identity: String,
    counters: PlanarBooleanIntervalEventExtractionCounters,
    human_reason: String,
}

impl PlanarBooleanIntervalEventExtractionDenial {
    pub(crate) fn new(
        kind: PlanarBooleanIntervalEventExtractionDenialKind,
        collinear_relation_receipt_identity: impl Into<String>,
        collinear_relation_identity: impl Into<String>,
        segment_pair_identity: impl Into<String>,
        counters: PlanarBooleanIntervalEventExtractionCounters,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            collinear_relation_receipt_identity: collinear_relation_receipt_identity.into(),
            collinear_relation_identity: collinear_relation_identity.into(),
            segment_pair_identity: segment_pair_identity.into(),
            counters,
            human_reason: human_reason.into(),
        }
    }

    pub fn kind(&self) -> PlanarBooleanIntervalEventExtractionDenialKind {
        self.kind
    }

    pub fn collinear_relation_receipt_identity(&self) -> &str {
        &self.collinear_relation_receipt_identity
    }

    pub fn collinear_relation_identity(&self) -> &str {
        &self.collinear_relation_identity
    }

    pub fn segment_pair_identity(&self) -> &str {
        &self.segment_pair_identity
    }

    pub fn counters(&self) -> PlanarBooleanIntervalEventExtractionCounters {
        self.counters
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
