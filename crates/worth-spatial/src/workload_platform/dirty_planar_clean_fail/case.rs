use crate::planar_contracts::clean_fail_boundary::PlanarDirtyInputKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirtyPlanarCleanFailCase {
    SelfIntersectingLoop,
    NonManifoldWire,
    ThinWallOrInvalidLocalBasis,
    OrientationInconsistency,
}

impl DirtyPlanarCleanFailCase {
    pub fn from_dirty_input_kind(kind: PlanarDirtyInputKind) -> Self {
        match kind {
            PlanarDirtyInputKind::SelfIntersectingLoop => Self::SelfIntersectingLoop,
            PlanarDirtyInputKind::NonManifoldWire => Self::NonManifoldWire,
            PlanarDirtyInputKind::ThinWall => Self::ThinWallOrInvalidLocalBasis,
            PlanarDirtyInputKind::OrientationInconsistency => Self::OrientationInconsistency,
        }
    }

    pub fn human_name(self) -> &'static str {
        match self {
            Self::SelfIntersectingLoop => "self-intersecting loop",
            Self::NonManifoldWire => "non-manifold wire",
            Self::ThinWallOrInvalidLocalBasis => "thin wall or invalid local basis",
            Self::OrientationInconsistency => "orientation inconsistency",
        }
    }
}
