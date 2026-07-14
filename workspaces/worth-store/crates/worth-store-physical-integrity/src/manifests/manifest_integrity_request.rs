use crate::{ManifestIntegrityDenial, ManifestIntegrityDenialKind};
use worth_store_physical_format::{
    PhysicalGenerationOwner, PhysicalReferenceAdmissionWitness, PhysicalReferenceScope,
    PhysicalRootManifest, PhysicalScopeFamily, RootManifestIntegrityPosture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestIntegrityInspectionRequest {
    root_evidence: ManifestRootIntegrityEvidence,
    expected_references: Vec<ManifestExpectedReference>,
    backend_residue_fallback: Option<PhysicalReferenceAdmissionWitness>,
    derived_override_attempt: Option<DerivedManifestOverrideAttempt>,
}

impl ManifestIntegrityInspectionRequest {
    pub fn from_root_publication(
        root: PhysicalRootManifest,
        root_admission: PhysicalReferenceAdmissionWitness,
    ) -> Self {
        Self {
            root_evidence: ManifestRootIntegrityEvidence::RootPublication {
                root,
                root_admission,
            },
            expected_references: Vec::new(),
            backend_residue_fallback: None,
            derived_override_attempt: None,
        }
    }

    pub fn missing_root() -> Self {
        Self::from_root_evidence(ManifestRootIntegrityEvidence::MissingRoot)
    }

    pub fn damaged_root(locality: PhysicalGenerationOwner) -> Self {
        Self::from_root_evidence(ManifestRootIntegrityEvidence::RootDamage {
            posture: RootManifestIntegrityPosture::DamagedRoot,
            denial: ManifestIntegrityDenialKind::DamagedRoot,
            locality: Some(locality),
        })
    }

    pub fn torn_root_pointer(locality: PhysicalGenerationOwner) -> Self {
        Self::from_root_evidence(ManifestRootIntegrityEvidence::RootDamage {
            posture: RootManifestIntegrityPosture::TornRootPointer,
            denial: ManifestIntegrityDenialKind::TornRootPointer,
            locality: Some(locality),
        })
    }

    pub fn multiple_valid_roots(first: PhysicalRootManifest, second: PhysicalRootManifest) -> Self {
        Self::from_root_evidence(ManifestRootIntegrityEvidence::MultipleValidRoots {
            first,
            second,
        })
    }

    pub fn root_generation_mismatch(
        root: PhysicalRootManifest,
        root_admission: PhysicalReferenceAdmissionWitness,
    ) -> Self {
        Self::from_root_evidence(ManifestRootIntegrityEvidence::RootPublication {
            root,
            root_admission,
        })
    }

    pub fn recovery_blocking_root_damage(locality: PhysicalGenerationOwner) -> Self {
        Self::from_root_evidence(ManifestRootIntegrityEvidence::RootDamage {
            posture: RootManifestIntegrityPosture::RecoveryBlockingRootDamage,
            denial: ManifestIntegrityDenialKind::RecoveryBlockingRootDamage,
            locality: Some(locality),
        })
    }

    pub fn with_expected_reference(mut self, reference: ManifestExpectedReference) -> Self {
        self.expected_references.push(reference);
        self
    }

    pub fn with_backend_residue_fallback(
        mut self,
        admission: PhysicalReferenceAdmissionWitness,
    ) -> Self {
        self.backend_residue_fallback = Some(admission);
        self
    }

    pub fn with_derived_override_attempt(
        mut self,
        attempt: DerivedManifestOverrideAttempt,
    ) -> Self {
        self.derived_override_attempt = Some(attempt);
        self
    }

    pub(crate) const fn root_evidence(&self) -> &ManifestRootIntegrityEvidence {
        &self.root_evidence
    }

    pub(crate) fn expected_references(&self) -> &[ManifestExpectedReference] {
        &self.expected_references
    }

    pub(crate) const fn backend_residue_fallback(
        &self,
    ) -> Option<PhysicalReferenceAdmissionWitness> {
        self.backend_residue_fallback
    }

    pub(crate) const fn derived_override_attempt(&self) -> Option<DerivedManifestOverrideAttempt> {
        self.derived_override_attempt
    }

    fn from_root_evidence(root_evidence: ManifestRootIntegrityEvidence) -> Self {
        Self {
            root_evidence,
            expected_references: Vec::new(),
            backend_residue_fallback: None,
            derived_override_attempt: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ManifestRootIntegrityEvidence {
    RootPublication {
        root: PhysicalRootManifest,
        root_admission: PhysicalReferenceAdmissionWitness,
    },
    MissingRoot,
    RootDamage {
        posture: RootManifestIntegrityPosture,
        denial: ManifestIntegrityDenialKind,
        locality: Option<PhysicalGenerationOwner>,
    },
    MultipleValidRoots {
        first: PhysicalRootManifest,
        second: PhysicalRootManifest,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManifestExpectedReference {
    PageSlot(PhysicalReferenceAdmissionWitness),
    Extent(PhysicalReferenceAdmissionWitness),
    FreeSpaceReuse(PhysicalReferenceAdmissionWitness),
}

impl ManifestExpectedReference {
    pub const fn page_slot(admission: PhysicalReferenceAdmissionWitness) -> Self {
        Self::PageSlot(admission)
    }

    pub const fn extent(admission: PhysicalReferenceAdmissionWitness) -> Self {
        Self::Extent(admission)
    }

    pub const fn free_space_reuse(admission: PhysicalReferenceAdmissionWitness) -> Self {
        Self::FreeSpaceReuse(admission)
    }

    pub const fn admission(self) -> PhysicalReferenceAdmissionWitness {
        match self {
            Self::PageSlot(admission)
            | Self::Extent(admission)
            | Self::FreeSpaceReuse(admission) => admission,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DerivedManifestOverrideAttempt {
    authoritative_failure: AuthoritativeManifestFailure,
    derived_scope: PhysicalReferenceScope,
}

impl DerivedManifestOverrideAttempt {
    pub fn against_authoritative_manifest_denial(
        authoritative_denial: &ManifestIntegrityDenial,
        derived_scope: PhysicalReferenceScope,
    ) -> Option<Self> {
        if derived_scope.family() != PhysicalScopeFamily::DerivedIndex {
            return None;
        }
        Some(Self {
            authoritative_failure: AuthoritativeManifestFailure::from_denial(authoritative_denial)?,
            derived_scope,
        })
    }

    pub const fn authoritative_failure(self) -> AuthoritativeManifestFailure {
        self.authoritative_failure
    }

    pub const fn derived_scope(self) -> PhysicalReferenceScope {
        self.derived_scope
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AuthoritativeManifestFailure {
    kind: ManifestIntegrityDenialKind,
    locality: PhysicalGenerationOwner,
}

impl AuthoritativeManifestFailure {
    pub fn from_denial(denial: &ManifestIntegrityDenial) -> Option<Self> {
        if !is_authoritative_manifest_failure_kind(denial.kind()) {
            return None;
        }
        Some(Self {
            kind: denial.kind(),
            locality: denial.locality()?,
        })
    }

    pub const fn kind(self) -> ManifestIntegrityDenialKind {
        self.kind
    }

    pub const fn locality(self) -> PhysicalGenerationOwner {
        self.locality
    }
}

const fn is_authoritative_manifest_failure_kind(kind: ManifestIntegrityDenialKind) -> bool {
    matches!(
        kind,
        ManifestIntegrityDenialKind::DamagedRoot
            | ManifestIntegrityDenialKind::TornRootPointer
            | ManifestIntegrityDenialKind::RecoveryBlockingRootDamage
            | ManifestIntegrityDenialKind::RootGenerationMismatch
            | ManifestIntegrityDenialKind::StaleManifestGeneration
            | ManifestIntegrityDenialKind::WrongSegmentId
            | ManifestIntegrityDenialKind::MismatchedExtentId
            | ManifestIntegrityDenialKind::DamagedAllocationMap
    )
}
