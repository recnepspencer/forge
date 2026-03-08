//! Spec envelope — kernel output backed by `SpecState` plus lazy B-Rep projection.
//!
//! DOMAIN: Transitional output type for the spec-graph migration. Owns truth
//! (`SpecState`) and geometry, and materializes `ProjectedTopology` lazily on
//! demand. This does not replace `SolidEnvelope` yet.

mod queries;
mod validation;

use std::cell::OnceCell;

use serde::{Deserialize, Serialize};

use forge_core::KernelError;
use forge_spec::facade::SpecState;
use forge_topo::projection::{
    ProjectedBodyId, ProjectedEdgeId, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedLoopId,
    ProjectedLumpId, ProjectedRegionId, ProjectedShellId, ProjectedTopology,
    ProjectedTopologyError, ProjectedVertexId, ProjectionBuilder,
};

use crate::geometry::facade::GeometryStore;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecEnvelope {
    spec: SpecState,
    geometry: GeometryStore,
    #[serde(skip)]
    projection: OnceCell<Result<ProjectedTopology, ProjectedTopologyError>>,
    #[serde(skip)]
    bodies: OnceCell<Result<Vec<ProjectedBodyId>, ProjectedTopologyError>>,
    #[serde(skip)]
    lumps: OnceCell<Result<Vec<ProjectedLumpId>, ProjectedTopologyError>>,
    #[serde(skip)]
    regions: OnceCell<Result<Vec<ProjectedRegionId>, ProjectedTopologyError>>,
    #[serde(skip)]
    shells: OnceCell<Result<Vec<ProjectedShellId>, ProjectedTopologyError>>,
    #[serde(skip)]
    faces: OnceCell<Result<Vec<ProjectedFaceId>, ProjectedTopologyError>>,
    #[serde(skip)]
    loops: OnceCell<Result<Vec<ProjectedLoopId>, ProjectedTopologyError>>,
    #[serde(skip)]
    half_edges: OnceCell<Result<Vec<ProjectedHalfEdgeId>, ProjectedTopologyError>>,
    #[serde(skip)]
    edges: OnceCell<Result<Vec<ProjectedEdgeId>, ProjectedTopologyError>>,
    #[serde(skip)]
    vertices: OnceCell<Result<Vec<ProjectedVertexId>, ProjectedTopologyError>>,
}

impl SpecEnvelope {
    pub fn new(spec: SpecState, geometry: GeometryStore) -> Self {
        Self {
            spec,
            geometry,
            projection: OnceCell::new(),
            bodies: OnceCell::new(),
            lumps: OnceCell::new(),
            regions: OnceCell::new(),
            shells: OnceCell::new(),
            faces: OnceCell::new(),
            loops: OnceCell::new(),
            half_edges: OnceCell::new(),
            edges: OnceCell::new(),
            vertices: OnceCell::new(),
        }
    }

    pub fn from_spec(spec: SpecState) -> Self {
        Self::new(spec, GeometryStore::default())
    }

    pub fn spec(&self) -> &SpecState {
        &self.spec
    }

    pub fn geometry(&self) -> &GeometryStore {
        &self.geometry
    }

    pub fn geometry_mut(&mut self) -> &mut GeometryStore {
        &mut self.geometry
    }

    pub fn projection(&self) -> Result<&ProjectedTopology, KernelError> {
        self.projection
            .get_or_init(|| ProjectionBuilder::build(&self.spec))
            .as_ref()
            .map_err(projected_topology_error_to_kernel_ref)
    }

    pub fn into_parts(self) -> (SpecState, GeometryStore) {
        (self.spec, self.geometry)
    }
}

fn projected_ids<'a, T, FCount, FId>(
    cell: &'a OnceCell<Result<Vec<T>, ProjectedTopologyError>>,
    projection: Result<&ProjectedTopology, KernelError>,
    count: FCount,
    make_id: FId,
) -> Result<&'a [T], KernelError>
where
    T: Copy,
    FCount: Fn(&ProjectedTopology) -> usize,
    FId: Fn(u32) -> T,
{
    let projection = projection?;
    cell.get_or_init(|| Ok((0..count(projection) as u32).map(make_id).collect()))
        .as_ref()
        .map(|ids| ids.as_slice())
        .map_err(projected_topology_error_to_kernel_ref)
}

fn projected_topology_error_to_kernel(error: ProjectedTopologyError) -> KernelError {
    KernelError::InvalidInput {
        message: format!("Spec projection failed: {}", error),
        context: None,
    }
}

fn projected_topology_error_to_kernel_ref(error: &ProjectedTopologyError) -> KernelError {
    KernelError::InvalidInput {
        message: format!("Spec projection failed: {}", error),
        context: None,
    }
}

fn projected_topology_error_to_kernel_owned(error: ProjectedTopologyError) -> KernelError {
    projected_topology_error_to_kernel(error)
}
