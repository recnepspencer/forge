use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::authoring::WorthKernelAuthorityError;
use crate::construction::certification::arbitration::{
    prepare_primitive_construction_intent_arbitration_representative_evidence,
    prepare_primitive_intent_arbitration_policy_report,
    prepare_primitive_intent_conflict_dx_surface_report,
    PrimitiveConstructionIntentArbitrationBundleCase,
    PrimitiveConstructionIntentArbitrationPolicyReportError,
    PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError,
};
use crate::construction::certification::closeout::milestone_four_kernel_evidence_verified::{
    verify_closeout, PrimitiveConstructionMilestoneFourKernelCloseoutAssembly,
    PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport,
    PrimitiveConstructionMilestoneFourKernelCloseoutVerificationFailure,
};
use crate::construction::certification::closeout::milestone_four_kernel_representative_evidence::{
    prepare_continuity_representative_evidence, prepare_policy_profile_representative_evidence,
    prepare_preview_representative_evidence,
};
use crate::construction::certification::closeout::{
    prepare_primitive_construction_phase_five_six_closeout_report,
    PrimitiveConstructionPhaseFiveSixCloseoutReportError,
};
use crate::construction::certification::continuity::{
    prepare_primitive_construction_continuity_hostility_suite_report,
    prepare_primitive_construction_continuity_surface_report, PrimitiveConstructionContinuityCase,
    PrimitiveConstructionContinuitySurfaceReportError,
};
use crate::construction::certification::motion::{
    prepare_primitive_construction_motion_dx_surface_report,
    prepare_primitive_construction_motion_resolution_policy_report,
    PrimitiveConstructionMotionDxSurfaceReportError,
    PrimitiveConstructionMotionResolutionPolicyReportError,
};
use crate::construction::certification::preview::{
    prepare_primitive_construction_preview_hostility_suite_report,
    prepare_primitive_construction_preview_surface_report, PrimitiveConstructionPreviewCase,
    PrimitiveConstructionPreviewSurfaceReportError,
};
use crate::construction::certification::profile::{
    prepare_primitive_construction_policy_profile_report,
    prepare_primitive_construction_preview_continuity_hostility_suite_report,
    PrimitiveConstructionPolicyProfileCase,
    PrimitiveConstructionPreviewContinuityHostilitySuiteError,
};
use crate::construction::certification::realization::prepare_primitive_construction_realization_exhaustion_witness_report;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::proof::substrate_closeout_report::{
    prepare_primitive_construction_proof_substrate_closeout_report,
    PrimitiveConstructionProofSubstrateCloseoutReportError,
};
use crate::construction::query::basis_preview_parity::prepare_primitive_construction_query_basis_preview_parity_report;
use crate::construction::query::boundary_gap_register::prepare_primitive_construction_query_boundary_gap_register;
use crate::construction::query::existing_truth_binding::prepare_primitive_construction_query_existing_truth_binding_report;
use crate::construction::query::graph_composition_parity::{
    prepare_primitive_construction_query_graph_composition_parity_report,
    PrimitiveConstructionQueryGraphCompositionParityError,
};
use crate::construction::query::no_local_runtime_workaround_audit::prepare_primitive_construction_query_no_local_runtime_workaround_audit;
use crate::construction::runtime_basis::PrimitiveConstructionRuntimeBasisError;
use crate::construction::specs::{OrthotopeSpec, SimplexSolidSpec};

#[derive(Debug)]
pub enum PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError {
    PhaseFiveSix(PrimitiveConstructionPhaseFiveSixCloseoutReportError),
    ProofSubstrateCloseout(PrimitiveConstructionProofSubstrateCloseoutReportError),
    QueryBoundaryGapRegister(WorthKernelAuthorityError),
    QueryGraphComposition(PrimitiveConstructionQueryGraphCompositionParityError),
    QueryBasisPreview(PrimitiveConstructionRuntimeBasisError),
    MotionPolicy(PrimitiveConstructionMotionResolutionPolicyReportError),
    MotionDx(PrimitiveConstructionMotionDxSurfaceReportError),
    IntentPolicy(PrimitiveConstructionIntentArbitrationPolicyReportError),
    IntentDx(PrimitiveConstructionIntentArbitrationPolicyReportError),
    IntentRepresentative(PrimitiveConstructionIntentArbitrationRepresentativeEvidenceError),
    PreviewSurface(PrimitiveConstructionPreviewSurfaceReportError),
    PreviewRepresentative(String),
    ContinuitySurface(PrimitiveConstructionContinuitySurfaceReportError),
    ContinuityRepresentative(String),
    PreviewContinuitySuite(PrimitiveConstructionPreviewContinuityHostilitySuiteError),
    PolicyProfileRepresentative(String),
    Verification(PrimitiveConstructionMilestoneFourKernelCloseoutVerificationFailure),
}

impl std::fmt::Display for PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PhaseFiveSix(error) => write!(f, "{error}"),
            Self::ProofSubstrateCloseout(error) => write!(f, "{error}"),
            Self::QueryBoundaryGapRegister(error) => write!(f, "{error:?}"),
            Self::QueryGraphComposition(error) => write!(f, "{error}"),
            Self::QueryBasisPreview(error) => write!(f, "{error}"),
            Self::MotionPolicy(error) => write!(f, "{error}"),
            Self::MotionDx(error) => write!(f, "{error}"),
            Self::IntentPolicy(error) => write!(f, "{error}"),
            Self::IntentDx(error) => write!(f, "{error}"),
            Self::IntentRepresentative(error) => write!(f, "{error}"),
            Self::PreviewSurface(error) => write!(f, "{error}"),
            Self::PreviewRepresentative(error) => write!(f, "{error}"),
            Self::ContinuitySurface(error) => write!(f, "{error}"),
            Self::ContinuityRepresentative(error) => write!(f, "{error}"),
            Self::PreviewContinuitySuite(error) => write!(f, "{error}"),
            Self::PolicyProfileRepresentative(error) => write!(f, "{error}"),
            Self::Verification(failure) => {
                write!(
                    f,
                    "milestone four kernel closeout failed verification: {:?}",
                    failure.mismatches()
                )
            }
        }
    }
}

impl std::error::Error for PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError {}

pub fn prepare_primitive_construction_milestone_four_kernel_closeout_evidence_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport,
    PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError,
> {
    let phase_five_six_closeout = prepare_primitive_construction_phase_five_six_closeout_report(
        workspace,
    )
    .map_err(PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::PhaseFiveSix)?;
    let proof_substrate_closeout = prepare_primitive_construction_proof_substrate_closeout_report(
    )
    .map_err(
        PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::ProofSubstrateCloseout,
    )?;
    let query_boundary_gap_register =
        prepare_primitive_construction_query_boundary_gap_register(workspace).map_err(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::QueryBoundaryGapRegister,
        )?;
    let query_no_local_runtime_workaround_audit =
        prepare_primitive_construction_query_no_local_runtime_workaround_audit();
    let query_existing_truth_binding_report =
        prepare_primitive_construction_query_existing_truth_binding_report(
            PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec::new(1.0)),
        );
    let query_graph_composition_parity_report =
        prepare_primitive_construction_query_graph_composition_parity_report(
            workspace,
            representative_query_intent(),
        )
        .map_err(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::QueryGraphComposition,
        )?;
    let query_basis_preview_parity_report =
        prepare_primitive_construction_query_basis_preview_parity_report(
            workspace,
            representative_query_intent(),
        )
        .map_err(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::QueryBasisPreview,
        )?;
    let motion_policy_report = prepare_primitive_construction_motion_resolution_policy_report(
        workspace,
    )
    .map_err(PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::MotionPolicy)?;
    let motion_dx_surface_report = prepare_primitive_construction_motion_dx_surface_report(
        workspace,
    )
    .map_err(PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::MotionDx)?;
    let intent_arbitration_policy_report = prepare_primitive_intent_arbitration_policy_report()
        .map_err(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::IntentPolicy,
        )?;
    let intent_conflict_dx_surface_report = prepare_primitive_intent_conflict_dx_surface_report()
        .map_err(
        PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::IntentDx,
    )?;
    let representative_intent_evidence =
        prepare_primitive_construction_intent_arbitration_representative_evidence(
            workspace,
            PrimitiveConstructionIntentArbitrationBundleCase::GrazingSnapExplicitChoice,
        )
        .map_err(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::IntentRepresentative,
        )?;
    let preview_surface_report = prepare_primitive_construction_preview_surface_report().map_err(
        PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::PreviewSurface,
    )?;
    let preview_suite = prepare_primitive_construction_preview_hostility_suite_report()
        .expect("preview hostility suite should be available during kernel evidence closeout");
    let representative_preview_evidence = prepare_preview_representative_evidence(
        &preview_suite,
        workspace,
        PrimitiveConstructionPreviewCase::OverlapHighFidelity,
    )
    .map_err(|error| {
        PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::PreviewRepresentative(
            error.to_string(),
        )
    })?;
    let continuity_surface_report = prepare_primitive_construction_continuity_surface_report()
        .map_err(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::ContinuitySurface,
        )?;
    let continuity_suite = prepare_primitive_construction_continuity_hostility_suite_report()
        .expect("continuity hostility suite should be available during kernel evidence closeout");
    let representative_continuity_evidence = prepare_continuity_representative_evidence(
            &continuity_suite,
            workspace,
            PrimitiveConstructionContinuityCase::ExplicitMergeIdentityMerged,
        )
        .map_err(|error| {
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::ContinuityRepresentative(
                error.to_string(),
            )
        })?;
    let policy_profile_report = prepare_primitive_construction_policy_profile_report();
    let preview_continuity_suite =
        prepare_primitive_construction_preview_continuity_hostility_suite_report().map_err(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::PreviewContinuitySuite,
        )?;
    let representative_policy_profile_row = policy_profile_report
        .row(PrimitiveConstructionPolicyProfileCase::HighFidelityPreview)
        .expect("policy profile representative row should exist")
        .clone();
    let representative_policy_profile_evidence = prepare_policy_profile_representative_evidence(
            &preview_continuity_suite,
            workspace,
            PrimitiveConstructionPolicyProfileCase::HighFidelityPreview,
            representative_policy_profile_row,
        )
        .map_err(|error| {
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::PolicyProfileRepresentative(
                error.to_string(),
            )
        })?;
    let realization_exhaustion_witness_report =
        prepare_primitive_construction_realization_exhaustion_witness_report();
    verify_closeout(
        PrimitiveConstructionMilestoneFourKernelCloseoutAssembly::new(
            phase_five_six_closeout,
            proof_substrate_closeout,
            query_boundary_gap_register,
            query_no_local_runtime_workaround_audit,
            query_existing_truth_binding_report,
            query_graph_composition_parity_report,
            query_basis_preview_parity_report,
            motion_policy_report,
            motion_dx_surface_report,
            intent_arbitration_policy_report,
            intent_conflict_dx_surface_report,
            representative_intent_evidence,
            preview_surface_report,
            representative_preview_evidence,
            continuity_surface_report,
            representative_continuity_evidence,
            policy_profile_report,
            representative_policy_profile_evidence,
            realization_exhaustion_witness_report,
        ),
    )
    .map_err(PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::Verification)
}

fn representative_query_intent() -> PrimitiveConstructionIntent {
    PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
        half_extents: [1.0, 2.0, 3.0],
    })
}
