use super::declaration::{PhysicalKeyDomain, PhysicalKeyDomainWitness};
use super::encoding::encode_concrete_physical_key;
use super::value::ConcretePhysicalKeyWitness;
use crate::catalog::ArtifactFamilyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashCollisionBehavior {
    ImpossibleByCanonicalIdentity,
    RequiresCanonicalByteVerification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HashCollisionLaw {
    domain: PhysicalKeyDomainWitness,
    behavior: HashCollisionBehavior,
}

impl HashCollisionLaw {
    pub(crate) const fn new(
        domain: PhysicalKeyDomainWitness,
        behavior: HashCollisionBehavior,
    ) -> Self {
        Self { domain, behavior }
    }

    pub const fn domain(self) -> PhysicalKeyDomainWitness {
        self.domain
    }

    pub const fn behavior(self) -> HashCollisionBehavior {
        self.behavior
    }
}

pub(crate) const fn declare_hash_collision_law(
    domain: PhysicalKeyDomainWitness,
) -> HashCollisionLaw {
    let behavior = match domain.domain() {
        PhysicalKeyDomain::RootManifestKey | PhysicalKeyDomain::WalRecordKey => {
            HashCollisionBehavior::ImpossibleByCanonicalIdentity
        }
        PhysicalKeyDomain::PageAddressKey
        | PhysicalKeyDomain::SegmentAddressKey
        | PhysicalKeyDomain::ExtentAddressKey
        | PhysicalKeyDomain::PhysicalReferenceKey
        | PhysicalKeyDomain::BlobIdentityKey => {
            HashCollisionBehavior::RequiresCanonicalByteVerification
        }
    };

    HashCollisionLaw::new(domain, behavior)
}

pub(crate) const fn require_exact_hash_identity_claim(
    law: HashCollisionLaw,
) -> Result<HashCollisionLaw, ArtifactFamilyDenial> {
    match law.behavior() {
        HashCollisionBehavior::ImpossibleByCanonicalIdentity => Ok(law),
        HashCollisionBehavior::RequiresCanonicalByteVerification => {
            Err(ArtifactFamilyDenial::HashIdentityRequiresCollisionVerification)
        }
    }
}

pub(crate) fn hash_digest_for_key(
    law: HashCollisionLaw,
    key: ConcretePhysicalKeyWitness,
) -> Result<u64, ArtifactFamilyDenial> {
    let bytes = encode_concrete_physical_key(
        super::encoding::require_canonical_key_encoding(law.domain()),
        key,
    )?;
    Ok(stable_fnv64(bytes.as_bytes()))
}

pub(crate) fn verify_hash_identity(
    law: HashCollisionLaw,
    left: ConcretePhysicalKeyWitness,
    right: ConcretePhysicalKeyWitness,
) -> Result<(), ArtifactFamilyDenial> {
    let left_hash = hash_digest_for_key(law, left.clone())?;
    let right_hash = hash_digest_for_key(law, right.clone())?;
    if left_hash != right_hash {
        return Err(ArtifactFamilyDenial::HashIdentityRequiresCollisionVerification);
    }
    match law.behavior() {
        HashCollisionBehavior::ImpossibleByCanonicalIdentity => Ok(()),
        HashCollisionBehavior::RequiresCanonicalByteVerification => {
            let left_bytes = encode_concrete_physical_key(
                super::encoding::require_canonical_key_encoding(law.domain()),
                left,
            )?;
            let right_bytes = encode_concrete_physical_key(
                super::encoding::require_canonical_key_encoding(law.domain()),
                right,
            )?;
            if left_bytes == right_bytes {
                Ok(())
            } else {
                Err(ArtifactFamilyDenial::HashIdentityRequiresCollisionVerification)
            }
        }
    }
}

fn stable_fnv64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
