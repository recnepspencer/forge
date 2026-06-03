use forge_proof::raw::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, PhaseMarker, Proof, ProofMarker,
};

use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::proof::{
    prepare_primitive_construction_digest_protocol_report,
    prepare_primitive_construction_proof_boundary_compile_fail_report,
    prepare_primitive_construction_truth_projection_matrix,
    prepare_primitive_construction_verified_artifact_surface_report,
    PrimitiveConstructionDigestProtocolReport, PrimitiveConstructionProofBoundaryCompileFailReport,
    PrimitiveConstructionProofGrade, PrimitiveConstructionProofSubject,
    PrimitiveConstructionTruthProjectionMatrix, PrimitiveConstructionVerifiedArtifactSurfaceReport,
    PROOF_BOUNDARY_COMPILE_FAIL_FIXTURES,
};

type ProofSubstrateBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<()>>;
type VerifiedProofSubstrateArtifact = Artifact<
    VerifiedProofSubstrateCloseoutPhase,
    PrimitiveConstructionVerifiedProofSubstrateCloseoutPayload,
    Proof<ProofSubstrateCloseoutProven, ProofSubstrateCloseoutAuthority>,
    ProofSubstrateBasis,
>;

struct VerifiedProofSubstrateCloseoutPhase;
impl PhaseMarker for VerifiedProofSubstrateCloseoutPhase {}
struct ProofSubstrateCloseoutProven;
impl ProofMarker for ProofSubstrateCloseoutProven {}
struct ProofSubstrateCloseoutAuthority;
impl AuthorityMarker for ProofSubstrateCloseoutAuthority {}
impl AuthorityProves<ProofSubstrateCloseoutProven> for ProofSubstrateCloseoutAuthority {}

#[derive(Clone, Debug, PartialEq)]
struct PrimitiveConstructionVerifiedProofSubstrateCloseoutPayload {
    digest_protocol_report: PrimitiveConstructionDigestProtocolReport,
    verified_artifact_surface_report: PrimitiveConstructionVerifiedArtifactSurfaceReport,
    truth_projection_matrix: PrimitiveConstructionTruthProjectionMatrix,
    proof_boundary_compile_fail_report: PrimitiveConstructionProofBoundaryCompileFailReport,
    report_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch {
    DigestProtocolVersionDrift,
    VerifiedArtifactCoverageDrift,
    TruthProjectionCoverageDrift,
    CompileFailCoverageDrift,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionProofSubstrateCloseoutVerificationFailure {
    digest_protocol_report: PrimitiveConstructionDigestProtocolReport,
    verified_artifact_surface_report: PrimitiveConstructionVerifiedArtifactSurfaceReport,
    truth_projection_matrix: PrimitiveConstructionTruthProjectionMatrix,
    proof_boundary_compile_fail_report: PrimitiveConstructionProofBoundaryCompileFailReport,
    mismatches: Vec<PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch>,
    report_digest: String,
}

impl PrimitiveConstructionProofSubstrateCloseoutVerificationFailure {
    pub fn mismatches(&self) -> &[PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch] {
        &self.mismatches
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub struct PrimitiveConstructionProofSubstrateCloseoutReport(VerifiedProofSubstrateArtifact);

impl Clone for PrimitiveConstructionProofSubstrateCloseoutReport {
    fn clone(&self) -> Self {
        Self::from_parts(
            self.0.payload().digest_protocol_report.clone(),
            self.0.payload().verified_artifact_surface_report.clone(),
            self.0.payload().truth_projection_matrix.clone(),
            self.0.payload().proof_boundary_compile_fail_report.clone(),
        )
    }
}

impl std::fmt::Debug for PrimitiveConstructionProofSubstrateCloseoutReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimitiveConstructionProofSubstrateCloseoutReport")
            .field("report_digest", &self.report_digest())
            .finish()
    }
}

impl PartialEq for PrimitiveConstructionProofSubstrateCloseoutReport {
    fn eq(&self, other: &Self) -> bool {
        self.0.payload().digest_protocol_report == other.0.payload().digest_protocol_report
            && self.0.payload().verified_artifact_surface_report
                == other.0.payload().verified_artifact_surface_report
            && self.0.payload().truth_projection_matrix == other.0.payload().truth_projection_matrix
            && self.0.payload().proof_boundary_compile_fail_report
                == other.0.payload().proof_boundary_compile_fail_report
    }
}

impl PrimitiveConstructionProofSubstrateCloseoutReport {
    fn from_parts(
        digest_protocol_report: PrimitiveConstructionDigestProtocolReport,
        verified_artifact_surface_report: PrimitiveConstructionVerifiedArtifactSurfaceReport,
        truth_projection_matrix: PrimitiveConstructionTruthProjectionMatrix,
        proof_boundary_compile_fail_report: PrimitiveConstructionProofBoundaryCompileFailReport,
    ) -> Self {
        let authority = AuthorityWitness::from_authority_marker(ProofSubstrateCloseoutAuthority);
        let report_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ArtifactIdentity,
            &[
                digest_protocol_report.report_digest().to_string(),
                verified_artifact_surface_report.report_digest().to_string(),
                truth_projection_matrix.report_digest().to_string(),
                proof_boundary_compile_fail_report
                    .report_digest()
                    .to_string(),
            ],
        );
        Self(Artifact::with_proofs_and_current_basis(
            PrimitiveConstructionVerifiedProofSubstrateCloseoutPayload {
                digest_protocol_report,
                verified_artifact_surface_report,
                truth_projection_matrix,
                proof_boundary_compile_fail_report,
                report_digest,
            },
            Proof::from_authority_witness(&authority),
            (),
            authority,
        ))
    }

    pub fn proof_grade(&self) -> PrimitiveConstructionProofGrade {
        PrimitiveConstructionProofGrade::ProofSubstrateCloseout
    }

    pub fn proof_subject(&self) -> PrimitiveConstructionProofSubject {
        PrimitiveConstructionProofSubject::ProofSubstrateCloseout
    }

    pub fn digest_protocol_report(&self) -> &PrimitiveConstructionDigestProtocolReport {
        &self.0.payload().digest_protocol_report
    }

    pub fn verified_artifact_surface_report(
        &self,
    ) -> &PrimitiveConstructionVerifiedArtifactSurfaceReport {
        &self.0.payload().verified_artifact_surface_report
    }

    pub fn truth_projection_matrix(&self) -> &PrimitiveConstructionTruthProjectionMatrix {
        &self.0.payload().truth_projection_matrix
    }

    pub fn proof_boundary_compile_fail_report(
        &self,
    ) -> &PrimitiveConstructionProofBoundaryCompileFailReport {
        &self.0.payload().proof_boundary_compile_fail_report
    }

    pub fn report_digest(&self) -> &str {
        &self.0.payload().report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionProofSubstrateCloseoutReportError {
    Verification(PrimitiveConstructionProofSubstrateCloseoutVerificationFailure),
}

impl std::fmt::Display for PrimitiveConstructionProofSubstrateCloseoutReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Verification(failure) => write!(
                f,
                "proof substrate closeout failed verification: {:?}",
                failure.mismatches()
            ),
        }
    }
}

impl std::error::Error for PrimitiveConstructionProofSubstrateCloseoutReportError {}

pub fn prepare_primitive_construction_proof_substrate_closeout_report() -> Result<
    PrimitiveConstructionProofSubstrateCloseoutReport,
    PrimitiveConstructionProofSubstrateCloseoutReportError,
> {
    let digest_protocol_report = prepare_primitive_construction_digest_protocol_report();
    let verified_artifact_surface_report =
        prepare_primitive_construction_verified_artifact_surface_report();
    let truth_projection_matrix = prepare_primitive_construction_truth_projection_matrix();
    let proof_boundary_compile_fail_report =
        prepare_primitive_construction_proof_boundary_compile_fail_report();
    let mut mismatches = Vec::new();
    if digest_protocol_report.version_prefix() != "worth-kernel.v1" {
        mismatches.push(PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch::DigestProtocolVersionDrift);
    }
    if verified_artifact_surface_report.rows().len() != 3 {
        mismatches.push(PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch::VerifiedArtifactCoverageDrift);
    }
    if truth_projection_matrix.rows().len() != 1 {
        mismatches.push(PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch::TruthProjectionCoverageDrift);
    }
    if proof_boundary_compile_fail_report.fixtures().len()
        != PROOF_BOUNDARY_COMPILE_FAIL_FIXTURES.len()
    {
        mismatches.push(PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch::CompileFailCoverageDrift);
    }
    if mismatches.is_empty() {
        return Ok(
            PrimitiveConstructionProofSubstrateCloseoutReport::from_parts(
                digest_protocol_report,
                verified_artifact_surface_report,
                truth_projection_matrix,
                proof_boundary_compile_fail_report,
            ),
        );
    }
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &[
            digest_protocol_report.report_digest().to_string(),
            verified_artifact_surface_report.report_digest().to_string(),
            truth_projection_matrix.report_digest().to_string(),
            proof_boundary_compile_fail_report
                .report_digest()
                .to_string(),
            format!("{mismatches:?}"),
        ],
    );
    Err(
        PrimitiveConstructionProofSubstrateCloseoutReportError::Verification(
            PrimitiveConstructionProofSubstrateCloseoutVerificationFailure {
                digest_protocol_report,
                verified_artifact_surface_report,
                truth_projection_matrix,
                proof_boundary_compile_fail_report,
                mismatches,
                report_digest,
            },
        ),
    )
}
