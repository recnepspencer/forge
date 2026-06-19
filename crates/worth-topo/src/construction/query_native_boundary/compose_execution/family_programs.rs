use super::super::admitted_handoff::TopologyPrimitiveConstructionQueryAdmittedHandoff;
use super::super::birth_synopsis::{
    TopologyPrimitiveConstructionBirthFamily, TopologyPrimitiveConstructionQueryBirthSynopsis,
};
use super::coverage::{
    TopologyPrimitiveConstructionBirthMaterializationCoverage,
    TopologyPrimitiveConstructionBirthTopologyKind,
};
use super::program::{
    TopologyPrimitiveConstructionBirthComposeProgram, TopologyPrimitiveConstructionBirthEntity,
};

pub(crate) fn build_primitive_construction_birth_compose_program(
    handoff: &TopologyPrimitiveConstructionQueryAdmittedHandoff,
) -> TopologyPrimitiveConstructionBirthComposeProgram {
    let family = handoff.topology_query_handoff().family();
    let synopsis = handoff.topology_query_handoff().birth_synopsis();
    TopologyPrimitiveConstructionBirthComposeProgram::new(
        family,
        handoff.admitted_handoff_digest(),
        compose_birth_entities_from_synopsis(synopsis),
        compose_materialization_coverage_from_synopsis(synopsis),
        routes_to_layout_violation_probe(handoff),
    )
}

fn routes_to_layout_violation_probe(
    handoff: &TopologyPrimitiveConstructionQueryAdmittedHandoff,
) -> bool {
    let synopsis = handoff.topology_query_handoff().birth_synopsis();
    synopsis.family() == TopologyPrimitiveConstructionBirthFamily::ShellWithHole
        && (synopsis.topology_birth_class() != "planar_shell_with_hole_body"
            || synopsis.supported_loop_count() < 2
            || synopsis.supported_face_count() == 0
            || synopsis.supported_shell_count() == 0
            || synopsis.supported_body_count() == 0)
}

fn compose_birth_entities_from_synopsis(
    synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis,
) -> Vec<TopologyPrimitiveConstructionBirthEntity> {
    let mut entities = Vec::new();
    push_birth_entities(
        &mut entities,
        ".vertex",
        "vertex",
        synopsis.supported_vertex_count(),
    );
    entities
}

fn compose_materialization_coverage_from_synopsis(
    synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis,
) -> TopologyPrimitiveConstructionBirthMaterializationCoverage {
    let mut unmaterialized_topology_kinds = Vec::new();
    push_unmaterialized_kind(
        &mut unmaterialized_topology_kinds,
        TopologyPrimitiveConstructionBirthTopologyKind::Edge,
        synopsis.supported_edge_count(),
    );
    push_unmaterialized_kind(
        &mut unmaterialized_topology_kinds,
        TopologyPrimitiveConstructionBirthTopologyKind::Loop,
        synopsis.supported_loop_count(),
    );
    push_unmaterialized_kind(
        &mut unmaterialized_topology_kinds,
        TopologyPrimitiveConstructionBirthTopologyKind::Wire,
        synopsis.supported_wire_count(),
    );
    push_unmaterialized_kind(
        &mut unmaterialized_topology_kinds,
        TopologyPrimitiveConstructionBirthTopologyKind::Face,
        synopsis.supported_face_count(),
    );
    push_unmaterialized_kind(
        &mut unmaterialized_topology_kinds,
        TopologyPrimitiveConstructionBirthTopologyKind::Shell,
        synopsis.supported_shell_count(),
    );
    push_unmaterialized_kind(
        &mut unmaterialized_topology_kinds,
        TopologyPrimitiveConstructionBirthTopologyKind::Body,
        synopsis.supported_body_count(),
    );
    TopologyPrimitiveConstructionBirthMaterializationCoverage::anchor_only(
        unmaterialized_topology_kinds,
    )
}

fn push_unmaterialized_kind(
    unmaterialized_topology_kinds: &mut Vec<TopologyPrimitiveConstructionBirthTopologyKind>,
    topology_kind: TopologyPrimitiveConstructionBirthTopologyKind,
    count: usize,
) {
    if count > 0 {
        unmaterialized_topology_kinds.push(topology_kind);
    }
}

fn push_birth_entities(
    entities: &mut Vec<TopologyPrimitiveConstructionBirthEntity>,
    topology_kind: &'static str,
    suffix_stem: &str,
    count: usize,
) {
    entities.extend((0..count).map(|index| {
        TopologyPrimitiveConstructionBirthEntity::new(
            topology_kind,
            format!("{suffix_stem}-{index}"),
        )
    }));
}
