use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::proof::{
    PrimitiveConstructionProofGrade, PrimitiveConstructionProofSubject,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionVerifiedArtifactSurfaceRow {
    subject: PrimitiveConstructionProofSubject,
    grade: PrimitiveConstructionProofGrade,
    truth_type: &'static str,
    verified_type: &'static str,
    failure_type: &'static str,
}

impl PrimitiveConstructionVerifiedArtifactSurfaceRow {
    pub fn subject(&self) -> PrimitiveConstructionProofSubject {
        self.subject
    }
    pub fn grade(&self) -> PrimitiveConstructionProofGrade {
        self.grade
    }
    pub fn truth_type(&self) -> &'static str {
        self.truth_type
    }
    pub fn verified_type(&self) -> &'static str {
        self.verified_type
    }
    pub fn failure_type(&self) -> &'static str {
        self.failure_type
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionVerifiedArtifactSurfaceReport {
    rows: Vec<PrimitiveConstructionVerifiedArtifactSurfaceRow>,
    report_digest: String,
}

impl PrimitiveConstructionVerifiedArtifactSurfaceReport {
    pub fn rows(&self) -> &[PrimitiveConstructionVerifiedArtifactSurfaceRow] {
        &self.rows
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_verified_artifact_surface_report(
) -> PrimitiveConstructionVerifiedArtifactSurfaceReport {
    let rows = vec![
        PrimitiveConstructionVerifiedArtifactSurfaceRow {
            subject: PrimitiveConstructionProofSubject::Motion,
            grade: PrimitiveConstructionProofGrade::BundleCoherence,
            truth_type: "PrimitiveConstructionMotionCanonicalTruth",
            verified_type: "PrimitiveConstructionVerifiedMotionReportBundle",
            failure_type: "PrimitiveConstructionMotionBundleVerificationFailure",
        },
        PrimitiveConstructionVerifiedArtifactSurfaceRow {
            subject: PrimitiveConstructionProofSubject::IntentArbitration,
            grade: PrimitiveConstructionProofGrade::BundleCoherence,
            truth_type: "PrimitiveConstructionIntentArbitrationCanonicalTruth",
            verified_type: "PrimitiveConstructionVerifiedIntentArbitrationReportBundle",
            failure_type: "PrimitiveConstructionIntentArbitrationBundleVerificationFailure",
        },
        PrimitiveConstructionVerifiedArtifactSurfaceRow {
            subject: PrimitiveConstructionProofSubject::CompoundParity,
            grade: PrimitiveConstructionProofGrade::BundleCoherence,
            truth_type: "PrimitiveConstructionCompoundParityCanonicalTruth",
            verified_type: "PrimitiveConstructionCompoundParityReport",
            failure_type: "PrimitiveConstructionCompoundParityVerificationFailure",
        },
        PrimitiveConstructionVerifiedArtifactSurfaceRow {
            subject: PrimitiveConstructionProofSubject::PolicyPressure,
            grade: PrimitiveConstructionProofGrade::BundleCoherence,
            truth_type: "PrimitiveConstructionPolicyPressureCanonicalTruth",
            verified_type: "PrimitiveConstructionPolicyPressureReportBundle",
            failure_type: "PrimitiveConstructionPolicyPressureBundleVerificationFailure",
        },
        PrimitiveConstructionVerifiedArtifactSurfaceRow {
            subject: PrimitiveConstructionProofSubject::PhaseFiveSixCloseout,
            grade: PrimitiveConstructionProofGrade::MilestoneCloseout,
            truth_type: "PrimitiveConstructionPhaseFiveSixCloseoutRegistry",
            verified_type: "PrimitiveConstructionPhaseFiveSixCloseoutReport",
            failure_type: "PrimitiveConstructionPhaseFiveSixCloseoutVerificationFailure",
        },
        PrimitiveConstructionVerifiedArtifactSurfaceRow {
            subject: PrimitiveConstructionProofSubject::MilestoneFourKernelCloseout,
            grade: PrimitiveConstructionProofGrade::MilestoneCloseout,
            truth_type: "PrimitiveConstructionMilestoneFourKernelCloseoutRegistry",
            verified_type: "PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport",
            failure_type: "PrimitiveConstructionMilestoneFourKernelCloseoutVerificationFailure",
        },
    ];
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &rows
            .iter()
            .flat_map(|row| {
                [
                    row.subject.as_str().to_string(),
                    row.grade.as_str().to_string(),
                    row.truth_type.to_string(),
                    row.verified_type.to_string(),
                    row.failure_type.to_string(),
                ]
            })
            .collect::<Vec<_>>(),
    );
    PrimitiveConstructionVerifiedArtifactSurfaceReport {
        rows,
        report_digest,
    }
}
