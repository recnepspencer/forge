use crate::runtime::diagnostics::mapping::activation::{
    diagnostic_for_activation_gate, diagnostic_for_activation_staging,
    diagnostic_for_reconciliation,
};
use crate::runtime::diagnostics::mapping::lane::diagnostic_for_lane_admission;
use crate::runtime::diagnostics::mapping::plan::{
    diagnostic_for_plan_inspection, diagnostic_for_plan_lowering,
};
use crate::runtime::diagnostics::mapping::query::{
    diagnostic_for_query_live_rebind, diagnostic_for_query_recovery,
};
use crate::runtime::diagnostics::mapping::reload::diagnostic_for_reload_failure;
use crate::runtime::diagnostics::mapping::replacement::{
    diagnostic_for_artifact_equivalence, diagnostic_for_candidate_admission,
    diagnostic_for_identity_matching, diagnostic_for_impact_narrowing,
    diagnostic_for_invalid_candidate, diagnostic_for_replacement_impact,
};
use crate::runtime::diagnostics::mapping::swap::diagnostic_for_plan_swap;
use crate::runtime::host::WorthUiRuntimeHost;
use crate::runtime::{
    WorthUiActivationGateDenial, WorthUiActivationStagingDenial, WorthUiCandidateAdmissionDenial,
    WorthUiDiagnosticRichnessPolicy, WorthUiDiagnosticSource,
    WorthUiDurableStateReconciliationDenial, WorthUiIdentityMatchDenial,
    WorthUiLaneAdmissionDenial, WorthUiPlanInspectionDenial, WorthUiPlanLoweringDenial,
    WorthUiPlanSwapRollback, WorthUiQueryBindingDriftDenial, WorthUiQueryLiveRebindPlanDenial,
    WorthUiReloadFailure, WorthUiReplacementCandidateDenial, WorthUiReplacementImpactDenial,
    WorthUiRuntimeArtifactComparisonDenial, WorthUiRuntimeDiagnostic, WorthUiRuntimeDiagnosticCode,
    WorthUiRuntimeDiagnosticFamily, WorthUiRuntimeDiagnosticReport,
    WorthUiRuntimeImpactNarrowingDenial,
};

pub struct WorthUiRuntimeDiagnostics<'a> {
    runtime: &'a WorthUiRuntimeHost,
}

pub struct WorthUiRuntimeDiagnosticRequest<'a> {
    runtime: &'a WorthUiRuntimeHost,
    rows: Vec<WorthUiRuntimeDiagnostic>,
    policy: WorthUiDiagnosticRichnessPolicy,
}

impl WorthUiRuntimeHost {
    pub fn diagnostics(&self) -> WorthUiRuntimeDiagnostics<'_> {
        WorthUiRuntimeDiagnostics { runtime: self }
    }
}

impl<'a> WorthUiRuntimeDiagnostics<'a> {
    pub fn for_reload_failure(
        self,
        failure: &WorthUiReloadFailure,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_reload_failure(failure)])
    }

    pub fn for_invalid_candidate(
        self,
        denial: WorthUiReplacementCandidateDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_invalid_candidate(denial)])
    }

    pub fn for_candidate_admission(
        self,
        denial: &WorthUiCandidateAdmissionDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_candidate_admission(denial)])
    }

    pub fn for_artifact_equivalence(
        self,
        denial: &WorthUiRuntimeArtifactComparisonDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_artifact_equivalence(denial)])
    }

    pub fn for_replacement_impact(
        self,
        denial: &WorthUiReplacementImpactDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_replacement_impact(denial)])
    }

    pub fn for_impact_narrowing(
        self,
        denial: &WorthUiRuntimeImpactNarrowingDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_impact_narrowing(denial)])
    }

    pub fn for_identity_matching(
        self,
        denial: &WorthUiIdentityMatchDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_identity_matching(denial)])
    }

    pub fn for_reconciliation(
        self,
        denial: &WorthUiDurableStateReconciliationDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_reconciliation(denial)])
    }

    pub fn for_query_live_rebind(
        self,
        denial: &WorthUiQueryLiveRebindPlanDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_query_live_rebind(denial)])
    }

    pub fn for_query_recovery(
        self,
        denial: &WorthUiQueryBindingDriftDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_query_recovery(denial)])
    }

    pub fn for_plan_lowering(
        self,
        denial: &WorthUiPlanLoweringDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_plan_lowering(denial)])
    }

    pub fn for_lane_admission(
        self,
        denial: &WorthUiLaneAdmissionDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_lane_admission(denial)])
    }

    pub fn for_activation_staging(
        self,
        denial: &WorthUiActivationStagingDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_activation_staging(denial)])
    }

    pub fn for_activation_gate(
        self,
        denial: &WorthUiActivationGateDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_activation_gate(denial)])
    }

    pub fn for_plan_swap(
        self,
        rollback: WorthUiPlanSwapRollback,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_plan_swap(rollback)])
    }

    pub fn for_plan_inspection(
        self,
        denial: &WorthUiPlanInspectionDenial,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![diagnostic_for_plan_inspection(denial)])
    }

    pub fn for_projection_hook(
        self,
        hook: &crate::runtime::WorthUiDiagnosticProjectionHook,
    ) -> WorthUiRuntimeDiagnosticRequest<'a> {
        self.request(vec![WorthUiRuntimeDiagnostic::new(
            WorthUiRuntimeDiagnosticFamily::DiagnosticsProjection,
            WorthUiRuntimeDiagnosticCode::DiagnosticsProjectionAdmitted,
            WorthUiDiagnosticSource::ProjectionHook {
                hook_digest: hook.projection_digest(),
            },
            Some(hook.projection_digest()),
        )])
    }

    fn request(self, rows: Vec<WorthUiRuntimeDiagnostic>) -> WorthUiRuntimeDiagnosticRequest<'a> {
        WorthUiRuntimeDiagnosticRequest {
            runtime: self.runtime,
            rows,
            policy: WorthUiDiagnosticRichnessPolicy::standard(),
        }
    }
}

impl WorthUiRuntimeDiagnosticRequest<'_> {
    pub fn with_policy(mut self, policy: WorthUiDiagnosticRichnessPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn materialize(self) -> WorthUiRuntimeDiagnosticReport {
        let active = self.runtime.inspect_active();
        WorthUiRuntimeDiagnosticReport::materialize(
            active.artifact_digest(),
            active.active_plan_digest(),
            self.rows,
            self.policy,
        )
    }
}
