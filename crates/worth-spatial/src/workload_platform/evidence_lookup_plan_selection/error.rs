use super::counters::EvidenceLookupPlanSelectionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupPlanSelectionErrorKind {
    CatalogAdmissionDigestMismatch,
    MissingTopologyPlanPosture,
    AdmittedSupportCardinalityMismatch,
    DuplicateAdmittedSupportFamily,
    MissingAdmittedSupportFamily,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupPlanSelectionError {
    kind: EvidenceLookupPlanSelectionErrorKind,
    detail: String,
    counters: EvidenceLookupPlanSelectionCounters,
}

impl EvidenceLookupPlanSelectionError {
    pub(crate) fn new(
        kind: EvidenceLookupPlanSelectionErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            counters: EvidenceLookupPlanSelectionCounters::default(),
        }
    }

    pub(crate) const fn with_counters(
        mut self,
        counters: EvidenceLookupPlanSelectionCounters,
    ) -> Self {
        self.counters = counters;
        self
    }

    pub const fn kind(&self) -> EvidenceLookupPlanSelectionErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> &EvidenceLookupPlanSelectionCounters {
        &self.counters
    }
}
