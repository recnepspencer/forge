use super::construction::mint_loop_identity_boundary;
use super::counters::PlanarBooleanLoopIdentityMintingCounters;
use super::denial::PlanarBooleanLoopIdentityMintingDenial;
use super::input::PlanarBooleanLoopIdentityMintingInput;
use super::row::{
    PlanarBooleanLoopIdentityRow, PlanarBooleanLoopPersistentNamePropagationRow,
    PlanarBooleanLoopSubshapeSignatureRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopIdentityMap {
    map_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanLoopIdentityRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopPersistentNamePropagationMap {
    map_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanLoopPersistentNamePropagationRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopSubshapeSignatureMap {
    map_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanLoopSubshapeSignatureRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopIdentityBoundary {
    loop_identity_map: PlanarBooleanLoopIdentityMap,
    persistent_name_propagation_map: PlanarBooleanLoopPersistentNamePropagationMap,
    subshape_signature_map: PlanarBooleanLoopSubshapeSignatureMap,
    counters: PlanarBooleanLoopIdentityMintingCounters,
}

impl PlanarBooleanLoopIdentityBoundary {
    pub fn mint(
        input: PlanarBooleanLoopIdentityMintingInput<'_>,
    ) -> Result<Self, PlanarBooleanLoopIdentityMintingDenial> {
        mint_loop_identity_boundary(input)
    }

    pub(crate) fn new(
        loop_identity_map: PlanarBooleanLoopIdentityMap,
        persistent_name_propagation_map: PlanarBooleanLoopPersistentNamePropagationMap,
        subshape_signature_map: PlanarBooleanLoopSubshapeSignatureMap,
        counters: PlanarBooleanLoopIdentityMintingCounters,
    ) -> Self {
        Self {
            loop_identity_map,
            persistent_name_propagation_map,
            subshape_signature_map,
            counters,
        }
    }

    pub fn loop_identity_map(&self) -> &PlanarBooleanLoopIdentityMap {
        &self.loop_identity_map
    }

    pub fn persistent_name_propagation_map(
        &self,
    ) -> &PlanarBooleanLoopPersistentNamePropagationMap {
        &self.persistent_name_propagation_map
    }

    pub fn subshape_signature_map(&self) -> &PlanarBooleanLoopSubshapeSignatureMap {
        &self.subshape_signature_map
    }

    pub fn counters(&self) -> PlanarBooleanLoopIdentityMintingCounters {
        self.counters
    }
}

impl PlanarBooleanLoopIdentityMap {
    pub(crate) fn new(
        map_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanLoopIdentityRow>,
    ) -> Self {
        Self {
            map_identity,
            request_identity,
            rows,
        }
    }

    pub fn map_identity(&self) -> &str {
        &self.map_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopIdentityRow] {
        &self.rows
    }
}

impl PlanarBooleanLoopPersistentNamePropagationMap {
    pub(crate) fn new(
        map_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanLoopPersistentNamePropagationRow>,
    ) -> Self {
        Self {
            map_identity,
            request_identity,
            rows,
        }
    }

    pub fn map_identity(&self) -> &str {
        &self.map_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopPersistentNamePropagationRow] {
        &self.rows
    }
}

impl PlanarBooleanLoopSubshapeSignatureMap {
    pub(crate) fn new(
        map_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanLoopSubshapeSignatureRow>,
    ) -> Self {
        Self {
            map_identity,
            request_identity,
            rows,
        }
    }

    pub fn map_identity(&self) -> &str {
        &self.map_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopSubshapeSignatureRow] {
        &self.rows
    }
}
