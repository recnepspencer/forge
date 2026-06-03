use forge_proof::raw::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, PhaseMarker, Proof, ProofMarker,
};

use crate::construction::certification::arbitration::truth::PrimitiveConstructionIntentArbitrationCanonicalTruth;
use crate::construction::certification::{
    PrimitiveConstructionChosenIntentResolutionRow,
    PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    PrimitiveConstructionIntentArbitrationPolicyRow,
    PrimitiveConstructionPreservedIntentResolutionRow,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::{
    PrimitiveConstructionIntentArbitrationReplayParityReport,
    PrimitiveConstructionQueryIntentArbitrationParityReport,
};

use super::PrimitiveConstructionIntentArbitrationBundleCase;

type ArbitrationBundleProofBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<()>>;
type VerifiedArbitrationBundleArtifact = Artifact<
    VerifiedArbitrationBundlePhase,
    PrimitiveConstructionVerifiedIntentArbitrationBundlePayload,
    Proof<ArbitrationBundleCoherenceProven, ArbitrationBundleProofAuthority>,
    ArbitrationBundleProofBasis,
>;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionIntentArbitrationReportBundle {
    case: PrimitiveConstructionIntentArbitrationBundleCase,
    policy_row: PrimitiveConstructionIntentArbitrationPolicyRow,
    chosen_row: Option<PrimitiveConstructionChosenIntentResolutionRow>,
    preserved_row: PrimitiveConstructionPreservedIntentResolutionRow,
    dx_surface_report: PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    replay_parity_report: PrimitiveConstructionIntentArbitrationReplayParityReport,
    query_inspection_parity_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
    query_projection_receipt_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
}

impl PrimitiveConstructionIntentArbitrationReportBundle {
    pub(crate) fn new(
        case: PrimitiveConstructionIntentArbitrationBundleCase,
        policy_row: PrimitiveConstructionIntentArbitrationPolicyRow,
        chosen_row: Option<PrimitiveConstructionChosenIntentResolutionRow>,
        preserved_row: PrimitiveConstructionPreservedIntentResolutionRow,
        dx_surface_report: PrimitiveConstructionIntentArbitrationDxSurfaceReport,
        replay_parity_report: PrimitiveConstructionIntentArbitrationReplayParityReport,
        query_inspection_parity_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
        query_projection_receipt_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
    ) -> Self {
        Self {
            case,
            policy_row,
            chosen_row,
            preserved_row,
            dx_surface_report,
            replay_parity_report,
            query_inspection_parity_report,
            query_projection_receipt_report,
        }
    }

    pub(crate) fn case(&self) -> PrimitiveConstructionIntentArbitrationBundleCase {
        self.case
    }

    pub(crate) fn policy_row(&self) -> &PrimitiveConstructionIntentArbitrationPolicyRow {
        &self.policy_row
    }

    pub(crate) fn chosen_row(&self) -> Option<&PrimitiveConstructionChosenIntentResolutionRow> {
        self.chosen_row.as_ref()
    }

    pub(crate) fn preserved_row(&self) -> &PrimitiveConstructionPreservedIntentResolutionRow {
        &self.preserved_row
    }

    pub(crate) fn dx_surface_report(
        &self,
    ) -> &PrimitiveConstructionIntentArbitrationDxSurfaceReport {
        &self.dx_surface_report
    }

    pub(crate) fn replay_parity_report(
        &self,
    ) -> &PrimitiveConstructionIntentArbitrationReplayParityReport {
        &self.replay_parity_report
    }

    pub(crate) fn query_inspection_parity_report(
        &self,
    ) -> &PrimitiveConstructionQueryIntentArbitrationParityReport {
        &self.query_inspection_parity_report
    }

    pub(crate) fn query_projection_receipt_report(
        &self,
    ) -> &PrimitiveConstructionQueryIntentArbitrationParityReport {
        &self.query_projection_receipt_report
    }
}

pub(crate) struct VerifiedArbitrationBundlePhase;
impl PhaseMarker for VerifiedArbitrationBundlePhase {}

pub(crate) struct ArbitrationBundleCoherenceProven;
impl ProofMarker for ArbitrationBundleCoherenceProven {}

pub(crate) struct ArbitrationBundleProofAuthority;
impl AuthorityMarker for ArbitrationBundleProofAuthority {}
impl AuthorityProves<ArbitrationBundleCoherenceProven> for ArbitrationBundleProofAuthority {}

#[derive(Clone, Debug, PartialEq)]
struct PrimitiveConstructionVerifiedIntentArbitrationBundlePayload {
    truth: PrimitiveConstructionIntentArbitrationCanonicalTruth,
    bundle: PrimitiveConstructionIntentArbitrationReportBundle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionIntentArbitrationBundleVerificationMismatch {
    PolicyProjectionDrift,
    PolicyResolutionSurfaceIncoherent,
    ChosenProjectionDrift,
    DxProjectionDrift,
    DxSurfaceIncoherent,
    ReplayParityProjectionDrift,
    QueryInspectionProjectionDrift,
    QueryProjectionReceiptDrift,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionIntentArbitrationBundleVerificationFailure {
    truth: PrimitiveConstructionIntentArbitrationCanonicalTruth,
    policy_row: PrimitiveConstructionIntentArbitrationPolicyRow,
    chosen_row: Option<PrimitiveConstructionChosenIntentResolutionRow>,
    preserved_row: PrimitiveConstructionPreservedIntentResolutionRow,
    dx_surface_report: PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    replay_parity_report: PrimitiveConstructionIntentArbitrationReplayParityReport,
    query_inspection_parity_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
    query_projection_receipt_report: PrimitiveConstructionQueryIntentArbitrationParityReport,
    mismatches: Vec<PrimitiveConstructionIntentArbitrationBundleVerificationMismatch>,
    report_digest: String,
}

impl PrimitiveConstructionIntentArbitrationBundleVerificationFailure {
    pub fn truth(&self) -> &PrimitiveConstructionIntentArbitrationCanonicalTruth {
        &self.truth
    }

    pub fn policy_row(&self) -> &PrimitiveConstructionIntentArbitrationPolicyRow {
        &self.policy_row
    }

    pub fn chosen_row(&self) -> Option<&PrimitiveConstructionChosenIntentResolutionRow> {
        self.chosen_row.as_ref()
    }

    pub fn preserved_row(&self) -> &PrimitiveConstructionPreservedIntentResolutionRow {
        &self.preserved_row
    }

    pub fn dx_surface_report(&self) -> &PrimitiveConstructionIntentArbitrationDxSurfaceReport {
        &self.dx_surface_report
    }

    pub fn replay_parity_report(
        &self,
    ) -> &PrimitiveConstructionIntentArbitrationReplayParityReport {
        &self.replay_parity_report
    }

    pub fn query_inspection_parity_report(
        &self,
    ) -> &PrimitiveConstructionQueryIntentArbitrationParityReport {
        &self.query_inspection_parity_report
    }

    pub fn query_projection_receipt_report(
        &self,
    ) -> &PrimitiveConstructionQueryIntentArbitrationParityReport {
        &self.query_projection_receipt_report
    }

    pub fn mismatches(
        &self,
    ) -> &[PrimitiveConstructionIntentArbitrationBundleVerificationMismatch] {
        &self.mismatches
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub struct PrimitiveConstructionVerifiedIntentArbitrationReportBundle(
    VerifiedArbitrationBundleArtifact,
);

impl Clone for PrimitiveConstructionVerifiedIntentArbitrationReportBundle {
    fn clone(&self) -> Self {
        Self::from_parts(
            self.0.payload().truth.clone(),
            self.0.payload().bundle.clone(),
        )
    }
}

impl std::fmt::Debug for PrimitiveConstructionVerifiedIntentArbitrationReportBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimitiveConstructionVerifiedIntentArbitrationReportBundle")
            .field("case", &self.case())
            .field("truth", &self.truth())
            .finish()
    }
}

impl PartialEq for PrimitiveConstructionVerifiedIntentArbitrationReportBundle {
    fn eq(&self, other: &Self) -> bool {
        self.0.payload().truth == other.0.payload().truth
            && self.0.payload().bundle == other.0.payload().bundle
    }
}

impl PrimitiveConstructionVerifiedIntentArbitrationReportBundle {
    pub(crate) fn from_parts(
        truth: PrimitiveConstructionIntentArbitrationCanonicalTruth,
        bundle: PrimitiveConstructionIntentArbitrationReportBundle,
    ) -> Self {
        let authority = AuthorityWitness::from_authority_marker(ArbitrationBundleProofAuthority);
        Self(Artifact::with_proofs_and_current_basis(
            PrimitiveConstructionVerifiedIntentArbitrationBundlePayload { truth, bundle },
            Proof::from_authority_witness(&authority),
            (),
            authority,
        ))
    }

    pub fn case(&self) -> PrimitiveConstructionIntentArbitrationBundleCase {
        self.0.payload().bundle.case()
    }

    pub fn truth(&self) -> &PrimitiveConstructionIntentArbitrationCanonicalTruth {
        &self.0.payload().truth
    }

    pub fn policy_row(&self) -> &PrimitiveConstructionIntentArbitrationPolicyRow {
        self.0.payload().bundle.policy_row()
    }

    pub fn chosen_row(&self) -> Option<&PrimitiveConstructionChosenIntentResolutionRow> {
        self.0.payload().bundle.chosen_row()
    }

    pub fn preserved_row(&self) -> &PrimitiveConstructionPreservedIntentResolutionRow {
        self.0.payload().bundle.preserved_row()
    }

    pub fn dx_surface_report(&self) -> &PrimitiveConstructionIntentArbitrationDxSurfaceReport {
        self.0.payload().bundle.dx_surface_report()
    }

    pub fn replay_parity_report(
        &self,
    ) -> &PrimitiveConstructionIntentArbitrationReplayParityReport {
        self.0.payload().bundle.replay_parity_report()
    }

    pub fn query_inspection_parity_report(
        &self,
    ) -> &PrimitiveConstructionQueryIntentArbitrationParityReport {
        self.0.payload().bundle.query_inspection_parity_report()
    }

    pub fn query_projection_receipt_report(
        &self,
    ) -> &PrimitiveConstructionQueryIntentArbitrationParityReport {
        self.0.payload().bundle.query_projection_receipt_report()
    }

    pub fn bundle_digest(&self) -> &str {
        self.0.payload().truth.truth_digest()
    }
}

pub(crate) fn verify_bundle(
    bundle: PrimitiveConstructionIntentArbitrationReportBundle,
) -> Result<
    PrimitiveConstructionVerifiedIntentArbitrationReportBundle,
    PrimitiveConstructionIntentArbitrationBundleVerificationFailure,
> {
    let truth = PrimitiveConstructionIntentArbitrationCanonicalTruth::from_preserved_row(
        bundle.preserved_row(),
    );
    let dx_row = bundle
        .dx_surface_report()
        .row(bundle.case().policy_case())
        .expect("bundle preparation must carry matching dx row");
    let mut mismatches = Vec::new();
    if !truth.policy_matches(bundle.policy_row()) {
        mismatches.push(
            PrimitiveConstructionIntentArbitrationBundleVerificationMismatch::PolicyProjectionDrift,
        );
    }
    if !truth.policy_resolution_surface_consistent(bundle.policy_row()) {
        mismatches.push(
            PrimitiveConstructionIntentArbitrationBundleVerificationMismatch::PolicyResolutionSurfaceIncoherent,
        );
    }
    if !truth.chosen_row_matches(bundle.chosen_row()) {
        mismatches.push(
            PrimitiveConstructionIntentArbitrationBundleVerificationMismatch::ChosenProjectionDrift,
        );
    }
    if !truth.dx_matches(dx_row) {
        mismatches.push(
            PrimitiveConstructionIntentArbitrationBundleVerificationMismatch::DxProjectionDrift,
        );
    }
    if !truth.dx_surface_consistent(dx_row) {
        mismatches.push(
            PrimitiveConstructionIntentArbitrationBundleVerificationMismatch::DxSurfaceIncoherent,
        );
    }
    if !truth.replay_matches(bundle.replay_parity_report()) {
        mismatches.push(
            PrimitiveConstructionIntentArbitrationBundleVerificationMismatch::ReplayParityProjectionDrift,
        );
    }
    if !truth.query_matches(bundle.query_inspection_parity_report()) {
        mismatches.push(
            PrimitiveConstructionIntentArbitrationBundleVerificationMismatch::QueryInspectionProjectionDrift,
        );
    }
    if !truth.query_matches(bundle.query_projection_receipt_report()) {
        mismatches.push(
            PrimitiveConstructionIntentArbitrationBundleVerificationMismatch::QueryProjectionReceiptDrift,
        );
    }
    if mismatches.is_empty() {
        return Ok(
            PrimitiveConstructionVerifiedIntentArbitrationReportBundle::from_parts(truth, bundle),
        );
    }
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ParityIdentity,
        &[
            truth.truth_digest().to_string(),
            bundle.policy_row().row_digest().to_string(),
            bundle
                .chosen_row()
                .map(|row| row.row_digest().to_string())
                .unwrap_or_else(|| "unresolved".to_string()),
            bundle.preserved_row().row_digest().to_string(),
            bundle.dx_surface_report().report_digest().to_string(),
            bundle.replay_parity_report().report_digest().to_string(),
            bundle
                .query_inspection_parity_report()
                .report_digest()
                .to_string(),
            bundle
                .query_projection_receipt_report()
                .report_digest()
                .to_string(),
            format!("{mismatches:?}"),
        ],
    );
    Err(
        PrimitiveConstructionIntentArbitrationBundleVerificationFailure {
            truth,
            policy_row: bundle.policy_row().clone(),
            chosen_row: bundle.chosen_row().cloned(),
            preserved_row: bundle.preserved_row().clone(),
            dx_surface_report: bundle.dx_surface_report().clone(),
            replay_parity_report: bundle.replay_parity_report().clone(),
            query_inspection_parity_report: bundle.query_inspection_parity_report().clone(),
            query_projection_receipt_report: bundle.query_projection_receipt_report().clone(),
            mismatches,
            report_digest,
        },
    )
}
