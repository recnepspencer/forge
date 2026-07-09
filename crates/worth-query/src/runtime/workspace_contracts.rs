use super::{
    WorthQueryLiveView, WorthQueryRuntimeDownstreamDelivery,
    WorthQueryRuntimeDownstreamDeliveryContract, WorthQueryRuntimeError, WorthQueryWorkspace,
};

impl WorthQueryWorkspace {
    pub fn public_downstream_delivery_contract(
        &self,
    ) -> WorthQueryRuntimeDownstreamDeliveryContract {
        self.runtime.public_downstream_delivery_contract()
    }

    pub fn downstream_delivery<T>(
        &self,
        view: &WorthQueryLiveView<T>,
    ) -> Result<Option<WorthQueryRuntimeDownstreamDelivery>, WorthQueryRuntimeError> {
        self.runtime.downstream_delivery(view)
    }
}
