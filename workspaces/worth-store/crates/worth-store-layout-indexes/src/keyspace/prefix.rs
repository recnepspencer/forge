use super::declaration::PhysicalKeyDomain;
use super::encoding::{
    encode_blob_identity_prefix, encode_scope_prefix, exclusive_bound_sentinel,
    physical_reference_kind_code, CanonicalKeyBytes, CanonicalKeyEncoding,
};
use super::value::ConcretePhysicalKeyWitness;
use crate::catalog::ArtifactFamilyDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefixBoundaryBehavior {
    TenantScopedSuccessor,
    BlobGenerationSuccessor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefixLawWitness {
    encoding: CanonicalKeyEncoding,
    behavior: PrefixBoundaryBehavior,
}

impl PrefixLawWitness {
    pub(crate) const fn new(
        encoding: CanonicalKeyEncoding,
        behavior: PrefixBoundaryBehavior,
    ) -> Self {
        Self { encoding, behavior }
    }

    pub const fn encoding(self) -> CanonicalKeyEncoding {
        self.encoding
    }

    pub const fn behavior(self) -> PrefixBoundaryBehavior {
        self.behavior
    }
}

pub(crate) const fn require_prefix_law(
    encoding: CanonicalKeyEncoding,
) -> Result<PrefixLawWitness, ArtifactFamilyDenial> {
    let behavior = match encoding.domain().domain() {
        PhysicalKeyDomain::PageAddressKey
        | PhysicalKeyDomain::SegmentAddressKey
        | PhysicalKeyDomain::ExtentAddressKey
        | PhysicalKeyDomain::PhysicalReferenceKey => PrefixBoundaryBehavior::TenantScopedSuccessor,
        PhysicalKeyDomain::BlobIdentityKey => PrefixBoundaryBehavior::BlobGenerationSuccessor,
        PhysicalKeyDomain::RootManifestKey | PhysicalKeyDomain::WalRecordKey => {
            return Err(ArtifactFamilyDenial::PhysicalKeyDomainDoesNotSupportPrefixBounds);
        }
    };

    Ok(PrefixLawWitness::new(encoding, behavior))
}

pub(crate) fn prefix_bytes_for_key(
    law: PrefixLawWitness,
    key: ConcretePhysicalKeyWitness,
) -> Result<CanonicalKeyBytes, ArtifactFamilyDenial> {
    if law.encoding().domain() != key.domain() {
        return Err(ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain);
    }

    let bytes = match law.behavior() {
        PrefixBoundaryBehavior::TenantScopedSuccessor => {
            let mut bytes = encode_scope_prefix(law.encoding(), &key);
            match law.encoding().domain().domain() {
                PhysicalKeyDomain::PageAddressKey | PhysicalKeyDomain::ExtentAddressKey => {
                    bytes.extend_from_slice(
                        &key.segment_id()
                            .ok_or(
                                ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain,
                            )?
                            .get()
                            .to_be_bytes(),
                    );
                }
                PhysicalKeyDomain::SegmentAddressKey => {
                    bytes.extend_from_slice(
                        &key.segment_id()
                            .ok_or(
                                ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain,
                            )?
                            .get()
                            .to_be_bytes(),
                    );
                }
                PhysicalKeyDomain::PhysicalReferenceKey => {
                    let reference = key.physical_reference().ok_or(
                        ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain,
                    )?;
                    bytes.push(physical_reference_kind_code(reference.kind()));
                    if let Some(segment_id) = reference.segment_id() {
                        bytes.extend_from_slice(&segment_id.get().to_be_bytes());
                    }
                }
                _ => {}
            }
            bytes
        }
        PrefixBoundaryBehavior::BlobGenerationSuccessor => {
            let identity = key
                .blob_identity()
                .ok_or(ArtifactFamilyDenial::ConcreteKeyKindDoesNotMatchPhysicalKeyDomain)?;
            encode_blob_identity_prefix(law.encoding(), identity)
                .as_bytes()
                .to_vec()
        }
    };

    Ok(CanonicalKeyBytes::new(law.encoding(), bytes))
}

pub(crate) fn prefix_successor_bytes(prefix: &CanonicalKeyBytes) -> CanonicalKeyBytes {
    let mut successor = prefix.as_bytes().to_vec();
    successor.push(exclusive_bound_sentinel(prefix.encoding()));
    CanonicalKeyBytes::new(prefix.encoding(), successor)
}
