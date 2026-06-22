use forge_query::facade::ForgeQueryGraphObligationSupportLane;
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};

use crate::construction::{
    TopologyPrimitiveConstructionBirthComposeEvidence, TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionBirthTopologyKind,
    TopologyPrimitiveConstructionQueryBirthSynopsis,
};
use crate::facade::{topology_runtime, TopologyRuntimeAdapters};
use crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces;
use crate::test_support::primitive_corpus::validated_topology::build_test_runtime;

#[derive(Clone, Copy)]
pub(super) struct BirthCounts {
    pub(super) supported_vertex_count: usize,
    pub(super) supported_edge_count: usize,
    pub(super) supported_loop_count: usize,
    pub(super) supported_wire_count: usize,
    pub(super) supported_face_count: usize,
    pub(super) supported_shell_count: usize,
    pub(super) supported_body_count: usize,
}

impl BirthCounts {
    pub(super) fn expected_unmaterialized_topology_kinds(
        self,
    ) -> Vec<TopologyPrimitiveConstructionBirthTopologyKind> {
        let mut topology_kinds = Vec::new();
        push_expected_unmaterialized_topology_kind(
            &mut topology_kinds,
            TopologyPrimitiveConstructionBirthTopologyKind::Edge,
            self.supported_edge_count,
        );
        push_expected_unmaterialized_topology_kind(
            &mut topology_kinds,
            TopologyPrimitiveConstructionBirthTopologyKind::Loop,
            self.supported_loop_count,
        );
        push_expected_unmaterialized_topology_kind(
            &mut topology_kinds,
            TopologyPrimitiveConstructionBirthTopologyKind::Wire,
            self.supported_wire_count,
        );
        push_expected_unmaterialized_topology_kind(
            &mut topology_kinds,
            TopologyPrimitiveConstructionBirthTopologyKind::Face,
            self.supported_face_count,
        );
        push_expected_unmaterialized_topology_kind(
            &mut topology_kinds,
            TopologyPrimitiveConstructionBirthTopologyKind::Shell,
            self.supported_shell_count,
        );
        push_expected_unmaterialized_topology_kind(
            &mut topology_kinds,
            TopologyPrimitiveConstructionBirthTopologyKind::Body,
            self.supported_body_count,
        );
        topology_kinds
    }
}

pub(super) fn compose_family_cases() -> Vec<(
    TopologyPrimitiveConstructionBirthFamily,
    PrimitiveWitnessDescriptor,
    &'static str,
    BirthCounts,
)> {
    vec![
        (
            TopologyPrimitiveConstructionBirthFamily::SimplexSolid,
            PrimitiveWitnessDescriptor::SimplexSolid,
            "closed_simplex_solid_body",
            BirthCounts {
                supported_vertex_count: 4,
                supported_edge_count: 6,
                supported_loop_count: 4,
                supported_wire_count: 0,
                supported_face_count: 4,
                supported_shell_count: 1,
                supported_body_count: 1,
            },
        ),
        (
            TopologyPrimitiveConstructionBirthFamily::Orthotope,
            PrimitiveWitnessDescriptor::Orthotope,
            "closed_orthotope_body",
            BirthCounts {
                supported_vertex_count: 8,
                supported_edge_count: 12,
                supported_loop_count: 6,
                supported_wire_count: 0,
                supported_face_count: 6,
                supported_shell_count: 1,
                supported_body_count: 1,
            },
        ),
        (
            TopologyPrimitiveConstructionBirthFamily::RegularPrism,
            PrimitiveWitnessDescriptor::RegularPrism { side_count: 6 },
            "closed_regular_prism_body",
            BirthCounts {
                supported_vertex_count: 12,
                supported_edge_count: 18,
                supported_loop_count: 8,
                supported_wire_count: 0,
                supported_face_count: 8,
                supported_shell_count: 1,
                supported_body_count: 1,
            },
        ),
        (
            TopologyPrimitiveConstructionBirthFamily::RegularPyramid,
            PrimitiveWitnessDescriptor::RegularPyramid { side_count: 5 },
            "closed_regular_pyramid_body",
            BirthCounts {
                supported_vertex_count: 6,
                supported_edge_count: 10,
                supported_loop_count: 6,
                supported_wire_count: 0,
                supported_face_count: 6,
                supported_shell_count: 1,
                supported_body_count: 1,
            },
        ),
        (
            TopologyPrimitiveConstructionBirthFamily::WireBody,
            PrimitiveWitnessDescriptor::WireBody { edge_count: 8 },
            "planar_wire_body",
            BirthCounts {
                supported_vertex_count: 8,
                supported_edge_count: 8,
                supported_loop_count: 1,
                supported_wire_count: 1,
                supported_face_count: 0,
                supported_shell_count: 0,
                supported_body_count: 1,
            },
        ),
        (
            TopologyPrimitiveConstructionBirthFamily::ShellWithHole,
            PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 4,
                hole_loop_edge_counts: vec![4],
            },
            "planar_shell_with_hole_body",
            BirthCounts {
                supported_vertex_count: 8,
                supported_edge_count: 8,
                supported_loop_count: 2,
                supported_wire_count: 0,
                supported_face_count: 1,
                supported_shell_count: 1,
                supported_body_count: 1,
            },
        ),
    ]
}

pub(super) fn birth_synopsis(
    family: TopologyPrimitiveConstructionBirthFamily,
    descriptor: PrimitiveWitnessDescriptor,
    topology_birth_class: &str,
    counts: BirthCounts,
) -> TopologyPrimitiveConstructionQueryBirthSynopsis {
    TopologyPrimitiveConstructionQueryBirthSynopsis::new(
        family,
        PrimitiveConstructionFamilyContractRegistry::contract_for(&descriptor),
        format!("{}-scaffold", family.as_str()),
        format!("{}-birth", family.as_str()),
        topology_birth_class.to_string(),
        counts.supported_vertex_count,
        counts.supported_edge_count,
        counts.supported_loop_count,
        counts.supported_wire_count,
        counts.supported_face_count,
        counts.supported_shell_count,
        counts.supported_body_count,
    )
}

pub(super) fn topology_workspace(stem: &str) -> forge_query::facade::ForgeQueryWorkspace {
    let runtime = build_test_runtime().expect("topology relational test runtime");
    topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        format!("primitive-birth-compose.{stem}"),
    )
    .expect("topology query runtime workspace")
}

pub(super) fn assert_primitive_birth_compose_obligation_evidence(
    evidence: &TopologyPrimitiveConstructionBirthComposeEvidence,
) {
    assert_eq!(evidence.selected_obligation_rows().len(), 1);
    let row = &evidence.selected_obligation_rows()[0];
    assert_eq!(row.rule_namespace(), "worth-topo.primitive-construction");
    assert_eq!(
        row.rule_name(),
        "primitive-construction-birth-compose.graph-obligation"
    );
    assert_eq!(row.rule_semantic_version(), "v1");
    assert_eq!(
        row.support_lane(),
        ForgeQueryGraphObligationSupportLane::GraphComposition
    );
}

pub(super) fn committed_birth_anchor_count(
    workspace: &mut forge_query::facade::ForgeQueryWorkspace,
    family: TopologyPrimitiveConstructionBirthFamily,
) -> usize {
    let surfaces = declare_topology_query_surfaces(workspace).expect("declare topology surfaces");
    let prefix = format!("primitive-construction-birth.{}.", family.as_str());
    workspace
        .read(surfaces.entities())
        .iter()
        .filter(|row| {
            row.external_row()
                .get("naming")
                .and_then(|value| value.get("persistent_name"))
                .and_then(|value| value.as_str())
                .is_some_and(|persistent_name| persistent_name.starts_with(&prefix))
        })
        .count()
}

fn push_expected_unmaterialized_topology_kind(
    topology_kinds: &mut Vec<TopologyPrimitiveConstructionBirthTopologyKind>,
    topology_kind: TopologyPrimitiveConstructionBirthTopologyKind,
    count: usize,
) {
    if count > 0 {
        topology_kinds.push(topology_kind);
    }
}
