use worth_ui_inspection::{
    UiInspectionPosture, UiInspectionQuery, UiInspectionScope, UiInspectionSupportReport,
    UiInspectionSupportStatus,
};

use crate::declaration::{UiDeclarationArtifact, UiDeclarationCloseoutReport};
use crate::graph::{UiGraphAuthority, UiGraphCloseoutReport, UiGraphSnapshot};
use crate::facade::{
    runtime_bridge::{WorthUiCapabilityRegistrationFreezeCore, WorthUiFacadeLifecycleBootstrap},
    CapabilitySnapshot, UiInspectionClosureReport, UiInspectionFacadeObservation,
    UiInspectionReceipt, WorthUiRuntimeHost, WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial,
    WorthUiRuntimeSupportInventory,
};

/// Runtime facade entrypoint for building Worth UI applications.
pub struct WorthUi {
    _sealed: (),
}

impl WorthUi {
    /// Start a Worth UI application definition.
    pub fn app() -> crate::facade::WorthUiBuilder {
        crate::facade::WorthUiBuilder::new()
    }
}

/// Worth UI application after capability registration has frozen.
pub struct WorthUiApp {
    capability_snapshot: CapabilitySnapshot,
    declaration_artifacts: Vec<UiDeclarationArtifact>,
    graph_snapshot: UiGraphSnapshot,
    lifecycle: WorthUiFacadeLifecycleBootstrap,
}

impl WorthUiApp {
    pub(crate) fn from_freeze_core(core: WorthUiCapabilityRegistrationFreezeCore) -> Self {
        let (capability_snapshot, declaration_artifacts, graph_snapshot, lifecycle) = core.into_parts();

        Self {
            capability_snapshot,
            declaration_artifacts,
            graph_snapshot,
            lifecycle,
        }
    }

    /// Inspect the immutable capability snapshot owned by this app.
    pub fn capabilities(&self) -> &CapabilitySnapshot {
        &self.capability_snapshot
    }

    /// Inspect the canonical declaration artifacts admitted during app freeze.
    pub fn declaration_artifacts(&self) -> &[UiDeclarationArtifact] {
        &self.declaration_artifacts
    }

    /// Inspect the proof-bearing graph authority surface owned by this app.
    pub fn graph(&self) -> UiGraphAuthority<'_> {
        UiGraphAuthority::new(&self.graph_snapshot)
    }

    #[cfg(test)]
    pub(crate) fn graph_snapshot(&self) -> &UiGraphSnapshot {
        &self.graph_snapshot
    }

    pub fn graph_closeout_report(&self) -> UiGraphCloseoutReport {
        UiGraphCloseoutReport::milestone33()
    }

    /// Inspect milestone-closeout metadata owned by the declaration boundary.
    pub fn declaration_closeout_report(&self) -> UiDeclarationCloseoutReport {
        UiDeclarationCloseoutReport::milestone32()
    }

    /// Enter the runtime-owned inspection surface through one formal facade lane.
    pub fn inspect(&self, query: UiInspectionQuery) -> UiInspectionReceipt {
        self.lifecycle.record_inspection_query();
        let support_report = self.inspection_support_report(query.scope());
        let posture = match support_report.status() {
            UiInspectionSupportStatus::Supported => UiInspectionPosture::available(),
            UiInspectionSupportStatus::Unsupported => {
                self.lifecycle.record_unsupported_inspection_query();
                UiInspectionPosture::unsupported(
                    support_report.reason().expect(
                        "unsupported inspection scopes must declare a typed support reason",
                    ),
                    support_report.expected_in(),
                )
            }
        };

        UiInspectionReceipt::new(query, posture)
    }

    pub fn inspection_support_report(&self, scope: UiInspectionScope) -> UiInspectionSupportReport {
        self.lifecycle.inspection_support_report(scope)
    }

    pub fn inspection_closure_report(&self) -> UiInspectionClosureReport {
        self.lifecycle.inspection_closure_report()
    }

    pub fn runtime_support_inventory(&self) -> &WorthUiRuntimeSupportInventory {
        self.lifecycle.runtime_support_inventory()
    }

    pub fn inspection_observation(&self) -> UiInspectionFacadeObservation {
        self.lifecycle.inspection_observation()
    }

    /// Launch a runtime host from canonical artifact truth validated against this app snapshot.
    pub fn launch_runtime(
        &self,
        launch: WorthUiRuntimeLaunch,
    ) -> Result<WorthUiRuntimeHost, WorthUiRuntimeLaunchDenial> {
        WorthUiRuntimeHost::launch(launch, self.capability_snapshot.digest())
    }
}
