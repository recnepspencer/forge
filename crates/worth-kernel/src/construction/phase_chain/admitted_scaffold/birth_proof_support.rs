use crate::construction::digest::digest_owned_parts;
use crate::construction::request::PrimitiveConstructionFamily;
use worth_geom::facade::{tangent_frame, Plane, PrimitiveRealizationReport};
use worth_math::{canonical_perpendicular_unit_vector, FinitePoint3, UnitVector3};
use worth_spatial::facade::placement::SpatialPlacementSpec;
use worth_spatial::facade::refs::{SpatialAxis, SpatialDirectionWitnessRef, SpatialFrameRef};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionBirthPlacementFacts {
    origin: [f64; 3],
    facing_vector: [f64; 3],
}

impl PrimitiveConstructionBirthPlacementFacts {
    pub(crate) fn origin(&self) -> [f64; 3] {
        self.origin
    }

    pub(crate) fn facing_vector(&self) -> [f64; 3] {
        self.facing_vector
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionAdmittedBirthProofSupport {
    scaffold_digest: String,
    birth_digest: String,
    birth_completeness_digest: String,
    birth_mapping_digest: String,
    realization_fact_digest: String,
    realization_geometry_digest: String,
    placement_facts: PrimitiveConstructionBirthPlacementFacts,
}

impl PrimitiveConstructionAdmittedBirthProofSupport {
    pub(crate) fn scaffold_digest(&self) -> &str {
        &self.scaffold_digest
    }

    pub(crate) fn birth_digest(&self) -> &str {
        &self.birth_digest
    }

    pub(crate) fn birth_completeness_digest(&self) -> &str {
        &self.birth_completeness_digest
    }

    pub(crate) fn birth_mapping_digest(&self) -> &str {
        &self.birth_mapping_digest
    }

    pub(crate) fn realization_fact_digest(&self) -> &str {
        &self.realization_fact_digest
    }

    pub(crate) fn realization_geometry_digest(&self) -> &str {
        &self.realization_geometry_digest
    }

    pub(crate) fn placement_facts(&self) -> PrimitiveConstructionBirthPlacementFacts {
        self.placement_facts
    }
}

#[derive(Debug)]
pub(crate) struct PrimitiveConstructionBirthProofSupportError(String);

impl PrimitiveConstructionBirthProofSupportError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl std::fmt::Display for PrimitiveConstructionBirthProofSupportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for PrimitiveConstructionBirthProofSupportError {}

pub(crate) fn materialize_primitive_construction_birth_proof_support(
    family: PrimitiveConstructionFamily,
    topology_birth_class: &str,
    intent_digest: &str,
    placement_spec: SpatialPlacementSpec,
    support_planes: &[Plane],
    local_vertices: &[[f64; 3]],
    realization_report: PrimitiveRealizationReport,
    supported_vertex_count: usize,
    supported_edge_count: usize,
    supported_loop_count: usize,
    supported_wire_count: usize,
    supported_face_count: usize,
    supported_shell_count: usize,
    supported_body_count: usize,
) -> Result<
    PrimitiveConstructionAdmittedBirthProofSupport,
    PrimitiveConstructionBirthProofSupportError,
> {
    let placement_facts = primitive_construction_birth_placement_facts(placement_spec)?;
    let scaffold_digest = scaffold_digest(
        family,
        topology_birth_class,
        intent_digest,
        placement_facts,
        support_planes,
        local_vertices,
    );
    let birth_digest = digest_owned_parts(&[
        family.as_str().to_string(),
        topology_birth_class.to_string(),
        scaffold_digest.clone(),
    ]);
    let birth_completeness_digest = digest_owned_parts(&[
        birth_digest.clone(),
        supported_vertex_count.to_string(),
        supported_edge_count.to_string(),
        supported_loop_count.to_string(),
        supported_wire_count.to_string(),
        supported_face_count.to_string(),
        supported_shell_count.to_string(),
        supported_body_count.to_string(),
    ]);
    let realization_fact_digest = realization_report.report_digest().to_string();
    let realization_geometry_digest = realization_report.geometry_digest().to_string();
    let birth_mapping_digest = digest_owned_parts(&[
        birth_completeness_digest.clone(),
        realization_fact_digest.clone(),
        realization_geometry_digest.clone(),
    ]);
    Ok(PrimitiveConstructionAdmittedBirthProofSupport {
        scaffold_digest,
        birth_digest,
        birth_completeness_digest,
        birth_mapping_digest,
        realization_fact_digest,
        realization_geometry_digest,
        placement_facts,
    })
}

pub(crate) fn primitive_construction_birth_placement_facts(
    spec: SpatialPlacementSpec,
) -> Result<PrimitiveConstructionBirthPlacementFacts, PrimitiveConstructionBirthProofSupportError> {
    let frame = admit_frame(spec.reference_frame())?;
    let world_origin = embed_point(frame, spec.origin());
    let world_w_axis = resolve_direction(spec.direction_witness())?;
    let facing_frame = admit_frame(&SpatialFrameRef::workplane(
        "primitive-construction-birth-facing",
        world_origin,
        world_w_axis,
    ))?;
    Ok(PrimitiveConstructionBirthPlacementFacts {
        origin: world_origin,
        facing_vector: facing_frame.w_axis,
    })
}

#[derive(Clone, Copy)]
struct FrameBasis {
    origin: [f64; 3],
    u_axis: [f64; 3],
    v_axis: [f64; 3],
    w_axis: [f64; 3],
}

fn scaffold_digest(
    family: PrimitiveConstructionFamily,
    topology_birth_class: &str,
    intent_digest: &str,
    placement_facts: PrimitiveConstructionBirthPlacementFacts,
    support_planes: &[Plane],
    local_vertices: &[[f64; 3]],
) -> String {
    let mut parts = vec![
        family.as_str().to_string(),
        topology_birth_class.to_string(),
        intent_digest.to_string(),
        format!("{:?}", placement_facts.origin.map(f64::to_bits)),
        format!("{:?}", placement_facts.facing_vector.map(f64::to_bits)),
    ];
    parts.extend(support_planes.iter().map(|plane| {
        let (a, b, c, d) = plane.exact_coefficients();
        format!("{a}|{b}|{c}|{d}")
    }));
    parts.extend(
        local_vertices
            .iter()
            .map(|point| format!("{:?}", point.map(f64::to_bits))),
    );
    digest_owned_parts(&parts)
}

fn admit_frame(
    spec: &SpatialFrameRef,
) -> Result<FrameBasis, PrimitiveConstructionBirthProofSupportError> {
    match spec {
        SpatialFrameRef::World | SpatialFrameRef::ShapeLocal => Ok(FrameBasis {
            origin: [0.0, 0.0, 0.0],
            u_axis: [1.0, 0.0, 0.0],
            v_axis: [0.0, 1.0, 0.0],
            w_axis: [0.0, 0.0, 1.0],
        }),
        SpatialFrameRef::Workplane { origin, normal, .. }
        | SpatialFrameRef::FeatureLocal { origin, normal, .. } => {
            let origin = FinitePoint3::try_new(*origin)
                .map(FinitePoint3::as_array)
                .map_err(|_| {
                    PrimitiveConstructionBirthProofSupportError::new(
                        "frame origin must stay finite",
                    )
                })?;
            let w_axis = UnitVector3::try_new(*normal)
                .map(UnitVector3::as_array)
                .map_err(|_| {
                    PrimitiveConstructionBirthProofSupportError::new(
                        "frame normal must stay finite and non-zero",
                    )
                })?;
            let (u_axis, v_axis) = tangent_frame(&w_axis);
            Ok(FrameBasis {
                origin,
                u_axis,
                v_axis,
                w_axis,
            })
        }
    }
}

fn resolve_direction(
    witness: &SpatialDirectionWitnessRef,
) -> Result<[f64; 3], PrimitiveConstructionBirthProofSupportError> {
    match witness {
        SpatialDirectionWitnessRef::WorldDirection(direction) => normalize_direction(*direction),
        SpatialDirectionWitnessRef::FrameAxis { frame, axis } => {
            Ok(axis_from_basis(admit_frame(frame)?, *axis))
        }
        SpatialDirectionWitnessRef::FramePerpendicularAxis { frame, axis } => {
            fallback_perpendicular(axis_from_basis(admit_frame(frame)?, *axis))
        }
        _ => Err(PrimitiveConstructionBirthProofSupportError::new(
            "unsupported placement direction witness for primitive construction birth proof support",
        )),
    }
}

fn axis_from_basis(basis: FrameBasis, axis: SpatialAxis) -> [f64; 3] {
    match axis {
        SpatialAxis::U => basis.u_axis,
        SpatialAxis::V => basis.v_axis,
        SpatialAxis::W => basis.w_axis,
    }
}

fn fallback_perpendicular(
    parallel: [f64; 3],
) -> Result<[f64; 3], PrimitiveConstructionBirthProofSupportError> {
    let unit = UnitVector3::try_new(parallel).map_err(|_| {
        PrimitiveConstructionBirthProofSupportError::new(
            "unsupported degenerate frame axis for perpendicular placement witness",
        )
    })?;
    Ok(canonical_perpendicular_unit_vector(unit).as_array())
}

fn normalize_direction(
    vector: [f64; 3],
) -> Result<[f64; 3], PrimitiveConstructionBirthProofSupportError> {
    if vector.iter().any(|value| !value.is_finite()) {
        return Err(PrimitiveConstructionBirthProofSupportError::new(
            "placement direction witness must stay finite",
        ));
    }
    UnitVector3::try_new(vector)
        .map(UnitVector3::as_array)
        .map_err(|_| {
            PrimitiveConstructionBirthProofSupportError::new(
                "placement direction witness must stay non-zero",
            )
        })
}

fn embed_point(frame: FrameBasis, local: [f64; 3]) -> [f64; 3] {
    [
        frame.origin[0]
            + frame.u_axis[0] * local[0]
            + frame.v_axis[0] * local[1]
            + frame.w_axis[0] * local[2],
        frame.origin[1]
            + frame.u_axis[1] * local[0]
            + frame.v_axis[1] * local[1]
            + frame.w_axis[1] * local[2],
        frame.origin[2]
            + frame.u_axis[2] * local[0]
            + frame.v_axis[2] * local[1]
            + frame.w_axis[2] * local[2],
    ]
}
