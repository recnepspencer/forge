use worth_query::facade::installed::{self, operation};

use super::{
    WorthUiConsumedScalarTextProjection, WorthUiExecutedScalarTextConsumer,
    WorthUiPublishedScalarTextConsumer, WorthUiSettledScalarTextProjection,
};

pub(crate) enum WorthUiScalarTextExecutionOutcome {
    Executed(Box<WorthUiExecutedScalarTextConsumer>),
    Deferred(
        Box<
            operation::WorthQueryDeferredDomainOperation<
                crate::WorthUiDomainEntry,
                crate::WorthUiScalarTextProjection,
                crate::WorthUiScalarTextProjectionFamily,
                worth_query::facade::foundation::ObservationLaneWitness,
            >,
        >,
    ),
    ResourceAdmission(Box<installed::transition::WorthQueryResourceAdmissionStop>),
    Denied(Box<operation::WorthQueryBoundExecutionDenial>),
    Stale(Box<operation::WorthQueryBoundExecutionDenial>),
    RebindRequired(Box<operation::WorthQueryBoundExecutionDenial>),
    Failed(Box<operation::WorthQueryBoundExecutionDenial>),
}

pub(crate) enum WorthUiScalarTextPublicationOutcome {
    Published(Box<WorthUiPublishedScalarTextConsumer>),
    Denied(Box<operation::WorthQueryPublicationDenial>),
    Stale(Box<operation::WorthQueryPublicationDenial>),
    RebindRequired(Box<operation::WorthQueryPublicationDenial>),
    Failed(Box<operation::WorthQueryPublicationDenial>),
}

pub(crate) enum WorthUiScalarTextConsumptionOutcome {
    Consumed(Box<WorthUiConsumedScalarTextProjection>),
    Denied(Box<operation::WorthQueryProgressionDenial>),
    Deferred(Box<operation::WorthQueryProgressionDenial>),
    Stale(Box<operation::WorthQueryProgressionDenial>),
    RebindRequired(Box<operation::WorthQueryProgressionDenial>),
    Failed(Box<operation::WorthQueryProgressionDenial>),
}

pub(crate) enum WorthUiScalarTextSettlementOutcome {
    Settled(Box<WorthUiSettledScalarTextProjection>),
    Denied(Box<operation::WorthQueryProgressionDenial>),
    Stale(Box<operation::WorthQueryProgressionDenial>),
    RebindRequired(Box<operation::WorthQueryProgressionDenial>),
    Failed(Box<operation::WorthQueryProgressionDenial>),
}
