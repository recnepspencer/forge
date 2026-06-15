#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedTopologyLoopBasis2D {
    loop_topology_identity: String,
    loop_membership_fact_digest: String,
    topology_to_spatial_contract_digest: String,
}

impl CertifiedTopologyLoopBasis2D {
    pub fn from_topology_loop_fact(
        loop_topology_identity: impl Into<String>,
        loop_membership_fact_digest: impl Into<String>,
        topology_to_spatial_contract_digest: impl Into<String>,
    ) -> Self {
        Self {
            loop_topology_identity: loop_topology_identity.into(),
            loop_membership_fact_digest: loop_membership_fact_digest.into(),
            topology_to_spatial_contract_digest: topology_to_spatial_contract_digest.into(),
        }
    }

    pub fn loop_topology_identity(&self) -> &str {
        &self.loop_topology_identity
    }

    pub fn loop_membership_fact_digest(&self) -> &str {
        &self.loop_membership_fact_digest
    }

    pub fn topology_to_spatial_contract_digest(&self) -> &str {
        &self.topology_to_spatial_contract_digest
    }
}
