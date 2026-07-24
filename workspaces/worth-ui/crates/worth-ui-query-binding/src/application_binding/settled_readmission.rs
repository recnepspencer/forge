#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSettledSnapshotReadmissionDenial {
    QueryNotInstalled,
    ForeignInstalledReference,
    ProjectionNotAdmitted,
    StaleSettlementReference,
}

/// Borrowed proof that the binding owner matched a supplied UI consequence to
/// the settlement revision currently retained for its exact Query binding.
pub struct WorthUiReadmittedSettledSnapshotFact<'a> {
    fact: &'a super::WorthUiSettledSnapshotFact,
}

impl<'a> WorthUiReadmittedSettledSnapshotFact<'a> {
    pub(crate) fn new(fact: &'a super::WorthUiSettledSnapshotFact) -> Self {
        Self { fact }
    }

    pub fn fact(&self) -> &'a super::WorthUiSettledSnapshotFact {
        self.fact
    }
}
