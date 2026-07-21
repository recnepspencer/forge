#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiCrossLaneSemanticFamily {
    LaneChangeIdentity,
    CommandMeaning,
    QueryBindingMeaning,
    AccessibilityMeaning,
    DiagnosticsMeaning,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiCrossLaneSemanticAuthority {
    DirectReferenceMatch,
    QueryOwnedRebindReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthUiCrossLaneSemanticReference {
    family: WorthUiCrossLaneSemanticFamily,
    identity: String,
    active_digest: u64,
    candidate_digest: u64,
    authority: WorthUiCrossLaneSemanticAuthority,
}

impl WorthUiCrossLaneSemanticReference {
    pub(crate) fn new(
        family: WorthUiCrossLaneSemanticFamily,
        identity: impl Into<String>,
        active_digest: u64,
        candidate_digest: u64,
        authority: WorthUiCrossLaneSemanticAuthority,
    ) -> Self {
        Self {
            family,
            identity: identity.into(),
            active_digest,
            candidate_digest,
            authority,
        }
    }

    pub fn family(&self) -> WorthUiCrossLaneSemanticFamily {
        self.family
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn active_digest(&self) -> u64 {
        self.active_digest
    }

    pub fn candidate_digest(&self) -> u64 {
        self.candidate_digest
    }

    pub fn authority(&self) -> WorthUiCrossLaneSemanticAuthority {
        self.authority
    }

    pub fn preserves_meaning(&self) -> bool {
        self.active_digest == self.candidate_digest
            || self.authority == WorthUiCrossLaneSemanticAuthority::QueryOwnedRebindReceipt
    }
}
