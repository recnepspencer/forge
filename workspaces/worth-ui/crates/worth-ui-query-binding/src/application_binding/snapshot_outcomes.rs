use worth_query::facade::installed::operation;

use super::{
    WorthUiConsumedSnapshotProjection, WorthUiDeferredSnapshotConsumer,
    WorthUiExecutedSnapshotConsumer, WorthUiPublishedSnapshotConsumer,
    WorthUiSettledSnapshotDerivationStop, WorthUiSettledSnapshotProjection,
};

pub enum WorthUiSnapshotConsumerExecutionOutcome {
    Executed(Box<WorthUiExecutedSnapshotConsumer>),
    Deferred(Box<WorthUiDeferredSnapshotConsumer>),
    Denied(Box<operation::WorthQueryBoundExecutionDenial>),
    Stale(Box<operation::WorthQueryBoundExecutionDenial>),
    RebindRequired(Box<operation::WorthQueryBoundExecutionDenial>),
    Failed(Box<operation::WorthQueryBoundExecutionDenial>),
}

pub enum WorthUiSnapshotProjectionPublicationOutcome {
    Published(Box<WorthUiPublishedSnapshotConsumer>),
    Denied(Box<operation::WorthQueryPublicationDenial>),
    Stale(Box<operation::WorthQueryPublicationDenial>),
    RebindRequired(Box<operation::WorthQueryPublicationDenial>),
    Failed(Box<operation::WorthQueryPublicationDenial>),
}

pub enum WorthUiSnapshotProjectionConsumptionOutcome {
    Consumed(Box<WorthUiConsumedSnapshotProjection>),
    Denied(Box<operation::WorthQueryProgressionDenial>),
    Deferred(Box<operation::WorthQueryProgressionDenial>),
    Stale(Box<operation::WorthQueryProgressionDenial>),
    RebindRequired(Box<operation::WorthQueryProgressionDenial>),
    Failed(Box<operation::WorthQueryProgressionDenial>),
}

pub enum WorthUiSnapshotProjectionSettlementOutcome {
    Settled(Box<WorthUiSettledSnapshotProjection>),
    DerivationStopped(Box<WorthUiSettledSnapshotDerivationStop>),
    Denied(Box<operation::WorthQueryProgressionDenial>),
    Stale(Box<operation::WorthQueryProgressionDenial>),
    RebindRequired(Box<operation::WorthQueryProgressionDenial>),
    Failed(Box<operation::WorthQueryProgressionDenial>),
}

impl WorthUiSnapshotConsumerExecutionOutcome {
    pub fn unwrap(self) -> WorthUiExecutedSnapshotConsumer {
        match self {
            Self::Executed(value) => *value,
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
            Self::Published(value) => *value,
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
            Self::Consumed(value) => *value,
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
            Self::Settled(value) => *value,
            Self::DerivationStopped(_) => panic!("snapshot measurement derivation stopped"),
            Self::Denied(_) => panic!("snapshot settlement denied"),
            Self::Stale(_) => panic!("snapshot settlement stale"),
            Self::RebindRequired(_) => panic!("snapshot settlement requires rebind"),
            Self::Failed(_) => panic!("snapshot settlement failed"),
        }
    }
}
