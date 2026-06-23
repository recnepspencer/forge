#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorthBackfillDocKind {
    Feature,
    Boundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorthBackfillSurfaceExpectation {
    pub crate_name: &'static str,
    pub doc_kind: WorthBackfillDocKind,
    pub surface_id: &'static str,
    pub relative_path: &'static str,
    pub readme_link_path: &'static str,
    pub required_jump_link: &'static str,
}

pub const WORTH_BACKFILL_SURFACE_EXPECTATIONS: [WorthBackfillSurfaceExpectation; 10] = [
    WorthBackfillSurfaceExpectation {
        crate_name: "worth-topo",
        doc_kind: WorthBackfillDocKind::Feature,
        surface_id: "topology-graph-authority",
        relative_path: "features/topology-graph-authority.md",
        readme_link_path: "./features/topology-graph-authority.md",
        required_jump_link: "./runtime-support.md",
    },
    WorthBackfillSurfaceExpectation {
        crate_name: "worth-topo",
        doc_kind: WorthBackfillDocKind::Feature,
        surface_id: "topology-certification-and-parity",
        relative_path: "features/topology-certification-and-parity.md",
        readme_link_path: "./features/topology-certification-and-parity.md",
        required_jump_link: "../boundaries/topo-query-runtime-boundary.md",
    },
    WorthBackfillSurfaceExpectation {
        crate_name: "worth-topo",
        doc_kind: WorthBackfillDocKind::Feature,
        surface_id: "topology-workloads-and-seeds",
        relative_path: "features/topology-workloads-and-seeds.md",
        readme_link_path: "./features/topology-workloads-and-seeds.md",
        required_jump_link: "../../../worth-kernel/docs/features/primitive-construction.md",
    },
    WorthBackfillSurfaceExpectation {
        crate_name: "worth-topo",
        doc_kind: WorthBackfillDocKind::Boundary,
        surface_id: "topo-query-runtime-boundary",
        relative_path: "boundaries/topo-query-runtime-boundary.md",
        readme_link_path: "./boundaries/topo-query-runtime-boundary.md",
        required_jump_link: "../features/runtime-support.md",
    },
    WorthBackfillSurfaceExpectation {
        crate_name: "worth-geom",
        doc_kind: WorthBackfillDocKind::Feature,
        surface_id: "analytic-primitives-and-planes",
        relative_path: "features/analytic-primitives-and-planes.md",
        readme_link_path: "./features/analytic-primitives-and-planes.md",
        required_jump_link: "../boundaries/geom-to-spatial-authority-boundary.md",
    },
    WorthBackfillSurfaceExpectation {
        crate_name: "worth-geom",
        doc_kind: WorthBackfillDocKind::Feature,
        surface_id: "curve-and-surface-schema",
        relative_path: "features/curve-and-surface-schema.md",
        readme_link_path: "./features/curve-and-surface-schema.md",
        required_jump_link:
            "../../../worth-spatial/docs/features/construction-time-birth-bindings.md",
    },
    WorthBackfillSurfaceExpectation {
        crate_name: "worth-geom",
        doc_kind: WorthBackfillDocKind::Feature,
        surface_id: "spatial-acceleration-and-matching",
        relative_path: "features/spatial-acceleration-and-matching.md",
        readme_link_path: "./features/spatial-acceleration-and-matching.md",
        required_jump_link: "../../../worth-topo/docs/features/runtime-support.md",
    },
    WorthBackfillSurfaceExpectation {
        crate_name: "worth-geom",
        doc_kind: WorthBackfillDocKind::Feature,
        surface_id: "boundary-certification-and-intersection",
        relative_path: "features/boundary-certification-and-intersection.md",
        readme_link_path: "./features/boundary-certification-and-intersection.md",
        required_jump_link:
            "../../../worth-topo/docs/features/topology-certification-and-parity.md",
    },
    WorthBackfillSurfaceExpectation {
        crate_name: "worth-geom",
        doc_kind: WorthBackfillDocKind::Feature,
        surface_id: "primitive-realization-strategies",
        relative_path: "features/primitive-realization-strategies.md",
        readme_link_path: "./features/primitive-realization-strategies.md",
        required_jump_link: "../../../worth-kernel/docs/features/primitive-construction.md",
    },
    WorthBackfillSurfaceExpectation {
        crate_name: "worth-geom",
        doc_kind: WorthBackfillDocKind::Boundary,
        surface_id: "geom-to-spatial-authority-boundary",
        relative_path: "boundaries/geom-to-spatial-authority-boundary.md",
        readme_link_path: "./boundaries/geom-to-spatial-authority-boundary.md",
        required_jump_link: "../../../worth-spatial/docs/foundations/spatial-overview.md",
    },
];
