use crate::workload_platform::evidence_lookup_family_catalog::{
    EvidenceLookupFamilyQueryPosture, EvidenceLookupFamilyQueryPostureKind,
    EvidenceLookupProjectionFactFamily, EvidenceLookupQueryImportEvidence,
};
use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupQuerySurfaceContractProvenance {
    SupportAdmission,
    SupportPinning,
    ProjectionConsumption,
    LowerRuntimeBoundaryEnvelope,
    TypedArtifactIdentity,
    ConsumerKitProof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupQuerySurfaceContract {
    query_surface: EvidenceLookupQuerySurface,
    query_surface_type_name: &'static str,
    projection_fact_family: Option<EvidenceLookupProjectionFactFamily>,
    proof_digest: String,
    provenance: EvidenceLookupQuerySurfaceContractProvenance,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupProductQuerySurfaceContractRow {
    family_identity: String,
    contract: EvidenceLookupQuerySurfaceContract,
}

impl EvidenceLookupQuerySurfaceContract {
    pub(crate) fn from_family_query_posture(
        posture: &EvidenceLookupFamilyQueryPosture,
    ) -> Option<Self> {
        let imported_evidence = posture.imported_evidence()?;
        Some(Self::from_imported_evidence(
            posture.kind(),
            imported_evidence,
        ))
    }

    pub(crate) fn from_imported_evidence(
        kind: EvidenceLookupFamilyQueryPostureKind,
        imported_evidence: &EvidenceLookupQueryImportEvidence,
    ) -> Self {
        Self {
            query_surface: query_surface_from_kind(kind),
            query_surface_type_name: imported_evidence.query_surface_type_name(),
            projection_fact_family: imported_evidence.projection_fact_family(),
            proof_digest: imported_evidence.evidence_digest().to_string(),
            provenance: provenance_from_kind(kind),
        }
    }

    pub const fn query_surface(&self) -> EvidenceLookupQuerySurface {
        self.query_surface
    }

    pub const fn query_surface_type_name(&self) -> &'static str {
        self.query_surface_type_name
    }

    pub const fn projection_fact_family(&self) -> Option<EvidenceLookupProjectionFactFamily> {
        self.projection_fact_family
    }

    pub fn proof_digest(&self) -> &str {
        &self.proof_digest
    }

    pub const fn provenance(&self) -> EvidenceLookupQuerySurfaceContractProvenance {
        self.provenance
    }
}

impl EvidenceLookupProductQuerySurfaceContractRow {
    pub(crate) fn new(
        family_identity: impl Into<String>,
        contract: EvidenceLookupQuerySurfaceContract,
    ) -> Self {
        Self {
            family_identity: family_identity.into(),
            contract,
        }
    }

    pub fn family_identity(&self) -> &str {
        &self.family_identity
    }

    pub const fn contract(&self) -> &EvidenceLookupQuerySurfaceContract {
        &self.contract
    }
}

const fn query_surface_from_kind(
    kind: EvidenceLookupFamilyQueryPostureKind,
) -> EvidenceLookupQuerySurface {
    match kind {
        EvidenceLookupFamilyQueryPostureKind::NotRequired => EvidenceLookupQuerySurface::NotQuery,
        EvidenceLookupFamilyQueryPostureKind::ImportedSupportAdmissionRequired => {
            EvidenceLookupQuerySurface::SupportAdmission
        }
        EvidenceLookupFamilyQueryPostureKind::ImportedSupportPinRequired => {
            EvidenceLookupQuerySurface::SupportPinning
        }
        EvidenceLookupFamilyQueryPostureKind::ImportedProjectionConsumptionRequired => {
            EvidenceLookupQuerySurface::ProjectionConsumption
        }
        EvidenceLookupFamilyQueryPostureKind::ImportedLowerRuntimeBoundaryEnvelopeRequired => {
            EvidenceLookupQuerySurface::LowerRuntimeBoundaryEnvelope
        }
        EvidenceLookupFamilyQueryPostureKind::ImportedTypedArtifactIdentityRequired => {
            EvidenceLookupQuerySurface::TypedArtifactIdentity
        }
        EvidenceLookupFamilyQueryPostureKind::ImportedConsumerKitProofRequired => {
            EvidenceLookupQuerySurface::ConsumerKitProof
        }
    }
}

const fn provenance_from_kind(
    kind: EvidenceLookupFamilyQueryPostureKind,
) -> EvidenceLookupQuerySurfaceContractProvenance {
    match kind {
        EvidenceLookupFamilyQueryPostureKind::ImportedSupportAdmissionRequired => {
            EvidenceLookupQuerySurfaceContractProvenance::SupportAdmission
        }
        EvidenceLookupFamilyQueryPostureKind::ImportedSupportPinRequired => {
            EvidenceLookupQuerySurfaceContractProvenance::SupportPinning
        }
        EvidenceLookupFamilyQueryPostureKind::ImportedProjectionConsumptionRequired => {
            EvidenceLookupQuerySurfaceContractProvenance::ProjectionConsumption
        }
        EvidenceLookupFamilyQueryPostureKind::ImportedLowerRuntimeBoundaryEnvelopeRequired => {
            EvidenceLookupQuerySurfaceContractProvenance::LowerRuntimeBoundaryEnvelope
        }
        EvidenceLookupFamilyQueryPostureKind::ImportedTypedArtifactIdentityRequired => {
            EvidenceLookupQuerySurfaceContractProvenance::TypedArtifactIdentity
        }
        EvidenceLookupFamilyQueryPostureKind::ImportedConsumerKitProofRequired => {
            EvidenceLookupQuerySurfaceContractProvenance::ConsumerKitProof
        }
        EvidenceLookupFamilyQueryPostureKind::NotRequired => {
            EvidenceLookupQuerySurfaceContractProvenance::SupportAdmission
        }
    }
}
