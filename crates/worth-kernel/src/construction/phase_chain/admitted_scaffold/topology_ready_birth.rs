use crate::construction::digest::digest_owned_parts;
use topology::facade::{
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    TopologyConstructionQueryAdmittedHandoffError, TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionQueryAdmittedHandoff,
    TopologyPrimitiveConstructionQueryBirthSynopsis,
};
use worth_spatial::facade::birth::{
    evaluate_primitive_construction_birth_consequence, plan_primitive_construction_birth,
    AdmittedPrimitiveConstructionBirthConsequence, PrimitiveConstructionBirthFamily,
    PrimitiveConstructionBirthScaffoldInput, SpatialConstructionBirthConsequence,
    SpatialConstructionBirthMappingKind, SpatialConstructionBirthPlan,
    SpatialConstructionBirthRejectionKind,
};

pub(super) struct PreparedPrimitiveConstructionTopologyReadyBirth {
    topology_query_admitted_handoff: TopologyPrimitiveConstructionQueryAdmittedHandoff,
    birth_consequence: AdmittedPrimitiveConstructionBirthConsequence,
}

impl PreparedPrimitiveConstructionTopologyReadyBirth {
    pub(super) fn into_parts(
        self,
    ) -> (
        TopologyPrimitiveConstructionQueryAdmittedHandoff,
        AdmittedPrimitiveConstructionBirthConsequence,
    ) {
        (self.topology_query_admitted_handoff, self.birth_consequence)
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
    let birth_consequence =
        match evaluate_primitive_construction_birth_consequence(birth_input, &birth_plan) {
            SpatialConstructionBirthConsequence::Admitted(admitted) => admitted,
            SpatialConstructionBirthConsequence::Rejected(rejected) => {
                let error = match rejected.kind() {
                    SpatialConstructionBirthRejectionKind::FamilyMismatch
                    | SpatialConstructionBirthRejectionKind::ScaffoldDigestMismatch
                    | SpatialConstructionBirthRejectionKind::TopologyBirthClassMismatch => {
                        TopologyConstructionQueryAdmittedHandoffError::ImpossibleBirthAttachment(
                            rejected.reason().to_string(),
                        )
                    }
                    SpatialConstructionBirthRejectionKind::ContractCountsOrSupportMismatch => {
                        TopologyConstructionQueryAdmittedHandoffError::BirthCompleteness(
                            rejected.reason().to_string(),
                        )
                    }
                };
                return Err(error);
            }
        };
    let topology_query_admitted_handoff =
        prepare_primitive_construction_query_admitted_handoff_from_synopsis(
            &topology_query_birth_synopsis,
            birth_consequence.consequence_digest(),
            &birth_mapping_digest(&birth_consequence),
            consequence_mapped_count(
                &birth_consequence,
                SpatialConstructionBirthMappingKind::Loop,
            ),
            consequence_mapped_count(
                &birth_consequence,
                SpatialConstructionBirthMappingKind::Body,
            ),
        )?;
    Ok(PreparedPrimitiveConstructionTopologyReadyBirth {
        topology_query_admitted_handoff,
        birth_consequence,
    })
}

fn build_topology_query_birth_synopsis(
    birth_plan: &SpatialConstructionBirthPlan,
) -> TopologyPrimitiveConstructionQueryBirthSynopsis {
    TopologyPrimitiveConstructionQueryBirthSynopsis::new(
        topology_family_from_spatial_family(birth_plan.family()),
        birth_plan.birth_contract(),
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

fn birth_mapping_digest(consequence: &AdmittedPrimitiveConstructionBirthConsequence) -> String {
    digest_owned_parts(
        &consequence
            .rows()
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

fn consequence_mapped_count(
    consequence: &AdmittedPrimitiveConstructionBirthConsequence,
    kind: SpatialConstructionBirthMappingKind,
) -> usize {
    consequence
        .row_for(kind)
        .expect("admitted primitive birth consequence should include every mapping row")
        .mapped_count()
}
