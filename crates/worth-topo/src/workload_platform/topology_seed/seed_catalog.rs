use super::{TopologySeedKind, TopologySeedRecipe};

pub struct TopologySeed;

impl TopologySeed {
    pub fn cube() -> TopologySeedRecipe {
        TopologySeedRecipe::new(TopologySeedKind::Cube, None)
    }

    pub fn tetrahedron() -> TopologySeedRecipe {
        TopologySeedRecipe::new(TopologySeedKind::Tetrahedron, None)
    }

    pub fn single_face_loop(edge_count: usize) -> TopologySeedRecipe {
        TopologySeedRecipe::new(TopologySeedKind::SingleFaceLoop, Some(edge_count))
    }

    pub fn multi_face_shell(face_count: usize) -> TopologySeedRecipe {
        TopologySeedRecipe::new(TopologySeedKind::MultiFaceShell, Some(face_count))
    }

    pub fn open_sheet() -> TopologySeedRecipe {
        TopologySeedRecipe::new(TopologySeedKind::OpenSheet, None)
    }

    pub fn open_wire() -> TopologySeedRecipe {
        TopologySeedRecipe::new(TopologySeedKind::OpenWire, None)
    }

    pub fn high_valence_vertex() -> TopologySeedRecipe {
        TopologySeedRecipe::new(TopologySeedKind::HighValenceVertex, None)
    }

    pub fn self_intersecting_loop() -> TopologySeedRecipe {
        TopologySeedRecipe::new(TopologySeedKind::SelfIntersectingLoop, None)
    }

    pub fn non_manifold_wire() -> TopologySeedRecipe {
        TopologySeedRecipe::new(TopologySeedKind::NonManifoldWire, None)
    }
}
