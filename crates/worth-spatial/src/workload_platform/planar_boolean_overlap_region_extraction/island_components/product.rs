use super::candidates::build_island_candidate_set;
use super::counters::PlanarBooleanOverlapIslandComponentCounters;
use super::denial::PlanarBooleanOverlapIslandComponentDenial;
use super::input::PlanarBooleanOverlapIslandCandidateInput;
use super::partition::build_island_partition;
use super::rows::{
    PlanarBooleanAreaOverlapComponentRow, PlanarBooleanBoundaryContactComponentRow,
    PlanarBooleanOverlapIslandCandidateRow, PlanarBooleanOverlapIslandRow,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanBoundaryContactClassificationBundle,
    PlanarBooleanBoundaryContactClassificationDenial,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapIslandCandidateSet {
    candidate_set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanOverlapIslandCandidateRow>,
    counters: PlanarBooleanOverlapIslandComponentCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapIslandSet {
    island_set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanOverlapIslandRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanBoundaryContactComponentSet {
    component_set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanBoundaryContactComponentRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanAreaOverlapComponentSet {
    component_set_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    rows: Vec<PlanarBooleanAreaOverlapComponentRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapIslandPartition {
    partition_identity: String,
    request_identity: String,
    arrangement_graph_identity: String,
    cell_set_identity: String,
    ordering_basis_identity: String,
    overlap_islands: PlanarBooleanOverlapIslandSet,
    boundary_contact_components: PlanarBooleanBoundaryContactComponentSet,
    area_overlap_components: PlanarBooleanAreaOverlapComponentSet,
    counters: PlanarBooleanOverlapIslandComponentCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapIslandComponentBundle {
    bundle_identity: String,
    island_candidates: PlanarBooleanOverlapIslandCandidateSet,
    island_partition: PlanarBooleanOverlapIslandPartition,
}

impl PlanarBooleanOverlapIslandCandidateSet {
    pub fn admit(
        input: PlanarBooleanOverlapIslandCandidateInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapIslandComponentDenial> {
        build_island_candidate_set(input)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        candidate_set_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanOverlapIslandCandidateRow>,
        counters: PlanarBooleanOverlapIslandComponentCounters,
    ) -> Self {
        Self {
            candidate_set_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            rows,
            counters,
        }
    }

    pub fn candidate_set_identity(&self) -> &str {
        &self.candidate_set_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn arrangement_graph_identity(&self) -> &str {
        &self.arrangement_graph_identity
    }

    pub fn cell_set_identity(&self) -> &str {
        &self.cell_set_identity
    }

    pub fn ordering_basis_identity(&self) -> &str {
        &self.ordering_basis_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanOverlapIslandCandidateRow] {
        &self.rows
    }

    pub fn counters(&self) -> PlanarBooleanOverlapIslandComponentCounters {
        self.counters
    }
}

impl PlanarBooleanOverlapIslandSet {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        island_set_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanOverlapIslandRow>,
    ) -> Self {
        Self {
            island_set_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            rows,
        }
    }

    pub fn rows(&self) -> &[PlanarBooleanOverlapIslandRow] {
        &self.rows
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn arrangement_graph_identity(&self) -> &str {
        &self.arrangement_graph_identity
    }

    pub fn cell_set_identity(&self) -> &str {
        &self.cell_set_identity
    }

    pub fn ordering_basis_identity(&self) -> &str {
        &self.ordering_basis_identity
    }
}

impl PlanarBooleanBoundaryContactComponentSet {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        component_set_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanBoundaryContactComponentRow>,
    ) -> Self {
        Self {
            component_set_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            rows,
        }
    }

    pub fn rows(&self) -> &[PlanarBooleanBoundaryContactComponentRow] {
        &self.rows
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn arrangement_graph_identity(&self) -> &str {
        &self.arrangement_graph_identity
    }

    pub fn cell_set_identity(&self) -> &str {
        &self.cell_set_identity
    }

    pub fn ordering_basis_identity(&self) -> &str {
        &self.ordering_basis_identity
    }
}

impl PlanarBooleanAreaOverlapComponentSet {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        component_set_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        rows: Vec<PlanarBooleanAreaOverlapComponentRow>,
    ) -> Self {
        Self {
            component_set_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            rows,
        }
    }

    pub fn rows(&self) -> &[PlanarBooleanAreaOverlapComponentRow] {
        &self.rows
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn arrangement_graph_identity(&self) -> &str {
        &self.arrangement_graph_identity
    }

    pub fn cell_set_identity(&self) -> &str {
        &self.cell_set_identity
    }

    pub fn ordering_basis_identity(&self) -> &str {
        &self.ordering_basis_identity
    }
}

impl PlanarBooleanOverlapIslandPartition {
    pub fn admit(
        island_candidates: &PlanarBooleanOverlapIslandCandidateSet,
    ) -> Result<Self, PlanarBooleanOverlapIslandComponentDenial> {
        build_island_partition(island_candidates)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        partition_identity: String,
        request_identity: String,
        arrangement_graph_identity: String,
        cell_set_identity: String,
        ordering_basis_identity: String,
        overlap_islands: PlanarBooleanOverlapIslandSet,
        boundary_contact_components: PlanarBooleanBoundaryContactComponentSet,
        area_overlap_components: PlanarBooleanAreaOverlapComponentSet,
        counters: PlanarBooleanOverlapIslandComponentCounters,
    ) -> Self {
        Self {
            partition_identity,
            request_identity,
            arrangement_graph_identity,
            cell_set_identity,
            ordering_basis_identity,
            overlap_islands,
            boundary_contact_components,
            area_overlap_components,
            counters,
        }
    }

    pub fn overlap_islands(&self) -> &PlanarBooleanOverlapIslandSet {
        &self.overlap_islands
    }

    pub fn partition_identity(&self) -> &str {
        &self.partition_identity
    }

    pub fn boundary_contact_components(&self) -> &PlanarBooleanBoundaryContactComponentSet {
        &self.boundary_contact_components
    }

    pub fn area_overlap_components(&self) -> &PlanarBooleanAreaOverlapComponentSet {
        &self.area_overlap_components
    }

    pub fn counters(&self) -> PlanarBooleanOverlapIslandComponentCounters {
        self.counters
    }
}

impl PlanarBooleanOverlapIslandComponentBundle {
    pub fn admit(
        input: PlanarBooleanOverlapIslandCandidateInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapIslandComponentDenial> {
        let island_candidates = PlanarBooleanOverlapIslandCandidateSet::admit(input)?;
        let island_partition = PlanarBooleanOverlapIslandPartition::admit(&island_candidates)?;
        Ok(Self {
            bundle_identity: format!(
                "overlap-island-components:{}:{}",
                island_candidates.candidate_set_identity(),
                island_partition.partition_identity()
            ),
            island_candidates,
            island_partition,
        })
    }

    pub fn bundle_identity(&self) -> &str {
        &self.bundle_identity
    }

    pub fn island_candidates(&self) -> &PlanarBooleanOverlapIslandCandidateSet {
        &self.island_candidates
    }

    pub fn island_partition(&self) -> &PlanarBooleanOverlapIslandPartition {
        &self.island_partition
    }

    pub fn overlap_islands(&self) -> &PlanarBooleanOverlapIslandSet {
        self.island_partition.overlap_islands()
    }

    pub fn boundary_contact_components(&self) -> &PlanarBooleanBoundaryContactComponentSet {
        self.island_partition.boundary_contact_components()
    }

    pub fn area_overlap_components(&self) -> &PlanarBooleanAreaOverlapComponentSet {
        self.island_partition.area_overlap_components()
    }

    pub fn counters(&self) -> PlanarBooleanOverlapIslandComponentCounters {
        self.island_partition.counters()
    }

    pub fn classify_boundary_contact_components(
        &self,
    ) -> Result<
        PlanarBooleanBoundaryContactClassificationBundle,
        PlanarBooleanBoundaryContactClassificationDenial,
    > {
        PlanarBooleanBoundaryContactClassificationBundle::admit(
            crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanBoundaryContactClassificationInput::from_island_component_bundle(self),
        )
    }
}
