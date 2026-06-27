#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupProductSeparationProof;

impl EvidenceLookupProductSeparationProof {
    pub(crate) const fn admission_only() -> Self {
        Self
    }

    pub const fn claims_lookup_product_construction(&self) -> bool {
        false
    }

    pub const fn claims_lookup_execution(&self) -> bool {
        false
    }

    pub const fn claims_query_descriptor_authority(&self) -> bool {
        false
    }

    pub const fn claims_topology_product_authority(&self) -> bool {
        false
    }
}
