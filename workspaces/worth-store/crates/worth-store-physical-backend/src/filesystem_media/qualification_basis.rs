use worth_proof::{Artifact, AuthorityMarker, AuthorityWitness, PhaseMarker};

const QUALIFICATION_CONTRACT_VERSION: u16 = 2;

pub(super) const fn qualification_contract_version() -> u16 {
    QUALIFICATION_CONTRACT_VERSION
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RootProfileBinding {
    pub(super) contract_version: u16,
    pub(super) root_identity: [u8; 32],
    pub(super) volume_identity: [u8; 32],
    pub(super) profile_digest: [u8; 32],
    pub(super) backend_build_identity: [u8; 32],
    pub(super) access_contract: super::FilesystemAccessContract,
}

struct QualifiedRootProfile;
impl PhaseMarker for QualifiedRootProfile {}

pub struct RootProfileQualificationBasis {
    artifact: Artifact<
        QualifiedRootProfile,
        (),
        worth_proof::NoProofs,
        worth_proof::FreshnessScopedBasis<
            worth_proof::CurrentValidity,
            worth_proof::AssumptionBasis<RootProfileBinding>,
        >,
    >,
}

impl core::fmt::Debug for RootProfileQualificationBasis {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("RootProfileQualificationBasis")
            .field("contract_version", &self.binding().contract_version)
            .field("root_identity", &"<physical-root-identity>")
            .field("volume_identity", &"<physical-volume-identity>")
            .finish_non_exhaustive()
    }
}

impl RootProfileQualificationBasis {
    pub(super) fn new(binding: RootProfileBinding) -> Self {
        Self {
            artifact: Artifact::with_current_basis((), binding, qualification_authority()),
        }
    }

    pub(super) fn binding(&self) -> &RootProfileBinding {
        self.artifact.basis().basis().value()
    }

    pub const fn contract_version(&self) -> u16 {
        QUALIFICATION_CONTRACT_VERSION
    }

    pub fn root_identity(&self) -> [u8; 32] {
        self.binding().root_identity
    }

    pub fn volume_identity(&self) -> [u8; 32] {
        self.binding().volume_identity
    }
}

struct RootQualificationAuthority(());
impl AuthorityMarker for RootQualificationAuthority {}

fn qualification_authority() -> AuthorityWitness<RootQualificationAuthority> {
    AuthorityWitness::from_authority_marker(RootQualificationAuthority(()))
}
