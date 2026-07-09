use worth_foundational::{
    admit_requested_foundational_profile, claim_derived_projection_boundary_surface,
    claim_receipt_evidence_boundary_surface, claim_support_only_boundary_surface,
    foundational_profile_progression_authority, materialize_admitted_foundational_profile,
    plan_artifact_boundary_bundle, plan_descriptive_boundary_materialization,
    request_foundational_profile_set, AdmissionReadinessProfile, CertificationPostureProfile,
    CompatibilityPostureProfile, DiagnosticRichnessProfile, FoundationalBoundaryArtifactSurface,
    FoundationalBoundaryMaterializationBundle, FoundationalBoundaryMaterializationSeam,
    FoundationalBoundaryMaterializationSource, FoundationalBoundaryReceiptSurface,
    FoundationalBoundaryReportSurface, FoundationalProfileSet, FoundationalProfileSetInput,
    MaterializedFoundationalProfileSet, RetentionDeliveryProfile, SupportPostureProfile,
};
use worth_proof::TransitionOutcome;

use super::authority::{evidence_bundle_readmission_authority, EvidenceBundleReadmissionAuthority};
use super::{PhysicalCertificationEvidenceBundle, PhysicalEvidenceBundlePrimary};

pub type FoundationalPhysicalEvidenceMaterialization = FoundationalBoundaryMaterializationBundle<
    PhysicalEvidenceBundlePrimary,
    PhysicalEvidenceReportRow,
>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalEvidenceReportRow {
    name: String,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalPhysicalCertificationEvidenceBundle {
    materialized: FoundationalPhysicalEvidenceMaterialization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryBridgedPhysicalCertificationEvidenceBundle {
    materialized: FoundationalPhysicalEvidenceMaterialization,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReadmittedPhysicalCertificationEvidenceBundle {
    materialized: FoundationalPhysicalEvidenceMaterialization,
    authority: worth_proof::AuthorityWitness<EvidenceBundleReadmissionAuthority>,
}

impl FoundationalPhysicalCertificationEvidenceBundle {
    pub(crate) fn from_store_evidence(evidence: &PhysicalCertificationEvidenceBundle) -> Self {
        Self {
            materialized: materialize_store_evidence(evidence),
        }
    }

    pub const fn materialized(&self) -> &FoundationalPhysicalEvidenceMaterialization {
        &self.materialized
    }

    pub fn bridge_trust_boundary(self) -> BoundaryBridgedPhysicalCertificationEvidenceBundle {
        BoundaryBridgedPhysicalCertificationEvidenceBundle {
            materialized: self.materialized,
        }
    }
}

impl BoundaryBridgedPhysicalCertificationEvidenceBundle {
    pub const fn materialized(&self) -> &FoundationalPhysicalEvidenceMaterialization {
        &self.materialized
    }
}

impl ReadmittedPhysicalCertificationEvidenceBundle {
    pub const fn materialized(&self) -> &FoundationalPhysicalEvidenceMaterialization {
        &self.materialized
    }

    pub const fn supports_later_certification_comparison(&self) -> bool {
        true
    }
}

pub fn readmit_foundational_physical_evidence_after_boundary(
    bridged: BoundaryBridgedPhysicalCertificationEvidenceBundle,
) -> ReadmittedPhysicalCertificationEvidenceBundle {
    ReadmittedPhysicalCertificationEvidenceBundle {
        materialized: bridged.materialized,
        authority: evidence_bundle_readmission_authority(),
    }
}

impl PhysicalEvidenceReportRow {
    fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

fn materialize_store_evidence(
    evidence: &PhysicalCertificationEvidenceBundle,
) -> FoundationalPhysicalEvidenceMaterialization {
    let profile = materialized_profile_set();
    let primary = plan_descriptive_boundary_materialization(
        claim_derived_projection_boundary_surface(FoundationalBoundaryArtifactSurface::new(
            evidence.primary(),
            evidence.replay().oracle_verdicts().len(),
        )),
        FoundationalBoundaryMaterializationSource::NativeAuthority,
        FoundationalBoundaryMaterializationSeam::BoundaryExchange,
        profile,
    )
    .expect("physical evidence primary materialization plan is valid");
    let report = plan_descriptive_boundary_materialization(
        claim_support_only_boundary_surface(
            FoundationalBoundaryReportSurface::new(report_rows(evidence), 1)
                .expect("physical evidence report has rows"),
        ),
        FoundationalBoundaryMaterializationSource::NativeAuthority,
        FoundationalBoundaryMaterializationSeam::BoundaryExchange,
        profile,
    )
    .expect("physical evidence report materialization plan is valid");
    let receipt = plan_descriptive_boundary_materialization(
        claim_receipt_evidence_boundary_surface(
            FoundationalBoundaryReceiptSurface::new(
                "store physical simulation evidence bundle materialized",
                evidence.replay().counter_receipt().rows().len(),
            )
            .expect("physical evidence receipt is named"),
        ),
        FoundationalBoundaryMaterializationSource::NativeAuthority,
        FoundationalBoundaryMaterializationSeam::BoundaryExchange,
        profile,
    )
    .expect("physical evidence receipt materialization plan is valid");
    plan_artifact_boundary_bundle(primary)
        .with_report(report)
        .expect("physical evidence report member is legal")
        .with_receipt(receipt)
        .expect("physical evidence receipt member is legal")
        .materialize()
        .expect("physical evidence foundational bundle materializes")
}

fn report_rows(evidence: &PhysicalCertificationEvidenceBundle) -> Vec<PhysicalEvidenceReportRow> {
    vec![
        PhysicalEvidenceReportRow::new(
            "transcript.canonical_basis_entries",
            evidence
                .replay()
                .transcript_identity()
                .canonical_basis_entry_count()
                .to_string(),
        ),
        PhysicalEvidenceReportRow::new(
            "oracle.verdict_count",
            evidence.replay().oracle_verdicts().len().to_string(),
        ),
        PhysicalEvidenceReportRow::new(
            "counter.row_count",
            evidence.replay().counter_receipt().rows().len().to_string(),
        ),
    ]
}

fn materialized_profile_set() -> MaterializedFoundationalProfileSet {
    let profile = FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Forensic,
        support_posture: SupportPostureProfile::CertificationReady,
        compatibility_posture: CompatibilityPostureProfile::NativeOnly,
        admission_readiness: AdmissionReadinessProfile::ProductionGateReady,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
    })
    .expect("physical evidence profile composition is valid");
    let requested = request_foundational_profile_set(profile);
    let admitted = match admit_requested_foundational_profile(
        requested,
        profile,
        None,
        foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(admitted) => admitted,
        _ => panic!("physical evidence profile admission must not narrow"),
    };
    match materialize_admitted_foundational_profile(
        admitted,
        profile,
        None,
        foundational_profile_progression_authority(),
    ) {
        TransitionOutcome::Success(materialized) => *materialized.payload(),
        _ => panic!("physical evidence profile materialization must not narrow"),
    }
}
