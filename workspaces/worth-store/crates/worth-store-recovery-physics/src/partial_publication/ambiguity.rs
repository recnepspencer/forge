use super::PartialPublicationCounterSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmbiguousPublicationReport {
    ambiguity_digest: String,
    counters: PartialPublicationCounterSnapshot,
}

impl AmbiguousPublicationReport {
    pub fn insufficient_persisted_evidence(ambiguity_digest: impl Into<String>) -> Self {
        Self {
            ambiguity_digest: ambiguity_digest.into(),
            counters: PartialPublicationCounterSnapshot::default().with_ambiguous_outcome(),
        }
    }

    pub fn ambiguity_digest(&self) -> &str {
        &self.ambiguity_digest
    }

    pub const fn counters(&self) -> PartialPublicationCounterSnapshot {
        self.counters
    }
}
