use crate::runtime::{
    WorthUiCapabilityReloadEvidence, WorthUiComponentInteractionReceipt,
    WorthUiDropdownSelectionInteractionReceipt, WorthUiLiveViewEditReceipt,
    WorthUiQueryRuntimeFactLoweringReceipt, WorthUiValidationReloadEvidence,
};

use super::{
    WorthUiRuntimeChangeActivationPosture, WorthUiRuntimeChangeFamilyRow,
    WorthUiRuntimeChangeFamilyStatus, WorthUiRuntimeChangeMixedPosture,
    WorthUiRuntimeInstanceWitness,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiClassifiedRuntimeChange {
    runtime_instance: WorthUiRuntimeInstanceWitness,
    family_rows: Vec<WorthUiRuntimeChangeFamilyRow>,
    posture: WorthUiRuntimeChangeActivationPosture,
}

impl WorthUiClassifiedRuntimeChange {
    pub(crate) fn from_validation_reload(evidence: &WorthUiValidationReloadEvidence) -> Self {
        Self::from_rows(vec![
            WorthUiRuntimeChangeFamilyRow::from_validation_evidence(evidence),
        ])
        .expect("single validation evidence row is internally coherent")
    }

    pub(crate) fn from_capability_reload(evidence: &WorthUiCapabilityReloadEvidence) -> Self {
        Self::from_rows(vec![
            WorthUiRuntimeChangeFamilyRow::from_capability_evidence(evidence),
        ])
        .expect("single capability evidence row is internally coherent")
    }

    pub(crate) fn from_query_lowering_receipt(
        runtime_instance: WorthUiRuntimeInstanceWitness,
        receipt: &WorthUiQueryRuntimeFactLoweringReceipt,
    ) -> Self {
        Self::from_rows(vec![
            WorthUiRuntimeChangeFamilyRow::from_query_lowering_receipt(runtime_instance, receipt),
        ])
        .expect("single query evidence row is internally coherent")
    }

    pub(crate) fn from_dropdown_selection_interaction(
        runtime_instance: WorthUiRuntimeInstanceWitness,
        receipt: &WorthUiDropdownSelectionInteractionReceipt,
    ) -> Self {
        Self::from_rows(vec![
            WorthUiRuntimeChangeFamilyRow::from_dropdown_selection_interaction(
                runtime_instance,
                receipt,
            ),
        ])
        .expect("single interaction evidence row is internally coherent")
    }

    pub(crate) fn from_component_interaction(
        runtime_instance: WorthUiRuntimeInstanceWitness,
        receipt: &WorthUiComponentInteractionReceipt,
    ) -> Self {
        Self::from_rows(vec![
            WorthUiRuntimeChangeFamilyRow::from_component_interaction(runtime_instance, receipt),
        ])
        .expect("single component interaction evidence row is internally coherent")
    }

    pub(crate) fn from_live_view_state_edit(
        runtime_instance: WorthUiRuntimeInstanceWitness,
        receipt: &WorthUiLiveViewEditReceipt,
    ) -> Self {
        Self::from_rows(vec![
            WorthUiRuntimeChangeFamilyRow::from_live_view_state_edit(runtime_instance, receipt),
        ])
        .expect("single live view state edit row is internally coherent")
    }

    pub(crate) fn from_rows(
        mut family_rows: Vec<WorthUiRuntimeChangeFamilyRow>,
    ) -> Result<Self, WorthUiRuntimeChangeClassificationDenial> {
        let runtime_instance = family_rows
            .first()
            .ok_or(WorthUiRuntimeChangeClassificationDenial::EmptyRows)?
            .runtime_instance();
        if family_rows
            .iter()
            .any(|row| row.runtime_instance() != runtime_instance)
        {
            return Err(WorthUiRuntimeChangeClassificationDenial::RuntimeInstanceMismatch);
        }
        family_rows.sort_by(WorthUiRuntimeChangeFamilyRow::canonical_cmp);
        let posture = classify_posture(&family_rows);
        Ok(Self {
            runtime_instance,
            family_rows,
            posture,
        })
    }

    pub fn runtime_instance(&self) -> WorthUiRuntimeInstanceWitness {
        self.runtime_instance
    }

    pub fn posture(&self) -> WorthUiRuntimeChangeActivationPosture {
        self.posture
    }

    pub fn family_rows(&self) -> &[WorthUiRuntimeChangeFamilyRow] {
        &self.family_rows
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiRuntimeChangeClassificationDenial {
    EmptyRows,
    RuntimeInstanceMismatch,
}

fn classify_posture(
    family_rows: &[WorthUiRuntimeChangeFamilyRow],
) -> WorthUiRuntimeChangeActivationPosture {
    let equivalent_count = count_status(
        family_rows,
        WorthUiRuntimeChangeFamilyStatus::EquivalentNoOp,
    );
    let ready_count = count_status(
        family_rows,
        WorthUiRuntimeChangeFamilyStatus::ReadyForFrameBoundary,
    );
    let activated_count = count_status(family_rows, WorthUiRuntimeChangeFamilyStatus::Activated);
    let denied_count = count_status(family_rows, WorthUiRuntimeChangeFamilyStatus::Denied);
    let nonzero_status_count = [equivalent_count, ready_count, activated_count, denied_count]
        .iter()
        .filter(|count| **count > 0)
        .count();
    if nonzero_status_count > 1 {
        return WorthUiRuntimeChangeActivationPosture::Mixed(
            WorthUiRuntimeChangeMixedPosture::new(
                equivalent_count,
                ready_count,
                activated_count,
                denied_count,
            ),
        );
    }
    match family_rows[0].status() {
        WorthUiRuntimeChangeFamilyStatus::EquivalentNoOp => {
            WorthUiRuntimeChangeActivationPosture::EquivalentNoOp
        }
        WorthUiRuntimeChangeFamilyStatus::ReadyForFrameBoundary => {
            WorthUiRuntimeChangeActivationPosture::ReadyForFrameBoundary
        }
        WorthUiRuntimeChangeFamilyStatus::Activated => {
            WorthUiRuntimeChangeActivationPosture::Activated
        }
        WorthUiRuntimeChangeFamilyStatus::Denied => WorthUiRuntimeChangeActivationPosture::Denied,
    }
}

fn count_status(
    family_rows: &[WorthUiRuntimeChangeFamilyRow],
    status: WorthUiRuntimeChangeFamilyStatus,
) -> usize {
    family_rows
        .iter()
        .filter(|row| row.status() == status)
        .count()
}
