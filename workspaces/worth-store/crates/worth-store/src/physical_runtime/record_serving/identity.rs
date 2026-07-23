use worth_store_physical_format::{store_namespace::StableStoreIdentity, PersistedRecordIdentity};

use super::RecordAppendDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalRecordId(PersistedRecordIdentity);

impl PhysicalRecordId {
    pub const fn allocation_epoch(self) -> [u8; 16] {
        self.0.allocation_epoch()
    }

    pub const fn ordinal(self) -> u64 {
        self.0.ordinal()
    }

    pub(super) const fn from_persisted(identity: PersistedRecordIdentity) -> Self {
        Self(identity)
    }

    pub(super) const fn persisted(self) -> PersistedRecordIdentity {
        self.0
    }
}

pub(super) fn allocate_candidate_record_identities(
    count: usize,
    manifest: &super::access::manifest_routing::ManifestReader<'_>,
) -> Result<Vec<PersistedRecordIdentity>, RecordAppendDenial> {
    if count == 0 || count > u64::MAX as usize {
        return Err(RecordAppendDenial::RecordIdentityExhausted);
    }
    let mut allocation_epoch = [0_u8; 16];
    getrandom::fill(&mut allocation_epoch)
        .map_err(|_| RecordAppendDenial::IdentityEntropyUnavailable)?;
    let first_candidate = PersistedRecordIdentity::new(allocation_epoch, 1)
        .ok_or(RecordAppendDenial::IdentityEntropyUnavailable)?;
    let mut counters = super::access::manifest_routing::ManifestDiscoveryCounterSnapshot::default();
    if manifest
        .locate(first_candidate, &mut counters)
        .map_err(|_| RecordAppendDenial::PublishedLayoutDamaged)?
        .is_some()
    {
        return Err(RecordAppendDenial::IdentityEntropyUnavailable);
    }
    (1..=count as u64)
        .map(|ordinal| {
            PersistedRecordIdentity::new(allocation_epoch, ordinal)
                .ok_or(RecordAppendDenial::RecordIdentityExhausted)
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalPhysicalRecordLocator {
    store: [u8; 16],
    record: PhysicalRecordId,
}

impl ExternalPhysicalRecordLocator {
    pub const fn new(store: StableStoreIdentity, record: PhysicalRecordId) -> Self {
        Self {
            store: store.bytes(),
            record,
        }
    }

    pub const fn store_identity_bytes(self) -> [u8; 16] {
        self.store
    }

    pub(super) const fn readmitted_record_id(self) -> PhysicalRecordId {
        self.record
    }

    pub const fn encode(self) -> [u8; 40] {
        let mut bytes = [0_u8; 40];
        let epoch = self.record.allocation_epoch();
        let ordinal = self.record.ordinal().to_le_bytes();
        let mut index = 0;
        while index < 16 {
            bytes[index] = self.store[index];
            index += 1;
        }
        index = 0;
        while index < 16 {
            bytes[16 + index] = epoch[index];
            index += 1;
        }
        index = 0;
        while index < 8 {
            bytes[32 + index] = ordinal[index];
            index += 1;
        }
        bytes
    }

    pub fn decode(bytes: [u8; 40]) -> Option<Self> {
        let store: [u8; 16] = bytes[..16].try_into().ok()?;
        if store == [0; 16] {
            return None;
        }
        let record = PersistedRecordIdentity::new(
            bytes[16..32].try_into().ok()?,
            u64::from_le_bytes(bytes[32..40].try_into().ok()?),
        )?;
        Some(Self {
            store,
            record: PhysicalRecordId::from_persisted(record),
        })
    }
}
