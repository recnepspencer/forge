use forge_proof::{
    Artifact, AssumptionBasis, BoundaryBridgedAuthorityRevalidationRequiredBasis, CurrentValidity,
    FreshnessScopedBasis, Proof,
};

use crate::canonicalization::CanonicalBasisReadyArtifact;

use super::authority::{
    FoundationalPerformanceCertifiedAttachmentAuthority,
    FoundationalPerformanceCertifiedReadmissionAuthority,
};
use super::vocabulary::{
    FoundationalCertifiedPerformanceClass, FoundationalCertifiedPerformanceSourceKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceCertifiedPhase;
impl forge_proof::PhaseMarker for FoundationalPerformanceCertifiedPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformanceCertified;
impl forge_proof::ProofMarker for FoundationalPerformanceCertified {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalCertifiedPerformanceSourceDigest {
    domain: crate::canonicalization::CanonicalBasisDomain,
    version: crate::canonicalization::CanonicalizationRuleVersion,
    entry_count: u32,
}

impl FoundationalCertifiedPerformanceSourceDigest {
    pub(crate) fn from_basis(basis: &CanonicalBasisReadyArtifact) -> Self {
        Self {
            domain: basis.payload().domain(),
            version: basis.payload().version().clone(),
            entry_count: basis.payload().entries().len() as u32,
        }
    }

    pub const fn domain(&self) -> crate::canonicalization::CanonicalBasisDomain {
        self.domain
    }

    pub fn version(&self) -> &crate::canonicalization::CanonicalizationRuleVersion {
        &self.version
    }

    pub const fn entry_count(&self) -> u32 {
        self.entry_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalCertifiedPerformancePayload<Source> {
    source: Source,
    source_kind: FoundationalCertifiedPerformanceSourceKind,
    certified_class: FoundationalCertifiedPerformanceClass,
    source_digest: FoundationalCertifiedPerformanceSourceDigest,
}

impl<Source> FoundationalCertifiedPerformancePayload<Source> {
    pub(crate) fn new(
        source: Source,
        source_kind: FoundationalCertifiedPerformanceSourceKind,
        certified_class: FoundationalCertifiedPerformanceClass,
        source_digest: FoundationalCertifiedPerformanceSourceDigest,
    ) -> Self {
        Self {
            source,
            source_kind,
            certified_class,
            source_digest,
        }
    }
}

type CertifiedPerformanceInner<Source> = Artifact<
    FoundationalPerformanceCertifiedPhase,
    FoundationalCertifiedPerformancePayload<Source>,
    Proof<FoundationalPerformanceCertified, FoundationalPerformanceCertifiedAttachmentAuthority>,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<CanonicalBasisReadyArtifact>>,
>;

type BridgedCertifiedPerformanceInner<Source> = Artifact<
    FoundationalPerformanceCertifiedPhase,
    FoundationalCertifiedPerformancePayload<Source>,
    Proof<FoundationalPerformanceCertified, FoundationalPerformanceCertifiedAttachmentAuthority>,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<CanonicalBasisReadyArtifact>,
>;

pub struct FoundationalCertifiedPerformanceBundle<Source> {
    pub(crate) inner: CertifiedPerformanceInner<Source>,
}

impl<Source> FoundationalCertifiedPerformanceBundle<Source> {
    pub(crate) fn new(inner: CertifiedPerformanceInner<Source>) -> Self {
        Self { inner }
    }

    pub fn source(&self) -> &Source {
        &self.inner.payload().source
    }

    pub fn source_kind(&self) -> FoundationalCertifiedPerformanceSourceKind {
        self.inner.payload().source_kind
    }

    pub fn certified_class(&self) -> FoundationalCertifiedPerformanceClass {
        self.inner.payload().certified_class
    }

    pub fn source_digest(&self) -> &FoundationalCertifiedPerformanceSourceDigest {
        &self.inner.payload().source_digest
    }

    pub fn proofs(
        &self,
    ) -> &Proof<FoundationalPerformanceCertified, FoundationalPerformanceCertifiedAttachmentAuthority>
    {
        self.inner.proofs()
    }

    pub fn readmission_basis(&self) -> &CanonicalBasisReadyArtifact {
        self.inner.strong_basis().value()
    }
}

pub struct BoundaryBridgedCertifiedPerformanceBundle<Source> {
    inner: BridgedCertifiedPerformanceInner<Source>,
}

impl<Source> BoundaryBridgedCertifiedPerformanceBundle<Source> {
    pub(crate) fn new(inner: BridgedCertifiedPerformanceInner<Source>) -> Self {
        Self { inner }
    }

    pub fn source(&self) -> &Source {
        &self.inner.payload().source
    }

    pub fn certified_class(&self) -> FoundationalCertifiedPerformanceClass {
        self.inner.payload().certified_class
    }
}

pub fn bridge_certified_performance_bundle_trust_boundary<Source>(
    bundle: FoundationalCertifiedPerformanceBundle<Source>,
) -> BoundaryBridgedCertifiedPerformanceBundle<Source> {
    BoundaryBridgedCertifiedPerformanceBundle::new(bundle.inner.bridge_trust_boundary())
}

pub fn readmit_certified_performance_bundle_after_boundary<Source>(
    bundle: BoundaryBridgedCertifiedPerformanceBundle<Source>,
    basis: CanonicalBasisReadyArtifact,
    authority: forge_proof::AuthorityWitness<FoundationalPerformanceCertifiedReadmissionAuthority>,
) -> FoundationalCertifiedPerformanceBundle<Source> {
    FoundationalCertifiedPerformanceBundle::new(
        bundle.inner.readmit_with_authority(basis, authority),
    )
}
