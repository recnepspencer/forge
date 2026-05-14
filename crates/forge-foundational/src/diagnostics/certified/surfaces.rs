use forge_proof::{
    Artifact, AssumptionBasis, BoundaryBridgedAuthorityRevalidationRequiredBasis, CurrentValidity,
    FreshnessScopedBasis, Proof,
};

use super::authority::{
    FoundationalDiagnosticCertifiedAttachmentAuthority,
    FoundationalDiagnosticCertifiedReadmissionAuthority,
};
use super::vocabulary::{
    FoundationalCertifiedDiagnosticProvenanceHook, FoundationalCertifiedDiagnosticSourceKind,
    FoundationalDiagnosticCertifiedCoverageClass, FoundationalDiagnosticCoverageMatrix,
};
use crate::canonicalization::CanonicalBasisReadyArtifact;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticCertifiedPhase;
impl forge_proof::PhaseMarker for FoundationalDiagnosticCertifiedPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalDiagnosticCertified;
impl forge_proof::ProofMarker for FoundationalDiagnosticCertified {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalCertifiedDiagnosticSourceDigest {
    domain: crate::canonicalization::CanonicalBasisDomain,
    version: crate::canonicalization::CanonicalizationRuleVersion,
    entry_count: u32,
}

impl FoundationalCertifiedDiagnosticSourceDigest {
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
pub struct FoundationalCertifiedDiagnosticPayload<Source, Bundle> {
    source: Source,
    source_kind: FoundationalCertifiedDiagnosticSourceKind,
    source_digest: FoundationalCertifiedDiagnosticSourceDigest,
    bundle: Bundle,
    coverage_class: FoundationalDiagnosticCertifiedCoverageClass,
    coverage_matrix: FoundationalDiagnosticCoverageMatrix,
    provenance_hook: FoundationalCertifiedDiagnosticProvenanceHook,
}

impl<Source, Bundle> FoundationalCertifiedDiagnosticPayload<Source, Bundle> {
    pub(crate) fn new(
        source: Source,
        source_kind: FoundationalCertifiedDiagnosticSourceKind,
        source_digest: FoundationalCertifiedDiagnosticSourceDigest,
        bundle: Bundle,
        coverage_class: FoundationalDiagnosticCertifiedCoverageClass,
        coverage_matrix: FoundationalDiagnosticCoverageMatrix,
        provenance_hook: FoundationalCertifiedDiagnosticProvenanceHook,
    ) -> Self {
        Self {
            source,
            source_kind,
            source_digest,
            bundle,
            coverage_class,
            coverage_matrix,
            provenance_hook,
        }
    }
}

type CertifiedDiagnosticInner<Source, Bundle> = Artifact<
    FoundationalDiagnosticCertifiedPhase,
    FoundationalCertifiedDiagnosticPayload<Source, Bundle>,
    Proof<FoundationalDiagnosticCertified, FoundationalDiagnosticCertifiedAttachmentAuthority>,
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<CanonicalBasisReadyArtifact>>,
>;

type BridgedCertifiedDiagnosticInner<Source, Bundle> = Artifact<
    FoundationalDiagnosticCertifiedPhase,
    FoundationalCertifiedDiagnosticPayload<Source, Bundle>,
    Proof<FoundationalDiagnosticCertified, FoundationalDiagnosticCertifiedAttachmentAuthority>,
    BoundaryBridgedAuthorityRevalidationRequiredBasis<CanonicalBasisReadyArtifact>,
>;

pub struct FoundationalCertifiedDiagnosticBundle<Source, Bundle> {
    pub(crate) inner: CertifiedDiagnosticInner<Source, Bundle>,
}

impl<Source, Bundle> FoundationalCertifiedDiagnosticBundle<Source, Bundle> {
    pub(crate) fn new(inner: CertifiedDiagnosticInner<Source, Bundle>) -> Self {
        Self { inner }
    }

    pub fn source(&self) -> &Source {
        &self.inner.payload().source
    }

    pub fn source_kind(&self) -> FoundationalCertifiedDiagnosticSourceKind {
        self.inner.payload().source_kind
    }

    pub fn source_digest(&self) -> &FoundationalCertifiedDiagnosticSourceDigest {
        &self.inner.payload().source_digest
    }

    pub fn bundle(&self) -> &Bundle {
        &self.inner.payload().bundle
    }

    pub fn coverage_class(&self) -> FoundationalDiagnosticCertifiedCoverageClass {
        self.inner.payload().coverage_class
    }

    pub fn coverage_matrix(&self) -> &FoundationalDiagnosticCoverageMatrix {
        &self.inner.payload().coverage_matrix
    }

    pub fn provenance_hook(&self) -> FoundationalCertifiedDiagnosticProvenanceHook {
        self.inner.payload().provenance_hook
    }

    pub fn proofs(
        &self,
    ) -> &Proof<FoundationalDiagnosticCertified, FoundationalDiagnosticCertifiedAttachmentAuthority>
    {
        self.inner.proofs()
    }

    pub fn strong_basis(&self) -> &CanonicalBasisReadyArtifact {
        self.inner.strong_basis().value()
    }
}

pub struct BoundaryBridgedCertifiedDiagnosticBundle<Source, Bundle> {
    inner: BridgedCertifiedDiagnosticInner<Source, Bundle>,
}

impl<Source, Bundle> BoundaryBridgedCertifiedDiagnosticBundle<Source, Bundle> {
    pub(crate) fn new(inner: BridgedCertifiedDiagnosticInner<Source, Bundle>) -> Self {
        Self { inner }
    }

    pub fn source(&self) -> &Source {
        &self.inner.payload().source
    }

    pub fn bundle(&self) -> &Bundle {
        &self.inner.payload().bundle
    }
}

pub fn bridge_certified_diagnostic_bundle_trust_boundary<Source, Bundle>(
    bundle: FoundationalCertifiedDiagnosticBundle<Source, Bundle>,
) -> BoundaryBridgedCertifiedDiagnosticBundle<Source, Bundle> {
    BoundaryBridgedCertifiedDiagnosticBundle::new(bundle.inner.bridge_trust_boundary())
}

pub fn readmit_certified_diagnostic_bundle_after_boundary<Source, Bundle>(
    bundle: BoundaryBridgedCertifiedDiagnosticBundle<Source, Bundle>,
    basis: CanonicalBasisReadyArtifact,
    authority: forge_proof::AuthorityWitness<FoundationalDiagnosticCertifiedReadmissionAuthority>,
) -> FoundationalCertifiedDiagnosticBundle<Source, Bundle> {
    FoundationalCertifiedDiagnosticBundle::new(
        bundle.inner.readmit_with_authority(basis, authority),
    )
}
