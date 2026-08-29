#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMotionProducedFactKind {
    Started,
    Retargeted(super::UiMotionRetargetDisposition),
    Terminal(super::UiMotionTerminalCause),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiMotionProducedFact {
    publication_sequence: u64,
    track: super::UiMotionTrackIdentity,
    target: super::UiMotionTargetIdentity,
    predecessor_revision: u64,
    successor_revision: u64,
    channels: super::UiMotionPropertyChannels,
    kind: UiMotionProducedFactKind,
}

impl UiMotionProducedFact {
    pub(super) const fn new(
        publication_sequence: u64,
        track: super::UiMotionTrackIdentity,
        request: super::UiMotionTransitionRequest,
        kind: UiMotionProducedFactKind,
    ) -> Self {
        Self {
            publication_sequence,
            track,
            target: request.successor().target(),
            predecessor_revision: request.predecessor().owner_revision(),
            successor_revision: request.successor().owner_revision(),
            channels: request.declaration().channels(),
            kind,
        }
    }

    pub(crate) const fn publication_sequence(self) -> u64 {
        self.publication_sequence
    }

    pub(crate) const fn track(self) -> super::UiMotionTrackIdentity {
        self.track
    }

    pub(crate) const fn target(self) -> super::UiMotionTargetIdentity {
        self.target
    }

    pub(crate) const fn predecessor_revision(self) -> u64 {
        self.predecessor_revision
    }

    pub(crate) const fn successor_revision(self) -> u64 {
        self.successor_revision
    }

    pub(crate) const fn channels(self) -> super::UiMotionPropertyChannels {
        self.channels
    }

    pub(crate) const fn kind(self) -> UiMotionProducedFactKind {
        self.kind
    }
}
