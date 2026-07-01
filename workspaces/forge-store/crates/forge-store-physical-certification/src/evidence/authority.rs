use forge_proof::{AuthorityMarker, AuthorityWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceBundleAuthority {
    _private: (),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceBundleReadmissionAuthority {
    _private: (),
}

impl AuthorityMarker for EvidenceBundleReadmissionAuthority {}

impl EvidenceBundleAuthority {
    pub(crate) const fn current_store_authority() -> Self {
        Self { _private: () }
    }
}

impl EvidenceBundleReadmissionAuthority {
    pub(crate) const fn current_store_authority() -> Self {
        Self { _private: () }
    }
}

pub(crate) fn evidence_bundle_readmission_authority(
) -> AuthorityWitness<EvidenceBundleReadmissionAuthority> {
    AuthorityWitness::from_authority_marker(
        EvidenceBundleReadmissionAuthority::current_store_authority(),
    )
}
