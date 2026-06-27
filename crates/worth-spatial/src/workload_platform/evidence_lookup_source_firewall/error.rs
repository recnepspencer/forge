#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupSourceFirewallErrorKind {
    MissingScanRoot,
    DuplicateFirewallRow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupSourceFirewallError {
    kind: EvidenceLookupSourceFirewallErrorKind,
    detail: String,
}

impl EvidenceLookupSourceFirewallError {
    pub(crate) fn new(
        kind: EvidenceLookupSourceFirewallErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> EvidenceLookupSourceFirewallErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
