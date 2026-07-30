use crate::{
    BackendResidueKind, PartialPublicationCrashEdge, PartialPublicationEvidence,
    TornPublicationDenial,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartialPublicationObservedSource {
    PersistedCrashEdge(PartialPublicationCrashEdge),
    BackendResidue {
        residue: BackendResidueKind,
        residue_digest: String,
    },
    LiveAcknowledgmentMemory {
        memory_digest: String,
    },
    LogOnly {
        log_digest: String,
    },
    TornPublication(TornPublicationDenial),
    InsufficientPersistedEvidence {
        ambiguity_digest: String,
    },
}

impl PartialPublicationObservedSource {
    pub fn persisted_crash_edge(edge: PartialPublicationCrashEdge) -> Self {
        Self::PersistedCrashEdge(edge)
    }

    pub fn backend_residue(residue: BackendResidueKind, residue_digest: impl Into<String>) -> Self {
        Self::BackendResidue {
            residue,
            residue_digest: residue_digest.into(),
        }
    }

    pub fn live_ack_memory(memory_digest: impl Into<String>) -> Self {
        Self::LiveAcknowledgmentMemory {
            memory_digest: memory_digest.into(),
        }
    }

    pub fn log_only(log_digest: impl Into<String>) -> Self {
        Self::LogOnly {
            log_digest: log_digest.into(),
        }
    }

    pub fn torn_publication(denial: TornPublicationDenial) -> Self {
        Self::TornPublication(denial)
    }

    pub fn insufficient_persisted_evidence(ambiguity_digest: impl Into<String>) -> Self {
        Self::InsufficientPersistedEvidence {
            ambiguity_digest: ambiguity_digest.into(),
        }
    }

    pub(crate) fn into_evidence(self) -> PartialPublicationEvidence {
        match self {
            Self::PersistedCrashEdge(edge) => {
                PartialPublicationEvidence::from_persisted_crash_edge(edge)
            }
            Self::BackendResidue {
                residue,
                residue_digest,
            } => PartialPublicationEvidence::from_backend_residue(residue, residue_digest),
            Self::LiveAcknowledgmentMemory { memory_digest } => {
                PartialPublicationEvidence::from_live_ack_memory(memory_digest)
            }
            Self::LogOnly { log_digest } => PartialPublicationEvidence::from_log_only(log_digest),
            Self::TornPublication(denial) => {
                PartialPublicationEvidence::from_torn_publication(denial)
            }
            Self::InsufficientPersistedEvidence { ambiguity_digest } => {
                PartialPublicationEvidence::insufficient_persisted_evidence(ambiguity_digest)
            }
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PartialPublicationObservationSet {
    torn_publication: Option<PartialPublicationObservedSource>,
    persisted_crash_edge: Option<PartialPublicationObservedSource>,
    backend_residue: Option<PartialPublicationObservedSource>,
    live_ack_memory: Option<PartialPublicationObservedSource>,
    log_only: Option<PartialPublicationObservedSource>,
    insufficient_persisted_evidence: Option<PartialPublicationObservedSource>,
}

impl PartialPublicationObservationSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_persisted_bytes(self, bytes: super::PartialPublicationPersistedBytes) -> Self {
        self.with_observed_source(bytes.observe())
    }

    pub fn with_persisted_crash_edge(self, edge: PartialPublicationCrashEdge) -> Self {
        self.with_observed_source(PartialPublicationObservedSource::persisted_crash_edge(edge))
    }

    pub fn with_backend_residue(
        self,
        residue: BackendResidueKind,
        residue_digest: impl Into<String>,
    ) -> Self {
        self.with_observed_source(PartialPublicationObservedSource::backend_residue(
            residue,
            residue_digest,
        ))
    }

    pub fn with_live_ack_memory(self, memory_digest: impl Into<String>) -> Self {
        self.with_observed_source(PartialPublicationObservedSource::live_ack_memory(
            memory_digest,
        ))
    }

    pub fn with_log_only(self, log_digest: impl Into<String>) -> Self {
        self.with_observed_source(PartialPublicationObservedSource::log_only(log_digest))
    }

    pub fn with_torn_publication(self, denial: TornPublicationDenial) -> Self {
        self.with_observed_source(PartialPublicationObservedSource::torn_publication(denial))
    }

    pub fn with_insufficient_persisted_evidence(self, ambiguity_digest: impl Into<String>) -> Self {
        self.with_observed_source(
            PartialPublicationObservedSource::insufficient_persisted_evidence(ambiguity_digest),
        )
    }

    pub fn with_observed_source(mut self, source: PartialPublicationObservedSource) -> Self {
        match source {
            PartialPublicationObservedSource::TornPublication(_) => {
                self.torn_publication = Some(source);
            }
            PartialPublicationObservedSource::PersistedCrashEdge(_) => {
                self.persisted_crash_edge = Some(source);
            }
            PartialPublicationObservedSource::BackendResidue { .. } => {
                self.backend_residue = Some(source);
            }
            PartialPublicationObservedSource::LiveAcknowledgmentMemory { .. } => {
                self.live_ack_memory = Some(source);
            }
            PartialPublicationObservedSource::LogOnly { .. } => {
                self.log_only = Some(source);
            }
            PartialPublicationObservedSource::InsufficientPersistedEvidence { .. } => {
                self.insufficient_persisted_evidence = Some(source);
            }
        }
        self
    }

    pub(crate) fn into_sources(self) -> PartialPublicationObservationSources {
        PartialPublicationObservationSources {
            torn_publication: self.torn_publication,
            persisted_crash_edge: self.persisted_crash_edge,
            backend_residue: self.backend_residue,
            live_ack_memory: self.live_ack_memory,
            log_only: self.log_only,
            insufficient_persisted_evidence: self.insufficient_persisted_evidence,
        }
    }
}

pub(crate) struct PartialPublicationObservationSources {
    pub(crate) torn_publication: Option<PartialPublicationObservedSource>,
    pub(crate) persisted_crash_edge: Option<PartialPublicationObservedSource>,
    pub(crate) backend_residue: Option<PartialPublicationObservedSource>,
    pub(crate) live_ack_memory: Option<PartialPublicationObservedSource>,
    pub(crate) log_only: Option<PartialPublicationObservedSource>,
    pub(crate) insufficient_persisted_evidence: Option<PartialPublicationObservedSource>,
}
