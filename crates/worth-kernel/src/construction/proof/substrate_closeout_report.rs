use forge_proof::raw::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, PhaseMarker, Proof, ProofMarker,
};

use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::proof::canonical_witness_parity_report::{
    prepare_primitive_canonical_witness_parity_report, PrimitiveCanonicalWitnessParityReport,
};
use crate::construction::proof::compile_fail_report::{
    prepare_primitive_construction_proof_boundary_compile_fail_report,
    PrimitiveConstructionProofBoundaryCompileFailReport, PROOF_BOUNDARY_COMPILE_FAIL_FIXTURES,
};
use crate::construction::proof::digest_protocol_report::{
    prepare_primitive_construction_digest_protocol_report,
    PrimitiveConstructionDigestProtocolReport,
};
use crate::construction::proof::geometry_digest_sensitivity_report::{
    prepare_primitive_geometry_digest_sensitivity_report, PrimitiveGeometryDigestSensitivityReport,
};
use crate::construction::proof::proof_grade::{
    PrimitiveConstructionProofGrade, PrimitiveConstructionProofSubject,
};
use crate::construction::proof::shell_with_hole_layout_hostility_suite::{
    prepare_shell_with_hole_layout_hostility_suite, ShellWithHoleLayoutHostilitySuite,
};
use crate::construction::proof::simplex_canonical_ratio_report::{
    prepare_simplex_canonical_ratio_report, SimplexCanonicalRatioReport,
};
use crate::construction::proof::truth_projection_matrix::{
    prepare_primitive_construction_truth_projection_matrix,
    PrimitiveConstructionTruthProjectionMatrix,
};
use crate::construction::proof::verified_artifact_surface_report::{
    prepare_primitive_construction_verified_artifact_surface_report,
    PrimitiveConstructionVerifiedArtifactSurfaceReport,
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
    geometry_digest_sensitivity_report: PrimitiveGeometryDigestSensitivityReport,
    canonical_witness_parity_report: PrimitiveCanonicalWitnessParityReport,
    shell_with_hole_layout_hostility_suite: ShellWithHoleLayoutHostilitySuite,
    simplex_canonical_ratio_report: SimplexCanonicalRatioReport,
    verified_artifact_surface_report: PrimitiveConstructionVerifiedArtifactSurfaceReport,
    truth_projection_matrix: PrimitiveConstructionTruthProjectionMatrix,
    proof_boundary_compile_fail_report: PrimitiveConstructionProofBoundaryCompileFailReport,
    report_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch {
    DigestProtocolVersionDrift,
    GeometryDigestSensitivityDrift,
    CanonicalWitnessParityDrift,
    ShellWithHoleLayoutHostilityDrift,
    SimplexCanonicalRatioDrift,
    VerifiedArtifactCoverageDrift,
    TruthProjectionCoverageDrift,
    CompileFailCoverageDrift,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionProofSubstrateCloseoutVerificationFailure {
    mismatches: Vec<PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch>,
}

impl PrimitiveConstructionProofSubstrateCloseoutVerificationFailure {
    pub fn mismatches(&self) -> &[PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch] {
        &self.mismatches
    }
}

pub struct PrimitiveConstructionProofSubstrateCloseoutReport(VerifiedProofSubstrateArtifact);

impl Clone for PrimitiveConstructionProofSubstrateCloseoutReport {
    fn clone(&self) -> Self {
        Self::from_payload(self.0.payload().clone())
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
        self.report_digest() == other.report_digest()
    }
}

impl PrimitiveConstructionProofSubstrateCloseoutReport {
    fn from_payload(payload: PrimitiveConstructionVerifiedProofSubstrateCloseoutPayload) -> Self {
        let authority = AuthorityWitness::from_authority_marker(ProofSubstrateCloseoutAuthority);
        Self(Artifact::with_proofs_and_current_basis(
            payload,
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
    let geometry_digest_sensitivity_report = prepare_primitive_geometry_digest_sensitivity_report();
    let canonical_witness_parity_report = prepare_primitive_canonical_witness_parity_report();
    let shell_with_hole_layout_hostility_suite = prepare_shell_with_hole_layout_hostility_suite();
    let simplex_canonical_ratio_report = prepare_simplex_canonical_ratio_report();
    let verified_artifact_surface_report =
        prepare_primitive_construction_verified_artifact_surface_report();
    let truth_projection_matrix = prepare_primitive_construction_truth_projection_matrix();
    let proof_boundary_compile_fail_report =
        prepare_primitive_construction_proof_boundary_compile_fail_report();

    let mut mismatches = Vec::new();
    if digest_protocol_report.version_prefix() != "worth-primitives-digest:v1" {
        mismatches.push(PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch::DigestProtocolVersionDrift);
    }
    if geometry_digest_sensitivity_report.rows().len() != 3 {
        mismatches.push(PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch::GeometryDigestSensitivityDrift);
    }
    if canonical_witness_parity_report.rows().len() != 6 {
        mismatches.push(PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch::CanonicalWitnessParityDrift);
    }
    if !shell_with_hole_layout_hostility_suite
        .containment()
        .containment_verified()
        || !shell_with_hole_layout_hostility_suite
            .non_overlap()
            .non_overlap_verified()
        || !shell_with_hole_layout_hostility_suite.rejected_missing_hole_loop()
    {
        mismatches.push(PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch::ShellWithHoleLayoutHostilityDrift);
    }
    if (simplex_canonical_ratio_report.definition().lateral_ratio()
        - worth_primitives::CANONICAL_SIMPLEX_LATERAL_RATIO)
        .abs()
        > f64::EPSILON
    {
        mismatches.push(PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch::SimplexCanonicalRatioDrift);
    }
    if verified_artifact_surface_report.rows().len() != 7 {
        mismatches.push(PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch::VerifiedArtifactCoverageDrift);
    }
    if truth_projection_matrix.rows().len() != 5 {
        mismatches.push(PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch::TruthProjectionCoverageDrift);
    }
    if proof_boundary_compile_fail_report.fixtures().len()
        != PROOF_BOUNDARY_COMPILE_FAIL_FIXTURES.len()
    {
        mismatches.push(PrimitiveConstructionProofSubstrateCloseoutVerificationMismatch::CompileFailCoverageDrift);
    }

    if !mismatches.is_empty() {
        return Err(
            PrimitiveConstructionProofSubstrateCloseoutReportError::Verification(
                PrimitiveConstructionProofSubstrateCloseoutVerificationFailure { mismatches },
            ),
        );
    }

    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &[
            digest_protocol_report.report_digest().to_string(),
            geometry_digest_sensitivity_report
                .report_digest()
                .to_string(),
            canonical_witness_parity_report.report_digest().to_string(),
            shell_with_hole_layout_hostility_suite
                .report_digest()
                .to_string(),
            simplex_canonical_ratio_report.report_digest().to_string(),
            verified_artifact_surface_report.report_digest().to_string(),
            truth_projection_matrix.report_digest().to_string(),
            proof_boundary_compile_fail_report
                .report_digest()
                .to_string(),
        ],
    );
    Ok(
        PrimitiveConstructionProofSubstrateCloseoutReport::from_payload(
            PrimitiveConstructionVerifiedProofSubstrateCloseoutPayload {
                digest_protocol_report,
                geometry_digest_sensitivity_report,
                canonical_witness_parity_report,
                shell_with_hole_layout_hostility_suite,
                simplex_canonical_ratio_report,
                verified_artifact_surface_report,
                truth_projection_matrix,
                proof_boundary_compile_fail_report,
                report_digest,
            },
        ),
    )
}
