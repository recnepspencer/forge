use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::certification::closeout::milestone_four_kernel_requirements::{
    arbitration_policy_inventory_present, continuity_inventory_present,
    motion_policy_inventory_present, policy_profile_inventory_present, preview_inventory_present,
    query_boundary_closeout_verified, realization_exhaustion_inventory_present,
};
use crate::construction::certification::{
    prepare_primitive_construction_continuity_bundle_from_hostility_suite,
    prepare_primitive_construction_continuity_hostility_suite_report,
    prepare_primitive_construction_continuity_surface_report,
    prepare_primitive_construction_motion_dx_surface_report,
    prepare_primitive_construction_motion_resolution_policy_report,
    prepare_primitive_construction_phase_five_six_closeout_report,
    prepare_primitive_construction_policy_profile_report,
    prepare_primitive_construction_policy_profile_report_bundle,
    prepare_primitive_construction_preview_bundle_from_hostility_suite,
    prepare_primitive_construction_preview_hostility_suite_report,
    prepare_primitive_construction_preview_surface_report,
    prepare_primitive_construction_realization_exhaustion_witness_report,
    prepare_primitive_intent_arbitration_policy_report,
    prepare_primitive_intent_conflict_dx_surface_report, PrimitiveConstructionContinuityCase,
    PrimitiveConstructionContinuityReportBundle, PrimitiveConstructionContinuityReportBundleError,
    PrimitiveConstructionContinuitySurfaceReport,
    PrimitiveConstructionContinuitySurfaceReportError,
    PrimitiveConstructionIntentArbitrationBundleCase,
    PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    PrimitiveConstructionIntentArbitrationPolicyReport,
    PrimitiveConstructionIntentArbitrationPolicyReportError,
    PrimitiveConstructionIntentArbitrationReportBundle,
    PrimitiveConstructionIntentArbitrationReportBundleError,
    PrimitiveConstructionMotionDxSurfaceReport, PrimitiveConstructionMotionDxSurfaceReportError,
    PrimitiveConstructionMotionResolutionPolicyReport,
    PrimitiveConstructionMotionResolutionPolicyReportError,
    PrimitiveConstructionPhaseFiveSixCloseoutReport,
    PrimitiveConstructionPhaseFiveSixCloseoutReportError, PrimitiveConstructionPolicyProfileCase,
    PrimitiveConstructionPolicyProfileReportBundle,
    PrimitiveConstructionPolicyProfileReportBundleError,
    PrimitiveConstructionPolicyProfileSurfaceReport, PrimitiveConstructionPreviewCase,
    PrimitiveConstructionPreviewReportBundle, PrimitiveConstructionPreviewReportBundleError,
    PrimitiveConstructionPreviewSurfaceReport, PrimitiveConstructionPreviewSurfaceReportError,
    PrimitiveConstructionRealizationExhaustionWitnessReport,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::query::{
    prepare_primitive_construction_query_basis_preview_parity_report,
    prepare_primitive_construction_query_boundary_gap_register,
    prepare_primitive_construction_query_existing_truth_binding_report,
    prepare_primitive_construction_query_graph_composition_parity_report,
    prepare_primitive_construction_query_no_local_runtime_workaround_audit,
    PrimitiveConstructionQueryBasisPreviewParityReport,
    PrimitiveConstructionQueryBoundaryGapRegister,
    PrimitiveConstructionQueryGraphCompositionParityError,
    PrimitiveConstructionQueryGraphCompositionParityReport,
    PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit,
};
use crate::construction::{
    prepare_primitive_construction_intent_arbitration_report_bundle, OrthotopeSpec,
    PrimitiveConstructionExistingTruthBindingPosture, PrimitiveConstructionIntent,
    PrimitiveConstructionRuntimeBasisError, SimplexSolidSpec, WorthKernelAuthorityError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport {
    phase_five_six_closeout: PrimitiveConstructionPhaseFiveSixCloseoutReport,
    query_boundary_gap_register: PrimitiveConstructionQueryBoundaryGapRegister,
    query_no_local_runtime_workaround_audit:
        PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit,
    query_existing_truth_binding_report:
        crate::construction::PrimitiveConstructionQueryExistingTruthBindingReport,
    query_graph_composition_parity_report: PrimitiveConstructionQueryGraphCompositionParityReport,
    query_basis_preview_parity_report: PrimitiveConstructionQueryBasisPreviewParityReport,
    motion_policy_report: PrimitiveConstructionMotionResolutionPolicyReport,
    motion_dx_surface_report: PrimitiveConstructionMotionDxSurfaceReport,
    intent_arbitration_policy_report: PrimitiveConstructionIntentArbitrationPolicyReport,
    intent_conflict_dx_surface_report: PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    representative_intent_bundle: PrimitiveConstructionIntentArbitrationReportBundle,
    preview_surface_report: PrimitiveConstructionPreviewSurfaceReport,
    representative_preview_bundle: PrimitiveConstructionPreviewReportBundle,
    continuity_surface_report: PrimitiveConstructionContinuitySurfaceReport,
    representative_continuity_bundle: PrimitiveConstructionContinuityReportBundle,
    policy_profile_report: PrimitiveConstructionPolicyProfileSurfaceReport,
    representative_policy_profile_bundle: PrimitiveConstructionPolicyProfileReportBundle,
    realization_exhaustion_witness_report: PrimitiveConstructionRealizationExhaustionWitnessReport,
    query_closeout_verified: bool,
    spatial_intent_closeout_verified: bool,
    realization_closeout_verified: bool,
    kernel_evidence_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport {
    fn new(
        phase_five_six_closeout: PrimitiveConstructionPhaseFiveSixCloseoutReport,
        query_boundary_gap_register: PrimitiveConstructionQueryBoundaryGapRegister,
        query_no_local_runtime_workaround_audit: PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit,
        query_existing_truth_binding_report: crate::construction::PrimitiveConstructionQueryExistingTruthBindingReport,
        query_graph_composition_parity_report: PrimitiveConstructionQueryGraphCompositionParityReport,
        query_basis_preview_parity_report: PrimitiveConstructionQueryBasisPreviewParityReport,
        motion_policy_report: PrimitiveConstructionMotionResolutionPolicyReport,
        motion_dx_surface_report: PrimitiveConstructionMotionDxSurfaceReport,
        intent_arbitration_policy_report: PrimitiveConstructionIntentArbitrationPolicyReport,
        intent_conflict_dx_surface_report: PrimitiveConstructionIntentArbitrationDxSurfaceReport,
        representative_intent_bundle: PrimitiveConstructionIntentArbitrationReportBundle,
        preview_surface_report: PrimitiveConstructionPreviewSurfaceReport,
        representative_preview_bundle: PrimitiveConstructionPreviewReportBundle,
        continuity_surface_report: PrimitiveConstructionContinuitySurfaceReport,
        representative_continuity_bundle: PrimitiveConstructionContinuityReportBundle,
        policy_profile_report: PrimitiveConstructionPolicyProfileSurfaceReport,
        representative_policy_profile_bundle: PrimitiveConstructionPolicyProfileReportBundle,
        realization_exhaustion_witness_report: PrimitiveConstructionRealizationExhaustionWitnessReport,
    ) -> Self {
        let query_closeout_verified = query_boundary_closeout_verified(&query_boundary_gap_register)
            && query_no_local_runtime_workaround_audit.violation_count() == 0
            && query_existing_truth_binding_report.posture()
                == PrimitiveConstructionExistingTruthBindingPosture::NotRequiredForFreshPrimitiveBirth
            && query_existing_truth_binding_report.forbidden_pattern_count() == 0
            && query_graph_composition_parity_report.parity_verified()
            && query_basis_preview_parity_report.parity_verified();
        let spatial_intent_closeout_verified =
            motion_policy_inventory_present(&motion_policy_report)
                && motion_dx_surface_report.rows().len() == 9
                && arbitration_policy_inventory_present(&intent_arbitration_policy_report)
                && intent_conflict_dx_surface_report.rows().len() == 6
                && representative_intent_bundle.bundle_verified()
                && preview_inventory_present(&preview_surface_report)
                && representative_preview_bundle.parity_verified()
                && continuity_inventory_present(&continuity_surface_report)
                && representative_continuity_bundle.parity_verified()
                && policy_profile_inventory_present(&policy_profile_report)
                && representative_policy_profile_bundle.parity_verified();
        let realization_closeout_verified =
            realization_exhaustion_inventory_present(&realization_exhaustion_witness_report);
        let kernel_evidence_verified = phase_five_six_closeout.closeout_verified()
            && query_closeout_verified
            && spatial_intent_closeout_verified
            && realization_closeout_verified;
        let report_digest = digest_owned_parts(&[
            phase_five_six_closeout.report_digest().to_string(),
            query_boundary_gap_register.report_digest().to_string(),
            query_no_local_runtime_workaround_audit
                .report_digest()
                .to_string(),
            query_existing_truth_binding_report
                .report_digest()
                .to_string(),
            query_graph_composition_parity_report
                .report_digest()
                .to_string(),
            query_basis_preview_parity_report
                .report_digest()
                .to_string(),
            motion_policy_report.report_digest().to_string(),
            motion_dx_surface_report.report_digest().to_string(),
            intent_arbitration_policy_report.report_digest().to_string(),
            intent_conflict_dx_surface_report
                .report_digest()
                .to_string(),
            representative_intent_bundle.bundle_digest().to_string(),
            preview_surface_report.report_digest().to_string(),
            representative_preview_bundle.report_digest().to_string(),
            continuity_surface_report.report_digest().to_string(),
            representative_continuity_bundle.report_digest().to_string(),
            policy_profile_report.report_digest().to_string(),
            representative_policy_profile_bundle
                .report_digest()
                .to_string(),
            realization_exhaustion_witness_report
                .report_digest()
                .to_string(),
            query_closeout_verified.to_string(),
            spatial_intent_closeout_verified.to_string(),
            realization_closeout_verified.to_string(),
            kernel_evidence_verified.to_string(),
        ]);
        Self {
            phase_five_six_closeout,
            query_boundary_gap_register,
            query_no_local_runtime_workaround_audit,
            query_existing_truth_binding_report,
            query_graph_composition_parity_report,
            query_basis_preview_parity_report,
            motion_policy_report,
            motion_dx_surface_report,
            intent_arbitration_policy_report,
            intent_conflict_dx_surface_report,
            representative_intent_bundle,
            preview_surface_report,
            representative_preview_bundle,
            continuity_surface_report,
            representative_continuity_bundle,
            policy_profile_report,
            representative_policy_profile_bundle,
            realization_exhaustion_witness_report,
            query_closeout_verified,
            spatial_intent_closeout_verified,
            realization_closeout_verified,
            kernel_evidence_verified,
            report_digest,
        }
    }

    pub fn phase_five_six_closeout(&self) -> &PrimitiveConstructionPhaseFiveSixCloseoutReport {
        &self.phase_five_six_closeout
    }

    pub fn query_closeout_verified(&self) -> bool {
        self.query_closeout_verified
    }

    pub fn spatial_intent_closeout_verified(&self) -> bool {
        self.spatial_intent_closeout_verified
    }

    pub fn realization_closeout_verified(&self) -> bool {
        self.realization_closeout_verified
    }

    pub fn kernel_evidence_verified(&self) -> bool {
        self.kernel_evidence_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError {
    PhaseFiveSix(PrimitiveConstructionPhaseFiveSixCloseoutReportError),
    QueryBoundaryGapRegister(WorthKernelAuthorityError),
    QueryGraphComposition(PrimitiveConstructionQueryGraphCompositionParityError),
    QueryBasisPreview(PrimitiveConstructionRuntimeBasisError),
    MotionPolicy(PrimitiveConstructionMotionResolutionPolicyReportError),
    MotionDx(PrimitiveConstructionMotionDxSurfaceReportError),
    IntentPolicy(PrimitiveConstructionIntentArbitrationPolicyReportError),
    IntentDx(PrimitiveConstructionIntentArbitrationPolicyReportError),
    IntentBundle(PrimitiveConstructionIntentArbitrationReportBundleError),
    PreviewSurface(PrimitiveConstructionPreviewSurfaceReportError),
    PreviewBundle(PrimitiveConstructionPreviewReportBundleError),
    ContinuitySurface(PrimitiveConstructionContinuitySurfaceReportError),
    ContinuityBundle(PrimitiveConstructionContinuityReportBundleError),
    PolicyProfileBundle(PrimitiveConstructionPolicyProfileReportBundleError),
}

impl std::fmt::Display for PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PhaseFiveSix(error) => write!(f, "{error}"),
            Self::QueryBoundaryGapRegister(error) => write!(f, "{error:?}"),
            Self::QueryGraphComposition(error) => write!(f, "{error}"),
            Self::QueryBasisPreview(error) => write!(f, "{error}"),
            Self::MotionPolicy(error) => write!(f, "{error}"),
            Self::MotionDx(error) => write!(f, "{error}"),
            Self::IntentPolicy(error) => write!(f, "{error}"),
            Self::IntentDx(error) => write!(f, "{error}"),
            Self::IntentBundle(error) => write!(f, "{error}"),
            Self::PreviewSurface(error) => write!(f, "{error}"),
            Self::PreviewBundle(error) => write!(f, "{error}"),
            Self::ContinuitySurface(error) => write!(f, "{error}"),
            Self::ContinuityBundle(error) => write!(f, "{error}"),
            Self::PolicyProfileBundle(error) => write!(f, "{error}"),
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
    let representative_intent_bundle =
        prepare_primitive_construction_intent_arbitration_report_bundle(
            workspace,
            PrimitiveConstructionIntentArbitrationBundleCase::GrazingSnapExplicitChoice,
        )
        .map_err(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::IntentBundle,
        )?;
    let preview_surface_report = prepare_primitive_construction_preview_surface_report().map_err(
        PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::PreviewSurface,
    )?;
    let preview_suite = prepare_primitive_construction_preview_hostility_suite_report()
        .expect("preview hostility suite should be available during kernel evidence closeout");
    let representative_preview_bundle =
        prepare_primitive_construction_preview_bundle_from_hostility_suite(
            &preview_suite,
            workspace,
            PrimitiveConstructionPreviewCase::OverlapHighFidelity,
        )
        .map_err(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::PreviewBundle,
        )?;
    let continuity_surface_report = prepare_primitive_construction_continuity_surface_report()
        .map_err(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::ContinuitySurface,
        )?;
    let continuity_suite = prepare_primitive_construction_continuity_hostility_suite_report()
        .expect("continuity hostility suite should be available during kernel evidence closeout");
    let representative_continuity_bundle =
        prepare_primitive_construction_continuity_bundle_from_hostility_suite(
            &continuity_suite,
            workspace,
            PrimitiveConstructionContinuityCase::ExplicitMergeIdentityMerged,
        )
        .map_err(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::ContinuityBundle,
        )?;
    let policy_profile_report = prepare_primitive_construction_policy_profile_report();
    let representative_policy_profile_bundle =
        prepare_primitive_construction_policy_profile_report_bundle(
            workspace,
            PrimitiveConstructionPolicyProfileCase::HighFidelityPreview,
        )
        .map_err(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReportError::PolicyProfileBundle,
        )?;
    let realization_exhaustion_witness_report =
        prepare_primitive_construction_realization_exhaustion_witness_report();
    Ok(
        PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport::new(
            phase_five_six_closeout,
            query_boundary_gap_register,
            query_no_local_runtime_workaround_audit,
            query_existing_truth_binding_report,
            query_graph_composition_parity_report,
            query_basis_preview_parity_report,
            motion_policy_report,
            motion_dx_surface_report,
            intent_arbitration_policy_report,
            intent_conflict_dx_surface_report,
            representative_intent_bundle,
            preview_surface_report,
            representative_preview_bundle,
            continuity_surface_report,
            representative_continuity_bundle,
            policy_profile_report,
            representative_policy_profile_bundle,
            realization_exhaustion_witness_report,
        ),
    )
}

fn representative_query_intent() -> PrimitiveConstructionIntent {
    PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
        half_extents: [1.0, 2.0, 3.0],
    })
}
