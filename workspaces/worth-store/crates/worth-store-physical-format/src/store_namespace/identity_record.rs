use sha2::{Digest, Sha256};

use super::store_identity::STORE_IDENTITY_BYTES;
use super::{ProposedStoreIdentity, StoreNamespaceVersion};

const MAGIC: [u8; 8] = *b"WSTNSID\0";
pub(super) const STORE_NAMESPACE_IDENTITY_ENCODING_VERSION: u16 = 1;
const IDENTITY_FIELD_TAG: u16 = 1;
const HEADER_LENGTH: usize = 20;
const FIELD_HEADER_LENGTH: usize = 4;
const CHECKSUM_LENGTH: usize = 32;
pub const STORE_NAMESPACE_IDENTITY_RECORD_LENGTH: usize =
    HEADER_LENGTH + FIELD_HEADER_LENGTH + STORE_IDENTITY_BYTES + CHECKSUM_LENGTH;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreNamespaceIdentityRecord {
    namespace_version: StoreNamespaceVersion,
    proposed_identity: ProposedStoreIdentity,
}

impl StoreNamespaceIdentityRecord {
    pub const fn new(
        namespace_version: StoreNamespaceVersion,
        proposed_identity: ProposedStoreIdentity,
    ) -> Self {
        Self {
            namespace_version,
            proposed_identity,
        }
    }

    pub const fn namespace_version(&self) -> StoreNamespaceVersion {
        self.namespace_version
    }

    pub const fn proposed_identity(&self) -> ProposedStoreIdentity {
        self.proposed_identity
    }

    /// Returns the stable meaning after the caller establishes that these
    /// decoded bytes occupy the published namespace role.
    ///
    /// Decoding remains non-authoritative: the physical backend owns the
    /// publication-placement check and all filesystem authority.
    pub const fn published_identity(&self) -> super::StableStoreIdentity {
        super::StableStoreIdentity::from_published_record(self.proposed_identity)
    }

    pub fn encode(&self) -> [u8; STORE_NAMESPACE_IDENTITY_RECORD_LENGTH] {
        let mut bytes = [0; STORE_NAMESPACE_IDENTITY_RECORD_LENGTH];
        bytes[..8].copy_from_slice(&MAGIC);
        bytes[8..10].copy_from_slice(&STORE_NAMESPACE_IDENTITY_ENCODING_VERSION.to_le_bytes());
        bytes[10..12].copy_from_slice(&self.namespace_version.value().to_le_bytes());
        bytes[12..16]
            .copy_from_slice(&(STORE_NAMESPACE_IDENTITY_RECORD_LENGTH as u32).to_le_bytes());
        bytes[16..18].copy_from_slice(&1_u16.to_le_bytes());
        bytes[20..22].copy_from_slice(&IDENTITY_FIELD_TAG.to_le_bytes());
        bytes[22..24].copy_from_slice(&(STORE_IDENTITY_BYTES as u16).to_le_bytes());
        bytes[24..40].copy_from_slice(&self.proposed_identity.bytes());
        let digest = Sha256::digest(&bytes[..40]);
        bytes[40..].copy_from_slice(&digest);
        bytes
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, StoreNamespaceIdentityDecodeError> {
        if bytes.len() < HEADER_LENGTH {
            return Err(StoreNamespaceIdentityDecodeError::IncorrectLength);
        }
        if bytes[..8] != MAGIC {
            return Err(StoreNamespaceIdentityDecodeError::BadMagic);
        }
        let declared_length =
            u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]) as usize;
        if declared_length != STORE_NAMESPACE_IDENTITY_RECORD_LENGTH {
            return Err(StoreNamespaceIdentityDecodeError::IncorrectLength);
        }
        if bytes.len() > declared_length {
            return Err(StoreNamespaceIdentityDecodeError::TrailingBytes);
        }
        if bytes.len() < declared_length {
            return Err(StoreNamespaceIdentityDecodeError::IncorrectLength);
        }
        let expected = Sha256::digest(&bytes[..40]);
        if bytes[40..] != expected[..] {
            return Err(StoreNamespaceIdentityDecodeError::ChecksumMismatch);
        }
        let encoding = u16::from_le_bytes([bytes[8], bytes[9]]);
        if encoding != STORE_NAMESPACE_IDENTITY_ENCODING_VERSION {
            return Err(StoreNamespaceIdentityDecodeError::UnsupportedEncodingVersion(encoding));
        }
        let namespace = u16::from_le_bytes([bytes[10], bytes[11]]);
        if namespace != StoreNamespaceVersion::CURRENT.value() {
            return Err(StoreNamespaceIdentityDecodeError::UnsupportedNamespaceVersion(namespace));
        }
        if bytes[18..20] != [0, 0] {
            return Err(StoreNamespaceIdentityDecodeError::ReservedBytesNonzero);
        }
        let field_count = u16::from_le_bytes([bytes[16], bytes[17]]);
        if field_count == 0 {
            return Err(StoreNamespaceIdentityDecodeError::MissingIdentityField);
        }
        if field_count > 1 {
            return Err(StoreNamespaceIdentityDecodeError::DuplicateIdentityField {
                declared_count: field_count,
            });
        }
        let tag = u16::from_le_bytes([bytes[20], bytes[21]]);
        let field_length = u16::from_le_bytes([bytes[22], bytes[23]]) as usize;
        if tag != IDENTITY_FIELD_TAG {
            return Err(StoreNamespaceIdentityDecodeError::UnexpectedIdentityFieldTag(tag));
        }
        if field_length != STORE_IDENTITY_BYTES {
            return Err(
                StoreNamespaceIdentityDecodeError::IncorrectIdentityFieldLength(
                    field_length as u16,
                ),
            );
        }
        let mut identity = [0; STORE_IDENTITY_BYTES];
        identity.copy_from_slice(&bytes[24..40]);
        let proposed_identity = ProposedStoreIdentity::from_nonzero_bytes(identity)
            .ok_or(StoreNamespaceIdentityDecodeError::ZeroIdentity)?;
        Ok(Self::new(StoreNamespaceVersion::CURRENT, proposed_identity))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreNamespaceIdentityDecodeError {
    BadMagic,
    IncorrectLength,
    TrailingBytes,
    UnsupportedEncodingVersion(u16),
    UnsupportedNamespaceVersion(u16),
    ReservedBytesNonzero,
    MissingIdentityField,
    DuplicateIdentityField { declared_count: u16 },
    UnexpectedIdentityFieldTag(u16),
    IncorrectIdentityFieldLength(u16),
    ChecksumMismatch,
    ZeroIdentity,
}
