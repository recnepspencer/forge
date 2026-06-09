use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveGeometryIdentityBundle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeBindingSite {
    topology_edge_identity: String,
    persistent_name: Option<String>,
}

impl EdgeBindingSite {
    pub fn new(topology_edge_identity: impl Into<String>) -> Self {
        Self {
            topology_edge_identity: topology_edge_identity.into(),
            persistent_name: None,
        }
    }

    pub fn with_persistent_name(mut self, persistent_name: impl Into<String>) -> Self {
        self.persistent_name = Some(persistent_name.into());
        self
    }

    pub fn topology_edge_identity(&self) -> &str {
        &self.topology_edge_identity
    }

    pub fn persistent_name(&self) -> Option<&str> {
        self.persistent_name.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdgeCurveBindingSpec {
    site: EdgeBindingSite,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    geometry_identity: PrimitiveGeometryIdentityBundle,
}

impl EdgeCurveBindingSpec {
    pub fn new(
        site: EdgeBindingSite,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        geometry_identity: PrimitiveGeometryIdentityBundle,
    ) -> Self {
        Self {
            site,
            birth_contract,
            geometry_identity,
        }
    }

    pub fn site(&self) -> &EdgeBindingSite {
        &self.site
    }

    pub fn birth_contract(&self) -> PrimitiveConstructionBirthSynopsisContract {
        self.birth_contract
    }

    pub fn geometry_identity(&self) -> &PrimitiveGeometryIdentityBundle {
        &self.geometry_identity
    }
}
