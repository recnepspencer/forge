#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyAwareExecutionMode {
    CurrentRead,
    BranchRead,
    HistoricalRead,
    HistoricalDiff,
    LiveSubscription,
    DeliveryShape,
    OptimizerInput,
    GraphMutation,
}

impl PolicyAwareExecutionMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CurrentRead => "current_read",
            Self::BranchRead => "branch_read",
            Self::HistoricalRead => "historical_read",
            Self::HistoricalDiff => "historical_diff",
            Self::LiveSubscription => "live_subscription",
            Self::DeliveryShape => "delivery_shape",
            Self::OptimizerInput => "optimizer_input",
            Self::GraphMutation => "graph_mutation",
        }
    }
}
