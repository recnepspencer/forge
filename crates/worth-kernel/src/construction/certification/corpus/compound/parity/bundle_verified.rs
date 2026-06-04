use forge_proof::raw::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, PhaseMarker, Proof, ProofMarker,
};

use super::super::builder::PrimitiveConstructionCompoundAdversarialSiegeError;
use super::super::ordering_report::PrimitiveConstructionCompoundOrderingParityReport;
use super::super::report::{
    PrimitiveConstructionCompoundAdversarialSiegeReport,
    PrimitiveConstructionCompoundExhaustionWitnessParityReport,
    PrimitiveConstructionCompoundGrazingBoundaryReport,
    PrimitiveConstructionCompoundMotionParityReport,
};
use super::truth::PrimitiveConstructionCompoundParityCanonicalTruth;
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};

type CompoundParityProofBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<()>>;
type VerifiedCompoundParityArtifact = Artifact<
    VerifiedCompoundParityPhase,
    PrimitiveConstructionVerifiedCompoundParityReportPayload,
    Proof<CompoundParityCoherenceProven, CompoundParityProofAuthority>,
    CompoundParityProofBasis,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionCompoundParityReportBundle {
    siege: PrimitiveConstructionCompoundAdversarialSiegeReport,
    ordering: PrimitiveConstructionCompoundOrderingParityReport,
    motion: PrimitiveConstructionCompoundMotionParityReport,
    grazing: PrimitiveConstructionCompoundGrazingBoundaryReport,
    exhaustion: PrimitiveConstructionCompoundExhaustionWitnessParityReport,
    report_digest: String,
}

impl PrimitiveConstructionCompoundParityReportBundle {
    pub(crate) fn new(
        siege: PrimitiveConstructionCompoundAdversarialSiegeReport,
        ordering: PrimitiveConstructionCompoundOrderingParityReport,
        motion: PrimitiveConstructionCompoundMotionParityReport,
        grazing: PrimitiveConstructionCompoundGrazingBoundaryReport,
        exhaustion: PrimitiveConstructionCompoundExhaustionWitnessParityReport,
    ) -> Self {
        let report_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ParityIdentity,
            &[
                siege.report_digest().to_string(),
                ordering.report_digest().to_string(),
                motion.report_digest().to_string(),
                grazing.report_digest().to_string(),
                exhaustion.report_digest().to_string(),
            ],
        );
        Self {
            siege,
            ordering,
            motion,
            grazing,
            exhaustion,
            report_digest,
        }
    }

    pub(crate) fn siege(&self) -> &PrimitiveConstructionCompoundAdversarialSiegeReport {
        &self.siege
    }

    pub(crate) fn ordering(&self) -> &PrimitiveConstructionCompoundOrderingParityReport {
        &self.ordering
    }

    pub(crate) fn motion(&self) -> &PrimitiveConstructionCompoundMotionParityReport {
        &self.motion
    }

    pub(crate) fn grazing(&self) -> &PrimitiveConstructionCompoundGrazingBoundaryReport {
        &self.grazing
    }

    pub(crate) fn exhaustion(&self) -> &PrimitiveConstructionCompoundExhaustionWitnessParityReport {
        &self.exhaustion
    }

    pub(crate) fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub(crate) struct VerifiedCompoundParityPhase;
impl PhaseMarker for VerifiedCompoundParityPhase {}

pub(crate) struct CompoundParityCoherenceProven;
impl ProofMarker for CompoundParityCoherenceProven {}

pub(crate) struct CompoundParityProofAuthority;
impl AuthorityMarker for CompoundParityProofAuthority {}
impl AuthorityProves<CompoundParityCoherenceProven> for CompoundParityProofAuthority {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PrimitiveConstructionVerifiedCompoundParityReportPayload {
    truth: PrimitiveConstructionCompoundParityCanonicalTruth,
    bundle: PrimitiveConstructionCompoundParityReportBundle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionCompoundParityVerificationMismatch {
    OrderingProjectionDrift,
    MotionProjectionDrift,
    GrazingProjectionDrift,
    ExhaustionProjectionDrift,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundParityVerificationFailure {
    truth: PrimitiveConstructionCompoundParityCanonicalTruth,
    siege: PrimitiveConstructionCompoundAdversarialSiegeReport,
    ordering: PrimitiveConstructionCompoundOrderingParityReport,
    motion: PrimitiveConstructionCompoundMotionParityReport,
    grazing: PrimitiveConstructionCompoundGrazingBoundaryReport,
    exhaustion: PrimitiveConstructionCompoundExhaustionWitnessParityReport,
    mismatches: Vec<PrimitiveConstructionCompoundParityVerificationMismatch>,
    report_digest: String,
}

impl PrimitiveConstructionCompoundParityVerificationFailure {
    pub fn truth(&self) -> &PrimitiveConstructionCompoundParityCanonicalTruth {
        &self.truth
    }

    pub fn siege(&self) -> &PrimitiveConstructionCompoundAdversarialSiegeReport {
        &self.siege
    }

    pub fn ordering(&self) -> &PrimitiveConstructionCompoundOrderingParityReport {
        &self.ordering
    }

    pub fn motion(&self) -> &PrimitiveConstructionCompoundMotionParityReport {
        &self.motion
    }

    pub fn grazing(&self) -> &PrimitiveConstructionCompoundGrazingBoundaryReport {
        &self.grazing
    }

    pub fn exhaustion(&self) -> &PrimitiveConstructionCompoundExhaustionWitnessParityReport {
        &self.exhaustion
    }

    pub fn mismatches(&self) -> &[PrimitiveConstructionCompoundParityVerificationMismatch] {
        &self.mismatches
    }
}

pub struct PrimitiveConstructionCompoundParityReport(VerifiedCompoundParityArtifact);

impl PrimitiveConstructionCompoundParityReport {
    pub(crate) fn from_parts(
        truth: PrimitiveConstructionCompoundParityCanonicalTruth,
        bundle: PrimitiveConstructionCompoundParityReportBundle,
    ) -> Self {
        let authority = AuthorityWitness::from_authority_marker(CompoundParityProofAuthority);
        Self(Artifact::with_proofs_and_current_basis(
            PrimitiveConstructionVerifiedCompoundParityReportPayload { truth, bundle },
            Proof::from_authority_witness(&authority),
            (),
            authority,
        ))
    }

    pub fn truth(&self) -> &PrimitiveConstructionCompoundParityCanonicalTruth {
        &self.0.payload().truth
    }

    pub fn siege(&self) -> &PrimitiveConstructionCompoundAdversarialSiegeReport {
        self.0.payload().bundle.siege()
    }

    pub fn ordering(&self) -> &PrimitiveConstructionCompoundOrderingParityReport {
        self.0.payload().bundle.ordering()
    }

    pub fn motion(&self) -> &PrimitiveConstructionCompoundMotionParityReport {
        self.0.payload().bundle.motion()
    }

    pub fn grazing(&self) -> &PrimitiveConstructionCompoundGrazingBoundaryReport {
        self.0.payload().bundle.grazing()
    }

    pub fn exhaustion(&self) -> &PrimitiveConstructionCompoundExhaustionWitnessParityReport {
        self.0.payload().bundle.exhaustion()
    }

    pub fn report_digest(&self) -> &str {
        self.0.payload().bundle.report_digest()
    }
}

impl Clone for PrimitiveConstructionCompoundParityReport {
    fn clone(&self) -> Self {
        Self::from_parts(self.truth().clone(), self.0.payload().bundle.clone())
    }
}

impl std::fmt::Debug for PrimitiveConstructionCompoundParityReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimitiveConstructionCompoundParityReport")
            .field("truth", self.truth())
            .field("report_digest", &self.0.payload().bundle.report_digest())
            .finish()
    }
}

impl PartialEq for PrimitiveConstructionCompoundParityReport {
    fn eq(&self, other: &Self) -> bool {
        self.truth() == other.truth() && self.0.payload().bundle == other.0.payload().bundle
    }
}

impl Eq for PrimitiveConstructionCompoundParityReport {}

pub(crate) fn verify_bundle(
    bundle: PrimitiveConstructionCompoundParityReportBundle,
) -> Result<
    PrimitiveConstructionCompoundParityReport,
    super::super::builder::PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let truth = PrimitiveConstructionCompoundParityCanonicalTruth::from_siege(bundle.siege());
    let mut mismatches = Vec::new();
    if !truth.ordering_matches(bundle.ordering()) {
        mismatches
            .push(PrimitiveConstructionCompoundParityVerificationMismatch::OrderingProjectionDrift);
    }
    if !truth.motion_matches(bundle.motion())? {
        mismatches
            .push(PrimitiveConstructionCompoundParityVerificationMismatch::MotionProjectionDrift);
    }
    if !truth.grazing_matches(bundle.grazing())? {
        mismatches
            .push(PrimitiveConstructionCompoundParityVerificationMismatch::GrazingProjectionDrift);
    }
    if !truth.exhaustion_matches(bundle.exhaustion())? {
        mismatches.push(
            PrimitiveConstructionCompoundParityVerificationMismatch::ExhaustionProjectionDrift,
        );
    }
    if mismatches.is_empty() {
        return Ok(PrimitiveConstructionCompoundParityReport::from_parts(
            truth, bundle,
        ));
    }
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ParityIdentity,
        &[
            truth.truth_digest().to_string(),
            bundle.siege().report_digest().to_string(),
            bundle.ordering().report_digest().to_string(),
            bundle.motion().report_digest().to_string(),
            bundle.grazing().report_digest().to_string(),
            bundle.exhaustion().report_digest().to_string(),
            format!("{mismatches:?}"),
        ],
    );
    Err(
        PrimitiveConstructionCompoundAdversarialSiegeError::Verification(
            PrimitiveConstructionCompoundParityVerificationFailure {
                truth,
                siege: bundle.siege().clone(),
                ordering: bundle.ordering().clone(),
                motion: bundle.motion().clone(),
                grazing: bundle.grazing().clone(),
                exhaustion: bundle.exhaustion().clone(),
                mismatches,
                report_digest,
            },
        ),
    )
}
