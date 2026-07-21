use crate::runtime::host_observation::reload_failure::evidence_digest::{
    activation_gate_denial_digest, activation_staging_denial_digest,
    invalid_candidate_denial_digest, plan_lowering_denial_digest,
    query_binding_drift_denial_digest, query_live_rebind_denial_digest,
    reconciliation_denial_digest,
};
use crate::runtime::WorthUiRuntime;
use crate::runtime::{
    WorthUiActivationGateDenial, WorthUiActivationStagingDenial,
    WorthUiDurableStateReconciliationDenial, WorthUiFailedActivationReport,
    WorthUiPlanLoweringDenial, WorthUiQueryBindingDriftDenial, WorthUiQueryLiveRebindPlanDenial,
    WorthUiReloadDenial, WorthUiReloadFailure, WorthUiReloadFailureCounters,
    WorthUiReloadFailureStage, WorthUiReloadPreservationReceipt, WorthUiReplacementCandidateDenial,
};

impl WorthUiRuntime {
    pub fn preserve_failed_reload(&self, denial: WorthUiReloadDenial) -> WorthUiReloadFailure {
        let counters = WorthUiReloadFailureCounters::preserved_without_runtime_mutation();
        let preservation_receipt = WorthUiReloadPreservationReceipt::from_active_and_last_valid(
            self.inspect_active(),
            self.last_valid(),
        );
        let failed_activation_report = WorthUiFailedActivationReport::new(
            denial.stage(),
            denial.checked_stop_posture(),
            preservation_receipt,
            counters,
        );
        WorthUiReloadFailure::new(
            denial,
            preservation_receipt,
            failed_activation_report,
            counters,
        )
    }

    pub fn preserve_invalid_candidate_reload(
        &self,
        denial: WorthUiReplacementCandidateDenial,
    ) -> WorthUiReloadFailure {
        self.preserve_failed_reload(WorthUiReloadDenial::ordinary(
            WorthUiReloadFailureStage::InvalidCandidate,
            Some(invalid_candidate_denial_digest(denial)),
        ))
    }

    pub fn preserve_failed_reconciliation(
        &self,
        denial: &WorthUiDurableStateReconciliationDenial,
    ) -> WorthUiReloadFailure {
        self.preserve_failed_reload(WorthUiReloadDenial::ordinary(
            WorthUiReloadFailureStage::DurableStateReconciliation,
            Some(reconciliation_denial_digest(denial)),
        ))
    }

    pub fn preserve_failed_activation_staging(
        &self,
        denial: &WorthUiActivationStagingDenial,
    ) -> WorthUiReloadFailure {
        self.preserve_failed_reload(WorthUiReloadDenial::ordinary(
            WorthUiReloadFailureStage::ActivationStaging,
            Some(activation_staging_denial_digest(denial)),
        ))
    }

    pub fn preserve_failed_plan_lowering(
        &self,
        denial: &WorthUiPlanLoweringDenial,
    ) -> WorthUiReloadFailure {
        self.preserve_failed_reload(WorthUiReloadDenial::ordinary(
            WorthUiReloadFailureStage::PlanLowering,
            Some(plan_lowering_denial_digest(denial)),
        ))
    }

    pub fn preserve_failed_activation_gate(
        &self,
        denial: &WorthUiActivationGateDenial,
    ) -> WorthUiReloadFailure {
        self.preserve_failed_reload(WorthUiReloadDenial::ordinary(
            WorthUiReloadFailureStage::ActivationGate,
            Some(activation_gate_denial_digest(denial)),
        ))
    }

    pub fn preserve_failed_query_live_rebind(
        &self,
        denial: &WorthUiQueryLiveRebindPlanDenial,
    ) -> WorthUiReloadFailure {
        self.preserve_failed_reload(WorthUiReloadDenial::query_checked_stop(
            WorthUiReloadFailureStage::QueryLiveRebind,
            Some(query_live_rebind_denial_digest(denial)),
        ))
    }

    pub fn preserve_query_recovery_checked_stop(
        &self,
        denial: &WorthUiQueryBindingDriftDenial,
    ) -> WorthUiReloadFailure {
        self.preserve_failed_reload(WorthUiReloadDenial::query_recovery_preserved(
            WorthUiReloadFailureStage::QueryLiveRebind,
            Some(query_binding_drift_denial_digest(denial)),
        ))
    }
}
