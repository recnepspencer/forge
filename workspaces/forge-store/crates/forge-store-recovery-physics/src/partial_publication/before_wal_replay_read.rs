use forge_store_physical_integrity::ProtectedPhysicalByteView;

use super::{
    PartialPublicationClassification, PartialPublicationPersistedBytes,
    PartialPublicationReplayReadDenial, UnacknowledgedPublicationOutcome,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialPublicationBeforeWalReplayRead {
    persisted_bytes: PartialPublicationPersistedBytes,
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
        if classification.outcome() != UnacknowledgedPublicationOutcome::NoWalAppendObserved {
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
            _seal: BeforeWalReplayReadSeal,
        })
    }

    pub(crate) fn into_persisted_bytes(self) -> PartialPublicationPersistedBytes {
        self.persisted_bytes
    }
}
