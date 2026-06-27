use super::row::EvidenceLookupSourceFirewallExceptionKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupSourceFirewallExceptionSummary {
    kind: EvidenceLookupSourceFirewallExceptionKind,
    row_count: usize,
}

impl EvidenceLookupSourceFirewallExceptionSummary {
    pub(crate) fn new(kind: EvidenceLookupSourceFirewallExceptionKind, row_count: usize) -> Self {
        Self { kind, row_count }
    }

    pub const fn kind(&self) -> EvidenceLookupSourceFirewallExceptionKind {
        self.kind
    }

    pub const fn row_count(&self) -> usize {
        self.row_count
    }
}
