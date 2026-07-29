use worth_query_host::facade::admission::authenticated_principal::WorthQueryRequestScope;
use worth_query_host::facade::primary_graph::{
    WorthQueryLiveDeliveryControlDenial, WorthQueryLiveDeliveryControls,
};

use crate::{BankReadControlDenial, BankReadControls};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankLiveControlDenial {
    Read(BankReadControlDenial),
    Delivery(WorthQueryLiveDeliveryControlDenial),
}

#[derive(Clone)]
pub struct BankLiveControls {
    read: BankReadControls,
    delivery: WorthQueryLiveDeliveryControls,
}

impl BankLiveControls {
    pub fn current(
        request: WorthQueryRequestScope,
        maximum_results: usize,
        buffer_capacity: usize,
    ) -> Result<Self, BankLiveControlDenial> {
        let read = BankReadControls::current(request.clone(), maximum_results)
            .map_err(BankLiveControlDenial::Read)?;
        let delivery = WorthQueryLiveDeliveryControls::bounded(request, buffer_capacity)
            .map_err(BankLiveControlDenial::Delivery)?;
        Ok(Self { read, delivery })
    }

    pub(crate) const fn read(&self) -> &BankReadControls {
        &self.read
    }

    pub(crate) const fn delivery(&self) -> &WorthQueryLiveDeliveryControls {
        &self.delivery
    }
}
