use worth_query::facade::installed::operation;

use super::{
    WorthUiConsumedSnapshotProjection, WorthUiDeferredSnapshotConsumer,
    WorthUiExecutedSnapshotConsumer, WorthUiPublishedSnapshotConsumer,
    WorthUiSettledSnapshotProjection,
};

pub enum WorthUiSnapshotConsumerExecutionOutcome {
    Executed(WorthUiExecutedSnapshotConsumer),
    Deferred(WorthUiDeferredSnapshotConsumer),
    Denied(operation::WorthQueryBoundExecutionDenial),
    Stale(operation::WorthQueryBoundExecutionDenial),
    RebindRequired(operation::WorthQueryBoundExecutionDenial),
    Failed(operation::WorthQueryBoundExecutionDenial),
}

pub enum WorthUiSnapshotProjectionPublicationOutcome {
    Published(WorthUiPublishedSnapshotConsumer),
    Denied(operation::WorthQueryPublicationDenial),
    Stale(operation::WorthQueryPublicationDenial),
    RebindRequired(operation::WorthQueryPublicationDenial),
    Failed(operation::WorthQueryPublicationDenial),
}

pub enum WorthUiSnapshotProjectionConsumptionOutcome {
    Consumed(WorthUiConsumedSnapshotProjection),
    Denied(operation::WorthQueryProgressionDenial),
    Deferred(operation::WorthQueryProgressionDenial),
    Stale(operation::WorthQueryProgressionDenial),
    RebindRequired(operation::WorthQueryProgressionDenial),
    Failed(operation::WorthQueryProgressionDenial),
}

pub enum WorthUiSnapshotProjectionSettlementOutcome {
    Settled(WorthUiSettledSnapshotProjection),
    Denied(operation::WorthQueryProgressionDenial),
    Stale(operation::WorthQueryProgressionDenial),
    RebindRequired(operation::WorthQueryProgressionDenial),
    Failed(operation::WorthQueryProgressionDenial),
}

impl WorthUiSnapshotConsumerExecutionOutcome {
    pub fn unwrap(self) -> WorthUiExecutedSnapshotConsumer {
        match self {
            Self::Executed(value) => value,
            Self::Deferred(_) => panic!("snapshot execution deferred"),
            Self::Denied(_) => panic!("snapshot execution denied"),
            Self::Stale(_) => panic!("snapshot execution stale"),
            Self::RebindRequired(_) => panic!("snapshot execution requires rebind"),
            Self::Failed(_) => panic!("snapshot execution failed"),
        }
    }
}

impl WorthUiSnapshotProjectionPublicationOutcome {
    pub fn unwrap(self) -> WorthUiPublishedSnapshotConsumer {
        match self {
            Self::Published(value) => value,
            Self::Denied(_) => panic!("snapshot publication denied"),
            Self::Stale(_) => panic!("snapshot publication stale"),
            Self::RebindRequired(_) => panic!("snapshot publication requires rebind"),
            Self::Failed(_) => panic!("snapshot publication failed"),
        }
    }
}

impl WorthUiSnapshotProjectionConsumptionOutcome {
    pub fn unwrap(self) -> WorthUiConsumedSnapshotProjection {
        match self {
            Self::Consumed(value) => value,
            Self::Denied(_) => panic!("snapshot consumption denied"),
            Self::Deferred(_) => panic!("snapshot consumption deferred"),
            Self::Stale(_) => panic!("snapshot consumption stale"),
            Self::RebindRequired(_) => panic!("snapshot consumption requires rebind"),
            Self::Failed(_) => panic!("snapshot consumption failed"),
        }
    }
}

impl WorthUiSnapshotProjectionSettlementOutcome {
    pub fn unwrap(self) -> WorthUiSettledSnapshotProjection {
        match self {
            Self::Settled(value) => value,
            Self::Denied(_) => panic!("snapshot settlement denied"),
            Self::Stale(_) => panic!("snapshot settlement stale"),
            Self::RebindRequired(_) => panic!("snapshot settlement requires rebind"),
            Self::Failed(_) => panic!("snapshot settlement failed"),
        }
    }
}
