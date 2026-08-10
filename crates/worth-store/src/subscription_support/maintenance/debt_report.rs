use super::super::{
    classification_error, cost_surface_for_program_path, SubscriptionSupportOperationalBasis,
    SubscriptionSupportPlanFamily, SubscriptionSupportResultCostSurface, SupportAllocationScope,
    SupportPathClass, SupportProgramDensityClass, SupportProgramPathPlan,
};
use super::admission_witness::SupportMaintenanceAdmissionWitness;
use super::batch_plan::SupportMaintenanceBatchPlan;
use super::debt_summary::SupportMaintenanceDebtSummary;
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportMaintenanceDebtReport {
    debt_summary: SupportMaintenanceDebtSummary,
    admissions: Vec<SupportMaintenanceAdmissionWitness>,
    translation_bases: Vec<SubscriptionSupportOperationalBasis>,
    cost_surface: SubscriptionSupportResultCostSurface,
}

impl SubscriptionSupportMaintenanceDebtReport {
    pub(crate) fn new(
        plan: &SupportMaintenanceBatchPlan,
        delay_reason: impl Into<String>,
        path_plan: &SupportProgramPathPlan,
    ) -> Result<Self, StoreError> {
        if path_plan.path_class() != SupportPathClass::OperatorReporting {
            return Err(classification_error(
                "subscription-support maintenance debt reports require operator-reporting paths",
            ));
        }
        if path_plan.density_class() != SupportProgramDensityClass::MaintenanceKeyBatch {
            return Err(classification_error(
                "subscription-support maintenance debt reports require maintenance-key density",
            ));
        }
        if path_plan.allocation_scope() != SupportAllocationScope::OperatorReport {
            return Err(classification_error(
                "subscription-support maintenance debt reports require operator-report allocation",
            ));
        }
        if path_plan.batch_width() != plan.affected_set().affected_count() {
            return Err(classification_error(
                "subscription-support maintenance debt reports must preserve affected-set breadth",
            ));
        }
        Ok(Self {
            debt_summary: SupportMaintenanceDebtSummary::new(
                plan.action_id(),
                plan.affected_set(),
                plan.decision(),
                plan.descriptors().len() as u64,
                plan.coalesced_duplicate_count(),
                delay_reason,
            )?,
            admissions: plan
                .descriptors()
                .iter()
                .map(SupportMaintenanceAdmissionWitness::new)
                .collect(),
            translation_bases: plan.affected_set().affected_bases().to_vec(),
            cost_surface: cost_surface_for_program_path(
                SubscriptionSupportPlanFamily::MaintenanceParticipationPlan,
                path_plan,
            ),
        })
    }

    pub fn debt_summary(&self) -> &SupportMaintenanceDebtSummary {
        &self.debt_summary
    }

    pub fn admissions(&self) -> &[SupportMaintenanceAdmissionWitness] {
        &self.admissions
    }

    pub fn translation_bases(&self) -> &[SubscriptionSupportOperationalBasis] {
        &self.translation_bases
    }

    pub fn cost_surface(&self) -> SubscriptionSupportResultCostSurface {
        self.cost_surface
    }
}
