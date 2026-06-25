use forge_query::facade::ForgeQueryGraphTouchDescriptor;

use crate::topology_operators::{
    TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedGraphBasis, TopologyTouchedGraphCounters,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedInvalidationTouchedClosure {
    semantic_family_key: &'static str,
    basis: TopologyTouchedGraphBasis,
    touch_descriptor: ForgeQueryGraphTouchDescriptor,
    closure_digest: String,
}

impl DerivedInvalidationTouchedClosure {
    pub fn from_declared_touch(proof: &TopologyDeclaredTouchedGraphBasisProof) -> Self {
        let basis = proof.basis().clone();
        let touch_descriptor = proof.touch_descriptor().clone();
        let closure_digest = closure_digest(basis.digest(), touch_descriptor.descriptor_digest());
        Self {
            semantic_family_key: proof.semantic_family_key(),
            basis,
            touch_descriptor,
            closure_digest,
        }
    }

    pub fn semantic_family_key(&self) -> &'static str {
        self.semantic_family_key
    }

    pub fn basis(&self) -> &TopologyTouchedGraphBasis {
        &self.basis
    }

    pub fn basis_digest(&self) -> &str {
        self.basis.digest()
    }

    pub fn touch_descriptor(&self) -> &ForgeQueryGraphTouchDescriptor {
        &self.touch_descriptor
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        self.touch_descriptor.descriptor_digest()
    }

    pub const fn counters(&self) -> TopologyTouchedGraphCounters {
        self.basis.counters()
    }

    pub fn closure_digest(&self) -> &str {
        &self.closure_digest
    }
}

fn closure_digest(basis_digest: &str, touch_descriptor_digest: &str) -> String {
    super::super::catalog::catalog_digest([
        "worth-topo:derived-invalidation-touched-closure:v1".to_string(),
        format!("basis:{basis_digest}"),
        format!("query-touch:{touch_descriptor_digest}"),
    ])
}
