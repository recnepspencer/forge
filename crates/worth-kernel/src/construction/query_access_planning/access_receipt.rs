use forge_query::facade::{
    ForgeQueryAdmittedGraphReadAccessPlan, ForgeQueryGraphReadAccessAdmissionPosture,
    ForgeQueryGraphReadAccessPlanConsumption, ForgeQueryReadFamily, ForgeQueryReadReceipt,
    ForgeQueryReadResult,
};

use super::access_denial::PrimitiveConstructionQueryAccessError;
use super::covered_surface::PrimitiveConstructionQueryAccessSurface;

pub(crate) struct PrimitiveConstructionPlannedQueryAccess {
    surface: PrimitiveConstructionQueryAccessSurface,
    family: ForgeQueryReadFamily,
    plan: ForgeQueryAdmittedGraphReadAccessPlan,
}

pub(crate) struct PrimitiveConstructionConsumedQueryAccess {
    result: ForgeQueryReadResult,
    receipt: PrimitiveConstructionExecutedQueryAccessReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionExecutedQueryAccessReceipt {
    surface: PrimitiveConstructionQueryAccessSurface,
    family_digest: String,
    plan_digest: String,
    admission_digest: String,
    admission_posture: ForgeQueryGraphReadAccessAdmissionPosture,
    requirement_set_digest: String,
    cost_estimate_digest: String,
    budget_digest: String,
    graph_index_inventory_match_report_digest: String,
    plan_consumption_digest: String,
    executor_entry_count: usize,
    strategy_recompute_count: usize,
    edge_scan_count: usize,
    per_result_neighbor_lookup_count: usize,
    persistent_artifact_bypass_count: usize,
    materialized_row_count: usize,
}

impl PrimitiveConstructionPlannedQueryAccess {
    pub(crate) fn new(
        surface: PrimitiveConstructionQueryAccessSurface,
        family: ForgeQueryReadFamily,
        plan: ForgeQueryAdmittedGraphReadAccessPlan,
    ) -> Self {
        Self {
            surface,
            family,
            plan,
        }
    }

    pub(crate) fn surface(&self) -> PrimitiveConstructionQueryAccessSurface {
        self.surface
    }

    pub(crate) fn family_digest(&self) -> &str {
        self.family.family_digest()
    }

    pub(crate) fn plan_digest(&self) -> &str {
        self.plan.digest()
    }

    pub(crate) fn admission_posture(&self) -> &ForgeQueryGraphReadAccessAdmissionPosture {
        self.plan.posture()
    }

    pub(crate) fn family(&self) -> &ForgeQueryReadFamily {
        &self.family
    }

    pub(crate) fn plan(&self) -> &ForgeQueryAdmittedGraphReadAccessPlan {
        &self.plan
    }
}

impl PrimitiveConstructionExecutedQueryAccessReceipt {
    pub(crate) fn from_result(
        planned: &PrimitiveConstructionPlannedQueryAccess,
        result: &ForgeQueryReadResult,
    ) -> Result<Self, PrimitiveConstructionQueryAccessError> {
        Self::from_receipt(planned, result.receipt())
    }

    fn from_receipt(
        planned: &PrimitiveConstructionPlannedQueryAccess,
        receipt: &ForgeQueryReadReceipt,
    ) -> Result<Self, PrimitiveConstructionQueryAccessError> {
        let executed_plan = receipt
            .graph_read_access_plan()
            .ok_or(PrimitiveConstructionQueryAccessError::MissingExecutedPlan)?;
        if executed_plan.digest() != planned.plan_digest() {
            return Err(PrimitiveConstructionQueryAccessError::PlanDigestDrift {
                planned_digest: planned.plan_digest().to_string(),
                executed_digest: executed_plan.digest().to_string(),
            });
        }
        let consumption = receipt
            .graph_read_access_plan_consumption()
            .ok_or(PrimitiveConstructionQueryAccessError::MissingPlanConsumption)?;
        Ok(Self::from_plan_consumption(
            planned.surface(),
            planned.family_digest(),
            executed_plan,
            consumption,
        ))
    }

    fn from_plan_consumption(
        surface: PrimitiveConstructionQueryAccessSurface,
        family_digest: &str,
        plan: &ForgeQueryAdmittedGraphReadAccessPlan,
        consumption: &ForgeQueryGraphReadAccessPlanConsumption,
    ) -> Self {
        let admission = plan.admission();
        let execution_counters = consumption.execution_counters();
        Self {
            surface,
            family_digest: family_digest.to_string(),
            plan_digest: plan.digest().to_string(),
            admission_digest: admission.digest().to_string(),
            admission_posture: plan.posture().clone(),
            requirement_set_digest: admission.requirement_set().digest().as_str().to_string(),
            cost_estimate_digest: admission.cost_estimate().digest().as_str().to_string(),
            budget_digest: admission.budget_check().budget_digest().to_string(),
            graph_index_inventory_match_report_digest: admission
                .graph_index_inventory_match_report()
                .digest()
                .to_string(),
            plan_consumption_digest: consumption.digest().to_string(),
            executor_entry_count: execution_counters.executor_entry_count(),
            strategy_recompute_count: execution_counters.strategy_recompute_count(),
            edge_scan_count: execution_counters.edge_scan_count(),
            per_result_neighbor_lookup_count: execution_counters.per_result_neighbor_lookup_count(),
            persistent_artifact_bypass_count: execution_counters.persistent_artifact_bypass_count(),
            materialized_row_count: execution_counters.materialized_row_count(),
        }
    }

    pub(crate) fn surface(&self) -> PrimitiveConstructionQueryAccessSurface {
        self.surface
    }

    pub(crate) fn family_digest(&self) -> &str {
        &self.family_digest
    }

    pub(crate) fn plan_digest(&self) -> &str {
        &self.plan_digest
    }

    pub(crate) fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub(crate) fn admission_posture(&self) -> &ForgeQueryGraphReadAccessAdmissionPosture {
        &self.admission_posture
    }

    pub(crate) fn requirement_set_digest(&self) -> &str {
        &self.requirement_set_digest
    }

    pub(crate) fn cost_estimate_digest(&self) -> &str {
        &self.cost_estimate_digest
    }

    pub(crate) fn budget_digest(&self) -> &str {
        &self.budget_digest
    }

    pub(crate) fn graph_index_inventory_match_report_digest(&self) -> &str {
        &self.graph_index_inventory_match_report_digest
    }

    pub(crate) fn plan_consumption_digest(&self) -> &str {
        &self.plan_consumption_digest
    }

    pub(crate) fn executor_entry_count(&self) -> usize {
        self.executor_entry_count
    }

    pub(crate) fn materialized_row_count(&self) -> usize {
        self.materialized_row_count
    }

    pub(crate) fn no_caller_owned_graph_work(&self) -> bool {
        self.strategy_recompute_count == 0
            && self.edge_scan_count == 0
            && self.per_result_neighbor_lookup_count == 0
            && self.persistent_artifact_bypass_count == 0
    }
}

impl PrimitiveConstructionConsumedQueryAccess {
    pub(crate) fn from_planned_result(
        planned: &PrimitiveConstructionPlannedQueryAccess,
        result: ForgeQueryReadResult,
    ) -> Result<Self, PrimitiveConstructionQueryAccessError> {
        let receipt =
            PrimitiveConstructionExecutedQueryAccessReceipt::from_result(planned, &result)?;
        Ok(Self { result, receipt })
    }

    pub(crate) fn result(&self) -> &ForgeQueryReadResult {
        &self.result
    }

    pub(crate) fn receipt(&self) -> &PrimitiveConstructionExecutedQueryAccessReceipt {
        &self.receipt
    }
}
