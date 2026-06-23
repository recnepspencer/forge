#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryJournalReplayDenialKind {
    InvalidSegmentBounds,
    UnknownSegmentIdentity,
    StaleBasisReplay,
    CrossSchemeReplay,
    JournalGap,
}

impl ForgeQueryJournalReplayDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidSegmentBounds => "invalid-segment-bounds",
            Self::UnknownSegmentIdentity => "unknown-segment-identity",
            Self::StaleBasisReplay => "stale-basis-replay",
            Self::CrossSchemeReplay => "cross-scheme-replay",
            Self::JournalGap => "journal-gap",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryJournalReplayDenial {
    kind: ForgeQueryJournalReplayDenialKind,
    message: String,
}

impl ForgeQueryJournalReplayDenial {
    pub(in crate::runtime) fn new(
        kind: ForgeQueryJournalReplayDenialKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> ForgeQueryJournalReplayDenialKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
