use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_m6_closeout::authoring::{
    m6_planar_closeout_entry, M6PlanarCloseoutCase, M6PlanarCloseoutEntry,
};
use crate::bindings::query_native_planar_m6_closeout::domain::M6PlanarCloseoutQueryDomain;
use crate::bindings::query_native_planar_m6_closeout::facts::{
    m6_planar_closeout_facts, M6PlanarCloseoutFactError,
};
use crate::planar_contracts::m6_closeout::{
    M6PlanarCloseoutCertification, M6PlanarCloseoutDenial, M6PlanarCloseoutReceipt,
};

pub struct M6PlanarCloseoutContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<M6PlanarCloseoutQueryDomain>,
{
    closeout_handle: ForgeQueryAdmittedConfiguredDomainHandle<M6PlanarCloseoutQueryDomain, WC>,
}

impl<WC> M6PlanarCloseoutContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<M6PlanarCloseoutQueryDomain>,
{
    pub fn new(
        closeout_handle: ForgeQueryAdmittedConfiguredDomainHandle<M6PlanarCloseoutQueryDomain, WC>,
    ) -> Self {
        Self { closeout_handle }
    }
}

pub struct M6PlanarCloseoutQueryCertification {
    certification: M6PlanarCloseoutCertification,
}

impl M6PlanarCloseoutQueryCertification {
    pub fn from_certification(certification: M6PlanarCloseoutCertification) -> Self {
        Self { certification }
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a M6PlanarCloseoutContracts<WC>,
    ) -> Result<M6PlanarCloseoutPlan<'a, WC>, M6PlanarCloseoutDenial>
    where
        WC: ForgeQueryDomainOperatingContext<M6PlanarCloseoutQueryDomain>,
    {
        let basis = self.certification.build()?;
        Ok(M6PlanarCloseoutPlan {
            entry: m6_planar_closeout_entry(M6PlanarCloseoutCase::from_basis(basis)),
            contracts,
        })
    }
}

pub struct M6PlanarCloseoutPlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<M6PlanarCloseoutQueryDomain>,
{
    entry: M6PlanarCloseoutEntry,
    contracts: &'a M6PlanarCloseoutContracts<WC>,
}

impl<WC> M6PlanarCloseoutPlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<M6PlanarCloseoutQueryDomain>,
{
    pub fn inspected_closeout_rows(&self) -> usize {
        self.entry.case().basis().closeout_rows()
    }

    pub fn certify(self) -> Result<M6PlanarCloseoutReceipt, M6PlanarCloseoutFactError> {
        m6_planar_closeout_facts(&self.entry, &self.contracts.closeout_handle)
    }
}
