use forge_store_physical_integrity::ProtectedPhysicalByteView;

use super::{
    PartialPublicationClassification, PartialPublicationPersistedBytes,
    PartialPublicationReplayReadDenial, UnacknowledgedPublicationOutcome,
};
use crate::PartialPublicationReplayReadArtifact;
use crate::{CrashBoundaryLayoutReport, RecoveryEntryIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialPublicationBeforeWalReplayRead {
    persisted_bytes: PartialPublicationPersistedBytes,
    classification: PartialPublicationClassification,
    crash_report: CrashBoundaryLayoutReport,
    _seal: BeforeWalReplayReadSeal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BeforeWalReplayReadSeal;

impl PartialPublicationBeforeWalReplayRead {
    pub fn from_protected_physical_bytes(
        protected_bytes: ProtectedPhysicalByteView<'_>,
    ) -> Result<Self, PartialPublicationReplayReadDenial> {
        let persisted_bytes =
            PartialPublicationPersistedBytes::from_replay_read_bytes(protected_bytes.as_bytes());
        let classification = PartialPublicationClassification::classify_observations(
            super::PartialPublicationObservationSet::new()
                .with_persisted_bytes(persisted_bytes.clone()),
        );
        let Ok(crash_report) =
            crate::layout_projection::admit_partial_publication_classification(&classification)
        else {
            return Err(PartialPublicationReplayReadDenial::NotBeforeWalAppend {
                actual_operation_digest: classification
                    .before_wal_append_operation_digest()
                    .map(str::to_owned),
            });
        };
        if crash_report.outcome() != UnacknowledgedPublicationOutcome::NoWalAppendObserved {
            return Err(PartialPublicationReplayReadDenial::NotBeforeWalAppend {
                actual_operation_digest: classification
                    .before_wal_append_operation_digest()
                    .map(str::to_owned),
            });
        }
        if classification
            .before_wal_append_operation_digest()
            .is_none()
        {
            return Err(PartialPublicationReplayReadDenial::NotBeforeWalAppend {
                actual_operation_digest: None,
            });
        }
        Ok(Self {
            persisted_bytes,
            classification,
            crash_report,
            _seal: BeforeWalReplayReadSeal,
        })
    }

    pub(crate) fn into_replay_read_artifact(
        self,
        entry_identity: RecoveryEntryIdentity,
    ) -> PartialPublicationReplayReadArtifact {
        PartialPublicationReplayReadArtifact::from_admitted_before_wal_read(
            entry_identity,
            self.persisted_bytes,
            self.classification,
            self.crash_report,
        )
    }
}
