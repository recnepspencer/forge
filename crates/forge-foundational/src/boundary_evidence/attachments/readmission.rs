use forge_proof::{
    Artifact, AuthorityMarker, AuthorityWitness, BoundaryBridgedAuthorityRevalidationRequiredBasis,
    CurrentValidity, FreshnessScopedBasis, NoProofs,
};

use super::materialization::FoundationalMaterializedBoundaryEvidenceAttachmentBundle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoundaryEvidenceAttachmentCurrentBasis;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BoundaryEvidenceAttachmentSupportBasis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceAttachmentReadmissionAuthority(());

impl FoundationalBoundaryEvidenceAttachmentReadmissionAuthority {
    pub(crate) const fn milestone_7_phase_6() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalBoundaryEvidenceAttachmentReadmissionAuthority {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceSupportReadmissionAuthority(());

impl FoundationalBoundaryEvidenceSupportReadmissionAuthority {
    pub(crate) const fn milestone_7_phase_6() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalBoundaryEvidenceSupportReadmissionAuthority {}

pub fn foundational_boundary_evidence_attachment_readmission_authority(
) -> AuthorityWitness<FoundationalBoundaryEvidenceAttachmentReadmissionAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalBoundaryEvidenceAttachmentReadmissionAuthority::milestone_7_phase_6(),
    )
}

pub fn foundational_boundary_evidence_support_readmission_authority(
) -> AuthorityWitness<FoundationalBoundaryEvidenceSupportReadmissionAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalBoundaryEvidenceSupportReadmissionAuthority::milestone_7_phase_6(),
    )
}

type CurrentBasisInner = Artifact<
    CurrentValidity,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
    NoProofs,
    FreshnessScopedBasis<
        CurrentValidity,
        forge_proof::AssumptionBasis<BoundaryEvidenceAttachmentCurrentBasis>,
    >,
>;

type BridgedCurrentBasisInner = Artifact<
    CurrentValidity,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
    NoProofs,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<BoundaryEvidenceAttachmentCurrentBasis>,
>;

type SupportBasisInner = Artifact<
    CurrentValidity,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
    NoProofs,
    FreshnessScopedBasis<
        CurrentValidity,
        forge_proof::AssumptionBasis<BoundaryEvidenceAttachmentSupportBasis>,
    >,
>;

type BridgedSupportBasisInner = Artifact<
    CurrentValidity,
    FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
    NoProofs,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<BoundaryEvidenceAttachmentSupportBasis>,
>;

pub struct CurrentBasisBoundaryEvidenceAttachmentBundle {
    inner: CurrentBasisInner,
}

impl CurrentBasisBoundaryEvidenceAttachmentBundle {
    fn new(inner: CurrentBasisInner) -> Self {
        Self { inner }
    }

    pub fn payload(&self) -> &FoundationalMaterializedBoundaryEvidenceAttachmentBundle {
        self.inner.payload()
    }
}

pub struct BoundaryBridgedCurrentBasisBoundaryEvidenceAttachmentBundle {
    inner: BridgedCurrentBasisInner,
}

impl BoundaryBridgedCurrentBasisBoundaryEvidenceAttachmentBundle {
    fn new(inner: BridgedCurrentBasisInner) -> Self {
        Self { inner }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalBoundaryEvidenceSupportReadmissionDenial {
    SupportAttachmentRequired,
}

pub struct SupportBasisBoundaryEvidenceAttachmentBundle {
    inner: SupportBasisInner,
}

impl SupportBasisBoundaryEvidenceAttachmentBundle {
    fn new(inner: SupportBasisInner) -> Self {
        Self { inner }
    }

    pub fn payload(&self) -> &FoundationalMaterializedBoundaryEvidenceAttachmentBundle {
        self.inner.payload()
    }
}

pub struct BoundaryBridgedSupportBasisBoundaryEvidenceAttachmentBundle {
    inner: BridgedSupportBasisInner,
}

impl BoundaryBridgedSupportBasisBoundaryEvidenceAttachmentBundle {
    fn new(inner: BridgedSupportBasisInner) -> Self {
        Self { inner }
    }
}

pub fn admit_current_basis_boundary_evidence_attachment_bundle(
    bundle: FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
    authority: AuthorityWitness<FoundationalBoundaryEvidenceAttachmentReadmissionAuthority>,
) -> CurrentBasisBoundaryEvidenceAttachmentBundle {
    CurrentBasisBoundaryEvidenceAttachmentBundle::new(Artifact::with_current_basis(
        bundle,
        BoundaryEvidenceAttachmentCurrentBasis,
        authority,
    ))
}

pub fn bridge_current_basis_boundary_evidence_attachment_bundle_trust_boundary(
    bundle: CurrentBasisBoundaryEvidenceAttachmentBundle,
) -> BoundaryBridgedCurrentBasisBoundaryEvidenceAttachmentBundle {
    BoundaryBridgedCurrentBasisBoundaryEvidenceAttachmentBundle::new(
        bundle.inner.bridge_trust_boundary(),
    )
}

pub fn readmit_current_basis_boundary_evidence_attachment_bundle_after_boundary(
    bundle: BoundaryBridgedCurrentBasisBoundaryEvidenceAttachmentBundle,
    authority: AuthorityWitness<FoundationalBoundaryEvidenceAttachmentReadmissionAuthority>,
) -> CurrentBasisBoundaryEvidenceAttachmentBundle {
    CurrentBasisBoundaryEvidenceAttachmentBundle::new(
        bundle
            .inner
            .readmit_with_authority(BoundaryEvidenceAttachmentCurrentBasis, authority),
    )
}

pub fn admit_support_basis_boundary_evidence_attachment_bundle(
    bundle: FoundationalMaterializedBoundaryEvidenceAttachmentBundle,
    authority: AuthorityWitness<FoundationalBoundaryEvidenceSupportReadmissionAuthority>,
) -> Result<
    SupportBasisBoundaryEvidenceAttachmentBundle,
    FoundationalBoundaryEvidenceSupportReadmissionDenial,
> {
    if bundle.support().is_none() {
        return Err(
            FoundationalBoundaryEvidenceSupportReadmissionDenial::SupportAttachmentRequired,
        );
    }

    Ok(SupportBasisBoundaryEvidenceAttachmentBundle::new(
        Artifact::with_current_basis(bundle, BoundaryEvidenceAttachmentSupportBasis, authority),
    ))
}

pub fn bridge_support_basis_boundary_evidence_attachment_bundle_trust_boundary(
    bundle: SupportBasisBoundaryEvidenceAttachmentBundle,
) -> BoundaryBridgedSupportBasisBoundaryEvidenceAttachmentBundle {
    BoundaryBridgedSupportBasisBoundaryEvidenceAttachmentBundle::new(
        bundle.inner.bridge_trust_boundary(),
    )
}

pub fn readmit_support_basis_boundary_evidence_attachment_bundle_after_boundary(
    bundle: BoundaryBridgedSupportBasisBoundaryEvidenceAttachmentBundle,
    authority: AuthorityWitness<FoundationalBoundaryEvidenceSupportReadmissionAuthority>,
) -> SupportBasisBoundaryEvidenceAttachmentBundle {
    SupportBasisBoundaryEvidenceAttachmentBundle::new(
        bundle
            .inner
            .readmit_with_authority(BoundaryEvidenceAttachmentSupportBasis, authority),
    )
}
