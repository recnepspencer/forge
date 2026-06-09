use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveGeometryIdentityBundle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoedgeBindingSite {
    topology_coedge_identity: String,
    persistent_name: Option<String>,
}

impl CoedgeBindingSite {
    pub fn new(topology_coedge_identity: impl Into<String>) -> Self {
        Self {
            topology_coedge_identity: topology_coedge_identity.into(),
            persistent_name: None,
        }
    }

    pub fn with_persistent_name(mut self, persistent_name: impl Into<String>) -> Self {
        self.persistent_name = Some(persistent_name.into());
        self
    }

    pub fn topology_coedge_identity(&self) -> &str {
        &self.topology_coedge_identity
    }

    pub fn persistent_name(&self) -> Option<&str> {
        self.persistent_name.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoedgePCurveBindingSpec {
    site: CoedgeBindingSite,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    geometry_identity: PrimitiveGeometryIdentityBundle,
}

impl CoedgePCurveBindingSpec {
    pub fn new(
        site: CoedgeBindingSite,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        geometry_identity: PrimitiveGeometryIdentityBundle,
    ) -> Self {
        Self {
            site,
            birth_contract,
            geometry_identity,
        }
    }

    pub fn site(&self) -> &CoedgeBindingSite {
        &self.site
    }

    pub fn birth_contract(&self) -> PrimitiveConstructionBirthSynopsisContract {
        self.birth_contract
    }

    pub fn geometry_identity(&self) -> &PrimitiveGeometryIdentityBundle {
        &self.geometry_identity
    }
}
