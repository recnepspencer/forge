use super::{
    ForgeQueryLiveView, ForgeQueryRuntimeDownstreamDelivery,
    ForgeQueryRuntimeDownstreamDeliveryContract, ForgeQueryRuntimeError, ForgeQueryWorkspace,
};

impl ForgeQueryWorkspace {
    pub fn public_downstream_delivery_contract(
        &self,
    ) -> ForgeQueryRuntimeDownstreamDeliveryContract {
        self.runtime.public_downstream_delivery_contract()
    }

    pub fn downstream_delivery<T>(
        &self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<Option<ForgeQueryRuntimeDownstreamDelivery>, ForgeQueryRuntimeError> {
        self.runtime.downstream_delivery(view)
    }
}
