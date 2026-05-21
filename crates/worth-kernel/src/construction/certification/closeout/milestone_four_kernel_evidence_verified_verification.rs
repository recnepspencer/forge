use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;

use crate::construction::certification::{
    PrimitiveConstructionContinuityCase, PrimitiveConstructionContinuityReportBundle,
    PrimitiveConstructionContinuitySurfaceReport,
    PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    PrimitiveConstructionIntentArbitrationPolicyCase,
    PrimitiveConstructionIntentArbitrationPolicyReport, PrimitiveConstructionMotionDxSurfaceReport,
    PrimitiveConstructionMotionResolutionPolicyCase,
    PrimitiveConstructionMotionResolutionPolicyReport,
    PrimitiveConstructionPhaseFiveSixCloseoutReport, PrimitiveConstructionPolicyProfileCase,
    PrimitiveConstructionPolicyProfileReportBundle,
    PrimitiveConstructionPolicyProfileSurfaceReport, PrimitiveConstructionPreviewCase,
    PrimitiveConstructionPreviewReportBundle, PrimitiveConstructionPreviewSurfaceReport,
    PrimitiveConstructionRealizationExhaustionWitnessReport,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};
use crate::construction::proof::PrimitiveConstructionProofGrade;
use crate::construction::query::{
    PrimitiveConstructionQueryBasisPreviewParityReport,
    PrimitiveConstructionQueryBoundaryGapRegister,
    PrimitiveConstructionQueryGraphCompositionParityReport,
    PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit,
};
use crate::construction::{
    PrimitiveConstructionExistingTruthBindingPosture,
    PrimitiveConstructionProofSubstrateCloseoutReport,
    PrimitiveConstructionQueryExistingTruthBindingReport,
    PrimitiveConstructionVerifiedIntentArbitrationReportBundle,
};

use super::milestone_four_kernel_evidence_verified_assembly::PrimitiveConstructionMilestoneFourKernelCloseoutAssembly;
use super::milestone_four_kernel_evidence_verified_registry::PrimitiveConstructionMilestoneFourKernelCloseoutRegistry;
use super::milestone_four_kernel_evidence_verified_report::PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport;
use super::milestone_four_kernel_evidence_verified_support::boundary_gap_mismatches;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch {
    ProofSubstrateCloseoutDrift,
    QueryBoundaryGapRegisterDrift,
    QueryWorkaroundAuditDrift,
    ExistingTruthBindingDrift,
    QueryGraphCompositionDrift,
    QueryBasisPreviewDrift,
    MotionPolicyInventoryDrift,
    MotionDxInventoryDrift,
    IntentArbitrationPolicyInventoryDrift,
    IntentConflictDxInventoryDrift,
    PreviewInventoryDrift,
    ContinuityInventoryDrift,
    PolicyProfileInventoryDrift,
    RealizationExhaustionInventoryDrift,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionMilestoneFourKernelCloseoutVerificationFailure {
    phase_five_six_closeout: PrimitiveConstructionPhaseFiveSixCloseoutReport,
    proof_substrate_closeout: PrimitiveConstructionProofSubstrateCloseoutReport,
    query_boundary_gap_register: PrimitiveConstructionQueryBoundaryGapRegister,
    query_no_local_runtime_workaround_audit:
        PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit,
    query_existing_truth_binding_report: PrimitiveConstructionQueryExistingTruthBindingReport,
    query_graph_composition_parity_report: PrimitiveConstructionQueryGraphCompositionParityReport,
    query_basis_preview_parity_report: PrimitiveConstructionQueryBasisPreviewParityReport,
    motion_policy_report: PrimitiveConstructionMotionResolutionPolicyReport,
    motion_dx_surface_report: PrimitiveConstructionMotionDxSurfaceReport,
    intent_arbitration_policy_report: PrimitiveConstructionIntentArbitrationPolicyReport,
    intent_conflict_dx_surface_report: PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    representative_intent_bundle: PrimitiveConstructionVerifiedIntentArbitrationReportBundle,
    preview_surface_report: PrimitiveConstructionPreviewSurfaceReport,
    representative_preview_bundle: PrimitiveConstructionPreviewReportBundle,
    continuity_surface_report: PrimitiveConstructionContinuitySurfaceReport,
    representative_continuity_bundle: PrimitiveConstructionContinuityReportBundle,
    policy_profile_report: PrimitiveConstructionPolicyProfileSurfaceReport,
    representative_policy_profile_bundle: PrimitiveConstructionPolicyProfileReportBundle,
    realization_exhaustion_witness_report: PrimitiveConstructionRealizationExhaustionWitnessReport,
    missing_motion_cases: Vec<PrimitiveConstructionMotionResolutionPolicyCase>,
    missing_arbitration_cases: Vec<PrimitiveConstructionIntentArbitrationPolicyCase>,
    missing_preview_cases: Vec<PrimitiveConstructionPreviewCase>,
    missing_continuity_cases: Vec<PrimitiveConstructionContinuityCase>,
    missing_policy_profile_cases: Vec<PrimitiveConstructionPolicyProfileCase>,
    missing_realization_witness_kinds: Vec<PrimitiveRealizationExhaustionWitnessKind>,
    mismatches: Vec<PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch>,
    report_digest: String,
}

impl PrimitiveConstructionMilestoneFourKernelCloseoutVerificationFailure {
    pub fn mismatches(
        &self,
    ) -> &[PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch] {
        &self.mismatches
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub(crate) fn verify_closeout(
    assembly: PrimitiveConstructionMilestoneFourKernelCloseoutAssembly,
) -> Result<
    PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport,
    PrimitiveConstructionMilestoneFourKernelCloseoutVerificationFailure,
> {
    let registry = PrimitiveConstructionMilestoneFourKernelCloseoutRegistry::new();
    let missing_motion_cases = registry
        .required_motion_cases
        .iter()
        .copied()
        .filter(|case| assembly.motion_policy_report.row(*case).is_none())
        .collect::<Vec<_>>();
    let missing_arbitration_cases = registry
        .required_arbitration_cases
        .iter()
        .copied()
        .filter(|case| {
            assembly
                .intent_arbitration_policy_report
                .row(*case)
                .is_none()
        })
        .collect::<Vec<_>>();
    let missing_preview_cases = registry
        .required_preview_cases
        .iter()
        .copied()
        .filter(|case| assembly.preview_surface_report.row(*case).is_none())
        .collect::<Vec<_>>();
    let missing_continuity_cases = registry
        .required_continuity_cases
        .iter()
        .copied()
        .filter(|case| assembly.continuity_surface_report.row(*case).is_none())
        .collect::<Vec<_>>();
    let missing_policy_profile_cases = registry
        .required_policy_profile_cases
        .iter()
        .copied()
        .filter(|case| assembly.policy_profile_report.row(*case).is_none())
        .collect::<Vec<_>>();
    let missing_realization_witness_kinds = registry
        .required_realization_witness_kinds
        .iter()
        .copied()
        .filter(|kind| {
            assembly
                .realization_exhaustion_witness_report
                .row_for(*kind)
                .is_none()
        })
        .collect::<Vec<_>>();

    let mut mismatches = boundary_gap_mismatches(&assembly);
    if assembly.proof_substrate_closeout.proof_grade()
        != PrimitiveConstructionProofGrade::ProofSubstrateCloseout
        || assembly.proof_substrate_closeout.proof_subject()
            != registry.required_proof_substrate_subject
    {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::ProofSubstrateCloseoutDrift,
        );
    }
    if assembly
        .query_no_local_runtime_workaround_audit
        .violation_count()
        != 0
    {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::QueryWorkaroundAuditDrift,
        );
    }
    if assembly.query_existing_truth_binding_report.posture()
        != PrimitiveConstructionExistingTruthBindingPosture::NotRequiredForFreshPrimitiveBirth
        || assembly
            .query_existing_truth_binding_report
            .forbidden_pattern_count()
            != 0
    {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::ExistingTruthBindingDrift,
        );
    }
    if !assembly
        .query_graph_composition_parity_report
        .parity_verified()
    {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::QueryGraphCompositionDrift,
        );
    }
    if !assembly.query_basis_preview_parity_report.parity_verified() {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::QueryBasisPreviewDrift,
        );
    }
    if !missing_motion_cases.is_empty() {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::MotionPolicyInventoryDrift,
        );
    }
    if assembly.motion_dx_surface_report.rows().len() != registry.required_motion_cases.len() {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::MotionDxInventoryDrift,
        );
    }
    if !missing_arbitration_cases.is_empty()
        || assembly.representative_intent_bundle.case() != registry.required_intent_bundle_case
    {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::IntentArbitrationPolicyInventoryDrift,
        );
    }
    if assembly.intent_conflict_dx_surface_report.rows().len()
        != registry.required_arbitration_cases.len()
    {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::IntentConflictDxInventoryDrift,
        );
    }
    if !missing_preview_cases.is_empty()
        || assembly.representative_preview_bundle.preview_row().case()
            != registry.required_preview_bundle_case
    {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::PreviewInventoryDrift,
        );
    }
    if !missing_continuity_cases.is_empty()
        || assembly
            .representative_continuity_bundle
            .continuity_row()
            .case()
            != registry.required_continuity_bundle_case
    {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::ContinuityInventoryDrift,
        );
    }
    if !missing_policy_profile_cases.is_empty()
        || assembly
            .representative_policy_profile_bundle
            .profile_row()
            .case()
            != registry.required_policy_profile_bundle_case
    {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::PolicyProfileInventoryDrift,
        );
    }
    if !missing_realization_witness_kinds.is_empty() {
        mismatches.push(
            PrimitiveConstructionMilestoneFourKernelCloseoutVerificationMismatch::RealizationExhaustionInventoryDrift,
        );
    }

    if mismatches.is_empty() {
        return Ok(
            PrimitiveConstructionMilestoneFourKernelCloseoutEvidenceReport::from_parts(
                registry, assembly,
            ),
        );
    }

    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &[
            registry.registry_digest.clone(),
            assembly.phase_five_six_closeout.report_digest().to_string(),
            assembly
                .proof_substrate_closeout
                .report_digest()
                .to_string(),
            assembly
                .query_boundary_gap_register
                .report_digest()
                .to_string(),
            assembly
                .query_no_local_runtime_workaround_audit
                .report_digest()
                .to_string(),
            assembly
                .query_existing_truth_binding_report
                .report_digest()
                .to_string(),
            assembly
                .query_graph_composition_parity_report
                .report_digest()
                .to_string(),
            assembly
                .query_basis_preview_parity_report
                .report_digest()
                .to_string(),
            assembly.motion_policy_report.report_digest().to_string(),
            assembly
                .motion_dx_surface_report
                .report_digest()
                .to_string(),
            assembly
                .intent_arbitration_policy_report
                .report_digest()
                .to_string(),
            assembly
                .intent_conflict_dx_surface_report
                .report_digest()
                .to_string(),
            assembly
                .representative_intent_bundle
                .bundle_digest()
                .to_string(),
            assembly.preview_surface_report.report_digest().to_string(),
            assembly
                .representative_preview_bundle
                .report_digest()
                .to_string(),
            assembly
                .continuity_surface_report
                .report_digest()
                .to_string(),
            assembly
                .representative_continuity_bundle
                .report_digest()
                .to_string(),
            assembly.policy_profile_report.report_digest().to_string(),
            assembly
                .representative_policy_profile_bundle
                .report_digest()
                .to_string(),
            assembly
                .realization_exhaustion_witness_report
                .report_digest()
                .to_string(),
            format!("{missing_motion_cases:?}"),
            format!("{missing_arbitration_cases:?}"),
            format!("{missing_preview_cases:?}"),
            format!("{missing_continuity_cases:?}"),
            format!("{missing_policy_profile_cases:?}"),
            format!("{missing_realization_witness_kinds:?}"),
            format!("{mismatches:?}"),
        ],
    );
    Err(
        PrimitiveConstructionMilestoneFourKernelCloseoutVerificationFailure {
            phase_five_six_closeout: assembly.phase_five_six_closeout,
            proof_substrate_closeout: assembly.proof_substrate_closeout,
            query_boundary_gap_register: assembly.query_boundary_gap_register,
            query_no_local_runtime_workaround_audit: assembly
                .query_no_local_runtime_workaround_audit,
            query_existing_truth_binding_report: assembly.query_existing_truth_binding_report,
            query_graph_composition_parity_report: assembly.query_graph_composition_parity_report,
            query_basis_preview_parity_report: assembly.query_basis_preview_parity_report,
            motion_policy_report: assembly.motion_policy_report,
            motion_dx_surface_report: assembly.motion_dx_surface_report,
            intent_arbitration_policy_report: assembly.intent_arbitration_policy_report,
            intent_conflict_dx_surface_report: assembly.intent_conflict_dx_surface_report,
            representative_intent_bundle: assembly.representative_intent_bundle,
            preview_surface_report: assembly.preview_surface_report,
            representative_preview_bundle: assembly.representative_preview_bundle,
            continuity_surface_report: assembly.continuity_surface_report,
            representative_continuity_bundle: assembly.representative_continuity_bundle,
            policy_profile_report: assembly.policy_profile_report,
            representative_policy_profile_bundle: assembly.representative_policy_profile_bundle,
            realization_exhaustion_witness_report: assembly.realization_exhaustion_witness_report,
            missing_motion_cases,
            missing_arbitration_cases,
            missing_preview_cases,
            missing_continuity_cases,
            missing_policy_profile_cases,
            missing_realization_witness_kinds,
            mismatches,
            report_digest,
        },
    )
}
