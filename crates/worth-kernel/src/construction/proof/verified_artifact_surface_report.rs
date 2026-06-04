use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::proof::proof_grade::{
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
            subject: PrimitiveConstructionProofSubject::CompoundParity,
            grade: PrimitiveConstructionProofGrade::BundleCoherence,
            truth_type: "PrimitiveConstructionCompoundParityCanonicalTruth",
            verified_type: "PrimitiveConstructionCompoundParityReport",
            failure_type: "PrimitiveConstructionCompoundParityVerificationFailure",
        },
        PrimitiveConstructionVerifiedArtifactSurfaceRow {
            subject: PrimitiveConstructionProofSubject::GeometryDigestSensitivity,
            grade: PrimitiveConstructionProofGrade::GeometryTruthHostility,
            truth_type: "PrimitiveGeometryIdentityBundle",
            verified_type: "PrimitiveGeometryDigestSensitivityReport",
            failure_type: "PrimitiveGeometryDigestMutationCase",
        },
        PrimitiveConstructionVerifiedArtifactSurfaceRow {
            subject: PrimitiveConstructionProofSubject::CanonicalWitnessParity,
            grade: PrimitiveConstructionProofGrade::GeometryTruthHostility,
            truth_type: "PrimitiveCanonicalWitnessGeometry",
            verified_type: "PrimitiveCanonicalWitnessParityReport",
            failure_type: "PrimitiveCanonicalWitnessParityMismatch",
        },
        PrimitiveConstructionVerifiedArtifactSurfaceRow {
            subject: PrimitiveConstructionProofSubject::ShellWithHoleLayoutHostility,
            grade: PrimitiveConstructionProofGrade::GeometryTruthHostility,
            truth_type: "ShellWithHoleWitnessLayout",
            verified_type: "ShellWithHoleLayoutHostilitySuite",
            failure_type: "PlanarWitnessContainmentReport/PlanarWitnessNonOverlapReport",
        },
        PrimitiveConstructionVerifiedArtifactSurfaceRow {
            subject: PrimitiveConstructionProofSubject::SimplexCanonicalRatio,
            grade: PrimitiveConstructionProofGrade::GeometryTruthHostility,
            truth_type: "SimplexCanonicalWitnessDefinition",
            verified_type: "SimplexCanonicalRatioReport",
            failure_type: "SimplexCanonicalWitnessDefinition",
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
