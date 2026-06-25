use forge_query::facade::{
    ForgeQueryAdmittedGraphReadAccessPlan, ForgeQueryGraphReadAccessAdmission,
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessExecutionCounters,
    ForgeQueryGraphReadAccessPlanConsumption,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionQuerySurfaceAnchors {
    access_admission_type: &'static str,
    admission_posture_type: &'static str,
    admitted_plan_type: &'static str,
    plan_consumption_type: &'static str,
    execution_counters_type: &'static str,
    receipt_fields: &'static [&'static str],
}

impl WorthGraphReadAccessPlanAdoptionQuerySurfaceAnchors {
    pub(in crate::graph_read_access_plan_adoption) fn current() -> Self {
        Self {
            access_admission_type: std::any::type_name::<ForgeQueryGraphReadAccessAdmission>(),
            admission_posture_type: std::any::type_name::<ForgeQueryGraphReadAccessAdmissionPosture>(
            ),
            admitted_plan_type: std::any::type_name::<ForgeQueryAdmittedGraphReadAccessPlan>(),
            plan_consumption_type: std::any::type_name::<ForgeQueryGraphReadAccessPlanConsumption>(
            ),
            execution_counters_type: std::any::type_name::<
                ForgeQueryGraphReadAccessExecutionCounters,
            >(),
            receipt_fields: &[
                "graph_read_access_plan_consumption",
                "ephemeral_graph_index_receipt",
                "graph_read_streaming_receipt",
                "live_graph_read_access",
            ],
        }
    }

    pub const fn access_admission_type(&self) -> &'static str {
        self.access_admission_type
    }

    pub const fn admission_posture_type(&self) -> &'static str {
        self.admission_posture_type
    }

    pub const fn admitted_plan_type(&self) -> &'static str {
        self.admitted_plan_type
    }

    pub const fn plan_consumption_type(&self) -> &'static str {
        self.plan_consumption_type
    }

    pub const fn execution_counters_type(&self) -> &'static str {
        self.execution_counters_type
    }

    pub const fn receipt_fields(&self) -> &'static [&'static str] {
        self.receipt_fields
    }
}
