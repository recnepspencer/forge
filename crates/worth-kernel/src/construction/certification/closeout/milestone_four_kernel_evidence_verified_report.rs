use forge_proof::raw::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, PhaseMarker, Proof, ProofMarker,
};

use crate::construction::certification::closeout::PrimitiveConstructionPhaseFiveSixCloseoutReport;
use crate::construction::certification::continuity::PrimitiveConstructionContinuitySurfaceReport;
use crate::construction::certification::motion::PrimitiveConstructionMotionResolutionPolicyReport;
use crate::construction::certification::preview::PrimitiveConstructionPreviewSurfaceReport;
use crate::construction::certification::profile::PrimitiveConstructionPolicyProfileSurfaceReport;
use crate::construction::certification::realization::PrimitiveConstructionRealizationExhaustionWitnessReport;
use crate::construction::proof::substrate_closeout_report::PrimitiveConstructionProofSubstrateCloseoutReport;
use crate::construction::query::boundary_gap_register::PrimitiveConstructionQueryBoundaryGapRegister;

use super::milestone_four_kernel_evidence_verified_assembly::PrimitiveConstructionMilestoneFourKernelCloseoutAssembly;
use super::milestone_four_kernel_evidence_verified_registry::PrimitiveConstructionMilestoneFourKernelCloseoutRegistry;
use super::milestone_four_kernel_evidence_verified_support::closeout_digest;

type MilestoneFourProofBasis = FreshnessScopedBasis<CurrentValidity, AssumptionBasis<()>>;
type VerifiedMilestoneFourArtifact = Artifact<
    VerifiedMilestoneFourKernelCloseoutPhase,
    PrimitiveConstructionVerifiedMilestoneFourKernelCloseoutPayload,
    Proof<MilestoneFourKernelCloseoutProven, MilestoneFourKernelCloseoutProofAuthority>,
    MilestoneFourProofBasis,
>;

pub(crate) struct VerifiedMilestoneFourKernelCloseoutPhase;
impl PhaseMarker for VerifiedMilestoneFourKernelCloseoutPhase {}

pub(crate) struct MilestoneFourKernelCloseoutProven;
impl ProofMarker for MilestoneFourKernelCloseoutProven {}

pub(crate) struct MilestoneFourKernelCloseoutProofAuthority;
impl AuthorityMarker for MilestoneFourKernelCloseoutProofAuthority {}
impl AuthorityProves<MilestoneFourKernelCloseoutProven>
    for MilestoneFourKernelCloseoutProofAuthority
{
}

#[derive(Clone, Debug, PartialEq)]
struct PrimitiveConstructionVerifiedMilestoneFourKernelCloseoutPayload {
    registry: PrimitiveConstructionMilestoneFourKernelCloseoutRegistry,
    assembly: PrimitiveConstructionMilestoneFourKernelCloseoutAssembly,
    report_digest: String,
}

pub struct PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport(
    VerifiedMilestoneFourArtifact,
);

impl Clone for PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport {
    fn clone(&self) -> Self {
        Self::from_parts(
            self.0.payload().registry.clone(),
            self.0.payload().assembly.clone(),
        )
    }
}

impl std::fmt::Debug for PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport")
            .field("report_digest", &self.report_digest())
            .finish()
    }
}

impl PartialEq for PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport {
    fn eq(&self, other: &Self) -> bool {
        self.0.payload().registry == other.0.payload().registry
            && self.0.payload().assembly == other.0.payload().assembly
    }
}

impl PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport {
    pub(super) fn from_parts(
        registry: PrimitiveConstructionMilestoneFourKernelCloseoutRegistry,
        assembly: PrimitiveConstructionMilestoneFourKernelCloseoutAssembly,
    ) -> Self {
        let authority =
            AuthorityWitness::from_authority_marker(MilestoneFourKernelCloseoutProofAuthority);
        let report_digest = closeout_digest(&registry, &assembly);
        Self(Artifact::with_proofs_and_current_basis(
            PrimitiveConstructionVerifiedMilestoneFourKernelCloseoutPayload {
                registry,
                assembly,
                report_digest,
            },
            Proof::from_authority_witness(&authority),
            (),
            authority,
        ))
    }

    pub fn phase_five_six_closeout(&self) -> &PrimitiveConstructionPhaseFiveSixCloseoutReport {
        &self.0.payload().assembly.phase_five_six_closeout
    }

    pub fn query_boundary_gap_register(&self) -> &PrimitiveConstructionQueryBoundaryGapRegister {
        &self.0.payload().assembly.query_boundary_gap_register
    }

    pub fn proof_substrate_closeout(&self) -> &PrimitiveConstructionProofSubstrateCloseoutReport {
        &self.0.payload().assembly.proof_substrate_closeout
    }

    pub fn motion_policy_report(&self) -> &PrimitiveConstructionMotionResolutionPolicyReport {
        &self.0.payload().assembly.motion_policy_report
    }

    pub fn preview_surface_report(&self) -> &PrimitiveConstructionPreviewSurfaceReport {
        &self.0.payload().assembly.preview_surface_report
    }

    pub fn continuity_surface_report(&self) -> &PrimitiveConstructionContinuitySurfaceReport {
        &self.0.payload().assembly.continuity_surface_report
    }

    pub fn policy_profile_report(&self) -> &PrimitiveConstructionPolicyProfileSurfaceReport {
        &self.0.payload().assembly.policy_profile_report
    }

    pub fn realization_exhaustion_witness_report(
        &self,
    ) -> &PrimitiveConstructionRealizationExhaustionWitnessReport {
        &self
            .0
            .payload()
            .assembly
            .realization_exhaustion_witness_report
    }

    pub fn report_digest(&self) -> &str {
        &self.0.payload().report_digest
    }
}
