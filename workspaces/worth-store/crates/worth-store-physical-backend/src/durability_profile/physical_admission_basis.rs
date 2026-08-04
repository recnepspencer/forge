use sha2::{Digest, Sha256};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

use crate::{
    BackendCapabilityClaimWitness, BackendCapabilityKind, BackendTargetProfile,
    CapabilityEvidenceClass,
};

const DOMAIN: &[u8] = b"worth.store.physical.durability.admission-basis.v1";

/// Stable identity of one C.4-qualified durability admission basis.
///
/// This value supports Store/runtime rebinding checks. It is descriptive and
/// cannot reconstruct the sealed admission basis.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PhysicalDurabilityAdmissionIdentity([u8; 32]);

impl PhysicalDurabilityAdmissionIdentity {
    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

/// Move-only C.4 proof that one qualified media generation carries every
/// capability required by the ordinary platform-durable Store policy.
pub struct PhysicalDurabilityAdmissionBasis {
    identity: PhysicalDurabilityAdmissionIdentity,
    store: StableStoreIdentity,
    target: BackendTargetProfile,
    file_sync: BackendCapabilityClaimWitness,
    directory_sync: BackendCapabilityClaimWitness,
    durable_rename: BackendCapabilityClaimWitness,
}

impl PhysicalDurabilityAdmissionBasis {
    pub(crate) fn from_qualified_media(input: QualifiedDurabilityBasisInput) -> Self {
        let identity = PhysicalDurabilityAdmissionIdentity(fingerprint(&input));
        Self {
            identity,
            store: input.store,
            target: input.target,
            file_sync: input.file_sync,
            directory_sync: input.directory_sync,
            durable_rename: input.durable_rename,
        }
    }

    pub const fn identity(&self) -> PhysicalDurabilityAdmissionIdentity {
        self.identity
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.store
    }

    pub const fn target_profile(&self) -> BackendTargetProfile {
        self.target
    }

    pub const fn file_sync_claim(&self) -> BackendCapabilityClaimWitness {
        self.file_sync
    }

    pub const fn directory_sync_claim(&self) -> BackendCapabilityClaimWitness {
        self.directory_sync
    }

    pub const fn durable_rename_claim(&self) -> BackendCapabilityClaimWitness {
        self.durable_rename
    }
}

pub(crate) struct QualifiedDurabilityBasisInput {
    pub(crate) store: StableStoreIdentity,
    pub(crate) qualification_contract_version: u16,
    pub(crate) root_identity: [u8; 32],
    pub(crate) volume_identity: [u8; 32],
    pub(crate) profile_digest: [u8; 32],
    pub(crate) backend_build_identity: [u8; 32],
    pub(crate) target: BackendTargetProfile,
    pub(crate) file_sync: BackendCapabilityClaimWitness,
    pub(crate) directory_sync: BackendCapabilityClaimWitness,
    pub(crate) durable_rename: BackendCapabilityClaimWitness,
}

fn fingerprint(input: &QualifiedDurabilityBasisInput) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update((DOMAIN.len() as u64).to_le_bytes());
    digest.update(DOMAIN);
    digest.update(input.qualification_contract_version.to_le_bytes());
    digest.update(input.store.bytes());
    digest.update(input.root_identity);
    digest.update(input.volume_identity);
    digest.update(input.profile_digest);
    digest.update(input.backend_build_identity);
    digest.update([target_code(input.target)]);
    for claim in [input.file_sync, input.directory_sync, input.durable_rename] {
        digest.update([capability_code(claim.kind())]);
        digest.update([target_code(claim.profile())]);
        digest.update([evidence_code(claim.evidence_class())]);
    }
    digest.finalize().into()
}

const fn target_code(target: BackendTargetProfile) -> u8 {
    match target {
        BackendTargetProfile::SimulatedStrictDurable => 1,
        BackendTargetProfile::PosixFileFsyncDirSync => 2,
        BackendTargetProfile::WindowsFlushFileBuffers => 3,
        BackendTargetProfile::MmapFlushNotDurabilityCertified => 4,
        BackendTargetProfile::AdversarialLostFlush => 5,
        BackendTargetProfile::AdversarialReorderedFlush => 6,
    }
}

const fn capability_code(kind: BackendCapabilityKind) -> u8 {
    match kind {
        BackendCapabilityKind::Fsync => 1,
        BackendCapabilityKind::DirectorySync => 2,
        BackendCapabilityKind::DurableRename => 3,
        _ => 0,
    }
}

const fn evidence_code(evidence: CapabilityEvidenceClass) -> u8 {
    match evidence {
        CapabilityEvidenceClass::DeclaredByConfig => 1,
        CapabilityEvidenceClass::ObservedByProbe => 2,
        CapabilityEvidenceClass::EstablishedByFilesystemAdmission => 3,
        CapabilityEvidenceClass::ExternallyGuaranteed => 4,
        CapabilityEvidenceClass::UnverifiableAssumption => 5,
        CapabilityEvidenceClass::CertifiedBackendProfile => 6,
    }
}
