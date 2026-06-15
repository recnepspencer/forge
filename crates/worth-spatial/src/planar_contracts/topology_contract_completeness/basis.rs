use topology::facade::{
    TopologyConstructionQueryFactKind, TopologyPrimitiveConstructionQueryReceipt,
};

use super::validation::validate_planar_topology_contract_completeness_basis;
use super::PlanarTopologyContractCompletenessDenial;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarTopologyContractCompletenessBasis {
    topology_query_receipt: TopologyPrimitiveConstructionQueryReceipt,
    declared_query_surface_identity: String,
    planar_neighborhood_identity: String,
}

impl PlanarTopologyContractCompletenessBasis {
    pub fn builder() -> PlanarTopologyContractCompletenessBuilder {
        PlanarTopologyContractCompletenessBuilder::default()
    }

    pub(crate) fn from_builder(
        builder: PlanarTopologyContractCompletenessBuilder,
    ) -> Result<Self, PlanarTopologyContractCompletenessDenial> {
        let basis = Self {
            topology_query_receipt: builder.topology_query_receipt.ok_or_else(|| {
                super::PlanarTopologyContractCompletenessDenial::new(
                    super::PlanarTopologyContractCompletenessDenialKind::MissingTopologyReceipt,
                    "topology completeness requires a Query-owned topology receipt",
                )
            })?,
            declared_query_surface_identity: builder.declared_query_surface_identity,
            planar_neighborhood_identity: builder.planar_neighborhood_identity,
        };
        validate_planar_topology_contract_completeness_basis(&basis)?;
        Ok(basis)
    }

    pub fn topology_query_receipt(&self) -> &TopologyPrimitiveConstructionQueryReceipt {
        &self.topology_query_receipt
    }

    pub fn declared_query_surface_identity(&self) -> &str {
        &self.declared_query_surface_identity
    }

    pub fn planar_neighborhood_identity(&self) -> &str {
        &self.planar_neighborhood_identity
    }

    pub fn topology_basis_identity(&self) -> &str {
        self.topology_query_receipt.source_birth_digest()
    }

    pub(crate) fn fact_count(&self, kind: TopologyConstructionQueryFactKind) -> usize {
        self.topology_query_receipt
            .row_for(kind)
            .map(|row| row.fact_count())
            .unwrap_or(0)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlanarTopologyContractCompletenessBuilder {
    topology_query_receipt: Option<TopologyPrimitiveConstructionQueryReceipt>,
    declared_query_surface_identity: String,
    planar_neighborhood_identity: String,
}

impl PlanarTopologyContractCompletenessBuilder {
    pub fn topology_query_receipt(
        mut self,
        receipt: TopologyPrimitiveConstructionQueryReceipt,
    ) -> Self {
        self.topology_query_receipt = Some(receipt);
        self
    }

    pub fn declared_query_surface(mut self, identity: impl Into<String>) -> Self {
        self.declared_query_surface_identity = identity.into();
        self
    }

    pub fn planar_neighborhood(mut self, identity: impl Into<String>) -> Self {
        self.planar_neighborhood_identity = identity.into();
        self
    }

    pub fn build(
        self,
    ) -> Result<PlanarTopologyContractCompletenessBasis, PlanarTopologyContractCompletenessDenial>
    {
        PlanarTopologyContractCompletenessBasis::from_builder(self)
    }
}
