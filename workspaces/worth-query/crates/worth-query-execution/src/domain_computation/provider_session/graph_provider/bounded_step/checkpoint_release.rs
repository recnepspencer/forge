use super::WorthQueryProviderCheckpointEvidence;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryProviderCheckpointReleaseDisposition {
    Released,
    Panicked,
}

impl WorthQueryProviderCheckpointReleaseDisposition {
    pub const fn recovery_required(self) -> bool {
        matches!(self, Self::Panicked)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProviderCheckpointReleaseEvidence {
    checkpoint: WorthQueryProviderCheckpointEvidence,
    disposition: WorthQueryProviderCheckpointReleaseDisposition,
}

impl WorthQueryProviderCheckpointReleaseEvidence {
    pub(super) const fn new(
        checkpoint: WorthQueryProviderCheckpointEvidence,
        disposition: WorthQueryProviderCheckpointReleaseDisposition,
    ) -> Self {
        Self {
            checkpoint,
            disposition,
        }
    }

    pub fn checkpoint(&self) -> &WorthQueryProviderCheckpointEvidence {
        &self.checkpoint
    }

    pub const fn disposition(&self) -> WorthQueryProviderCheckpointReleaseDisposition {
        self.disposition
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryProviderCheckpointRetentionFailureKind {
    RetainedByteProbePanicked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryProviderCheckpointRetentionFailure {
    kind: WorthQueryProviderCheckpointRetentionFailureKind,
    checkpoint_identity: std::sync::Arc<str>,
    provider_generation: u64,
    release_disposition: WorthQueryProviderCheckpointReleaseDisposition,
}

impl WorthQueryProviderCheckpointRetentionFailure {
    pub(super) const fn retained_byte_probe_panicked(
        checkpoint_identity: std::sync::Arc<str>,
        provider_generation: u64,
        release_disposition: WorthQueryProviderCheckpointReleaseDisposition,
    ) -> Self {
        Self {
            kind: WorthQueryProviderCheckpointRetentionFailureKind::RetainedByteProbePanicked,
            checkpoint_identity,
            provider_generation,
            release_disposition,
        }
    }

    pub const fn kind(&self) -> WorthQueryProviderCheckpointRetentionFailureKind {
        self.kind
    }

    pub fn checkpoint_identity(&self) -> &str {
        &self.checkpoint_identity
    }

    pub const fn provider_generation(&self) -> u64 {
        self.provider_generation
    }

    pub const fn release_disposition(&self) -> WorthQueryProviderCheckpointReleaseDisposition {
        self.release_disposition
    }
}
