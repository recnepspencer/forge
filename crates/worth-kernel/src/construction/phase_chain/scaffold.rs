use crate::construction::request::{PrimitiveConstructionFamily, PrimitiveConstructionPhaseError};
use crate::construction::topology_counts::PrimitiveConstructionTopologyCounts;
use topology::facade::{lower_primitive_construction_birth_plan, TopologyConstructionLoweringPlan};
use worth_geom::facade::{Plane, PrimitiveRealizationReport};
use worth_spatial::facade::{
    plan_primitive_construction_birth, PrimitiveConstructionBirthFamily,
    PrimitiveConstructionBirthScaffoldInput, SpatialConstructionBirthPlan,
};

#[derive(Clone, Debug)]
pub struct PrimitiveConstructionScaffold {
    family: PrimitiveConstructionFamily,
    request_digest: String,
    intent_digest: String,
    support_planes: Vec<Plane>,
    realization_report: PrimitiveRealizationReport,
    vertex_positions: Vec<[f64; 3]>,
    topology_counts: PrimitiveConstructionTopologyCounts,
    scaffold_digest: String,
}

impl PrimitiveConstructionScaffold {
    pub(crate) fn new(
        family: PrimitiveConstructionFamily,
        request_digest: String,
        intent_digest: String,
        support_planes: Vec<Plane>,
        realization_report: PrimitiveRealizationReport,
        vertex_positions: Vec<[f64; 3]>,
        topology_counts: PrimitiveConstructionTopologyCounts,
        scaffold_digest: String,
    ) -> Self {
        Self {
            family,
            request_digest,
            intent_digest,
            support_planes,
            realization_report,
            vertex_positions,
            topology_counts,
            scaffold_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn intent_digest(&self) -> &str {
        &self.intent_digest
    }

    pub fn support_planes(&self) -> &[Plane] {
        &self.support_planes
    }

    pub fn realization_report(&self) -> &PrimitiveRealizationReport {
        &self.realization_report
    }

    pub fn vertex_positions(&self) -> &[[f64; 3]] {
        &self.vertex_positions
    }

    pub fn topology_counts(&self) -> PrimitiveConstructionTopologyCounts {
        self.topology_counts
    }

    pub fn scaffold_digest(&self) -> &str {
        &self.scaffold_digest
    }

    pub fn birth_input(&self) -> PrimitiveConstructionBirthScaffoldInput {
        PrimitiveConstructionBirthScaffoldInput::new_with_realization(
            to_spatial_family(self.family),
            self.family.topology_birth_class(),
            self.scaffold_digest.clone(),
            self.support_planes.clone(),
            self.realization_report.clone(),
            self.vertex_positions.clone(),
            self.topology_counts.vertex_count(),
            self.topology_counts.edge_count(),
            self.topology_counts.loop_count(),
            self.topology_counts.wire_count(),
            self.topology_counts.face_count(),
            self.topology_counts.shell_count(),
            self.topology_counts.body_count(),
        )
    }

    pub fn plan_birth(
        &self,
    ) -> Result<SpatialConstructionBirthPlan, PrimitiveConstructionPhaseError> {
        plan_primitive_construction_birth(self.birth_input())
            .map_err(PrimitiveConstructionPhaseError::SpatialBirth)
    }
}

pub fn lower_scaffold_to_topology(
    scaffold: &PrimitiveConstructionScaffold,
) -> Result<
    (
        SpatialConstructionBirthPlan,
        TopologyConstructionLoweringPlan,
    ),
    PrimitiveConstructionPhaseError,
> {
    let birth_plan = scaffold.plan_birth()?;
    let lowering_plan = lower_primitive_construction_birth_plan(&birth_plan)
        .map_err(PrimitiveConstructionPhaseError::TopologyLowering)?;
    Ok((birth_plan, lowering_plan))
}

fn to_spatial_family(family: PrimitiveConstructionFamily) -> PrimitiveConstructionBirthFamily {
    match family {
        PrimitiveConstructionFamily::SimplexSolid => PrimitiveConstructionBirthFamily::SimplexSolid,
        PrimitiveConstructionFamily::Orthotope => PrimitiveConstructionBirthFamily::Orthotope,
        PrimitiveConstructionFamily::RegularPrism => PrimitiveConstructionBirthFamily::RegularPrism,
        PrimitiveConstructionFamily::RegularPyramid => {
            PrimitiveConstructionBirthFamily::RegularPyramid
        }
        PrimitiveConstructionFamily::WireBody => PrimitiveConstructionBirthFamily::WireBody,
        PrimitiveConstructionFamily::ShellWithHole => {
            PrimitiveConstructionBirthFamily::ShellWithHole
        }
    }
}
