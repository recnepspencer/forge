use forge_proof::raw::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, PhaseMarker, Proof, ProofMarker,
};

use crate::construction::certification::profile::pressure::delta::PrimitiveConstructionPolicyPressureDeltaReport;
use crate::construction::certification::profile::pressure::report::PrimitiveConstructionPolicyPressureSurfaceReport;
use crate::construction::certification::profile::pressure::truth::PrimitiveConstructionPolicyPressureCanonicalTruth;
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::proof::{
    PrimitiveConstructionProofGrade, PrimitiveConstructionProofSubject,
};

type PolicyPressureProofBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<()>>;
type VerifiedPolicyPressureArtifact = Artifact<
    VerifiedPolicyPressureBundlePhase,
    PrimitiveConstructionVerifiedPolicyPressureBundlePayload,
    Proof<PolicyPressureBundleCoherenceProven, PolicyPressureBundleProofAuthority>,
    PolicyPressureProofBasis,
>;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionPolicyPressureUnverifiedBundle {
    direct_report: PrimitiveConstructionPolicyPressureSurfaceReport,
    delta_report: PrimitiveConstructionPolicyPressureDeltaReport,
}

impl PrimitiveConstructionPolicyPressureUnverifiedBundle {
    pub(crate) fn new(
        direct_report: PrimitiveConstructionPolicyPressureSurfaceReport,
        delta_report: PrimitiveConstructionPolicyPressureDeltaReport,
    ) -> Self {
        Self {
            direct_report,
            delta_report,
        }
    }

    pub(crate) fn direct_report(&self) -> &PrimitiveConstructionPolicyPressureSurfaceReport {
        &self.direct_report
    }

    pub(crate) fn delta_report(&self) -> &PrimitiveConstructionPolicyPressureDeltaReport {
        &self.delta_report
    }
}

pub(crate) struct VerifiedPolicyPressureBundlePhase;
impl PhaseMarker for VerifiedPolicyPressureBundlePhase {}

pub(crate) struct PolicyPressureBundleCoherenceProven;
impl ProofMarker for PolicyPressureBundleCoherenceProven {}

pub(crate) struct PolicyPressureBundleProofAuthority;
impl AuthorityMarker for PolicyPressureBundleProofAuthority {}
impl AuthorityProves<PolicyPressureBundleCoherenceProven> for PolicyPressureBundleProofAuthority {}

#[derive(Clone, Debug, PartialEq)]
struct PrimitiveConstructionVerifiedPolicyPressureBundlePayload {
    truth: PrimitiveConstructionPolicyPressureCanonicalTruth,
    bundle: PrimitiveConstructionPolicyPressureUnverifiedBundle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPolicyPressureBundleVerificationMismatch {
    DirectProjectionDrift,
    DeltaProjectionDrift,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyPressureBundleVerificationFailure {
    truth: PrimitiveConstructionPolicyPressureCanonicalTruth,
    direct_report: PrimitiveConstructionPolicyPressureSurfaceReport,
    delta_report: PrimitiveConstructionPolicyPressureDeltaReport,
    mismatches: Vec<PrimitiveConstructionPolicyPressureBundleVerificationMismatch>,
    report_digest: String,
}

impl PrimitiveConstructionPolicyPressureBundleVerificationFailure {
    pub fn truth(&self) -> &PrimitiveConstructionPolicyPressureCanonicalTruth {
        &self.truth
    }

    pub fn direct_report(&self) -> &PrimitiveConstructionPolicyPressureSurfaceReport {
        &self.direct_report
    }

    pub fn delta_report(&self) -> &PrimitiveConstructionPolicyPressureDeltaReport {
        &self.delta_report
    }

    pub fn mismatches(&self) -> &[PrimitiveConstructionPolicyPressureBundleVerificationMismatch] {
        &self.mismatches
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub struct PrimitiveConstructionPolicyPressureReportBundle(VerifiedPolicyPressureArtifact);

impl Clone for PrimitiveConstructionPolicyPressureReportBundle {
    fn clone(&self) -> Self {
        Self::from_parts(
            self.0.payload().truth.clone(),
            self.0.payload().bundle.clone(),
        )
    }
}

impl std::fmt::Debug for PrimitiveConstructionPolicyPressureReportBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimitiveConstructionPolicyPressureReportBundle")
            .field("truth", &self.truth())
            .finish()
    }
}

impl PartialEq for PrimitiveConstructionPolicyPressureReportBundle {
    fn eq(&self, other: &Self) -> bool {
        self.0.payload().truth == other.0.payload().truth
            && self.0.payload().bundle == other.0.payload().bundle
    }
}

impl PrimitiveConstructionPolicyPressureReportBundle {
    pub(crate) fn from_parts(
        truth: PrimitiveConstructionPolicyPressureCanonicalTruth,
        bundle: PrimitiveConstructionPolicyPressureUnverifiedBundle,
    ) -> Self {
        let authority = AuthorityWitness::from_authority_marker(PolicyPressureBundleProofAuthority);
        Self(Artifact::with_proofs_and_current_basis(
            PrimitiveConstructionVerifiedPolicyPressureBundlePayload { truth, bundle },
            Proof::from_authority_witness(&authority),
            (),
            authority,
        ))
    }

    pub fn truth(&self) -> &PrimitiveConstructionPolicyPressureCanonicalTruth {
        &self.0.payload().truth
    }

    pub fn direct_report(&self) -> &PrimitiveConstructionPolicyPressureSurfaceReport {
        self.0.payload().bundle.direct_report()
    }

    pub fn delta_report(&self) -> &PrimitiveConstructionPolicyPressureDeltaReport {
        self.0.payload().bundle.delta_report()
    }

    pub fn required_direct_cases(
        &self,
    ) -> &'static [crate::construction::certification::profile::pressure::PrimitiveConstructionPolicyPressureCase]{
        self.truth().required_direct_cases()
    }

    pub fn required_delta_cases(
        &self,
    ) -> &'static [crate::construction::certification::profile::pressure::PrimitiveConstructionPolicyPressureDeltaCase]{
        self.truth().required_delta_cases()
    }

    pub fn report_digest(&self) -> &str {
        self.truth().truth_digest()
    }

    pub fn proof_grade(&self) -> PrimitiveConstructionProofGrade {
        PrimitiveConstructionProofGrade::BundleCoherence
    }

    pub fn proof_subject(&self) -> PrimitiveConstructionProofSubject {
        PrimitiveConstructionProofSubject::PolicyPressure
    }
}

pub(crate) fn verify_bundle(
    bundle: PrimitiveConstructionPolicyPressureUnverifiedBundle,
) -> Result<
    PrimitiveConstructionPolicyPressureReportBundle,
    PrimitiveConstructionPolicyPressureBundleVerificationFailure,
> {
    let truth = PrimitiveConstructionPolicyPressureCanonicalTruth::from_reports(
        bundle.direct_report(),
        bundle.delta_report(),
    );
    let mut mismatches = Vec::new();
    if !truth.direct_matches(bundle.direct_report()) {
        mismatches.push(
            PrimitiveConstructionPolicyPressureBundleVerificationMismatch::DirectProjectionDrift,
        );
    }
    if !truth.delta_matches(bundle.delta_report()) {
        mismatches.push(
            PrimitiveConstructionPolicyPressureBundleVerificationMismatch::DeltaProjectionDrift,
        );
    }
    if mismatches.is_empty() {
        return Ok(PrimitiveConstructionPolicyPressureReportBundle::from_parts(
            truth, bundle,
        ));
    }
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ParityIdentity,
        &[
            truth.truth_digest().to_string(),
            bundle.direct_report().report_digest().to_string(),
            bundle.delta_report().report_digest().to_string(),
            format!("{mismatches:?}"),
        ],
    );
    Err(
        PrimitiveConstructionPolicyPressureBundleVerificationFailure {
            truth,
            direct_report: bundle.direct_report().clone(),
            delta_report: bundle.delta_report().clone(),
            mismatches,
            report_digest,
        },
    )
}
