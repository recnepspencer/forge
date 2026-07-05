use super::classification::build_identity_lineage_bundle;
use super::counters::PlanarBooleanOverlapRegionIdentityLineageCounters;
use super::denial::PlanarBooleanOverlapRegionIdentityLineageDenial;
use super::input::PlanarBooleanOverlapRegionIdentityLineageInput;
use super::rows::{
    PlanarBooleanOverlapRegionIdentityRow, PlanarBooleanOverlapRegionPersistentNamePropagationRow,
    PlanarBooleanOverlapRegionSubshapeSignatureRow,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionLedgerAssemblyBundle, PlanarBooleanOverlapRegionLedgerAssemblyDenial,
    PlanarBooleanPostAdmissionNormalizationBundle,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionIdentityMap {
    map_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanOverlapRegionIdentityRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionPersistentNamePropagationMap {
    map_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanOverlapRegionPersistentNamePropagationRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionSubshapeSignatureMap {
    map_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanOverlapRegionSubshapeSignatureRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionIdentityLineageBundle {
    bundle_identity: String,
    overlap_region_identity_map: PlanarBooleanOverlapRegionIdentityMap,
    persistent_name_propagation_map: PlanarBooleanOverlapRegionPersistentNamePropagationMap,
    subshape_signature_map: PlanarBooleanOverlapRegionSubshapeSignatureMap,
    source_post_admission_normalization: PlanarBooleanPostAdmissionNormalizationBundle,
    counters: PlanarBooleanOverlapRegionIdentityLineageCounters,
}

macro_rules! impl_identity_product {
    ($name:ident, $row:ty) => {
        impl $name {
            pub fn map_identity(&self) -> &str { &self.map_identity }
            pub fn request_identity(&self) -> &str { &self.request_identity }
            pub fn rows(&self) -> &[$row] { &self.rows }
        }
    };
}

impl_identity_product!(PlanarBooleanOverlapRegionIdentityMap, PlanarBooleanOverlapRegionIdentityRow);

impl PlanarBooleanOverlapRegionIdentityMap {
    pub(crate) fn new(
        map_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanOverlapRegionIdentityRow>,
    ) -> Self {
        Self {
            map_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            rows,
        }
    }

    pub fn arrangement_graph_identity(&self) -> &str { &self.arrangement_graph_identity }
    pub fn cell_set_identity(&self) -> &str { &self.cell_set_identity }
    pub fn ordering_basis_identity(&self) -> &str { &self.ordering_basis_identity }
}

impl_identity_product!(
    PlanarBooleanOverlapRegionPersistentNamePropagationMap,
    PlanarBooleanOverlapRegionPersistentNamePropagationRow
);

impl PlanarBooleanOverlapRegionPersistentNamePropagationMap {
    pub(crate) fn new(
        map_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanOverlapRegionPersistentNamePropagationRow>,
    ) -> Self {
        Self {
            map_identity,
            request_identity,
            rows,
        }
    }
}

impl_identity_product!(
    PlanarBooleanOverlapRegionSubshapeSignatureMap,
    PlanarBooleanOverlapRegionSubshapeSignatureRow
);

impl PlanarBooleanOverlapRegionSubshapeSignatureMap {
    pub(crate) fn new(
        map_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanOverlapRegionSubshapeSignatureRow>,
    ) -> Self {
        Self {
            map_identity,
            request_identity,
            rows,
        }
    }
}

impl PlanarBooleanOverlapRegionIdentityLineageBundle {
    pub fn from_post_admission_normalization(
        post_admission_normalization: &PlanarBooleanPostAdmissionNormalizationBundle,
    ) -> Result<Self, PlanarBooleanOverlapRegionIdentityLineageDenial> {
        Self::admit(PlanarBooleanOverlapRegionIdentityLineageInput::from_post_admission_normalization(
            post_admission_normalization,
        ))
    }

    pub fn admit(
        input: PlanarBooleanOverlapRegionIdentityLineageInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapRegionIdentityLineageDenial> {
        build_identity_lineage_bundle(input)
    }

    pub(crate) fn new(
        bundle_identity: String,
        overlap_region_identity_map: PlanarBooleanOverlapRegionIdentityMap,
        persistent_name_propagation_map: PlanarBooleanOverlapRegionPersistentNamePropagationMap,
        subshape_signature_map: PlanarBooleanOverlapRegionSubshapeSignatureMap,
        source_post_admission_normalization: PlanarBooleanPostAdmissionNormalizationBundle,
        counters: PlanarBooleanOverlapRegionIdentityLineageCounters,
    ) -> Self {
        Self {
            bundle_identity,
            overlap_region_identity_map,
            persistent_name_propagation_map,
            subshape_signature_map,
            source_post_admission_normalization,
            counters,
        }
    }

    pub fn overlap_region_identity_map(&self) -> &PlanarBooleanOverlapRegionIdentityMap {
        &self.overlap_region_identity_map
    }

    pub fn persistent_name_propagation_map(
        &self,
    ) -> &PlanarBooleanOverlapRegionPersistentNamePropagationMap {
        &self.persistent_name_propagation_map
    }

    pub fn subshape_signature_map(&self) -> &PlanarBooleanOverlapRegionSubshapeSignatureMap {
        &self.subshape_signature_map
    }

    pub(crate) fn source_post_admission_normalization(
        &self,
    ) -> &PlanarBooleanPostAdmissionNormalizationBundle {
        &self.source_post_admission_normalization
    }

    pub fn mint_overlap_region_ledger(
        &self,
    ) -> Result<
        PlanarBooleanOverlapRegionLedgerAssemblyBundle,
        PlanarBooleanOverlapRegionLedgerAssemblyDenial,
    > {
        PlanarBooleanOverlapRegionLedgerAssemblyBundle::from_identity_lineage(self)
    }

    pub fn counters(&self) -> PlanarBooleanOverlapRegionIdentityLineageCounters {
        self.counters
    }
}
