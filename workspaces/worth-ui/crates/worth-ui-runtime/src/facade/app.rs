use worth_ui_inspection::{
    UiInspectionQuery, UiInspectionScope, UiInspectionScopeSupportRow, UiInspectionSupportPosture,
    UiInspectionSupportReason, UiInspectionSupportReport, UiInspectionSupportWorld,
    UiInspectionTarget,
};

use crate::admission::UiAdmissionBoundary;
use crate::declaration::{UiDeclarationArtifact, UiDeclarationCloseoutReport};
use crate::facade::{
    runtime_bridge::{WorthUiCapabilityRegistrationFreezeCore, WorthUiFacadeLifecycleBootstrap},
    CapabilitySnapshot, UiInspectionClosureReport, UiInspectionFacadeObservation,
    UiInspectionReceipt, WorthUiRuntimeHost, WorthUiRuntimeLaunch, WorthUiRuntimeLaunchDenial,
    WorthUiRuntimeSupportInventory,
};
use crate::graph::{UiGraphAuthority, UiGraphCloseoutReport, UiGraphSnapshot};
use crate::obligations::closeout::UiObligationCloseoutReport;

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
        let (capability_snapshot, declaration_artifacts, graph_snapshot, lifecycle) =
            core.into_parts();

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

    /// Enter the runtime-owned admission boundary through one formal facade lane.
    pub fn admission(&self) -> UiAdmissionBoundary<'_> {
        UiAdmissionBoundary::new(&self.declaration_artifacts, &self.graph_snapshot)
    }

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

    pub fn obligation_closeout_report(&self) -> UiObligationCloseoutReport {
        UiObligationCloseoutReport::milestone34()
    }

    /// Enter the runtime-owned inspection surface through one formal facade lane.
    pub fn inspect(&self, query: UiInspectionQuery) -> UiInspectionReceipt {
        self.lifecycle.record_inspection_query();
        match query.target() {
            UiInspectionTarget::ProductRoot | UiInspectionTarget::DeclaredSurface { .. } => {
                let support_report = self.inspection_support_report_for(&query);
                if !matches!(
                    support_report.posture(),
                    UiInspectionSupportPosture::Supported
                ) {
                    self.lifecycle.record_unsupported_inspection_query();
                }
                UiInspectionReceipt::from_support(query, support_report)
            }
            UiInspectionTarget::ObligationGraphNode { .. }
            | UiInspectionTarget::ObligationTouch { .. }
            | UiInspectionTarget::ObligationEvidenceHandle { .. }
            | _ => UiInspectionReceipt::from_obligation(
                query,
                worth_ui_inspection::UiInspectionObligationEvidenceReceipt::new(Box::new([])),
            ),
        }
    }

    pub fn inspection_support_report(&self, scope: UiInspectionScope) -> UiInspectionSupportReport {
        self.lifecycle.inspection_support_report(scope)
    }

    pub fn inspection_support_report_for(
        &self,
        query: &UiInspectionQuery,
    ) -> UiInspectionSupportReport {
        match query.target() {
            UiInspectionTarget::ProductRoot => self.inspection_support_report(query.scope()),
            UiInspectionTarget::DeclaredSurface {
                module_path,
                declaration_index,
            } => self.declared_surface_inspection_support_report(
                module_path,
                *declaration_index,
                query.scope(),
            ),
            _ => {
                let rows = [UiInspectionScopeSupportRow::unsupported(
                    "inspection",
                    query.scope(),
                    UiInspectionSupportReason::TargetOutsideInspectionBoundary,
                    None,
                    UiInspectionSupportWorld::Authoritative,
                )];
                UiInspectionSupportReport::from_scope_rows(query.scope(), &rows)
            }
        }
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

    fn declared_surface_inspection_support_report(
        &self,
        module_path: &str,
        declaration_index: usize,
        scope: UiInspectionScope,
    ) -> UiInspectionSupportReport {
        let Some(artifact) = self.declaration_artifacts.iter().find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == module_path
                && provenance.declaration_index() == declaration_index
        }) else {
            let rows = [UiInspectionScopeSupportRow::unsupported(
                "inspection",
                scope,
                UiInspectionSupportReason::TargetOutsideInspectionBoundary,
                None,
                UiInspectionSupportWorld::Authoritative,
            )];
            return UiInspectionSupportReport::from_scope_rows(scope, &rows);
        };

        let Ok(snapshot) = artifact.support_snapshot() else {
            let rows = [UiInspectionScopeSupportRow::unsupported(
                "inspection",
                scope,
                UiInspectionSupportReason::TargetOutsideInspectionBoundary,
                None,
                UiInspectionSupportWorld::Authoritative,
            )];
            return UiInspectionSupportReport::from_scope_rows(scope, &rows);
        };

        let rows = snapshot.inspection_rows(scope);
        if rows.is_empty() {
            let rows = [UiInspectionScopeSupportRow::unsupported(
                "inspection",
                scope,
                UiInspectionSupportReason::TargetOutsideInspectionBoundary,
                None,
                UiInspectionSupportWorld::Authoritative,
            )];
            return UiInspectionSupportReport::from_scope_rows(scope, &rows);
        }

        UiInspectionSupportReport::from_scope_rows(scope, rows.as_ref())
    }
}
