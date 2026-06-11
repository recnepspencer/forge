use worth_geom::facade::{Plane, PrimitiveRealizationReport};
use worth_primitives::{
    truth_digest_parts, PrimitiveConstructionBirthSynopsisContract, PrimitiveConstructionFamilyKey,
    PrimitiveGeometryIdentityBundle, PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity,
    TruthDigestScope,
};

use super::primitive_birth::PrimitiveConstructionBirthScaffoldInput;
use super::primitive_birth_placement::{
    primitive_construction_birth_placement_facts, realize_primitive_construction_birth_placement,
    PrimitiveConstructionBirthPlacementError, PrimitiveConstructionBirthPlacementIssue,
};
use crate::placement::SpatialPlacementSpec;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionBirthTopologyCounts {
    vertex_count: usize,
    edge_count: usize,
    loop_count: usize,
    wire_count: usize,
    face_count: usize,
    shell_count: usize,
    body_count: usize,
}

impl PrimitiveConstructionBirthTopologyCounts {
    pub fn new(
        vertex_count: usize,
        edge_count: usize,
        loop_count: usize,
        wire_count: usize,
        face_count: usize,
        shell_count: usize,
        body_count: usize,
    ) -> Self {
        Self {
            vertex_count,
            edge_count,
            loop_count,
            wire_count,
            face_count,
            shell_count,
            body_count,
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_count
    }
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
    pub fn loop_count(&self) -> usize {
        self.loop_count
    }
    pub fn wire_count(&self) -> usize {
        self.wire_count
    }
    pub fn face_count(&self) -> usize {
        self.face_count
    }
    pub fn shell_count(&self) -> usize {
        self.shell_count
    }
    pub fn body_count(&self) -> usize {
        self.body_count
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum PrimitiveConstructionBirthScaffoldRealization {
    SupportReport(PrimitiveRealizationReport),
    DirectPlanar { label: &'static str },
}

#[derive(Clone, Debug)]
pub struct PrimitiveConstructionBirthScaffoldMaterializationInput {
    family: PrimitiveConstructionFamilyKey,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    topology_birth_class: &'static str,
    intent_digest: String,
    placement_spec: SpatialPlacementSpec,
    support_planes: Vec<Plane>,
    local_vertices: Vec<[f64; 3]>,
    realization: PrimitiveConstructionBirthScaffoldRealization,
    topology_counts: PrimitiveConstructionBirthTopologyCounts,
}

impl PrimitiveConstructionBirthScaffoldMaterializationInput {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        family: PrimitiveConstructionFamilyKey,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        topology_birth_class: &'static str,
        intent_digest: String,
        placement_spec: SpatialPlacementSpec,
        support_planes: Vec<Plane>,
        local_vertices: Vec<[f64; 3]>,
        realization: PrimitiveConstructionBirthScaffoldRealization,
        topology_counts: PrimitiveConstructionBirthTopologyCounts,
    ) -> Self {
        Self {
            family,
            birth_contract,
            topology_birth_class,
            intent_digest,
            placement_spec,
            support_planes,
            local_vertices,
            realization,
            topology_counts,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_support_report(
        family: PrimitiveConstructionFamilyKey,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        topology_birth_class: &'static str,
        intent_digest: String,
        placement_spec: SpatialPlacementSpec,
        support_planes: Vec<Plane>,
        local_vertices: Vec<[f64; 3]>,
        realization_report: PrimitiveRealizationReport,
        topology_counts: PrimitiveConstructionBirthTopologyCounts,
    ) -> Self {
        Self::new(
            family,
            birth_contract,
            topology_birth_class,
            intent_digest,
            placement_spec,
            support_planes,
            local_vertices,
            PrimitiveConstructionBirthScaffoldRealization::SupportReport(realization_report),
            topology_counts,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_direct_planar(
        family: PrimitiveConstructionFamilyKey,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        topology_birth_class: &'static str,
        intent_digest: String,
        placement_spec: SpatialPlacementSpec,
        support_planes: Vec<Plane>,
        local_vertices: Vec<[f64; 3]>,
        label: &'static str,
        topology_counts: PrimitiveConstructionBirthTopologyCounts,
    ) -> Self {
        Self::new(
            family,
            birth_contract,
            topology_birth_class,
            intent_digest,
            placement_spec,
            support_planes,
            local_vertices,
            PrimitiveConstructionBirthScaffoldRealization::DirectPlanar { label },
            topology_counts,
        )
    }

    pub fn materialize_realization_geometry_digest(
        self,
    ) -> Result<String, SpatialConstructionBirthScaffoldMaterializationError> {
        Ok(
            materialize_primitive_construction_birth_scaffold_input(self)?
                .realization_geometry_digest()
                .to_string(),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpatialConstructionBirthScaffoldMaterializationError {
    InvalidPlacement(PrimitiveConstructionBirthPlacementIssue),
    PlacementGeometry(PrimitiveConstructionBirthPlacementIssue),
}

impl std::fmt::Display for SpatialConstructionBirthScaffoldMaterializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPlacement(error) | Self::PlacementGeometry(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for SpatialConstructionBirthScaffoldMaterializationError {}

impl From<PrimitiveConstructionBirthPlacementError>
    for SpatialConstructionBirthScaffoldMaterializationError
{
    fn from(error: PrimitiveConstructionBirthPlacementError) -> Self {
        match error {
            PrimitiveConstructionBirthPlacementError::InvalidPlacement(error) => {
                Self::InvalidPlacement(error)
            }
            PrimitiveConstructionBirthPlacementError::PlacementGeometry(error) => {
                Self::PlacementGeometry(error)
            }
        }
    }
}

pub fn materialize_primitive_construction_birth_scaffold_input(
    input: PrimitiveConstructionBirthScaffoldMaterializationInput,
) -> Result<
    PrimitiveConstructionBirthScaffoldInput,
    SpatialConstructionBirthScaffoldMaterializationError,
> {
    let PrimitiveConstructionBirthScaffoldMaterializationInput {
        family,
        birth_contract,
        topology_birth_class,
        intent_digest,
        placement_spec,
        support_planes,
        local_vertices,
        realization,
        topology_counts,
    } = input;

    let placement_facts = primitive_construction_birth_placement_facts(placement_spec.clone())
        .map_err(SpatialConstructionBirthScaffoldMaterializationError::InvalidPlacement)?;
    let (support_planes, vertex_positions) = realize_primitive_construction_birth_placement(
        placement_spec,
        &support_planes,
        &local_vertices,
    )
    .map_err(SpatialConstructionBirthScaffoldMaterializationError::from)?
    .into_parts();
    let realization =
        materialize_realization_facts(realization, &vertex_positions, &support_planes)
            .with_placement_facts(placement_facts);
    let geometry_identity = scaffold_geometry_identity(&support_planes, &vertex_positions);
    let scaffold_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            intent_digest,
            format!("family:{}", family.as_str()),
            format!("planes:{}", support_planes.len()),
            format!("vertices:{}", vertex_positions.len()),
            format!("edges:{}", topology_counts.edge_count()),
            format!("loops:{}", topology_counts.loop_count()),
            format!("wires:{}", topology_counts.wire_count()),
            format!("faces:{}", topology_counts.face_count()),
            format!("shells:{}", topology_counts.shell_count()),
            format!("bodies:{}", topology_counts.body_count()),
            format!(
                "scaffold-geometry:{}",
                geometry_identity.scaffold_geometry_digest().as_str()
            ),
            format!("realization:{}", realization.realization_fact_digest()),
        ],
    );

    Ok(
        PrimitiveConstructionBirthScaffoldInput::new_with_realization_facts_and_contract(
            family,
            birth_contract,
            topology_birth_class,
            scaffold_digest,
            support_planes.to_vec(),
            realization,
            vertex_positions.to_vec(),
            topology_counts.vertex_count(),
            topology_counts.edge_count(),
            topology_counts.loop_count(),
            topology_counts.wire_count(),
            topology_counts.face_count(),
            topology_counts.shell_count(),
            topology_counts.body_count(),
        ),
    )
}

fn materialize_realization_facts(
    realization: PrimitiveConstructionBirthScaffoldRealization,
    vertex_positions: &[[f64; 3]],
    support_planes: &[Plane],
) -> super::primitive_birth_runtime::PrimitiveConstructionBirthRealizationFacts {
    match realization {
        PrimitiveConstructionBirthScaffoldRealization::SupportReport(report) => {
            super::primitive_birth_runtime::PrimitiveConstructionBirthRealizationFacts::from_realization_report(report)
        }
        PrimitiveConstructionBirthScaffoldRealization::DirectPlanar { label } => {
            super::primitive_birth_runtime::PrimitiveConstructionBirthRealizationFacts::from_direct_planar_support(
                label,
                vertex_positions,
                support_planes,
            )
        }
    }
}

fn scaffold_geometry_identity(
    support_planes: &[Plane],
    vertex_positions: &[[f64; 3]],
) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(
        support_planes.iter().map(plane_identity).collect(),
        vertex_positions
            .iter()
            .copied()
            .map(PrimitiveVertexIdentity::from_position)
            .collect(),
    )
}

fn plane_identity(plane: &Plane) -> PrimitiveSupportPlaneIdentity {
    let (a, b, c, d) = plane.exact_coefficients();
    PrimitiveSupportPlaneIdentity::new(a.to_string(), b.to_string(), c.to_string(), d.to_string())
}
