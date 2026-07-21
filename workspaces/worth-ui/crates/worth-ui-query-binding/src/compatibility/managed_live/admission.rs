use super::WorthUiQueryLiveResource;
use crate::WorthUiQueryMeasurementFactSettlementDenial;

#[derive(Debug)]
pub enum WorthUiQueryLiveAdmissionDenial {
    QueryNotInstalled,
    CompatibilityNotInstalled,
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

    pub fn denial(&self) -> &WorthUiQueryLiveAdmissionDenial {
        &self.denial
    }

    pub fn into_resource(self) -> WorthUiQueryLiveResource {
        self.resource
    }
}
