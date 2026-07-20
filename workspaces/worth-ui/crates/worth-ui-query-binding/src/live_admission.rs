use crate::{WorthUiQueryLiveResource, WorthUiQueryMeasurementFactSettlementDenial};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryLiveAdmissionDenial {
    QueryNotInstalled,
    InstalledAuthorityMismatch,
    ViewDefinitionMismatch,
    ProjectionResourceMismatch,
    LiveResourceAlreadyAdmitted,
    Projection(WorthUiQueryMeasurementFactSettlementDenial),
}

#[must_use = "a live admission stop retains the Query resource for retry or close"]
#[derive(Debug)]
pub struct WorthUiQueryLiveAdmissionStop {
    denial: WorthUiQueryLiveAdmissionDenial,
    resource: WorthUiQueryLiveResource,
}

impl WorthUiQueryLiveAdmissionStop {
    pub(crate) fn new(
        denial: WorthUiQueryLiveAdmissionDenial,
        resource: WorthUiQueryLiveResource,
    ) -> Self {
        Self { denial, resource }
    }

    pub fn denial(&self) -> WorthUiQueryLiveAdmissionDenial {
        self.denial
    }

    pub fn into_resource(self) -> WorthUiQueryLiveResource {
        self.resource
    }
}
