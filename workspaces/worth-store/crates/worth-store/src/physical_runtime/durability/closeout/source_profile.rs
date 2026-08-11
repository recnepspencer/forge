use worth_store_physical_backend::{BackendTargetProfile, PhysicalDurabilityAdmissionIdentity};

const COMPILED_DURABILITY_SOURCE_IDENTITY: [u8; 32] =
    decode_source_identity(env!("WORTH_STORE_DURABILITY_SOURCE_IDENTITY"));

const fn decode_source_identity(encoded: &str) -> [u8; 32] {
    let encoded = encoded.as_bytes();
    assert!(
        encoded.len() == 64,
        "durability source identity must be 32 bytes"
    );
    let mut bytes = [0_u8; 32];
    let mut index = 0;
    while index < bytes.len() {
        bytes[index] =
            (decode_nibble(encoded[index * 2]) << 4) | decode_nibble(encoded[index * 2 + 1]);
        index += 1;
    }
    bytes
}

const fn decode_nibble(value: u8) -> u8 {
    match value {
        b'0'..=b'9' => value - b'0',
        b'a'..=b'f' => value - b'a' + 10,
        _ => panic!("durability source identity must be lowercase hexadecimal"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalDurabilitySourceIdentity([u8; 32]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalDurabilitySourceProfileIdentity {
    source: PhysicalDurabilitySourceIdentity,
    admission: PhysicalDurabilityAdmissionIdentity,
    policy: crate::physical_runtime::PhysicalDurabilityPolicyIdentity,
}

impl PhysicalDurabilitySourceIdentity {
    pub(in crate::physical_runtime) const fn compiled() -> Self {
        Self(COMPILED_DURABILITY_SOURCE_IDENTITY)
    }

    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

impl PhysicalDurabilitySourceProfileIdentity {
    pub(in crate::physical_runtime) const fn bind(
        admission: PhysicalDurabilityAdmissionIdentity,
        policy: crate::physical_runtime::PhysicalDurabilityPolicyIdentity,
    ) -> Self {
        Self {
            source: PhysicalDurabilitySourceIdentity::compiled(),
            admission,
            policy,
        }
    }

    pub const fn source(self) -> PhysicalDurabilitySourceIdentity {
        self.source
    }

    pub const fn admission_identity(self) -> PhysicalDurabilityAdmissionIdentity {
        self.admission
    }

    pub const fn policy_identity(
        self,
    ) -> crate::physical_runtime::PhysicalDurabilityPolicyIdentity {
        self.policy
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalBackendDurabilityCloseoutEvidence {
    admission: PhysicalDurabilityAdmissionIdentity,
    profile: BackendTargetProfile,
    durable_wal_lsn_end: Option<worth_store_wal::LogSequenceNumber>,
    root_namespace: crate::physical_runtime::PhysicalRootNamespaceDurabilityEvidence,
}

impl PhysicalBackendDurabilityCloseoutEvidence {
    pub(in crate::physical_runtime) const fn new(
        admission: PhysicalDurabilityAdmissionIdentity,
        profile: BackendTargetProfile,
        durable_wal_lsn_end: Option<worth_store_wal::LogSequenceNumber>,
        root_namespace: crate::physical_runtime::PhysicalRootNamespaceDurabilityEvidence,
    ) -> Self {
        Self {
            admission,
            profile,
            durable_wal_lsn_end,
            root_namespace,
        }
    }

    pub const fn admission_identity(self) -> PhysicalDurabilityAdmissionIdentity {
        self.admission
    }

    pub const fn profile(self) -> BackendTargetProfile {
        self.profile
    }

    pub const fn durable_wal_lsn_end(self) -> Option<worth_store_wal::LogSequenceNumber> {
        self.durable_wal_lsn_end
    }

    pub const fn root_namespace_evidence(
        self,
    ) -> crate::physical_runtime::PhysicalRootNamespaceDurabilityEvidence {
        self.root_namespace
    }
}
