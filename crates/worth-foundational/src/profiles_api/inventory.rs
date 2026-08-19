#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfilePublicLane {
    CommonPath,
    LowerLane,
    StrongerLane,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalProfilePublicSurfaceEntry {
    path: &'static str,
    lane: FoundationalProfilePublicLane,
    teaches: &'static str,
    does_not_hide: &'static str,
}

impl FoundationalProfilePublicSurfaceEntry {
    const fn new(
        path: &'static str,
        lane: FoundationalProfilePublicLane,
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

    pub const fn lane(&self) -> FoundationalProfilePublicLane {
        self.lane
    }

    pub const fn teaches(&self) -> &'static str {
        self.teaches
    }

    pub const fn does_not_hide(&self) -> &'static str {
        self.does_not_hide
    }
}

const PROFILE_PUBLIC_SURFACE: [FoundationalProfilePublicSurfaceEntry; 9] = [
    FoundationalProfilePublicSurfaceEntry::new(
        "worth_foundational::profiles_api::common_path",
        FoundationalProfilePublicLane::CommonPath,
        "staged profile composition, progression, attachment, materialization, and certification entry",
        "lower-lane profile artifacts, target legality, or stronger readiness proof",
    ),
    FoundationalProfilePublicSurfaceEntry::new(
        "worth_foundational::profiles_api::lower_lane::composition",
        FoundationalProfilePublicLane::LowerLane,
        "inspectable profile families and composed-set vocabulary",
        "common-path request ergonomics or stronger readiness proof",
    ),
    FoundationalProfilePublicSurfaceEntry::new(
        "worth_foundational::profiles_api::lower_lane::progression",
        FoundationalProfilePublicLane::LowerLane,
        "inspectable requested, admitted, and materialized progression vocabulary",
        "common-path staged progression",
    ),
    FoundationalProfilePublicSurfaceEntry::new(
        "worth_foundational::profiles_api::lower_lane::attachment",
        FoundationalProfilePublicLane::LowerLane,
        "inspectable target-specific attachment and profiled artifact vocabulary",
        "common-path target staging",
    ),
    FoundationalProfilePublicSurfaceEntry::new(
        "worth_foundational::profiles_api::lower_lane::materialization",
        FoundationalProfilePublicLane::LowerLane,
        "inspectable descriptive surface inventories, applicability, materialization planning, and explicit observation-disposition vocabulary",
        "common-path materialization staging",
    ),
    FoundationalProfilePublicSurfaceEntry::new(
        "worth_foundational::profiles_api::lower_lane::identity",
        FoundationalProfilePublicLane::LowerLane,
        "inspectable canonical identity, difference, and compatibility vocabulary",
        "common-path progression staging",
    ),
    FoundationalProfilePublicSurfaceEntry::new(
        "worth_foundational::profiles_api::lower_lane::certification",
        FoundationalProfilePublicLane::LowerLane,
        "inspectable proof-bearing certification and boundary readmission vocabulary",
        "common-path strengthening ergonomics or readiness proof",
    ),
    FoundationalProfilePublicSurfaceEntry::new(
        "worth_foundational::profiles_api::stronger_lane",
        FoundationalProfilePublicLane::StrongerLane,
        "grouped stronger proof-bearing profile lane",
        "common-path authoring or lower-lane inspection",
    ),
    FoundationalProfilePublicSurfaceEntry::new(
        "worth_foundational::profiles_api::stronger_lane::readiness",
        FoundationalProfilePublicLane::StrongerLane,
        "production-readiness certification and proof-bearing readiness requirement",
        "plain readiness report or lower-lane inspection",
    ),
];

pub const fn profile_public_surface_inventory() -> &'static [FoundationalProfilePublicSurfaceEntry]
{
    &PROFILE_PUBLIC_SURFACE
}
