use forge_proof::raw::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, PhaseMarker, Proof, ProofMarker,
};

use crate::construction::certification::motion::truth::PrimitiveConstructionMotionCanonicalTruth;
use crate::construction::certification::motion::witness_report::PrimitiveConstructionMotionWitnessResolutionReport;
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::proof::{
    PrimitiveConstructionProofGrade, PrimitiveConstructionProofSubject,
};
use crate::construction::{
    PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    PrimitiveConstructionMotionReplayParityReport,
    PrimitiveConstructionQueryMotionWitnessParityReport,
};

type MotionBundleProofBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<()>>;
type VerifiedMotionBundleArtifact = Artifact<
    VerifiedMotionBundlePhase,
    PrimitiveConstructionVerifiedMotionReportBundlePayload,
    Proof<MotionBundleCoherenceProven, MotionBundleProofAuthority>,
    MotionBundleProofBasis,
>;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionMotionReportBundle {
    witness_report: PrimitiveConstructionMotionWitnessResolutionReport,
    replay_parity_report: PrimitiveConstructionMotionReplayParityReport,
    query_inspection_parity_report: PrimitiveConstructionQueryMotionWitnessParityReport,
    query_projection_receipt_report: PrimitiveConstructionQueryMotionWitnessParityReport,
    branch_preview_runtime_report: PrimitiveConstructionMotionBranchPreviewRuntimeReport,
}

impl PrimitiveConstructionMotionReportBundle {
    pub(crate) fn new(
        witness_report: PrimitiveConstructionMotionWitnessResolutionReport,
        replay_parity_report: PrimitiveConstructionMotionReplayParityReport,
        query_inspection_parity_report: PrimitiveConstructionQueryMotionWitnessParityReport,
        query_projection_receipt_report: PrimitiveConstructionQueryMotionWitnessParityReport,
        branch_preview_runtime_report: PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    ) -> Self {
        Self {
            witness_report,
            replay_parity_report,
            query_inspection_parity_report,
            query_projection_receipt_report,
            branch_preview_runtime_report,
        }
    }

    pub(crate) fn witness_report(&self) -> &PrimitiveConstructionMotionWitnessResolutionReport {
        &self.witness_report
    }

    pub(crate) fn replay_parity_report(&self) -> &PrimitiveConstructionMotionReplayParityReport {
        &self.replay_parity_report
    }

    pub(crate) fn query_inspection_parity_report(
        &self,
    ) -> &PrimitiveConstructionQueryMotionWitnessParityReport {
        &self.query_inspection_parity_report
    }

    pub(crate) fn query_projection_receipt_report(
        &self,
    ) -> &PrimitiveConstructionQueryMotionWitnessParityReport {
        &self.query_projection_receipt_report
    }

    pub(crate) fn branch_preview_runtime_report(
        &self,
    ) -> &PrimitiveConstructionMotionBranchPreviewRuntimeReport {
        &self.branch_preview_runtime_report
    }
}

pub(crate) struct VerifiedMotionBundlePhase;
impl PhaseMarker for VerifiedMotionBundlePhase {}

pub(crate) struct MotionBundleCoherenceProven;
impl ProofMarker for MotionBundleCoherenceProven {}

pub(crate) struct MotionBundleProofAuthority;
impl AuthorityMarker for MotionBundleProofAuthority {}
impl AuthorityProves<MotionBundleCoherenceProven> for MotionBundleProofAuthority {}

#[derive(Clone, Debug, PartialEq)]
struct PrimitiveConstructionVerifiedMotionReportBundlePayload {
    truth: PrimitiveConstructionMotionCanonicalTruth,
    bundle: PrimitiveConstructionMotionReportBundle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionMotionBundleVerificationMismatch {
    ReplayParityProjectionDrift,
    QueryInspectionProjectionDrift,
    QueryProjectionReceiptDrift,
    BranchRuntimeProjectionDrift,
    BranchRuntimeSurfaceIncoherent,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMotionBundleVerificationFailure {
    truth: PrimitiveConstructionMotionCanonicalTruth,
    witness_report: PrimitiveConstructionMotionWitnessResolutionReport,
    replay_parity_report: PrimitiveConstructionMotionReplayParityReport,
    query_inspection_parity_report: PrimitiveConstructionQueryMotionWitnessParityReport,
    query_projection_receipt_report: PrimitiveConstructionQueryMotionWitnessParityReport,
    branch_preview_runtime_report: PrimitiveConstructionMotionBranchPreviewRuntimeReport,
    mismatches: Vec<PrimitiveConstructionMotionBundleVerificationMismatch>,
    report_digest: String,
}

impl PrimitiveConstructionMotionBundleVerificationFailure {
    pub fn truth(&self) -> &PrimitiveConstructionMotionCanonicalTruth {
        &self.truth
    }

    pub fn witness_report(&self) -> &PrimitiveConstructionMotionWitnessResolutionReport {
        &self.witness_report
    }

    pub fn replay_parity_report(&self) -> &PrimitiveConstructionMotionReplayParityReport {
        &self.replay_parity_report
    }

    pub fn query_inspection_parity_report(
        &self,
    ) -> &PrimitiveConstructionQueryMotionWitnessParityReport {
        &self.query_inspection_parity_report
    }

    pub fn query_projection_receipt_report(
        &self,
    ) -> &PrimitiveConstructionQueryMotionWitnessParityReport {
        &self.query_projection_receipt_report
    }

    pub fn branch_preview_runtime_report(
        &self,
    ) -> &PrimitiveConstructionMotionBranchPreviewRuntimeReport {
        &self.branch_preview_runtime_report
    }

    pub fn mismatches(&self) -> &[PrimitiveConstructionMotionBundleVerificationMismatch] {
        &self.mismatches
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub struct PrimitiveConstructionVerifiedMotionReportBundle(VerifiedMotionBundleArtifact);

impl PrimitiveConstructionVerifiedMotionReportBundle {
    pub(crate) fn from_parts(
        truth: PrimitiveConstructionMotionCanonicalTruth,
        bundle: PrimitiveConstructionMotionReportBundle,
    ) -> Self {
        let authority = AuthorityWitness::from_authority_marker(MotionBundleProofAuthority);
        Self(Artifact::with_proofs_and_current_basis(
            PrimitiveConstructionVerifiedMotionReportBundlePayload { truth, bundle },
            Proof::from_authority_witness(&authority),
            (),
            authority,
        ))
    }

    pub fn truth(&self) -> &PrimitiveConstructionMotionCanonicalTruth {
        &self.0.payload().truth
    }

    pub fn witness_report(&self) -> &PrimitiveConstructionMotionWitnessResolutionReport {
        self.0.payload().bundle.witness_report()
    }

    pub fn replay_parity_report(&self) -> &PrimitiveConstructionMotionReplayParityReport {
        self.0.payload().bundle.replay_parity_report()
    }

    pub fn query_inspection_parity_report(
        &self,
    ) -> &PrimitiveConstructionQueryMotionWitnessParityReport {
        self.0.payload().bundle.query_inspection_parity_report()
    }

    pub fn query_projection_receipt_report(
        &self,
    ) -> &PrimitiveConstructionQueryMotionWitnessParityReport {
        self.0.payload().bundle.query_projection_receipt_report()
    }

    pub fn branch_preview_runtime_report(
        &self,
    ) -> &PrimitiveConstructionMotionBranchPreviewRuntimeReport {
        self.0.payload().bundle.branch_preview_runtime_report()
    }

    pub fn proof_grade(&self) -> PrimitiveConstructionProofGrade {
        PrimitiveConstructionProofGrade::BundleCoherence
    }

    pub fn proof_subject(&self) -> PrimitiveConstructionProofSubject {
        PrimitiveConstructionProofSubject::Motion
    }
}

pub(crate) fn verify_bundle(
    bundle: PrimitiveConstructionMotionReportBundle,
) -> Result<
    PrimitiveConstructionVerifiedMotionReportBundle,
    PrimitiveConstructionMotionBundleVerificationFailure,
> {
    let truth =
        PrimitiveConstructionMotionCanonicalTruth::from_witness_report(bundle.witness_report());
    let mut mismatches = Vec::new();
    if !truth.replay_matches(bundle.replay_parity_report()) {
        mismatches.push(
            PrimitiveConstructionMotionBundleVerificationMismatch::ReplayParityProjectionDrift,
        );
    }
    if !truth.query_matches(bundle.query_inspection_parity_report()) {
        mismatches.push(
            PrimitiveConstructionMotionBundleVerificationMismatch::QueryInspectionProjectionDrift,
        );
    }
    if !truth.query_matches(bundle.query_projection_receipt_report()) {
        mismatches.push(
            PrimitiveConstructionMotionBundleVerificationMismatch::QueryProjectionReceiptDrift,
        );
    }
    if !truth.branch_matches(bundle.branch_preview_runtime_report()) {
        mismatches.push(
            PrimitiveConstructionMotionBundleVerificationMismatch::BranchRuntimeProjectionDrift,
        );
    }
    if !truth.runtime_surface_consistent(bundle.branch_preview_runtime_report()) {
        mismatches.push(
            PrimitiveConstructionMotionBundleVerificationMismatch::BranchRuntimeSurfaceIncoherent,
        );
    }
    if mismatches.is_empty() {
        return Ok(PrimitiveConstructionVerifiedMotionReportBundle::from_parts(
            truth, bundle,
        ));
    }
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ParityIdentity,
        &[
            truth.truth_digest().to_string(),
            bundle.witness_report().report_digest().to_string(),
            bundle.replay_parity_report().report_digest().to_string(),
            bundle
                .query_inspection_parity_report()
                .report_digest()
                .to_string(),
            bundle
                .query_projection_receipt_report()
                .report_digest()
                .to_string(),
            bundle
                .branch_preview_runtime_report()
                .report_digest()
                .to_string(),
            format!("{mismatches:?}"),
        ],
    );
    Err(PrimitiveConstructionMotionBundleVerificationFailure {
        truth,
        witness_report: bundle.witness_report().clone(),
        replay_parity_report: bundle.replay_parity_report().clone(),
        query_inspection_parity_report: bundle.query_inspection_parity_report().clone(),
        query_projection_receipt_report: bundle.query_projection_receipt_report().clone(),
        branch_preview_runtime_report: bundle.branch_preview_runtime_report().clone(),
        mismatches,
        report_digest,
    })
}
