use topology::facade::{
    prepare_primitive_construction_query_admitted_handoff_from_synopsis,
    TopologyPrimitiveConstructionBirthDeclaredTouchedBasis,
    TopologyPrimitiveConstructionBirthFamily, TopologyPrimitiveConstructionQueryBirthSynopsis,
};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};

#[derive(Clone, Debug)]
pub(super) struct PrimitiveConstructionBirthCase {
    family: TopologyPrimitiveConstructionBirthFamily,
    descriptor: PrimitiveWitnessDescriptor,
}

impl PrimitiveConstructionBirthCase {
    fn new(
        family: TopologyPrimitiveConstructionBirthFamily,
        descriptor: PrimitiveWitnessDescriptor,
    ) -> Self {
        Self { family, descriptor }
    }

    pub(super) fn family(&self) -> TopologyPrimitiveConstructionBirthFamily {
        self.family
    }

    pub(super) fn declared_touched_basis(
        &self,
        label: &str,
    ) -> TopologyPrimitiveConstructionBirthDeclaredTouchedBasis {
        primitive_construction_birth_declared_touched_basis(&self.descriptor, label)
    }
}

pub(super) fn primitive_construction_birth_cases() -> Vec<PrimitiveConstructionBirthCase> {
    vec![
        PrimitiveConstructionBirthCase::new(
            TopologyPrimitiveConstructionBirthFamily::SimplexSolid,
            PrimitiveWitnessDescriptor::SimplexSolid,
        ),
        PrimitiveConstructionBirthCase::new(
            TopologyPrimitiveConstructionBirthFamily::Orthotope,
            PrimitiveWitnessDescriptor::Orthotope,
        ),
        PrimitiveConstructionBirthCase::new(
            TopologyPrimitiveConstructionBirthFamily::RegularPrism,
            PrimitiveWitnessDescriptor::RegularPrism { side_count: 6 },
        ),
        PrimitiveConstructionBirthCase::new(
            TopologyPrimitiveConstructionBirthFamily::RegularPyramid,
            PrimitiveWitnessDescriptor::RegularPyramid { side_count: 5 },
        ),
        PrimitiveConstructionBirthCase::new(
            TopologyPrimitiveConstructionBirthFamily::WireBody,
            PrimitiveWitnessDescriptor::WireBody { edge_count: 8 },
        ),
        PrimitiveConstructionBirthCase::new(
            TopologyPrimitiveConstructionBirthFamily::ShellWithHole,
            PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 4,
                hole_loop_edge_counts: vec![4],
            },
        ),
    ]
}

pub(super) fn primitive_construction_birth_declared_touched_basis(
    descriptor: &PrimitiveWitnessDescriptor,
    label: &str,
) -> TopologyPrimitiveConstructionBirthDeclaredTouchedBasis {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(descriptor);
    let topology = contract.topology_contract();
    let synopsis = TopologyPrimitiveConstructionQueryBirthSynopsis::new(
        topology_family_for_descriptor(descriptor),
        contract,
        format!(
            "query-selection-{label}-{}-scaffold",
            descriptor.family().as_str()
        ),
        format!(
            "query-selection-{label}-{}-birth",
            descriptor.family().as_str()
        ),
        contract.topology_birth_class().to_string(),
        topology.vertex_count(),
        topology.edge_count(),
        topology.loop_count(),
        topology.wire_count(),
        topology.face_count(),
        topology.shell_count(),
        topology.body_count(),
    );
    let handoff = prepare_primitive_construction_query_admitted_handoff_from_synopsis(
        &synopsis,
        &format!(
            "query-selection-{label}-{}-birth-completeness",
            descriptor.family().as_str()
        ),
        &format!(
            "query-selection-{label}-{}-birth-mapping",
            descriptor.family().as_str()
        ),
        topology.vertex_count(),
        topology.body_count(),
    )
    .expect("real primitive construction synopsis should admit to topology handoff");
    TopologyPrimitiveConstructionBirthDeclaredTouchedBasis::from_admitted_handoff(&handoff)
        .expect("admitted topology handoff should lower to declared touched basis")
}

fn topology_family_for_descriptor(
    descriptor: &PrimitiveWitnessDescriptor,
) -> TopologyPrimitiveConstructionBirthFamily {
    match descriptor {
        PrimitiveWitnessDescriptor::SimplexSolid => {
            TopologyPrimitiveConstructionBirthFamily::SimplexSolid
        }
        PrimitiveWitnessDescriptor::Orthotope => {
            TopologyPrimitiveConstructionBirthFamily::Orthotope
        }
        PrimitiveWitnessDescriptor::RegularPrism { .. } => {
            TopologyPrimitiveConstructionBirthFamily::RegularPrism
        }
        PrimitiveWitnessDescriptor::RegularPyramid { .. } => {
            TopologyPrimitiveConstructionBirthFamily::RegularPyramid
        }
        PrimitiveWitnessDescriptor::WireBody { .. } => {
            TopologyPrimitiveConstructionBirthFamily::WireBody
        }
        PrimitiveWitnessDescriptor::ShellWithHole { .. } => {
            TopologyPrimitiveConstructionBirthFamily::ShellWithHole
        }
    }
}
