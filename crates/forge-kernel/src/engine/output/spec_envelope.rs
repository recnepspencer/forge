//! Spec envelope — kernel output backed by `SpecState` plus lazy B-Rep projection.
//!
//! DOMAIN: Transitional output type for the spec-graph migration. Owns truth
//! (`SpecState`) and geometry, and materializes `ProjectedTopology` lazily on
//! demand. This does not replace `SolidEnvelope` yet.

use std::cell::OnceCell;

use serde::{Deserialize, Serialize};

use forge_core::KernelError;
use forge_spec::facade::SpecState;
use forge_topo::projection::{
    ProjectedBodyId, ProjectedEdgeId, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedLoopId,
    ProjectedLumpId, ProjectedRegionId, ProjectedShellId, ProjectedTopology, ProjectedTopologyError,
    ProjectedVertexId, ProjectionBuilder, compute_projected_topology_hash,
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
            .map_err(projected_topology_error_to_kernel)
    }

    pub fn bodies(&self) -> Result<&[ProjectedBodyId], KernelError> {
        projected_ids(&self.bodies, self.projection(), |projection| projection.body_count(), ProjectedBodyId::new)
    }

    pub fn lumps(&self) -> Result<&[ProjectedLumpId], KernelError> {
        projected_ids(&self.lumps, self.projection(), |projection| projection.lump_count(), ProjectedLumpId::new)
    }

    pub fn regions(&self) -> Result<&[ProjectedRegionId], KernelError> {
        projected_ids(
            &self.regions,
            self.projection(),
            |projection| projection.region_count(),
            ProjectedRegionId::new,
        )
    }

    pub fn shells(&self) -> Result<&[ProjectedShellId], KernelError> {
        projected_ids(
            &self.shells,
            self.projection(),
            |projection| projection.shell_count(),
            ProjectedShellId::new,
        )
    }

    pub fn faces(&self) -> Result<&[ProjectedFaceId], KernelError> {
        projected_ids(&self.faces, self.projection(), |projection| projection.face_count(), ProjectedFaceId::new)
    }

    pub fn loops(&self) -> Result<&[ProjectedLoopId], KernelError> {
        projected_ids(&self.loops, self.projection(), |projection| projection.loop_count(), ProjectedLoopId::new)
    }

    pub fn half_edges(&self) -> Result<&[ProjectedHalfEdgeId], KernelError> {
        projected_ids(
            &self.half_edges,
            self.projection(),
            |projection| projection.half_edge_count(),
            ProjectedHalfEdgeId::new,
        )
    }

    pub fn edges(&self) -> Result<&[ProjectedEdgeId], KernelError> {
        projected_ids(&self.edges, self.projection(), |projection| projection.edge_count(), ProjectedEdgeId::new)
    }

    pub fn vertices(&self) -> Result<&[ProjectedVertexId], KernelError> {
        projected_ids(
            &self.vertices,
            self.projection(),
            |projection| projection.vertex_count(),
            ProjectedVertexId::new,
        )
    }

    pub fn body_count(&self) -> Result<usize, KernelError> {
        Ok(self.bodies()?.len())
    }

    pub fn shell_count(&self) -> Result<usize, KernelError> {
        Ok(self.shells()?.len())
    }

    pub fn face_count(&self) -> Result<usize, KernelError> {
        Ok(self.faces()?.len())
    }

    pub fn vertex_count(&self) -> Result<usize, KernelError> {
        Ok(self.vertices()?.len())
    }

    pub fn edge_count(&self) -> Result<usize, KernelError> {
        Ok(self.edges()?.len())
    }

    pub fn body(&self) -> Result<ProjectedBodyId, KernelError> {
        let bodies = self.bodies()?;
        if bodies.len() != 1 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "SpecEnvelope::body() requires exactly 1 body, found {}",
                    bodies.len()
                ),
                context: None,
            });
        }
        Ok(bodies[0])
    }

    pub fn shell(&self) -> Result<ProjectedShellId, KernelError> {
        let shells = self.shells()?;
        if shells.len() != 1 {
            return Err(KernelError::InvalidInput {
                message: format!(
                    "SpecEnvelope::shell() requires exactly 1 shell, found {}",
                    shells.len()
                ),
                context: None,
            });
        }
        Ok(shells[0])
    }

    pub fn spec_fingerprint(&self) -> u128 {
        self.spec.spec_hash()
    }

    pub fn projection_fingerprint(&self) -> Result<u128, KernelError> {
        Ok(compute_projected_topology_hash(self.projection()?))
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
        .map_err(projected_topology_error_to_kernel)
}

fn projected_topology_error_to_kernel(error: &ProjectedTopologyError) -> KernelError {
    KernelError::InvalidInput {
        message: format!("Spec projection failed: {}", error),
        context: None,
    }
}
