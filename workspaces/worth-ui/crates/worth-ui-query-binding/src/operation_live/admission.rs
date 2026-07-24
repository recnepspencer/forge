use crate::WorthUiOperationLiveResource;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiOperationLiveAdmissionDenial {
    QueryNotInstalled,
    ForeignInstalledReference,
    DuplicateResource,
    SourceGenerationExhausted,
    SourceOrderExhausted,
}

#[must_use = "a stopped admission retains the exact operation-live resource"]
pub struct WorthUiOperationLiveAdmissionStop {
    denial: WorthUiOperationLiveAdmissionDenial,
    resource: Box<WorthUiOperationLiveResource>,
}

impl WorthUiOperationLiveAdmissionStop {
    pub(crate) fn new(
        denial: WorthUiOperationLiveAdmissionDenial,
        resource: WorthUiOperationLiveResource,
    ) -> Self {
        Self {
            denial,
            resource: Box::new(resource),
        }
    }

    pub fn denial(&self) -> WorthUiOperationLiveAdmissionDenial {
        self.denial
    }

    pub fn into_resource(self) -> WorthUiOperationLiveResource {
        *self.resource
    }
}

impl std::fmt::Debug for WorthUiOperationLiveAdmissionStop {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthUiOperationLiveAdmissionStop")
            .field("denial", &self.denial)
            .field("resource", &"returned exact operation-live resource")
            .finish()
    }
}
