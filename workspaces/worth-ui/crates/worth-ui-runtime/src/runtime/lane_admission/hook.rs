use crate::runtime::WorthUiExecutionLane;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthUiLaneAdapterHookKind {
    SourceIngress,
    DebouncePolicy,
    IdentitySeedContribution,
    DurableStateFamilyAdmission,
    ComponentLowering,
    LaneAdapterMechanics,
    DiagnosticsProjection,
    CounterFamilies,
    ReportMaterialization,
    ActivePlanTruthOverride,
    QueryPostureOverride,
    StateCarryForwardOverride,
    LaneTaxonomyOverride,
    PerformanceCertificationOverride,
    PrivateLaneClaim,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneAdapterHook {
    hook_id: String,
    lane: WorthUiExecutionLane,
    kind: WorthUiLaneAdapterHookKind,
}

impl WorthUiLaneAdapterHook {
    pub fn source_ingress(hook_id: impl Into<String>) -> Self {
        Self::supported(
            hook_id,
            WorthUiExecutionLane::SpecialCaseExtension,
            WorthUiLaneAdapterHookKind::SourceIngress,
        )
    }

    pub fn debounce_policy(hook_id: impl Into<String>) -> Self {
        Self::supported(
            hook_id,
            WorthUiExecutionLane::SpecialCaseExtension,
            WorthUiLaneAdapterHookKind::DebouncePolicy,
        )
    }

    pub fn identity_seed_contribution(hook_id: impl Into<String>) -> Self {
        Self::supported(
            hook_id,
            WorthUiExecutionLane::SpecialCaseExtension,
            WorthUiLaneAdapterHookKind::IdentitySeedContribution,
        )
    }

    pub fn durable_state_family_admission(hook_id: impl Into<String>) -> Self {
        Self::supported(
            hook_id,
            WorthUiExecutionLane::SpecialCaseExtension,
            WorthUiLaneAdapterHookKind::DurableStateFamilyAdmission,
        )
    }

    pub fn component_lowering(hook_id: impl Into<String>) -> Self {
        Self::supported(
            hook_id,
            WorthUiExecutionLane::OrdinaryWidgetShell,
            WorthUiLaneAdapterHookKind::ComponentLowering,
        )
    }

    pub fn lane_adapter_mechanics(hook_id: impl Into<String>, lane: WorthUiExecutionLane) -> Self {
        Self::supported(
            hook_id,
            lane,
            WorthUiLaneAdapterHookKind::LaneAdapterMechanics,
        )
    }

    pub fn diagnostics_projection(hook_id: impl Into<String>) -> Self {
        Self::supported(
            hook_id,
            WorthUiExecutionLane::DiagnosticsProjection,
            WorthUiLaneAdapterHookKind::DiagnosticsProjection,
        )
    }

    pub fn counter_families(hook_id: impl Into<String>) -> Self {
        Self::supported(
            hook_id,
            WorthUiExecutionLane::SpecialCaseExtension,
            WorthUiLaneAdapterHookKind::CounterFamilies,
        )
    }

    pub fn report_materialization(hook_id: impl Into<String>) -> Self {
        Self::supported(
            hook_id,
            WorthUiExecutionLane::SpecialCaseExtension,
            WorthUiLaneAdapterHookKind::ReportMaterialization,
        )
    }

    pub(crate) fn supported(
        hook_id: impl Into<String>,
        lane: WorthUiExecutionLane,
        kind: WorthUiLaneAdapterHookKind,
    ) -> Self {
        Self {
            hook_id: hook_id.into(),
            lane,
            kind,
        }
    }

    #[cfg(test)]
    pub(crate) fn forbidden_for_test(
        hook_id: impl Into<String>,
        kind: WorthUiLaneAdapterHookKind,
    ) -> Self {
        Self {
            hook_id: hook_id.into(),
            lane: WorthUiExecutionLane::SpecialCaseExtension,
            kind,
        }
    }

    pub fn hook_id(&self) -> &str {
        &self.hook_id
    }

    pub fn lane(&self) -> WorthUiExecutionLane {
        self.lane
    }

    pub fn kind(&self) -> WorthUiLaneAdapterHookKind {
        self.kind
    }
}
