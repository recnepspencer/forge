#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReplayUndoForbiddenConsumerSurfaceKind {
    OldReplayHelper,
    BroadTopologyRediscovery,
    BroadEvidenceRediscovery,
    RawReceiptAdmission,
    LocalRollbackShortcut,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoForbiddenConsumerSurfaceEnforcement {
    CompileFail,
    SourceFirewall,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReplayUndoForbiddenConsumerSurfaceRow {
    kind: ReplayUndoForbiddenConsumerSurfaceKind,
    surface: &'static str,
    enforcement: ReplayUndoForbiddenConsumerSurfaceEnforcement,
    removal_trigger: &'static str,
}

impl ReplayUndoForbiddenConsumerSurfaceRow {
    pub const fn new(
        kind: ReplayUndoForbiddenConsumerSurfaceKind,
        surface: &'static str,
        enforcement: ReplayUndoForbiddenConsumerSurfaceEnforcement,
        removal_trigger: &'static str,
    ) -> Self {
        Self {
            kind,
            surface,
            enforcement,
            removal_trigger,
        }
    }

    pub const fn kind(self) -> ReplayUndoForbiddenConsumerSurfaceKind {
        self.kind
    }

    pub const fn surface(self) -> &'static str {
        self.surface
    }

    pub const fn enforcement(self) -> ReplayUndoForbiddenConsumerSurfaceEnforcement {
        self.enforcement
    }

    pub const fn removal_trigger(self) -> &'static str {
        self.removal_trigger
    }
}

impl ReplayUndoForbiddenConsumerSurfaceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OldReplayHelper => "old replay helper",
            Self::BroadTopologyRediscovery => "broad topology rediscovery",
            Self::BroadEvidenceRediscovery => "broad evidence rediscovery",
            Self::RawReceiptAdmission => "raw receipt admission",
            Self::LocalRollbackShortcut => "local rollback shortcut",
        }
    }
}
