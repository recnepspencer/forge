use forge_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityWitness,
    BoundaryBridgedAuthorityRevalidationRequiredBasis, CurrentValidity, FreshnessScopedBasis,
    NoProofs, TransitionOutcome,
};

use super::{
    CertificationPostureProfile, FoundationalProfileProgressionDeferred,
    FoundationalProfileProgressionFailure, FoundationalProfileProgressionRebindRequired,
    FoundationalProfileProgressionStale, FoundationalProfiledArtifact, ProofBearingArtifactTarget,
    ProofBearingProfiledArtifact,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalProfileCertificationProofLane {
    CurrentBasisArtifactWithBoundaryReadmission,
}

pub const fn foundational_profile_certification_proof_lane(
) -> FoundationalProfileCertificationProofLane {
    FoundationalProfileCertificationProofLane::CurrentBasisArtifactWithBoundaryReadmission
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvidenceBackedCertifiedProofBearingPhase;
impl forge_proof::PhaseMarker for EvidenceBackedCertifiedProofBearingPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductionCertifiedProofBearingPhase;
impl forge_proof::PhaseMarker for ProductionCertifiedProofBearingPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EvidenceBackedCertificationBasis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProductionCertifiedCertificationBasis;

type EvidenceBackedInner<T> = Artifact<
    EvidenceBackedCertifiedProofBearingPhase,
    FoundationalProfiledArtifact<ProofBearingArtifactTarget, T>,
    NoProofs,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<EvidenceBackedCertificationBasis>>,
>;

type BridgedEvidenceBackedInner<T> = Artifact<
    EvidenceBackedCertifiedProofBearingPhase,
    FoundationalProfiledArtifact<ProofBearingArtifactTarget, T>,
    NoProofs,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<EvidenceBackedCertificationBasis>,
>;

type ProductionCertifiedInner<T> = Artifact<
    ProductionCertifiedProofBearingPhase,
    FoundationalProfiledArtifact<ProofBearingArtifactTarget, T>,
    NoProofs,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<ProductionCertifiedCertificationBasis>>,
>;

type BridgedProductionCertifiedInner<T> = Artifact<
    ProductionCertifiedProofBearingPhase,
    FoundationalProfiledArtifact<ProofBearingArtifactTarget, T>,
    NoProofs,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<ProductionCertifiedCertificationBasis>,
>;

pub struct EvidenceBackedCertifiedProofBearingArtifact<T> {
    inner: EvidenceBackedInner<T>,
}

impl<T> EvidenceBackedCertifiedProofBearingArtifact<T> {
    fn new(inner: EvidenceBackedInner<T>) -> Self {
        Self { inner }
    }

    pub fn payload(&self) -> &T {
        self.inner.payload().payload()
    }

    pub fn profiled(&self) -> &FoundationalProfiledArtifact<ProofBearingArtifactTarget, T> {
        self.inner.payload()
    }
}

pub struct BoundaryBridgedEvidenceBackedCertifiedProofBearingArtifact<T> {
    inner: BridgedEvidenceBackedInner<T>,
}

impl<T> BoundaryBridgedEvidenceBackedCertifiedProofBearingArtifact<T> {
    fn new(inner: BridgedEvidenceBackedInner<T>) -> Self {
        Self { inner }
    }

    pub fn payload(&self) -> &T {
        self.inner.payload().payload()
    }
}

pub struct ProductionCertifiedProofBearingArtifact<T> {
    inner: ProductionCertifiedInner<T>,
}

impl<T> ProductionCertifiedProofBearingArtifact<T> {
    fn new(inner: ProductionCertifiedInner<T>) -> Self {
        Self { inner }
    }

    pub fn payload(&self) -> &T {
        self.inner.payload().payload()
    }

    pub fn profiled(&self) -> &FoundationalProfiledArtifact<ProofBearingArtifactTarget, T> {
        self.inner.payload()
    }
}

pub struct BoundaryBridgedProductionCertifiedProofBearingArtifact<T> {
    inner: BridgedProductionCertifiedInner<T>,
}

impl<T> BoundaryBridgedProductionCertifiedProofBearingArtifact<T> {
    fn new(inner: BridgedProductionCertifiedInner<T>) -> Self {
        Self { inner }
    }

    pub fn payload(&self) -> &T {
        self.inner.payload().payload()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileCertificationAuthority(());

impl FoundationalProfileCertificationAuthority {
    pub(crate) const fn milestone_3_phase_6() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalProfileCertificationAuthority {}

pub fn foundational_profile_certification_authority(
) -> AuthorityWitness<FoundationalProfileCertificationAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalProfileCertificationAuthority::milestone_3_phase_6(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfileCertificationReadmissionAuthority(());

impl FoundationalProfileCertificationReadmissionAuthority {
    pub(crate) const fn milestone_3_phase_6_boundary() -> Self {
        Self(())
    }
}

impl AuthorityMarker for FoundationalProfileCertificationReadmissionAuthority {}

pub fn foundational_profile_certification_readmission_authority(
) -> AuthorityWitness<FoundationalProfileCertificationReadmissionAuthority> {
    AuthorityWitness::from_authority_marker(
        FoundationalProfileCertificationReadmissionAuthority::milestone_3_phase_6_boundary(),
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalProfileCertificationDenial {
    EvidenceBackedStrengtheningRequiresEvidenceBackedProfile,
    ProductionCertifiedStrengtheningRequiresProductionCertifiedProfile,
}

pub type FoundationalProfileCertificationOutcome<S> = TransitionOutcome<
    S,
    FoundationalProfileCertificationDenial,
    FoundationalProfileProgressionDeferred,
    FoundationalProfileProgressionStale,
    FoundationalProfileProgressionRebindRequired,
    FoundationalProfileProgressionFailure,
>;

pub fn certify_evidence_backed_proof_bearing_artifact<T>(
    artifact: ProofBearingProfiledArtifact<T>,
    authority: AuthorityWitness<FoundationalProfileCertificationAuthority>,
) -> FoundationalProfileCertificationOutcome<EvidenceBackedCertifiedProofBearingArtifact<T>> {
    if artifact
        .payload()
        .profile()
        .materialized()
        .certification_posture()
        == CertificationPostureProfile::Uncertified
    {
        return TransitionOutcome::denied(
            FoundationalProfileCertificationDenial::EvidenceBackedStrengtheningRequiresEvidenceBackedProfile,
        );
    }

    let (payload, _, _) = artifact.into_parts().into_parts();
    TransitionOutcome::success(EvidenceBackedCertifiedProofBearingArtifact::new(
        Artifact::with_current_basis(payload, EvidenceBackedCertificationBasis, authority),
    ))
}

pub fn certify_production_certified_proof_bearing_artifact<T>(
    artifact: EvidenceBackedCertifiedProofBearingArtifact<T>,
    authority: AuthorityWitness<FoundationalProfileCertificationAuthority>,
) -> FoundationalProfileCertificationOutcome<ProductionCertifiedProofBearingArtifact<T>> {
    if artifact
        .profiled()
        .profile()
        .materialized()
        .certification_posture()
        != CertificationPostureProfile::ProductionCertified
    {
        return TransitionOutcome::denied(
            FoundationalProfileCertificationDenial::ProductionCertifiedStrengtheningRequiresProductionCertifiedProfile,
        );
    }

    let (payload, _, _) = artifact.inner.into_parts().into_parts();
    TransitionOutcome::success(ProductionCertifiedProofBearingArtifact::new(
        Artifact::with_current_basis(payload, ProductionCertifiedCertificationBasis, authority),
    ))
}

pub fn bridge_evidence_backed_proof_bearing_artifact_trust_boundary<T>(
    artifact: EvidenceBackedCertifiedProofBearingArtifact<T>,
) -> BoundaryBridgedEvidenceBackedCertifiedProofBearingArtifact<T> {
    BoundaryBridgedEvidenceBackedCertifiedProofBearingArtifact::new(
        artifact.inner.bridge_trust_boundary(),
    )
}

pub fn readmit_evidence_backed_proof_bearing_artifact_after_boundary<T>(
    artifact: BoundaryBridgedEvidenceBackedCertifiedProofBearingArtifact<T>,
    authority: AuthorityWitness<FoundationalProfileCertificationReadmissionAuthority>,
) -> EvidenceBackedCertifiedProofBearingArtifact<T> {
    EvidenceBackedCertifiedProofBearingArtifact::new(
        artifact
            .inner
            .readmit_with_authority(EvidenceBackedCertificationBasis, authority),
    )
}

pub fn bridge_production_certified_proof_bearing_artifact_trust_boundary<T>(
    artifact: ProductionCertifiedProofBearingArtifact<T>,
) -> BoundaryBridgedProductionCertifiedProofBearingArtifact<T> {
    BoundaryBridgedProductionCertifiedProofBearingArtifact::new(
        artifact.inner.bridge_trust_boundary(),
    )
}

pub fn readmit_production_certified_proof_bearing_artifact_after_boundary<T>(
    artifact: BoundaryBridgedProductionCertifiedProofBearingArtifact<T>,
    authority: AuthorityWitness<FoundationalProfileCertificationReadmissionAuthority>,
) -> ProductionCertifiedProofBearingArtifact<T> {
    ProductionCertifiedProofBearingArtifact::new(
        artifact
            .inner
            .readmit_with_authority(ProductionCertifiedCertificationBasis, authority),
    )
}
