use forge_query::facade::runtime::{
    ForgeQueryAdmittedGraphReadAccessPlan, ForgeQueryEphemeralGraphIndexReceipt,
    ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessComplexityCounters,
    ForgeQueryGraphReadAccessPlanConsumption, ForgeQueryGraphReadAccessReceiptSummary,
    ForgeQueryGraphReadStreamingReceipt, ForgeQueryReadReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryGraphReadReceiptField {
    AccessPlan,
    AccessAdmission,
    PlanConsumption,
    EphemeralGraphIndexReceipt,
    StreamingReceipt,
    AccessSummary,
    ComplexityCounters,
}

impl QueryGraphReadReceiptField {
    pub const ALL: [Self; 7] = [
        Self::AccessPlan,
        Self::AccessAdmission,
        Self::PlanConsumption,
        Self::EphemeralGraphIndexReceipt,
        Self::StreamingReceipt,
        Self::AccessSummary,
        Self::ComplexityCounters,
    ];

    pub fn query_label(&self) -> &'static str {
        match self {
            Self::AccessPlan => "graph_read_access_plan",
            Self::AccessAdmission => "graph_read_access_admission",
            Self::PlanConsumption => "graph_read_access_plan_consumption",
            Self::EphemeralGraphIndexReceipt => "ephemeral_graph_index_receipt",
            Self::StreamingReceipt => "graph_read_streaming_receipt",
            Self::AccessSummary => "graph_read_access_summary",
            Self::ComplexityCounters => "graph_read_access_complexity_counters",
        }
    }
}

pub(crate) fn anchor_query_read_receipt_accessors() {
    let _: fn(&ForgeQueryReadReceipt) -> Option<&ForgeQueryAdmittedGraphReadAccessPlan> =
        ForgeQueryReadReceipt::graph_read_access_plan;
    let _: fn(&ForgeQueryReadReceipt) -> Option<&ForgeQueryGraphReadAccessAdmission> =
        ForgeQueryReadReceipt::graph_read_access_admission;
    let _: fn(&ForgeQueryReadReceipt) -> Option<&ForgeQueryGraphReadAccessPlanConsumption> =
        ForgeQueryReadReceipt::graph_read_access_plan_consumption;
    let _: fn(&ForgeQueryReadReceipt) -> Option<&ForgeQueryEphemeralGraphIndexReceipt> =
        ForgeQueryReadReceipt::ephemeral_graph_index_receipt;
    let _: fn(&ForgeQueryReadReceipt) -> Option<&ForgeQueryGraphReadStreamingReceipt> =
        ForgeQueryReadReceipt::graph_read_streaming_receipt;
    let _: fn(&ForgeQueryReadReceipt) -> Option<&ForgeQueryGraphReadAccessReceiptSummary> =
        ForgeQueryReadReceipt::graph_read_access_summary;
    let _: fn(&ForgeQueryReadReceipt) -> Option<&ForgeQueryGraphReadAccessComplexityCounters> =
        ForgeQueryReadReceipt::graph_read_access_complexity_counters;
}
