use worth_store_physical_format::PhysicalChunkPayloadIntegrityWitness;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobBackendChunkWriteObservationKind {
    StoreChunkWrite,
    ScalarFramedRecordApi,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobBackendChunkWriteObservation {
    ordinal: u64,
    bytes_written: u64,
    kind: BlobBackendChunkWriteObservationKind,
    _seal: private::BlobBackendChunkWriteObservationSeal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobBackendChunkWriteSession {
    _seal: private::BlobBackendChunkWriteSessionSeal,
}

impl BlobBackendChunkWriteSession {
    #[allow(dead_code)]
    pub(crate) const fn store_owned() -> Self {
        Self {
            _seal: private::BlobBackendChunkWriteSessionSeal,
        }
    }

    pub fn observe_store_chunk_payload(
        self,
        ordinal: u64,
        payload: &PhysicalChunkPayloadIntegrityWitness,
    ) -> Option<BlobBackendChunkWriteObservation> {
        let bytes_written = payload.physical_receipt().bytes_written();
        if bytes_written == 0 || payload.bytes_checked() != bytes_written {
            return None;
        }
        Some(BlobBackendChunkWriteObservation {
            ordinal,
            bytes_written,
            kind: BlobBackendChunkWriteObservationKind::StoreChunkWrite,
            _seal: private::BlobBackendChunkWriteObservationSeal,
        })
    }

    #[cfg(feature = "certification-test-authority")]
    pub const fn for_certification_test_authority() -> Self {
        Self::store_owned()
    }
}

impl BlobBackendChunkWriteObservation {
    pub const fn reject_scalar_framed_record_api(bytes_written: u64) -> Self {
        Self {
            ordinal: 0,
            bytes_written,
            kind: BlobBackendChunkWriteObservationKind::ScalarFramedRecordApi,
            _seal: private::BlobBackendChunkWriteObservationSeal,
        }
    }

    pub const fn ordinal(self) -> u64 {
        self.ordinal
    }

    pub const fn bytes_written(self) -> u64 {
        self.bytes_written
    }

    pub const fn kind(self) -> BlobBackendChunkWriteObservationKind {
        self.kind
    }
}

mod private {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(super) struct BlobBackendChunkWriteObservationSeal;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(super) struct BlobBackendChunkWriteSessionSeal;
}
