use super::{
    UiObligationClosedSemanticLane, UiObligationCloseoutGuarantee, UiObligationCloseoutNonGoal,
    UiObligationCloseoutReport,
};

pub const MILESTONE34_CLOSEOUT_PROFILE: UiObligationCloseoutReport =
    UiObligationCloseoutReport::new(
        &[
            UiObligationClosedSemanticLane::TouchAuthority,
            UiObligationClosedSemanticLane::SupportAuthority,
            UiObligationClosedSemanticLane::FamilyCatalog,
            UiObligationClosedSemanticLane::SelectionAuthority,
            UiObligationClosedSemanticLane::DispatchPlanning,
            UiObligationClosedSemanticLane::VerdictAuthority,
            UiObligationClosedSemanticLane::QueryBoundary,
            UiObligationClosedSemanticLane::HostBoundary,
            UiObligationClosedSemanticLane::EvidenceRetention,
            UiObligationClosedSemanticLane::BudgetEnforcement,
            UiObligationClosedSemanticLane::AdmissionAggregation,
        ],
        &[
            UiObligationCloseoutGuarantee::CallerForgeryDiesAtCompileAndFacadeBoundary,
            UiObligationCloseoutGuarantee::LaterRuntimeSlicesConsumeSealedAuthorityHandoffs,
            UiObligationCloseoutGuarantee::QueryAndHostTruthRemainOwnerBound,
            UiObligationCloseoutGuarantee::EquivalentTouchesConvergeUnderStableBasis,
        ],
        &[
            UiObligationCloseoutNonGoal::MeasurementExecution,
            UiObligationCloseoutNonGoal::QueryExecution,
            UiObligationCloseoutNonGoal::IntentExecution,
            UiObligationCloseoutNonGoal::ServiceExecution,
            UiObligationCloseoutNonGoal::RebindExecution,
            UiObligationCloseoutNonGoal::RendererLocalLegality,
        ],
    );
