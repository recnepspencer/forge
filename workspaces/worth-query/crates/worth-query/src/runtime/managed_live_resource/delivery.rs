use super::super::WorthQueryRuntimeDeliveryBatch;

pub(crate) struct WorthQueryManagedLiveRuntimeDelivery {
    resource_name: String,
    batches: Vec<WorthQueryRuntimeDeliveryBatch>,
}

impl WorthQueryManagedLiveRuntimeDelivery {
    pub(crate) fn new(
        resource_name: impl Into<String>,
        batches: Vec<WorthQueryRuntimeDeliveryBatch>,
    ) -> Self {
        Self {
            resource_name: resource_name.into(),
            batches,
        }
    }

    pub(crate) fn into_parts(self) -> (String, Vec<WorthQueryRuntimeDeliveryBatch>) {
        (self.resource_name, self.batches)
    }
}
