use super::{
    ForgeQueryAdmittedGraphReadAccessPlan, ForgeQueryGraphReadAccessAdmission,
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessDenial,
    ForgeQueryGraphReadAccessInventoryMatch,
};
use crate::runtime::{
    ForgeQueryGraphIndexInventoryMatchReport, ForgeQueryGraphReadAccessRequirementRow,
    ForgeQueryGraphReadBudgetCheck, ForgeQueryGraphReadCostAttributionRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadAccessPlanExplanation {
    admission: ForgeQueryGraphReadAccessAdmission,
    admitted_plan_digest: Option<String>,
}

impl ForgeQueryGraphReadAccessPlanExplanation {
    pub fn admission_digest(&self) -> &str {
        self.admission.digest()
    }

    pub fn admitted_plan_digest(&self) -> Option<&str> {
        self.admitted_plan_digest.as_deref()
    }

    pub fn selected_posture(&self) -> &ForgeQueryGraphReadAccessAdmissionPosture {
        self.admission.posture()
    }

    pub fn requirement_rows(&self) -> &[ForgeQueryGraphReadAccessRequirementRow] {
        self.admission.requirement_set().rows()
    }

    pub fn attribution_rows(&self) -> &[ForgeQueryGraphReadCostAttributionRow] {
        self.admission.cost_estimate().attribution_rows()
    }

    pub fn inventory_matches(&self) -> &[ForgeQueryGraphReadAccessInventoryMatch] {
        self.admission.inventory_matches()
    }

    pub fn graph_index_support(&self) -> &ForgeQueryGraphIndexInventoryMatchReport {
        self.admission.graph_index_inventory_match_report()
    }

    pub fn budget_check(&self) -> &ForgeQueryGraphReadBudgetCheck {
        self.admission.budget_check()
    }

    pub fn denial(&self) -> Option<&ForgeQueryGraphReadAccessDenial> {
        self.admission.denial()
    }

    pub(crate) fn from_admission(admission: &ForgeQueryGraphReadAccessAdmission) -> Self {
        Self {
            admission: admission.clone(),
            admitted_plan_digest: None,
        }
    }

    pub(crate) fn from_admitted_plan(plan: &ForgeQueryAdmittedGraphReadAccessPlan) -> Self {
        Self {
            admission: plan.admission().clone(),
            admitted_plan_digest: Some(plan.digest().to_string()),
        }
    }
}
