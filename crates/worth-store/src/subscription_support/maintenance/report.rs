use super::super::{
    classification_error, cost_surface_for_program_path, CompletedSupportProgramAction,
    SubscriptionSupportPlanFamily, SubscriptionSupportResultCostSurface, SupportProgramPathPlan,
};
use super::admission_witness::SupportMaintenanceAdmissionWitness;
use super::affected_set::SupportMaintenanceAffectedSet;
use super::decision::SubscriptionSupportMaintenanceDecision;
use super::descriptor::SupportMaintenanceDescriptor;
use super::descriptor_record::SupportMaintenanceDescriptorRecord;
use super::participation_record::SupportMaintenanceParticipationRecord;
use crate::failure::StoreError;
use crate::MaintenanceAdmissionReceipt;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportMaintenanceReport {
    completed_action: CompletedSupportProgramAction,
    participation_record: SupportMaintenanceParticipationRecord,
    admissions: Vec<SupportMaintenanceAdmissionWitness>,
    descriptor_records: Vec<SupportMaintenanceDescriptorRecord>,
    cost_surface: SubscriptionSupportResultCostSurface,
}

impl SubscriptionSupportMaintenanceReport {
    pub(crate) fn new(
        completed_action: CompletedSupportProgramAction,
        affected_set: SupportMaintenanceAffectedSet,
        descriptors: Vec<SupportMaintenanceDescriptor>,
        maintenance_receipt: &MaintenanceAdmissionReceipt,
        coalesced_duplicate_count: u64,
        decision: &SubscriptionSupportMaintenanceDecision,
        path_plan: &SupportProgramPathPlan,
    ) -> Result<Self, StoreError> {
        let participation_record = SupportMaintenanceParticipationRecord::new(
            &completed_action,
            &affected_set,
            decision.kind(),
            descriptors.len() as u64,
            coalesced_duplicate_count,
        )?;
        let admissions = descriptors
            .iter()
            .map(SupportMaintenanceAdmissionWitness::new)
            .collect();
        let admitted_by_declaration = maintenance_receipt
            .admitted_declarations()
            .iter()
            .map(|admitted| (admitted.declaration().id().clone(), admitted))
            .collect::<BTreeMap<_, _>>();
        let descriptor_records = descriptors
            .iter()
            .map(|descriptor| {
                let declaration_id = descriptor.descriptor().declaration_id();
                admitted_by_declaration
                    .get(declaration_id)
                    .ok_or_else(|| {
                        classification_error(
                            "subscription-support maintenance report lost its admitted maintenance declaration",
                        )
                    })?;
                SupportMaintenanceDescriptorRecord::from_descriptor(
                    &completed_action,
                    &affected_set,
                    descriptor,
                    decision.kind(),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            completed_action,
            participation_record,
            admissions,
            descriptor_records,
            cost_surface: cost_surface_for_program_path(
                SubscriptionSupportPlanFamily::MaintenanceParticipationPlan,
                path_plan,
            ),
        })
    }

    pub fn completed_action(&self) -> &CompletedSupportProgramAction {
        &self.completed_action
    }

    pub fn participation_record(&self) -> &SupportMaintenanceParticipationRecord {
        &self.participation_record
    }

    pub fn admissions(&self) -> &[SupportMaintenanceAdmissionWitness] {
        &self.admissions
    }

    pub fn descriptor_records(&self) -> &[SupportMaintenanceDescriptorRecord] {
        &self.descriptor_records
    }

    pub fn cost_surface(&self) -> SubscriptionSupportResultCostSurface {
        self.cost_surface
    }
}
