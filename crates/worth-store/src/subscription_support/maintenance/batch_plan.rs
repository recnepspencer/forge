use super::super::{
    classification_error, SupportActionId, SupportAllocationScope, SupportPathClass,
    SupportProgramDensityClass, SupportProgramPathPlan,
};
use super::affected_set::SupportMaintenanceAffectedSet;
use super::decision::{
    SubscriptionSupportMaintenanceDecision, SubscriptionSupportMaintenanceDecisionKind,
};
use super::descriptor::SupportMaintenanceDescriptor;
use crate::failure::StoreError;
use crate::{MaintenanceAdmissionReceipt, MaintenanceBatchClass};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportMaintenanceBatchPlan {
    action_id: SupportActionId,
    affected_set: SupportMaintenanceAffectedSet,
    path_plan: SupportProgramPathPlan,
    descriptors: Vec<SupportMaintenanceDescriptor>,
    maintenance_receipt: MaintenanceAdmissionReceipt,
    coalesced_duplicate_count: u64,
    decision: SubscriptionSupportMaintenanceDecision,
}

impl SupportMaintenanceBatchPlan {
    pub(crate) fn new(
        action_id: SupportActionId,
        affected_set: SupportMaintenanceAffectedSet,
        path_plan: SupportProgramPathPlan,
        descriptors: Vec<SupportMaintenanceDescriptor>,
        maintenance_receipt: MaintenanceAdmissionReceipt,
        coalesced_duplicate_count: u64,
        decision: SubscriptionSupportMaintenanceDecision,
    ) -> Result<Self, StoreError> {
        validate_support_maintenance_path(&path_plan, &affected_set)?;
        validate_support_maintenance_coalescing(
            &descriptors,
            &affected_set,
            coalesced_duplicate_count,
        )?;
        validate_support_maintenance_receipt(&maintenance_receipt, &descriptors)?;
        verify_support_maintenance_receipt_alignment(
            &maintenance_receipt,
            &descriptors,
            &decision,
        )?;
        Ok(Self {
            action_id,
            affected_set,
            path_plan,
            descriptors,
            maintenance_receipt,
            coalesced_duplicate_count,
            decision,
        })
    }

    pub fn affected_set(&self) -> &SupportMaintenanceAffectedSet {
        &self.affected_set
    }

    pub fn action_id(&self) -> &SupportActionId {
        &self.action_id
    }

    pub fn descriptors(&self) -> &[SupportMaintenanceDescriptor] {
        &self.descriptors
    }

    pub fn maintenance_receipt(&self) -> &MaintenanceAdmissionReceipt {
        &self.maintenance_receipt
    }

    pub fn coalesced_duplicate_count(&self) -> u64 {
        self.coalesced_duplicate_count
    }

    pub fn decision(&self) -> &SubscriptionSupportMaintenanceDecision {
        &self.decision
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        SupportActionId,
        SupportMaintenanceAffectedSet,
        SupportProgramPathPlan,
        Vec<SupportMaintenanceDescriptor>,
        MaintenanceAdmissionReceipt,
        u64,
        SubscriptionSupportMaintenanceDecision,
    ) {
        (
            self.action_id,
            self.affected_set,
            self.path_plan,
            self.descriptors,
            self.maintenance_receipt,
            self.coalesced_duplicate_count,
            self.decision,
        )
    }
}

fn validate_support_maintenance_path(
    path_plan: &SupportProgramPathPlan,
    affected_set: &SupportMaintenanceAffectedSet,
) -> Result<(), StoreError> {
    if path_plan.path_class() != SupportPathClass::MaintenanceExecution {
        return Err(classification_error(
            "subscription-support maintenance plans require maintenance-execution paths",
        ));
    }
    if path_plan.density_class() != SupportProgramDensityClass::MaintenanceKeyBatch {
        return Err(classification_error(
            "subscription-support maintenance plans require maintenance-key density",
        ));
    }
    if path_plan.allocation_scope() != SupportAllocationScope::FamilyLocalBatch {
        return Err(classification_error(
            "subscription-support maintenance plans require family-local allocation",
        ));
    }
    if path_plan.batch_width() != affected_set.affected_count() {
        return Err(classification_error(
            "subscription-support maintenance plan width must match affected-set breadth",
        ));
    }
    Ok(())
}

fn validate_support_maintenance_coalescing(
    descriptors: &[SupportMaintenanceDescriptor],
    affected_set: &SupportMaintenanceAffectedSet,
    coalesced_duplicate_count: u64,
) -> Result<(), StoreError> {
    if descriptors.is_empty() {
        return Err(classification_error(
            "subscription-support maintenance plans require at least one descriptor",
        ));
    }
    if descriptors.len() as u64 + coalesced_duplicate_count != affected_set.affected_count() {
        return Err(classification_error(
            "subscription-support maintenance coalescing must account for every affected entry",
        ));
    }
    Ok(())
}

fn validate_support_maintenance_receipt(
    maintenance_receipt: &MaintenanceAdmissionReceipt,
    descriptors: &[SupportMaintenanceDescriptor],
) -> Result<(), StoreError> {
    if !maintenance_receipt.rejections().is_empty() {
        return Err(classification_error(
            "subscription-support maintenance plans require fully admitted maintenance receipts",
        ));
    }
    if maintenance_receipt.batch_summary().batch_class()
        != MaintenanceBatchClass::SubscriptionSupport
    {
        return Err(classification_error(
            "subscription-support maintenance plans require subscription-support maintenance batch class",
        ));
    }
    if maintenance_receipt.admitted_declarations().len() != descriptors.len() {
        return Err(classification_error(
            "subscription-support maintenance receipts must admit every unique descriptor",
        ));
    }
    Ok(())
}

fn verify_support_maintenance_receipt_alignment(
    maintenance_receipt: &MaintenanceAdmissionReceipt,
    descriptors: &[SupportMaintenanceDescriptor],
    decision: &SubscriptionSupportMaintenanceDecision,
) -> Result<(), StoreError> {
    let admitted_by_declaration = maintenance_receipt
        .admitted_declarations()
        .iter()
        .map(|admitted| (admitted.declaration().id().clone(), admitted))
        .collect::<BTreeMap<_, _>>();
    for descriptor in descriptors {
        let declaration_id = descriptor.descriptor().declaration_id();
        let admitted = admitted_by_declaration
            .get(declaration_id)
            .copied()
            .ok_or_else(|| {
                classification_error(
                    "subscription-support maintenance receipt is missing an admitted declaration for a descriptor",
                )
            })?;
        let expected_admitted_descriptor =
            expected_support_maintenance_admitted_descriptor(descriptor, decision);
        if admitted.descriptor() != &expected_admitted_descriptor
            || admitted.declaration() != descriptor.declaration()
        {
            return Err(classification_error(
                "subscription-support maintenance receipt drifted from the admitted descriptor",
            ));
        }
    }
    Ok(())
}

fn expected_support_maintenance_admitted_descriptor(
    descriptor: &SupportMaintenanceDescriptor,
    decision: &SubscriptionSupportMaintenanceDecision,
) -> crate::MaintenanceWorkDescriptor {
    if matches!(
        decision.kind(),
        SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered
    ) {
        descriptor
            .descriptor()
            .clone()
            .with_recovered_from_restart(false)
    } else {
        descriptor.descriptor().clone()
    }
}
