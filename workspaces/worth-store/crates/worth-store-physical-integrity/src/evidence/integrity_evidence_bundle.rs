use super::integrity_evidence_quarantine::{
    quarantine_evidence_denial_count, quarantine_evidence_outcome, quarantine_receipt_claim_basis,
};
use crate::authority::integrity_authority_claim_basis::{
    checkpoint_authority_digest, manifest_authority_digest, page_authority_digest,
    wal_frame_authority_digest,
};
use crate::{
    ContainerIntegrityCounters, IndexPageIntegrityCounters, IntegrityEvidenceMaterializationPath,
    ManifestIntegrityCounters, PhysicalIntegrityEvidenceDenial, PhysicalIntegrityEvidenceProfile,
    PhysicalLocalityReport, ScrubCounterSnapshot, StoreDerivedProjectionBoundaryClaim,
    StoreExecutedIntegrityEvidence, StorePhysicalAuthorityBoundaryClaim,
    StoreReceiptEvidenceBoundaryClaim, StoreSupportOnlyBoundaryClaim, WalFrameIntegrityCounters,
};
use worth_foundational::{FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole};
use worth_store_contracts::StableDigest;
use worth_store_physical_format::PhysicalReferenceScope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityEvidenceOutcome {
    IntactPhysicalBoundary,
    RebuildableDerivedDamage,
    QuarantinedPhysicalDamage,
    UnrecoverableAuthorityDamage,
    IndeterminatePhysicalDamage,
    SupportOnlyDiagnostic,
    ReceiptEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityEvidenceLocality {
    PhysicalScope(PhysicalReferenceScope),
    Quarantine(PhysicalLocalityReport),
    SupportReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityEvidenceCounters {
    Container(ContainerIntegrityCounters),
    WalFrame(WalFrameIntegrityCounters),
    Manifest(ManifestIntegrityCounters),
    DerivedIndex(IndexPageIntegrityCounters),
    Receipt { attested_record_count: u64 },
    Support { source_evidence_count: u64 },
    Scrub(ScrubCounterSnapshot),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityProvenanceAttachment {
    basis: StableDigest,
}

impl IntegrityProvenanceAttachment {
    pub fn basis(&self) -> &StableDigest {
        &self.basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityPerformanceReceipt {
    counters: IntegrityEvidenceCounters,
    basis: StableDigest,
}
impl IntegrityPerformanceReceipt {
    pub const fn counters(&self) -> IntegrityEvidenceCounters {
        self.counters
    }

    pub fn basis(&self) -> &StableDigest {
        &self.basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityDiagnosticReport {
    outcome: IntegrityEvidenceOutcome,
    locality: IntegrityEvidenceLocality,
    basis: StableDigest,
}
impl IntegrityDiagnosticReport {
    pub fn from_executed_evidence(evidence: &PhysicalIntegrityEvidenceBundle) -> Self {
        Self {
            outcome: evidence.integrity_outcome().clone(),
            locality: evidence.locality(),
            basis: evidence.diagnostic_report().basis.clone(),
        }
    }

    pub const fn locality(&self) -> IntegrityEvidenceLocality {
        self.locality
    }

    pub fn outcome(&self) -> &IntegrityEvidenceOutcome {
        &self.outcome
    }

    pub fn basis(&self) -> &StableDigest {
        &self.basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIntegrityCertificationReceipt {
    basis: StableDigest,
}

impl PhysicalIntegrityCertificationReceipt {
    pub fn basis(&self) -> &StableDigest {
        &self.basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreIntegrityBoundaryClaim {
    PhysicalAuthority(StorePhysicalAuthorityBoundaryClaim),
    DerivedProjection(StoreDerivedProjectionBoundaryClaim),
    SupportOnly(StoreSupportOnlyBoundaryClaim),
    ReceiptEvidence(StoreReceiptEvidenceBoundaryClaim),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIntegrityEvidenceBundle {
    category: FoundationalBoundaryArtifactCategory,
    role: FoundationalBoundaryArtifactRole,
    outcome: IntegrityEvidenceOutcome,
    locality: IntegrityEvidenceLocality,
    counters: IntegrityEvidenceCounters,
    denial_count: u8,
    optional_forensic_material_count: u8,
    diagnostic: IntegrityDiagnosticReport,
    provenance: IntegrityProvenanceAttachment,
    performance: IntegrityPerformanceReceipt,
    receipt: PhysicalIntegrityCertificationReceipt,
    store_claim: StoreIntegrityBoundaryClaim,
    materialization_path: IntegrityEvidenceMaterializationPath,
}

impl PhysicalIntegrityEvidenceBundle {
    pub(crate) fn from_source(
        source: StoreExecutedIntegrityEvidence<'_>,
        profile: PhysicalIntegrityEvidenceProfile,
    ) -> Result<Self, PhysicalIntegrityEvidenceDenial> {
        let parts = EvidenceParts::from_source(source)?;
        let basis = evidence_digest(&parts);
        Ok(Self {
            category: parts.category,
            role: parts.role,
            outcome: parts.outcome,
            locality: parts.locality,
            counters: parts.counters,
            denial_count: parts.denial_count,
            optional_forensic_material_count: profile.optional_forensic_material_count(),
            diagnostic: IntegrityDiagnosticReport {
                outcome: parts.outcome,
                locality: parts.locality,
                basis: diagnostic_digest(&basis),
            },
            provenance: IntegrityProvenanceAttachment {
                basis: provenance_digest(&basis),
            },
            performance: IntegrityPerformanceReceipt {
                counters: parts.counters,
                basis: performance_digest(&basis),
            },
            receipt: PhysicalIntegrityCertificationReceipt {
                basis: receipt_digest(&basis),
            },
            store_claim: parts.claim,
            materialization_path: source.materialization_path(),
        })
    }

    pub const fn category(&self) -> FoundationalBoundaryArtifactCategory {
        self.category
    }

    pub const fn boundary_role(&self) -> FoundationalBoundaryArtifactRole {
        self.role
    }

    pub fn integrity_outcome(&self) -> &IntegrityEvidenceOutcome {
        &self.outcome
    }

    pub const fn locality(&self) -> IntegrityEvidenceLocality {
        self.locality
    }

    pub const fn counters(&self) -> IntegrityEvidenceCounters {
        self.counters
    }

    pub const fn denial_count(&self) -> u8 {
        self.denial_count
    }

    pub const fn optional_forensic_material_count(&self) -> u8 {
        self.optional_forensic_material_count
    }

    pub const fn diagnostic_report(&self) -> &IntegrityDiagnosticReport {
        &self.diagnostic
    }

    pub const fn provenance(&self) -> &IntegrityProvenanceAttachment {
        &self.provenance
    }

    pub const fn performance_receipt(&self) -> &IntegrityPerformanceReceipt {
        &self.performance
    }

    pub const fn certification_receipt(&self) -> &PhysicalIntegrityCertificationReceipt {
        &self.receipt
    }

    pub const fn store_claim(&self) -> &StoreIntegrityBoundaryClaim {
        &self.store_claim
    }

    pub(crate) const fn materialization_path(&self) -> IntegrityEvidenceMaterializationPath {
        self.materialization_path
    }

    pub(crate) fn has_same_evidence_basis_as(&self, other: &Self) -> bool {
        self.category == other.category
            && self.role == other.role
            && self.outcome == other.outcome
            && self.locality == other.locality
            && self.counters == other.counters
            && self.denial_count == other.denial_count
            && self.diagnostic == other.diagnostic
            && self.provenance == other.provenance
            && self.performance == other.performance
            && self.receipt == other.receipt
            && self.store_claim == other.store_claim
    }
}

#[derive(Debug, Clone)]
struct EvidenceParts {
    category: FoundationalBoundaryArtifactCategory,
    role: FoundationalBoundaryArtifactRole,
    outcome: IntegrityEvidenceOutcome,
    locality: IntegrityEvidenceLocality,
    counters: IntegrityEvidenceCounters,
    denial_count: u8,
    claim: StoreIntegrityBoundaryClaim,
}

impl EvidenceParts {
    fn from_source(
        source: StoreExecutedIntegrityEvidence<'_>,
    ) -> Result<Self, PhysicalIntegrityEvidenceDenial> {
        match source {
            StoreExecutedIntegrityEvidence::AuthoritativePage { report, .. } => {
                let basis = report.basis();
                let claim = physical_authority_claim(page_authority_digest(report)?);
                Ok(Self {
                    category: FoundationalBoundaryArtifactCategory::Artifact,
                    role: FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
                    outcome: IntegrityEvidenceOutcome::IntactPhysicalBoundary,
                    locality: IntegrityEvidenceLocality::PhysicalScope(basis.scope()),
                    counters: IntegrityEvidenceCounters::Container(report.counters()),
                    denial_count: 0,
                    claim,
                })
            }
            StoreExecutedIntegrityEvidence::AuthoritativeWalFrame { report } => {
                let basis = report.basis();
                let claim = physical_authority_claim(wal_frame_authority_digest(report)?);
                Ok(Self {
                    category: FoundationalBoundaryArtifactCategory::Artifact,
                    role: FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
                    outcome: IntegrityEvidenceOutcome::IntactPhysicalBoundary,
                    locality: IntegrityEvidenceLocality::PhysicalScope(basis.scope()),
                    counters: IntegrityEvidenceCounters::WalFrame(report.counters()),
                    denial_count: 0,
                    claim,
                })
            }
            StoreExecutedIntegrityEvidence::AuthoritativeCheckpointRecord { report } => {
                let basis = report.basis();
                let claim = physical_authority_claim(checkpoint_authority_digest(report)?);
                Ok(Self {
                    category: FoundationalBoundaryArtifactCategory::Artifact,
                    role: FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
                    outcome: IntegrityEvidenceOutcome::IntactPhysicalBoundary,
                    locality: IntegrityEvidenceLocality::PhysicalScope(basis.scope()),
                    counters: IntegrityEvidenceCounters::WalFrame(report.counters()),
                    denial_count: 0,
                    claim,
                })
            }
            StoreExecutedIntegrityEvidence::AuthoritativeManifest { report } => {
                let claim = physical_authority_claim(manifest_authority_digest(report)?);
                Ok(Self {
                    category: FoundationalBoundaryArtifactCategory::Artifact,
                    role: FoundationalBoundaryArtifactRole::AuthoritativeCurrent,
                    outcome: IntegrityEvidenceOutcome::IntactPhysicalBoundary,
                    locality: report
                        .reference_basis()
                        .admitted_scopes()
                        .first()
                        .copied()
                        .map(IntegrityEvidenceLocality::PhysicalScope)
                        .unwrap_or(IntegrityEvidenceLocality::SupportReport),
                    counters: IntegrityEvidenceCounters::Manifest(report.counters()),
                    denial_count: 0,
                    claim,
                })
            }
            StoreExecutedIntegrityEvidence::RebuildableDerived { report, .. } => {
                let digest = digest(format!("derived:{:?}", report.derived_basis()));
                Ok(Self {
                    category: FoundationalBoundaryArtifactCategory::Report,
                    role: FoundationalBoundaryArtifactRole::DerivedProjection,
                    outcome: IntegrityEvidenceOutcome::RebuildableDerivedDamage,
                    locality: IntegrityEvidenceLocality::PhysicalScope(
                        report.derived_basis().scope(),
                    ),
                    counters: IntegrityEvidenceCounters::DerivedIndex(report.counters()),
                    denial_count: 0,
                    claim: StoreIntegrityBoundaryClaim::DerivedProjection(
                        StoreDerivedProjectionBoundaryClaim::new(digest),
                    ),
                })
            }
            StoreExecutedIntegrityEvidence::ReceiptEvidence { record, .. } => Ok(Self {
                category: FoundationalBoundaryArtifactCategory::Receipt,
                role: FoundationalBoundaryArtifactRole::ReceiptEvidence,
                outcome: quarantine_evidence_outcome(record),
                locality: IntegrityEvidenceLocality::Quarantine(record.locality()),
                counters: IntegrityEvidenceCounters::Receipt {
                    attested_record_count: 1,
                },
                denial_count: quarantine_evidence_denial_count(record),
                claim: StoreIntegrityBoundaryClaim::ReceiptEvidence(
                    StoreReceiptEvidenceBoundaryClaim::new(quarantine_receipt_claim_basis(record)),
                ),
            }),
            StoreExecutedIntegrityEvidence::SupportDiagnostic { diagnostic } => {
                let digest = diagnostic.basis().clone();
                Ok(Self {
                    category: FoundationalBoundaryArtifactCategory::Report,
                    role: FoundationalBoundaryArtifactRole::SupportOnly,
                    outcome: IntegrityEvidenceOutcome::SupportOnlyDiagnostic,
                    locality: diagnostic.locality(),
                    counters: IntegrityEvidenceCounters::Support {
                        source_evidence_count: 1,
                    },
                    denial_count: 0,
                    claim: StoreIntegrityBoundaryClaim::SupportOnly(
                        StoreSupportOnlyBoundaryClaim::new(digest),
                    ),
                })
            }
        }
    }
}

fn physical_authority_claim(
    digest: worth_store_aspect_native::StoreDigestEvidence,
) -> StoreIntegrityBoundaryClaim {
    StoreIntegrityBoundaryClaim::PhysicalAuthority(StorePhysicalAuthorityBoundaryClaim::new(digest))
}

fn evidence_digest(parts: &EvidenceParts) -> StableDigest {
    digest(format!(
        "new-evidence:{:?}:{:?}:{:?}:{:?}:{:?}:{}",
        parts.category,
        parts.role,
        parts.outcome,
        parts.locality,
        parts.counters,
        parts.denial_count
    ))
}

fn diagnostic_digest(basis: &StableDigest) -> StableDigest {
    digest(format!("new-diagnostic:{}", basis.as_str()))
}

fn provenance_digest(basis: &StableDigest) -> StableDigest {
    digest(format!("new-provenance:{}", basis.as_str()))
}

fn performance_digest(basis: &StableDigest) -> StableDigest {
    digest(format!("new-performance:{}", basis.as_str()))
}

fn receipt_digest(basis: &StableDigest) -> StableDigest {
    digest(format!("new-receipt:{}", basis.as_str()))
}

fn digest(value: impl Into<String>) -> StableDigest {
    StableDigest::new(value).expect("S.3 evidence digest basis is non-empty")
}
