use worth_query::facade::domain::WorthQueryDomainRebindReceipt;
pub use worth_query::facade::domain::{
    WorthQueryDomainRebindDenial as WorthUiQueryDomainRebindDenial,
    WorthQueryDomainRebindDenialKind as WorthUiQueryDomainRebindDenialKind,
    WorthQueryDomainRebindNextAction as WorthUiQueryDomainRebindNextAction,
};
use worth_query::facade::runtime::WorthQueryWorkspace;

use super::WorthUiInstalledQueryDomain;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryDomainRebindReceipt {
    prior: WorthUiInstalledQueryDomain,
    current: WorthUiInstalledQueryDomain,
    query_receipt: WorthQueryDomainRebindReceipt,
}

impl WorthUiInstalledQueryDomain {
    pub fn rebind_to(
        &self,
        workspace: &WorthQueryWorkspace,
    ) -> Result<WorthUiQueryDomainRebindReceipt, Box<WorthUiQueryDomainRebindDenial>> {
        let rebound = workspace.rebind_domain(self.handle().rebind_request())?;
        let query_receipt = rebound.receipt().clone();
        Ok(WorthUiQueryDomainRebindReceipt {
            prior: self.clone(),
            current: WorthUiInstalledQueryDomain::from_handle(rebound.into_handle()),
            query_receipt,
        })
    }
}

impl WorthUiQueryDomainRebindReceipt {
    pub fn prior(&self) -> &WorthUiInstalledQueryDomain {
        &self.prior
    }

    pub fn current(&self) -> &WorthUiInstalledQueryDomain {
        &self.current
    }

    pub fn into_current(self) -> WorthUiInstalledQueryDomain {
        self.current
    }

    pub fn query_receipt(&self) -> &WorthQueryDomainRebindReceipt {
        &self.query_receipt
    }
}
