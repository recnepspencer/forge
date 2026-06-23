use super::super::super::request::{
    primitive_construction_topology_birth_class, PrimitiveConstructionFamily,
    PrimitiveConstructionPhaseError,
};
use super::super::birth_proof_support::{
    materialize_primitive_construction_birth_proof_support,
    PrimitiveConstructionAdmittedBirthProofSupport,
};
use super::super::PrimitiveConstructionAdmittedBirthTopologyTruth;
use super::super::PrimitiveConstructionAdmittedRealizationPosture;
use super::error_mapping::map_placement_geometry;
use super::lower_layer_family_translation::to_lower_layer_birth_family;
use super::topology_counts::PrimitiveConstructionTopologyCounts;
use super::PrimitiveConstructionAdmittedBirthInput;
use worth_geom::facade::{Plane, PrimitiveRealizationReport};
use worth_primitives::PrimitiveConstructionBirthSynopsisContract;
use worth_spatial::facade::placement::SpatialPlacementSpec;

pub(super) struct PrimitiveConstructionBirthScaffoldPlan {
    family: PrimitiveConstructionFamily,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    support_planes: Vec<Plane>,
    local_vertices: Vec<[f64; 3]>,
    realization: PrimitiveConstructionBirthScaffoldRealizationPlan,
    topology_counts: PrimitiveConstructionTopologyCounts,
}

pub(super) enum PrimitiveConstructionBirthScaffoldRealizationPlan {
    SupportReport(PrimitiveRealizationReport),
    DirectPlanar { label: &'static str },
}

impl PrimitiveConstructionBirthScaffoldPlan {
    pub(super) fn from_realized_support_facts(
        family: PrimitiveConstructionFamily,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        support_planes: Vec<Plane>,
        local_vertices: Vec<[f64; 3]>,
        realization: PrimitiveRealizationReport,
        topology_counts: PrimitiveConstructionTopologyCounts,
    ) -> Self {
        Self {
            family,
            birth_contract,
            support_planes,
            local_vertices,
            realization: PrimitiveConstructionBirthScaffoldRealizationPlan::SupportReport(
                realization,
            ),
            topology_counts,
        }
    }

    pub(super) fn from_direct_planar_support(
        family: PrimitiveConstructionFamily,
        birth_contract: PrimitiveConstructionBirthSynopsisContract,
        label: &'static str,
        support_planes: Vec<Plane>,
        local_vertices: Vec<[f64; 3]>,
        topology_counts: PrimitiveConstructionTopologyCounts,
    ) -> Self {
        Self {
            family,
            birth_contract,
            support_planes,
            local_vertices,
            realization: PrimitiveConstructionBirthScaffoldRealizationPlan::DirectPlanar { label },
            topology_counts,
        }
    }
}

pub(super) fn lower_family_birth_scaffold_plan(
    intent_digest: &str,
    placement_spec: SpatialPlacementSpec,
    scaffold_plan: PrimitiveConstructionBirthScaffoldPlan,
) -> Result<PrimitiveConstructionAdmittedBirthInput, PrimitiveConstructionPhaseError> {
    let PrimitiveConstructionBirthScaffoldPlan {
        family,
        birth_contract,
        support_planes,
        local_vertices,
        realization,
        topology_counts,
    } = scaffold_plan;
    let birth_proof_support = match &realization {
        PrimitiveConstructionBirthScaffoldRealizationPlan::SupportReport(report) => {
            materialize_primitive_construction_birth_proof_support(
                family,
                primitive_construction_topology_birth_class(family),
                intent_digest,
                placement_spec.clone(),
                &support_planes,
                &local_vertices,
                report.clone(),
                topology_counts.vertex_count(),
                topology_counts.edge_count(),
                topology_counts.loop_count(),
                topology_counts.wire_count(),
                topology_counts.face_count(),
                topology_counts.shell_count(),
                topology_counts.body_count(),
            )
        }
        PrimitiveConstructionBirthScaffoldRealizationPlan::DirectPlanar { label } => {
            materialize_primitive_construction_birth_proof_support(
                family,
                primitive_construction_topology_birth_class(family),
                intent_digest,
                placement_spec.clone(),
                &support_planes,
                &local_vertices,
                worth_geom::facade::build_direct_realization_report(
                    label,
                    &local_vertices,
                    &support_planes,
                ),
                topology_counts.vertex_count(),
                topology_counts.edge_count(),
                topology_counts.loop_count(),
                topology_counts.wire_count(),
                topology_counts.face_count(),
                topology_counts.shell_count(),
                topology_counts.body_count(),
            )
        }
    }
    .map_err(map_placement_geometry)?;
    Ok(PrimitiveConstructionAdmittedBirthInput {
        birth_topology_truth: birth_topology_truth_from_proof_support(
            family,
            birth_contract,
            topology_counts,
            &birth_proof_support,
        ),
        realization_posture: realization_posture_from_plan(
            &realization,
            &local_vertices,
            &support_planes,
            birth_proof_support.realization_fact_digest().to_string(),
            birth_proof_support
                .realization_geometry_digest()
                .to_string(),
        ),
        placement_facts: birth_proof_support.placement_facts(),
    })
}

fn realization_posture_from_plan(
    realization: &PrimitiveConstructionBirthScaffoldRealizationPlan,
    local_vertices: &[[f64; 3]],
    support_planes: &[Plane],
    realization_digest: String,
    realization_geometry_digest: String,
) -> PrimitiveConstructionAdmittedRealizationPosture {
    match realization {
        PrimitiveConstructionBirthScaffoldRealizationPlan::SupportReport(report) => {
            realization_posture_from_report(
                report.clone(),
                realization_digest,
                realization_geometry_digest,
            )
        }
        PrimitiveConstructionBirthScaffoldRealizationPlan::DirectPlanar { label } => {
            realization_posture_from_report(
                worth_geom::facade::build_direct_realization_report(
                    label,
                    local_vertices,
                    support_planes,
                ),
                realization_digest,
                realization_geometry_digest,
            )
        }
    }
}

fn birth_topology_truth_from_proof_support(
    family: PrimitiveConstructionFamily,
    birth_contract: PrimitiveConstructionBirthSynopsisContract,
    topology_counts: PrimitiveConstructionTopologyCounts,
    birth_proof_support: &PrimitiveConstructionAdmittedBirthProofSupport,
) -> PrimitiveConstructionAdmittedBirthTopologyTruth {
    PrimitiveConstructionAdmittedBirthTopologyTruth {
        family: to_lower_layer_birth_family(family),
        birth_contract,
        scaffold_digest: birth_proof_support.scaffold_digest().to_string(),
        birth_digest: birth_proof_support.birth_digest().to_string(),
        topology_birth_class: primitive_construction_topology_birth_class(family).to_string(),
        supported_vertex_count: topology_counts.vertex_count(),
        supported_edge_count: topology_counts.edge_count(),
        supported_loop_count: topology_counts.loop_count(),
        supported_wire_count: topology_counts.wire_count(),
        supported_face_count: topology_counts.face_count(),
        supported_shell_count: topology_counts.shell_count(),
        supported_body_count: topology_counts.body_count(),
        consequence_digest: birth_proof_support.birth_completeness_digest().to_string(),
        birth_mapping_digest: birth_proof_support.birth_mapping_digest().to_string(),
    }
}

fn realization_posture_from_report(
    realization_posture: PrimitiveRealizationReport,
    realization_digest: String,
    realization_geometry_digest: String,
) -> PrimitiveConstructionAdmittedRealizationPosture {
    PrimitiveConstructionAdmittedRealizationPosture {
        selected_strategy: realization_posture.strategy(),
        attempted_strategies: realization_posture.attempted_strategies().to_vec(),
        conditioning_witness: realization_posture.conditioning_witness().clone(),
        stability_class: realization_posture.stability_class(),
        realization_digest,
        realization_geometry_digest,
    }
}
