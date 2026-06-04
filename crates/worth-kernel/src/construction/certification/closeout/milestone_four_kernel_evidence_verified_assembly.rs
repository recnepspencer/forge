use crate::construction::certification::arbitration::{
    PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    PrimitiveConstructionIntentArbitrationPolicyReport,
    PrimitiveConstructionIntentArbitrationRepresentativeEvidence,
};
use crate::construction::certification::closeout::milestone_four_kernel_representative_evidence::{
    PrimitiveConstructionContinuityRepresentativeEvidence,
    PrimitiveConstructionPolicyProfileRepresentativeEvidence,
    PrimitiveConstructionPreviewRepresentativeEvidence,
};
use crate::construction::certification::closeout::PrimitiveConstructionPhaseFiveSixCloseoutReport;
use crate::construction::certification::continuity::PrimitiveConstructionContinuitySurfaceReport;
use crate::construction::certification::motion::{
    PrimitiveConstructionMotionDxSurfaceReport, PrimitiveConstructionMotionResolutionPolicyReport,
};
use crate::construction::certification::preview::PrimitiveConstructionPreviewSurfaceReport;
use crate::construction::certification::profile::PrimitiveConstructionPolicyProfileSurfaceReport;
use crate::construction::certification::realization::PrimitiveConstructionRealizationExhaustionWitnessReport;
use crate::construction::proof::substrate_closeout_report::PrimitiveConstructionProofSubstrateCloseoutReport;
use crate::construction::query::basis_preview_parity::PrimitiveConstructionQueryBasisPreviewParityReport;
use crate::construction::query::boundary_gap_register::PrimitiveConstructionQueryBoundaryGapRegister;
use crate::construction::query::existing_truth_binding::PrimitiveConstructionQueryExistingTruthBindingReport;
use crate::construction::query::graph_composition_parity::PrimitiveConstructionQueryGraphCompositionParityReport;
use crate::construction::query::no_local_runtime_workaround_audit::PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionMilestoneFourKernelCloseoutAssembly {
    pub(super) phase_five_six_closeout: PrimitiveConstructionPhaseFiveSixCloseoutReport,
    pub(super) proof_substrate_closeout: PrimitiveConstructionProofSubstrateCloseoutReport,
    pub(super) query_boundary_gap_register: PrimitiveConstructionQueryBoundaryGapRegister,
    pub(super) query_no_local_runtime_workaround_audit:
        PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit,
    pub(super) query_existing_truth_binding_report:
        PrimitiveConstructionQueryExistingTruthBindingReport,
    pub(super) query_graph_composition_parity_report:
        PrimitiveConstructionQueryGraphCompositionParityReport,
    pub(super) query_basis_preview_parity_report:
        PrimitiveConstructionQueryBasisPreviewParityReport,
    pub(super) motion_policy_report: PrimitiveConstructionMotionResolutionPolicyReport,
    pub(super) motion_dx_surface_report: PrimitiveConstructionMotionDxSurfaceReport,
    pub(super) intent_arbitration_policy_report: PrimitiveConstructionIntentArbitrationPolicyReport,
    pub(super) intent_conflict_dx_surface_report:
        PrimitiveConstructionIntentArbitrationDxSurfaceReport,
    pub(super) representative_intent_evidence:
        PrimitiveConstructionIntentArbitrationRepresentativeEvidence,
    pub(super) preview_surface_report: PrimitiveConstructionPreviewSurfaceReport,
    pub(super) representative_preview_evidence: PrimitiveConstructionPreviewRepresentativeEvidence,
    pub(super) continuity_surface_report: PrimitiveConstructionContinuitySurfaceReport,
    pub(super) representative_continuity_evidence:
        PrimitiveConstructionContinuityRepresentativeEvidence,
    pub(super) policy_profile_report: PrimitiveConstructionPolicyProfileSurfaceReport,
    pub(super) representative_policy_profile_evidence:
        PrimitiveConstructionPolicyProfileRepresentativeEvidence,
    pub(super) realization_exhaustion_witness_report:
        PrimitiveConstructionRealizationExhaustionWitnessReport,
}

impl PrimitiveConstructionMilestoneFourKernelCloseoutAssembly {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        phase_five_six_closeout: PrimitiveConstructionPhaseFiveSixCloseoutReport,
        proof_substrate_closeout: PrimitiveConstructionProofSubstrateCloseoutReport,
        query_boundary_gap_register: PrimitiveConstructionQueryBoundaryGapRegister,
        query_no_local_runtime_workaround_audit: PrimitiveConstructionQueryNoLocalRuntimeWorkaroundAudit,
        query_existing_truth_binding_report: PrimitiveConstructionQueryExistingTruthBindingReport,
        query_graph_composition_parity_report: PrimitiveConstructionQueryGraphCompositionParityReport,
        query_basis_preview_parity_report: PrimitiveConstructionQueryBasisPreviewParityReport,
        motion_policy_report: PrimitiveConstructionMotionResolutionPolicyReport,
        motion_dx_surface_report: PrimitiveConstructionMotionDxSurfaceReport,
        intent_arbitration_policy_report: PrimitiveConstructionIntentArbitrationPolicyReport,
        intent_conflict_dx_surface_report: PrimitiveConstructionIntentArbitrationDxSurfaceReport,
        representative_intent_evidence: PrimitiveConstructionIntentArbitrationRepresentativeEvidence,
        preview_surface_report: PrimitiveConstructionPreviewSurfaceReport,
        representative_preview_evidence: PrimitiveConstructionPreviewRepresentativeEvidence,
        continuity_surface_report: PrimitiveConstructionContinuitySurfaceReport,
        representative_continuity_evidence: PrimitiveConstructionContinuityRepresentativeEvidence,
        policy_profile_report: PrimitiveConstructionPolicyProfileSurfaceReport,
        representative_policy_profile_evidence: PrimitiveConstructionPolicyProfileRepresentativeEvidence,
        realization_exhaustion_witness_report: PrimitiveConstructionRealizationExhaustionWitnessReport,
    ) -> Self {
        Self {
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
        }
    }
}
