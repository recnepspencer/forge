use crate::CrashBoundaryLayoutReport;
use crate::{RecoveryEntryIdentity, RecoveryReplayEntryGate};

use super::{
    PartialPublicationClassification, PartialPublicationCounterSnapshot,
    PartialPublicationObservationSet, PartialPublicationPersistedBytes,
    UnacknowledgedPublicationOutcome,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialPublicationReplayReadRecord {
    recovery_entry_identity: RecoveryEntryIdentity,
    persisted_bytes_digest: String,
    classification: PartialPublicationClassification,
    crash_report: CrashBoundaryLayoutReport,
    _seal: ReplayReadRecordSeal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReplayReadRecordSeal;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialPublicationReplayReadArtifact {
    recovery_entry_identity: RecoveryEntryIdentity,
    persisted_bytes_digest: String,
    classification: PartialPublicationClassification,
    crash_report: CrashBoundaryLayoutReport,
    _seal: ReplayReadArtifactSeal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReplayReadArtifactSeal;

#[derive(Debug, Clone, PartialEq, Eq)]
struct PartialPublicationReplayReadSource {
    recovery_entry_digest: String,
    persisted_bytes_digest: String,
    classification: PartialPublicationClassification,
    crash_report: CrashBoundaryLayoutReport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialPublicationReplayReadWitness {
    replay_read_identity: String,
    source: PartialPublicationReplayReadSource,
    operation_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartialPublicationReplayReadDenial {
    NotBeforeWalAppend {
        actual_operation_digest: Option<String>,
    },
}

impl PartialPublicationReplayReadArtifact {
    pub(crate) fn from_admitted_before_wal_read(
        recovery_entry_identity: RecoveryEntryIdentity,
        bytes: PartialPublicationPersistedBytes,
        classification: PartialPublicationClassification,
        crash_report: CrashBoundaryLayoutReport,
    ) -> Self {
        Self {
            recovery_entry_identity,
            persisted_bytes_digest: bytes.persisted_bytes_digest(),
            classification,
            crash_report,
            _seal: ReplayReadArtifactSeal,
        }
    }

    pub(crate) fn from_replay_entry_gate(
        replay_entry: &RecoveryReplayEntryGate,
        bytes: PartialPublicationPersistedBytes,
    ) -> Result<Self, PartialPublicationReplayReadDenial> {
        let persisted_bytes_digest = bytes.persisted_bytes_digest();
        let classification = PartialPublicationClassification::classify_observations(
            PartialPublicationObservationSet::new().with_persisted_bytes(bytes),
        );
        let crash_report = crate::layout_access::admit_partial_publication_classification(&classification)
            .map_err(|_| PartialPublicationReplayReadDenial::NotBeforeWalAppend {
                actual_operation_digest: classification
                    .before_wal_append_operation_digest()
                    .map(str::to_owned),
            })?;
        Ok(Self {
            recovery_entry_identity: replay_entry.entry_identity().clone(),
            persisted_bytes_digest,
            classification,
            crash_report,
            _seal: ReplayReadArtifactSeal,
        })
    }

    pub fn recovery_entry_digest(&self) -> &str {
        self.recovery_entry_identity.digest().as_str()
    }

    pub fn persisted_bytes_digest(&self) -> &str {
        &self.persisted_bytes_digest
    }

    pub(crate) fn classification(&self) -> &PartialPublicationClassification {
        &self.classification
    }

    pub const fn crash_report(&self) -> &CrashBoundaryLayoutReport {
        &self.crash_report
    }
}

impl PartialPublicationReplayReadRecord {
    pub fn from_replay_read_artifact(artifact: PartialPublicationReplayReadArtifact) -> Self {
        Self {
            recovery_entry_identity: artifact.recovery_entry_identity,
            persisted_bytes_digest: artifact.persisted_bytes_digest,
            classification: artifact.classification,
            crash_report: artifact.crash_report,
            _seal: ReplayReadRecordSeal,
        }
    }

    pub fn recovery_entry_digest(&self) -> &str {
        self.recovery_entry_identity.digest().as_str()
    }

    pub fn persisted_bytes_digest(&self) -> &str {
        &self.persisted_bytes_digest
    }

    pub(crate) fn classification(&self) -> &PartialPublicationClassification {
        &self.classification
    }

    pub const fn crash_report(&self) -> &CrashBoundaryLayoutReport {
        &self.crash_report
    }

    fn into_source(self) -> PartialPublicationReplayReadSource {
        PartialPublicationReplayReadSource {
            recovery_entry_digest: self.recovery_entry_identity.digest().as_str().to_owned(),
            persisted_bytes_digest: self.persisted_bytes_digest,
            classification: self.classification,
            crash_report: self.crash_report,
        }
    }
}

impl PartialPublicationReplayReadWitness {
    pub fn readmitted_before_wal_append(
        record: PartialPublicationReplayReadRecord,
    ) -> Result<Self, PartialPublicationReplayReadDenial> {
        let source = record.into_source();
        if source.crash_report.outcome() != UnacknowledgedPublicationOutcome::NoWalAppendObserved {
            return Err(PartialPublicationReplayReadDenial::NotBeforeWalAppend {
                actual_operation_digest: source
                    .classification
                    .before_wal_append_operation_digest()
                    .map(str::to_owned),
            });
        }
        let Some(operation_digest) = source.classification.before_wal_append_operation_digest()
        else {
            return Err(PartialPublicationReplayReadDenial::NotBeforeWalAppend {
                actual_operation_digest: None,
            });
        };
        let operation_digest = operation_digest.to_owned();
        let replay_read_identity = format!(
            "partial-publication-replay-read:v1:entry={}:bytes={}:kind=before-wal-append:operation={}",
            source.recovery_entry_digest,
            source.persisted_bytes_digest,
            operation_digest
        );
        Ok(Self {
            replay_read_identity,
            source,
            operation_digest,
        })
    }

    pub fn replay_read_identity(&self) -> &str {
        &self.replay_read_identity
    }

    pub fn recovery_entry_digest(&self) -> &str {
        &self.source.recovery_entry_digest
    }

    pub fn persisted_bytes_digest(&self) -> &str {
        &self.source.persisted_bytes_digest
    }

    pub fn operation_digest(&self) -> &str {
        &self.operation_digest
    }

    pub const fn counters(&self) -> PartialPublicationCounterSnapshot {
        self.source.classification.counters()
    }

    pub(crate) fn classification(&self) -> &PartialPublicationClassification {
        &self.source.classification
    }

    pub const fn crash_report(&self) -> &CrashBoundaryLayoutReport {
        &self.source.crash_report
    }
}
