use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};
use topology::facade::TopologyPrimitiveConstructionQueryReceipt;

use crate::bindings::query_native_planar_topology_contract::authoring::{
    planar_topology_contract_completeness_entry, PlanarTopologyContractCompletenessCase,
    PlanarTopologyContractCompletenessEntry,
};
use crate::bindings::query_native_planar_topology_contract::domain::PlanarTopologyContractCompletenessQueryDomain;
use crate::bindings::query_native_planar_topology_contract::facts::{
    planar_topology_contract_completeness_facts, PlanarTopologyContractCompletenessFactError,
};
use crate::bindings::query_native_planar_topology_contract::inspection::PlanarTopologyContractCompletenessInspectionRow;
use crate::planar_contracts::topology_contract_completeness::{
    PlanarTopologyContractCompletenessBasis, PlanarTopologyContractCompletenessDenial,
    PlanarTopologyContractCompletenessReceipt,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlanarTopologyContractCompleteness {
    builder: crate::planar_contracts::topology_contract_completeness::PlanarTopologyContractCompletenessBuilder,
}

impl PlanarTopologyContractCompleteness {
    pub fn from_topology_query_receipt(receipt: TopologyPrimitiveConstructionQueryReceipt) -> Self {
        Self {
            builder: PlanarTopologyContractCompletenessBasis::builder()
                .topology_query_receipt(receipt),
        }
    }

    pub fn consume_declared_topology_surfaces(mut self, identity: impl Into<String>) -> Self {
        self.builder = self.builder.declared_query_surface(identity);
        self
    }

    pub fn within_planar_neighborhood(mut self, identity: impl Into<String>) -> Self {
        self.builder = self.builder.planar_neighborhood(identity);
        self
    }

    pub fn compile<'a, WC>(
        self,
        contracts: &'a PlanarTopologyContractCompletenessContracts<WC>,
    ) -> Result<
        PlanarTopologyContractCompletenessPlan<'a, WC>,
        PlanarTopologyContractCompletenessDenial,
    >
    where
        WC: ForgeQueryDomainOperatingContext<PlanarTopologyContractCompletenessQueryDomain>,
    {
        let basis = self.builder.build()?;
        let entry = planar_topology_contract_completeness_entry(
            PlanarTopologyContractCompletenessCase::from_topology_query_receipt(basis),
        );
        Ok(PlanarTopologyContractCompletenessPlan { entry, contracts })
    }
}

pub struct PlanarTopologyContractCompletenessContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarTopologyContractCompletenessQueryDomain>,
{
    handle:
        ForgeQueryAdmittedConfiguredDomainHandle<PlanarTopologyContractCompletenessQueryDomain, WC>,
}

impl<WC> PlanarTopologyContractCompletenessContracts<WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarTopologyContractCompletenessQueryDomain>,
{
    pub fn new(
        handle: ForgeQueryAdmittedConfiguredDomainHandle<
            PlanarTopologyContractCompletenessQueryDomain,
            WC,
        >,
    ) -> Self {
        Self { handle }
    }
}

pub struct PlanarTopologyContractCompletenessPlan<'a, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarTopologyContractCompletenessQueryDomain>,
{
    entry: PlanarTopologyContractCompletenessEntry,
    contracts: &'a PlanarTopologyContractCompletenessContracts<WC>,
}

impl<WC> PlanarTopologyContractCompletenessPlan<'_, WC>
where
    WC: ForgeQueryDomainOperatingContext<PlanarTopologyContractCompletenessQueryDomain>,
{
    pub fn inspected_topology_rows(&self) -> usize {
        PlanarTopologyContractCompletenessInspectionRow::from_basis(self.entry.case().basis()).len()
    }

    pub fn certify(
        self,
    ) -> Result<
        PlanarTopologyContractCompletenessReceipt,
        PlanarTopologyContractCompletenessFactError,
    > {
        planar_topology_contract_completeness_facts(&self.entry, &self.contracts.handle)
    }
}
