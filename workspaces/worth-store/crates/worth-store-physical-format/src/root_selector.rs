use crate::record_framing::{
    decode_durable_frame, encode_durable_frame, DURABLE_FRAME_HEADER_BYTES,
};
use crate::store_namespace::{ProposedStoreIdentity, StableStoreIdentity};
use crate::{DurableFrameDenial, DurableFrameKind, PhysicalRecordFormatDeclaration};

const SELECTOR_PAYLOAD_BYTES: usize = 59;
pub const ROOT_SELECTOR_BYTES: usize = DURABLE_FRAME_HEADER_BYTES + SELECTOR_PAYLOAD_BYTES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RootSelectorIdentity(u64);

impl RootSelectorIdentity {
    pub const fn new(value: u64) -> Option<Self> {
        if value == 0 {
            None
        } else {
            Some(Self(value))
        }
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum RootSelectorRole {
    Current = 1,
    Previous = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurableRootSelector {
    store: StableStoreIdentity,
    format: PhysicalRecordFormatDeclaration,
    identity: RootSelectorIdentity,
    role: RootSelectorRole,
    root_generation: u64,
    linked_selector: Option<RootSelectorIdentity>,
    linked_root_generation: Option<u64>,
}

impl DurableRootSelector {
    pub const fn new(
        store: StableStoreIdentity,
        format: PhysicalRecordFormatDeclaration,
        identity: RootSelectorIdentity,
        role: RootSelectorRole,
        root_generation: u64,
        linked_selector: Option<RootSelectorIdentity>,
        linked_root_generation: Option<u64>,
    ) -> Option<Self> {
        if root_generation == 0
            || linked_selector.is_some() != linked_root_generation.is_some()
            || matches!(linked_root_generation, Some(0))
        {
            return None;
        }
        Some(Self {
            store,
            format,
            identity,
            role,
            root_generation,
            linked_selector,
            linked_root_generation,
        })
    }

    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn format(self) -> PhysicalRecordFormatDeclaration {
        self.format
    }

    pub const fn identity(self) -> RootSelectorIdentity {
        self.identity
    }

    pub const fn role(self) -> RootSelectorRole {
        self.role
    }

    pub const fn root_generation(self) -> u64 {
        self.root_generation
    }

    pub const fn linked_selector(self) -> Option<RootSelectorIdentity> {
        self.linked_selector
    }

    pub const fn linked_root_generation(self) -> Option<u64> {
        self.linked_root_generation
    }

    pub fn encode(self) -> [u8; ROOT_SELECTOR_BYTES] {
        let mut payload = [0_u8; SELECTOR_PAYLOAD_BYTES];
        payload[..16].copy_from_slice(&self.store.bytes());
        payload[16] = self.role as u8;
        payload[17..25].copy_from_slice(&self.root_generation.to_le_bytes());
        payload[25..33].copy_from_slice(
            &self
                .linked_selector
                .map_or(0, RootSelectorIdentity::get)
                .to_le_bytes(),
        );
        payload[33..41].copy_from_slice(&self.linked_root_generation.unwrap_or(0).to_le_bytes());
        payload[41..51].copy_from_slice(&self.format.encode());
        encode_durable_frame(
            DurableFrameKind::RootSelector,
            self.format,
            self.identity.get(),
            &payload,
        )
        .try_into()
        .expect("root selector has a fixed width")
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, RootSelectorDecodeDenial> {
        let (format, frame) = decode_durable_frame(bytes, DurableFrameKind::RootSelector)
            .map_err(RootSelectorDecodeDenial::Frame)?;
        if frame.payload.len() != SELECTOR_PAYLOAD_BYTES {
            return Err(RootSelectorDecodeDenial::PayloadLength);
        }
        if frame.payload[51..].iter().any(|byte| *byte != 0) {
            return Err(RootSelectorDecodeDenial::ReservedFieldNonZero);
        }
        let store = ProposedStoreIdentity::from_nonzero_bytes(
            frame.payload[..16].try_into().expect("fixed identity"),
        )
        .map(StableStoreIdentity::from_published_record)
        .ok_or(RootSelectorDecodeDenial::ZeroStoreIdentity)?;
        let role = match frame.payload[16] {
            1 => RootSelectorRole::Current,
            2 => RootSelectorRole::Previous,
            other => return Err(RootSelectorDecodeDenial::UnknownRole(other)),
        };
        let root_generation = u64::from_le_bytes(frame.payload[17..25].try_into().unwrap());
        let linked_identity = u64::from_le_bytes(frame.payload[25..33].try_into().unwrap());
        let linked_generation = u64::from_le_bytes(frame.payload[33..41].try_into().unwrap());
        let payload_format = PhysicalRecordFormatDeclaration::decode(
            frame.payload[41..51].try_into().expect("fixed format"),
        )
        .map_err(|_| RootSelectorDecodeDenial::FormatMismatch)?;
        if payload_format != format {
            return Err(RootSelectorDecodeDenial::FormatMismatch);
        }
        let identity = RootSelectorIdentity::new(frame.identity)
            .ok_or(RootSelectorDecodeDenial::ZeroSelectorIdentity)?;
        let linked_selector = RootSelectorIdentity::new(linked_identity);
        let linked_root_generation = (linked_generation != 0).then_some(linked_generation);
        Self::new(
            store,
            format,
            identity,
            role,
            root_generation,
            linked_selector,
            linked_root_generation,
        )
        .ok_or(RootSelectorDecodeDenial::InvalidLinkage)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootSelectorDecodeDenial {
    Frame(DurableFrameDenial),
    PayloadLength,
    ReservedFieldNonZero,
    ZeroStoreIdentity,
    ZeroSelectorIdentity,
    UnknownRole(u8),
    FormatMismatch,
    InvalidLinkage,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store_namespace::ProposedStoreIdentity;

    fn format() -> PhysicalRecordFormatDeclaration {
        crate::PhysicalRecordFormatDeclaration::builder()
            .admit()
            .unwrap()
    }

    fn store() -> StableStoreIdentity {
        StableStoreIdentity::from_published_record(
            ProposedStoreIdentity::from_nonzero_bytes([7; 16]).unwrap(),
        )
    }

    #[test]
    fn selector_roundtrip_preserves_exact_role_and_linkage() {
        let selector = DurableRootSelector::new(
            store(),
            format(),
            RootSelectorIdentity::new(19).unwrap(),
            RootSelectorRole::Current,
            8,
            RootSelectorIdentity::new(17),
            Some(7),
        )
        .unwrap();
        assert_eq!(
            DurableRootSelector::decode(&selector.encode()),
            Ok(selector)
        );
    }

    #[test]
    fn selector_rejects_torn_and_one_sided_linkage() {
        let selector = DurableRootSelector::new(
            store(),
            format(),
            RootSelectorIdentity::new(19).unwrap(),
            RootSelectorRole::Current,
            8,
            None,
            None,
        )
        .unwrap();
        let bytes = selector.encode();
        assert!(matches!(
            DurableRootSelector::decode(&bytes[..bytes.len() - 1]),
            Err(RootSelectorDecodeDenial::Frame(
                DurableFrameDenial::Truncated
            )) | Err(RootSelectorDecodeDenial::Frame(
                DurableFrameDenial::LengthMismatch
            ))
        ));
        assert!(DurableRootSelector::new(
            store(),
            format(),
            RootSelectorIdentity::new(20).unwrap(),
            RootSelectorRole::Previous,
            7,
            RootSelectorIdentity::new(21),
            None,
        )
        .is_none());
    }
}
