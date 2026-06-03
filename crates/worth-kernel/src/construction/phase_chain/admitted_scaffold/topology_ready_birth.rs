use topology::facade::{
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    TopologyConstructionQueryAdmittedHandoffError, TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyPrimitiveConstructionQueryBirthSynopsis,
};
use worth_spatial::facade::{
    build_primitive_construction_birth_mapping_report,
    certify_primitive_construction_birth_completeness,
    impossible_primitive_construction_birth_attachment, plan_primitive_construction_birth,
    PrimitiveConstructionBirthFamily, PrimitiveConstructionBirthScaffoldInput,
    SpatialConstructionBirthCompletenessReport, SpatialConstructionBirthMappingReport,
    SpatialConstructionBirthPlan,
};

pub(super) struct PreparedPrimitiveConstructionTopologyReadyBirth {
    topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
    birth_completeness_report: SpatialConstructionBirthCompletenessReport,
    birth_mapping_report: SpatialConstructionBirthMappingReport,
}

impl PreparedPrimitiveConstructionTopologyReadyBirth {
    pub(super) fn into_parts(
        self,
    ) -> (
        TopologyPrimitiveConstructionQueryAdmittedHandoff,
        SpatialConstructionBirthCompletenessReport,
        SpatialConstructionBirthMappingReport,
    ) {
        (
            self.topology_query_admitted_handoff,
            self.birth_completeness_report,
            self.birth_mapping_report,
        )
    }
}

pub(super) fn prepare_primitive_construction_topology_ready_birth(
    birth_input: &PrimitiveConstructionBirthScaffoldInput,
) -> Result<
    PreparedPrimitiveConstructionTopologyReadyBirth,
    TopologyConstructionQueryAdmittedHandoffError,
> {
    let birth_plan = plan_primitive_construction_birth(birth_input.clone()).map_err(|error| {
        TopologyConstructionQueryAdmittedHandoffError::BirthCompleteness(error.to_string())
    })?;
    let topology_query_birth_synopsis = build_topology_query_birth_synopsis(&birth_plan);
    if let Some(row) = impossible_primitive_construction_birth_attachment(birth_input, &birth_plan)
    {
        return Err(
            TopologyConstructionQueryAdmittedHandoffError::ImpossibleBirthAttachment(
                row.reason().to_string(),
            ),
        );
    }
    let birth_completeness_report =
        certify_primitive_construction_birth_completeness(birth_input, &birth_plan).map_err(
            |error| {
                TopologyConstructionQueryAdmittedHandoffError::BirthCompleteness(error.to_string())
            },
        )?;
    let birth_mapping_report =
        build_primitive_construction_birth_mapping_report(&birth_completeness_report);
    let topology_query_admitted_handoff =
        prepare_primitive_construction_query_admitted_handoff_from_synopsis(
            &topology_query_birth_synopsis,
            birth_completeness_report.completeness_digest(),
            birth_mapping_report.report_digest(),
            birth_completeness_report.supported_loop_count(),
            birth_completeness_report.supported_body_count(),
        )?;
    Ok(PreparedPrimitiveConstructionTopologyReadyBirth {
        topology_query_admitted_handoff,
        birth_completeness_report,
        birth_mapping_report,
    })
}

fn build_topology_query_birth_synopsis(
    birth_plan: &SpatialConstructionBirthPlan,
) -> TopologyPrimitiveConstructionQueryBirthSynopsis {
    TopologyPrimitiveConstructionQueryBirthSynopsis::new(
        topology_family_from_spatial_family(birth_plan.family()),
        birth_plan.scaffold_digest().to_string(),
        birth_plan.birth_digest().to_string(),
        birth_plan.topology_birth_class().to_string(),
        birth_plan.supported_vertex_count(),
        birth_plan.supported_edge_count(),
        birth_plan.supported_loop_count(),
        birth_plan.supported_wire_count(),
        birth_plan.supported_face_count(),
        birth_plan.supported_shell_count(),
        birth_plan.supported_body_count(),
    )
}

fn topology_family_from_spatial_family(
    family: PrimitiveConstructionBirthFamily,
) -> TopologyPrimitiveConstructionBirthFamily {
    match family {
        PrimitiveConstructionBirthFamily::SimplexSolid => {
            TopologyPrimitiveConstructionBirthFamily::SimplexSolid
        }
        PrimitiveConstructionBirthFamily::Orthotope => {
            TopologyPrimitiveConstructionBirthFamily::Orthotope
        }
        PrimitiveConstructionBirthFamily::RegularPrism => {
            TopologyPrimitiveConstructionBirthFamily::RegularPrism
        }
        PrimitiveConstructionBirthFamily::RegularPyramid => {
            TopologyPrimitiveConstructionBirthFamily::RegularPyramid
        }
        PrimitiveConstructionBirthFamily::WireBody => {
            TopologyPrimitiveConstructionBirthFamily::WireBody
        }
        PrimitiveConstructionBirthFamily::ShellWithHole => {
            TopologyPrimitiveConstructionBirthFamily::ShellWithHole
        }
    }
}
