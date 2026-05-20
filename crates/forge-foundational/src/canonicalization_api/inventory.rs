#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalPublicLane {
    CommonPath,
    LowerLane,
    StrongerLane,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanonicalPublicSurfaceEntry {
    path: &'static str,
    lane: CanonicalPublicLane,
    teaches: &'static str,
    does_not_hide: &'static str,
}

impl CanonicalPublicSurfaceEntry {
    const fn new(
        path: &'static str,
        lane: CanonicalPublicLane,
        teaches: &'static str,
        does_not_hide: &'static str,
    ) -> Self {
        Self {
            path,
            lane,
            teaches,
            does_not_hide,
        }
    }

    pub const fn path(&self) -> &'static str {
        self.path
    }

    pub const fn lane(&self) -> CanonicalPublicLane {
        self.lane
    }

    pub const fn teaches(&self) -> &'static str {
        self.teaches
    }

    pub const fn does_not_hide(&self) -> &'static str {
        self.does_not_hide
    }
}

const CANONICAL_PUBLIC_SURFACE: [CanonicalPublicSurfaceEntry; 7] = [
    CanonicalPublicSurfaceEntry::new(
        "forge_foundational::canonicalization_api::common_path",
        CanonicalPublicLane::CommonPath,
        "staged common-path canonical basis, comparison, export, digest, and readiness entry",
        "lower-lane readiness artifacts or equivalence declaration",
    ),
    CanonicalPublicSurfaceEntry::new(
        "forge_foundational::canonicalization_api::lower_lane::basis",
        CanonicalPublicLane::LowerLane,
        "inspectable basis preparation and readiness vocabulary",
        "common-path staging or stronger readiness proof",
    ),
    CanonicalPublicSurfaceEntry::new(
        "forge_foundational::canonicalization_api::lower_lane::comparison",
        CanonicalPublicLane::LowerLane,
        "inspectable comparison preparation, mismatch, and equivalence outcome vocabulary",
        "raw basis admission or grouped common-path ergonomics",
    ),
    CanonicalPublicSurfaceEntry::new(
        "forge_foundational::canonicalization_api::lower_lane::export",
        CanonicalPublicLane::LowerLane,
        "inspectable export bundle, manifest, and readmission vocabulary",
        "common-path export staging or stronger readiness proof",
    ),
    CanonicalPublicSurfaceEntry::new(
        "forge_foundational::canonicalization_api::lower_lane::digest",
        CanonicalPublicLane::LowerLane,
        "inspectable digest slots, admitted input-shape, and derived digest vocabulary",
        "basis authority or common-path staging",
    ),
    CanonicalPublicSurfaceEntry::new(
        "forge_foundational::canonicalization_api::stronger_lane",
        CanonicalPublicLane::StrongerLane,
        "grouped stronger proof-bearing canonicalization lane",
        "common-path authoring or lower-lane inspection",
    ),
    CanonicalPublicSurfaceEntry::new(
        "forge_foundational::canonicalization_api::stronger_lane::readiness",
        CanonicalPublicLane::StrongerLane,
        "production-readiness certification and proof-bearing readiness requirement",
        "plain readiness report or lower-lane inspection",
    ),
];

pub const fn canonical_public_surface_inventory() -> &'static [CanonicalPublicSurfaceEntry] {
    &CANONICAL_PUBLIC_SURFACE
}
