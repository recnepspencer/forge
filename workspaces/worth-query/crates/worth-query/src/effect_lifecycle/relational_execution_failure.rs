use super::execution::EffectExecutionDenialKind;

#[derive(Debug)]
pub struct RelationalEffectSettlementDeferred {
    message: String,
    settlement: worth_relational::facade::publication::DeferredPublicationSettlement,
}

impl RelationalEffectSettlementDeferred {
    pub fn message(&self) -> &str {
        &self.message
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        String,
        worth_relational::facade::publication::DeferredPublicationSettlement,
    ) {
        (self.message, self.settlement)
    }
}

#[derive(Debug)]
pub enum RelationalEffectExecutionFailure {
    Deferred {
        kind: super::EffectExecutionDeferredKind,
        message: String,
    },
    Denied {
        kind: EffectExecutionDenialKind,
        message: String,
    },
    ControlStopped {
        kind: super::EffectExecutionControlStopKind,
        message: String,
    },
    SettlementDeferred(RelationalEffectSettlementDeferred),
}

impl RelationalEffectExecutionFailure {
    pub fn deferred(kind: super::EffectExecutionDeferredKind, message: impl Into<String>) -> Self {
        Self::Deferred {
            kind,
            message: message.into(),
        }
    }

    pub(crate) fn settlement_deferred(
        message: impl Into<String>,
        settlement: worth_relational::facade::publication::DeferredPublicationSettlement,
    ) -> Self {
        Self::SettlementDeferred(RelationalEffectSettlementDeferred {
            message: message.into(),
            settlement,
        })
    }

    pub fn from_publication_failure(
        kind: EffectExecutionDenialKind,
        message: impl Into<String>,
        settlement: Option<worth_relational::facade::publication::DeferredPublicationSettlement>,
    ) -> Self {
        let message = message.into();
        match settlement {
            Some(settlement) => Self::SettlementDeferred(RelationalEffectSettlementDeferred {
                message,
                settlement,
            }),
            None => Self::Denied { kind, message },
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::Deferred { message, .. }
            | Self::Denied { message, .. }
            | Self::ControlStopped { message, .. } => message,
            Self::SettlementDeferred(deferred) => deferred.message(),
        }
    }
}

impl From<(EffectExecutionDenialKind, String)> for RelationalEffectExecutionFailure {
    fn from((kind, message): (EffectExecutionDenialKind, String)) -> Self {
        Self::Denied { kind, message }
    }
}
