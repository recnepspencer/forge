use worth_ui::facade::{
    CommandProjectionSelectionMode, WorthUiDropdownSelectionStateStatus, WorthUiHeaderFrame,
};
mod reload_entry_selectors;

use crate::header::{applied_header_style_receipt, ValidationHeaderAppliedStyleReceipt};
use crate::launch::ValidationObservedStartupEvidence;
use crate::pages::page_slot_interaction::{
    ValidationPageSlotInteractionProjection, ValidationPageSlotInteractionRenderPlan,
};
use crate::pages::product_summary::{
    ValidationProductSummaryProjection, ValidationProductSummaryRenderPlan,
};
use crate::reload::{
    ValidationHeaderRebindEvidence, ValidationPhaseExecutionEvidence,
    ValidationReloadEvidenceEntry, ValidationReloadEvidenceLog,
};
use crate::runtime_workbench::ValidationRuntimeWorkbench;
use crate::storm_proof::{
    ValidationAuthoringTruthFinalBossProof, ValidationMixedReloadStormBuildDenial,
    ValidationMixedReloadStormProof,
};
use crate::{
    ValidationAuthoringTruthFinalBossVisibleSummary, ValidationMixedReloadStormVisibleSummary,
    ValidationReloadEvidencePanelSnapshot,
};
use reload_entry_selectors::{
    latest_authored_structural, latest_header_rebind, latest_page_host_rebind,
    latest_phase_execution,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationAppProofSnapshot {
    header: ValidationHeaderProofSnapshot,
    product_summary: ValidationProductSummaryRenderPlan,
    page_slot_interaction: ValidationPageSlotInteractionRenderPlan,
    latest_evidence: Option<ValidationReloadEvidenceEntry>,
    latest_phase_execution: Option<ValidationPhaseExecutionEvidence>,
    observed_startup: Option<ValidationObservedStartupEvidence>,
    visible_evidence_panel: ValidationReloadEvidencePanelSnapshot,
    mixed_reload_storm: Option<ValidationMixedReloadStormProof>,
    mixed_reload_storm_denial: Option<ValidationMixedReloadStormBuildDenial>,
    visible_mixed_reload_storm: Option<ValidationMixedReloadStormVisibleSummary>,
    authoring_truth_final_boss: Option<ValidationAuthoringTruthFinalBossProof>,
    visible_authoring_truth_final_boss: Option<ValidationAuthoringTruthFinalBossVisibleSummary>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationHeaderProofSnapshot {
    frame_digest: u64,
    applied_style: ValidationHeaderAppliedStyleReceipt,
    menus: Vec<ValidationHeaderMenuProofSnapshot>,
    latest_rebind: Option<ValidationHeaderRebindEvidence>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationHeaderMenuProofSnapshot {
    title: String,
    projection_id: String,
    component_id: String,
    commands: Vec<ValidationHeaderCommandProofSnapshot>,
    selection_mode: CommandProjectionSelectionMode,
    selected_command_ids: Vec<String>,
    selection_reconciliation_status: WorthUiDropdownSelectionStateStatus,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationHeaderCommandProofSnapshot {
    command_id: String,
    label: String,
}

impl ValidationAppProofSnapshot {
    pub fn from_workbench(
        workbench: &ValidationRuntimeWorkbench,
        evidence_log: &ValidationReloadEvidenceLog,
        observed_startup: Option<&ValidationObservedStartupEvidence>,
    ) -> Self {
        let header_frame = workbench.header_frame_plan().execute_frame();
        let latest_evidence = evidence_log.latest().cloned();
        let mixed_reload_storm_inspection =
            ValidationMixedReloadStormProof::inspect_from_workbench(workbench, evidence_log);
        let (mixed_reload_storm, mixed_reload_storm_denial) = match mixed_reload_storm_inspection {
            Ok(proof) => (Some(proof), None),
            Err(denial) => (None, Some(denial)),
        };
        let authoring_truth_final_boss =
            ValidationAuthoringTruthFinalBossProof::from_workbench(workbench, evidence_log);
        Self {
            header: ValidationHeaderProofSnapshot::from_header_frame(
                header_frame,
                latest_evidence.as_ref(),
            ),
            product_summary: ValidationProductSummaryRenderPlan::from_projection(
                ValidationProductSummaryProjection::from_runtime_receipts(
                    workbench.runtime().inspect_active(),
                    workbench.page_host_plan(),
                    latest_evidence.as_ref(),
                ),
            ),
            page_slot_interaction: ValidationPageSlotInteractionProjection::from_workbench(
                workbench,
                latest_evidence
                    .as_ref()
                    .and_then(latest_page_host_rebind)
                    .as_ref(),
                latest_evidence
                    .as_ref()
                    .and_then(latest_authored_structural)
                    .as_ref(),
            )
            .into_render_plan(),
            latest_phase_execution: latest_evidence.as_ref().and_then(latest_phase_execution),
            observed_startup: observed_startup.cloned(),
            latest_evidence,
            visible_evidence_panel: ValidationReloadEvidencePanelSnapshot::from_log(evidence_log),
            mixed_reload_storm_denial,
            visible_mixed_reload_storm: mixed_reload_storm
                .as_ref()
                .map(ValidationMixedReloadStormVisibleSummary::from_proof),
            mixed_reload_storm,
            visible_authoring_truth_final_boss: authoring_truth_final_boss
                .as_ref()
                .map(ValidationAuthoringTruthFinalBossVisibleSummary::from_proof),
            authoring_truth_final_boss,
        }
    }

    pub fn header(&self) -> &ValidationHeaderProofSnapshot {
        &self.header
    }

    pub fn product_summary(&self) -> &ValidationProductSummaryRenderPlan {
        &self.product_summary
    }

    pub fn page_slot_interaction(&self) -> &ValidationPageSlotInteractionRenderPlan {
        &self.page_slot_interaction
    }

    pub fn latest_evidence(&self) -> Option<&ValidationReloadEvidenceEntry> {
        self.latest_evidence.as_ref()
    }

    pub fn latest_phase_execution(&self) -> Option<&ValidationPhaseExecutionEvidence> {
        self.latest_phase_execution.as_ref()
    }

    pub fn visible_evidence_panel(&self) -> &ValidationReloadEvidencePanelSnapshot {
        &self.visible_evidence_panel
    }

    pub fn observed_startup(&self) -> Option<&ValidationObservedStartupEvidence> {
        self.observed_startup.as_ref()
    }

    pub fn mixed_reload_storm(&self) -> Option<&ValidationMixedReloadStormProof> {
        self.mixed_reload_storm.as_ref()
    }

    pub fn visible_mixed_reload_storm(&self) -> Option<&ValidationMixedReloadStormVisibleSummary> {
        self.visible_mixed_reload_storm.as_ref()
    }

    pub fn mixed_reload_storm_denial(&self) -> Option<&ValidationMixedReloadStormBuildDenial> {
        self.mixed_reload_storm_denial.as_ref()
    }

    pub fn authoring_truth_final_boss(&self) -> Option<&ValidationAuthoringTruthFinalBossProof> {
        self.authoring_truth_final_boss.as_ref()
    }

    pub fn visible_authoring_truth_final_boss(
        &self,
    ) -> Option<&ValidationAuthoringTruthFinalBossVisibleSummary> {
        self.visible_authoring_truth_final_boss.as_ref()
    }
}

impl ValidationHeaderProofSnapshot {
    fn from_header_frame(
        header_frame: WorthUiHeaderFrame<'_>,
        latest_evidence: Option<&ValidationReloadEvidenceEntry>,
    ) -> Self {
        Self {
            frame_digest: header_frame.frame_digest(),
            applied_style: applied_header_style_receipt(
                header_frame.theme(),
                header_frame.appearance(),
            ),
            menus: header_frame
                .menu()
                .groups()
                .iter()
                .map(|group| ValidationHeaderMenuProofSnapshot {
                    title: group.title().to_owned(),
                    projection_id: group.projection_id().to_owned(),
                    component_id: group.dropdown_frame().component_id().to_owned(),
                    commands: group
                        .commands()
                        .iter()
                        .map(|command| ValidationHeaderCommandProofSnapshot {
                            command_id: command.command_id().to_owned(),
                            label: command.label().to_owned(),
                        })
                        .collect(),
                    selection_mode: group.selection_mode(),
                    selected_command_ids: group.selection_state().selected_command_ids(),
                    selection_reconciliation_status: group
                        .selection_reconciliation()
                        .status()
                        .clone(),
                })
                .collect(),
            latest_rebind: latest_evidence.and_then(latest_header_rebind),
        }
    }

    pub fn frame_digest(&self) -> u64 {
        self.frame_digest
    }

    pub fn applied_style(&self) -> &ValidationHeaderAppliedStyleReceipt {
        &self.applied_style
    }

    pub fn menus(&self) -> &[ValidationHeaderMenuProofSnapshot] {
        &self.menus
    }

    pub fn latest_rebind(&self) -> Option<&ValidationHeaderRebindEvidence> {
        self.latest_rebind.as_ref()
    }
}

impl ValidationHeaderMenuProofSnapshot {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn projection_id(&self) -> &str {
        &self.projection_id
    }

    pub fn component_id(&self) -> &str {
        &self.component_id
    }

    pub fn commands(&self) -> &[ValidationHeaderCommandProofSnapshot] {
        &self.commands
    }

    pub fn selection_mode(&self) -> CommandProjectionSelectionMode {
        self.selection_mode
    }

    pub fn selected_command_ids(&self) -> &[String] {
        &self.selected_command_ids
    }

    pub fn selection_reconciliation_status(&self) -> WorthUiDropdownSelectionStateStatus {
        self.selection_reconciliation_status.clone()
    }
}

impl ValidationHeaderCommandProofSnapshot {
    pub fn command_id(&self) -> &str {
        &self.command_id
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}
