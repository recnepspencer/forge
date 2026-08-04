use crate::store_namespace::{ProposedStoreIdentity, StableStoreIdentity};

use crate::record_framing::{
    decode_durable_frame, encode_durable_frame, DURABLE_FRAME_HEADER_BYTES,
};
use crate::{DurableFrameDenial, DurableFrameKind, PhysicalRecordFormatDeclaration};

const BOOTSTRAP_CATALOG_PAYLOAD_BYTES: usize = 34;
pub const BOOTSTRAP_CATALOG_BYTES: usize =
    DURABLE_FRAME_HEADER_BYTES + BOOTSTRAP_CATALOG_PAYLOAD_BYTES;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentRootCatalogGeneration(u64);

impl CurrentRootCatalogGeneration {
    pub const fn new(generation: u64) -> Option<Self> {
        if generation == 0 {
            None
        } else {
            Some(Self(generation))
        }
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentRootCatalogEntry {
    generation: CurrentRootCatalogGeneration,
}

impl CurrentRootCatalogEntry {
    pub const fn new(generation: CurrentRootCatalogGeneration) -> Self {
        Self { generation }
    }

    pub const fn generation(self) -> CurrentRootCatalogGeneration {
        self.generation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapCatalog {
    store_identity: StableStoreIdentity,
    format: PhysicalRecordFormatDeclaration,
    current_root: CurrentRootCatalogEntry,
}

impl BootstrapCatalog {
    pub const fn new(
        store_identity: StableStoreIdentity,
        format: PhysicalRecordFormatDeclaration,
        current_root: CurrentRootCatalogEntry,
    ) -> Self {
        Self {
            store_identity,
            format,
            current_root,
        }
    }

    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store_identity
    }

    pub const fn format(self) -> PhysicalRecordFormatDeclaration {
        self.format
    }

    pub const fn current_root(self) -> CurrentRootCatalogEntry {
        self.current_root
    }

    pub fn encode(self) -> [u8; BOOTSTRAP_CATALOG_BYTES] {
        let mut payload = [0_u8; BOOTSTRAP_CATALOG_PAYLOAD_BYTES];
        payload[..16].copy_from_slice(&self.store_identity.bytes());
        payload[16..24].copy_from_slice(&self.current_root.generation().get().to_le_bytes());
        payload[24..34].copy_from_slice(&self.format.encode());
        encode_durable_frame(
            DurableFrameKind::BootstrapCatalog,
            self.format,
            self.current_root.generation().get(),
            &payload,
        )
        .try_into()
        .expect("catalog frame has a fixed width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, BootstrapCatalogDenial> {
        let (format, frame) = decode_durable_frame(bytes, DurableFrameKind::BootstrapCatalog)
            .map_err(BootstrapCatalogDenial::Frame)?;
        if frame.payload.len() != BOOTSTRAP_CATALOG_PAYLOAD_BYTES {
            return Err(BootstrapCatalogDenial::PayloadLength);
        }
        let proposed = ProposedStoreIdentity::from_nonzero_bytes(
            frame.payload[..16].try_into().expect("fixed identity"),
        )
        .ok_or(BootstrapCatalogDenial::ZeroStoreIdentity)?;
        let store_identity = StableStoreIdentity::from_published_record(proposed);
        let generation = CurrentRootCatalogGeneration::new(u64::from_le_bytes(
            frame.payload[16..24].try_into().unwrap(),
        ))
        .ok_or(BootstrapCatalogDenial::IdentityMismatch)?;
        let payload_format =
            PhysicalRecordFormatDeclaration::decode(frame.payload[24..34].try_into().unwrap())
                .map_err(|denial| {
                    BootstrapCatalogDenial::Frame(DurableFrameDenial::UnsupportedFormat(denial))
                })?;
        if generation.get() != frame.identity || payload_format != format {
            return Err(BootstrapCatalogDenial::IdentityMismatch);
        }
        Ok(Self::new(
            store_identity,
            format,
            CurrentRootCatalogEntry::new(generation),
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootstrapCatalogDenial {
    Frame(DurableFrameDenial),
    PayloadLength,
    ZeroStoreIdentity,
    IdentityMismatch,
}
