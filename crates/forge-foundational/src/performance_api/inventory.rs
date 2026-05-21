#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformancePublicLane {
    CommonPath,
    LowerLane,
    StrongerLane,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalPerformancePublicSurfaceEntry {
    path: &'static str,
    lane: FoundationalPerformancePublicLane,
    teaches: &'static str,
    does_not_hide: &'static str,
}

impl FoundationalPerformancePublicSurfaceEntry {
    const fn new(
        path: &'static str,
        lane: FoundationalPerformancePublicLane,
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

    pub const fn lane(&self) -> FoundationalPerformancePublicLane {
        self.lane
    }

    pub const fn teaches(&self) -> &'static str {
        self.teaches
    }

    pub const fn does_not_hide(&self) -> &'static str {
        self.does_not_hide
    }
}

const PERFORMANCE_PUBLIC_SURFACE: [FoundationalPerformancePublicSurfaceEntry; 9] = [
    FoundationalPerformancePublicSurfaceEntry::new(
        "forge_foundational::performance_api::common_path",
        FoundationalPerformancePublicLane::CommonPath,
        "common-path performance claim authoring, layout intent definition, and primitive legality entrypoints",
        "lower-lane canonical lowering, explicit receipts, or stronger readiness proof",
    ),
    FoundationalPerformancePublicSurfaceEntry::new(
        "forge_foundational::performance_api::lower_lane::basis",
        FoundationalPerformancePublicLane::LowerLane,
        "inspectable canonical bundle, canonical-basis preparation, digest-ready lowering, contract name, counter spec, and comparison vocabulary",
        "common-path claim authoring or stronger proof-bearing certification",
    ),
    FoundationalPerformancePublicSurfaceEntry::new(
        "forge_foundational::performance_api::lower_lane::policy",
        FoundationalPerformancePublicLane::LowerLane,
        "inspectable budget and policy-admission receipt vocabulary",
        "executed counter-backed truth",
    ),
    FoundationalPerformancePublicSurfaceEntry::new(
        "forge_foundational::performance_api::lower_lane::receipts",
        FoundationalPerformancePublicLane::LowerLane,
        "inspectable counter-backed execution receipt vocabulary",
        "support/report materialization or stronger readiness proof",
    ),
    FoundationalPerformancePublicSurfaceEntry::new(
        "forge_foundational::performance_api::lower_lane::reports",
        FoundationalPerformancePublicLane::LowerLane,
        "inspectable attachment targets, report requests, report plans, and explicit materialization vocabulary",
        "common-path claim authoring or stronger readiness certification",
    ),
    FoundationalPerformancePublicSurfaceEntry::new(
        "forge_foundational::performance_api::lower_lane",
        FoundationalPerformancePublicLane::LowerLane,
        "grouped lower-lane performance lowering and inspection topology",
        "common-path authoring or stronger readiness certification",
    ),
    FoundationalPerformancePublicSurfaceEntry::new(
        "forge_foundational::performance_api::stronger_lane",
        FoundationalPerformancePublicLane::StrongerLane,
        "grouped stronger lane for certified performance bundles, trust-boundary readmission, and readiness certification",
        "common-path authoring or lower-lane inspection",
    ),
    FoundationalPerformancePublicSurfaceEntry::new(
        "forge_foundational::performance_api::stronger_lane::certified",
        FoundationalPerformancePublicLane::StrongerLane,
        "proof-bearing certified performance bundles and trust-boundary readmission over current-basis hot-path receipts and support-expansion reports",
        "plain lower-lane receipt/report inspection or readiness-only certification",
    ),
    FoundationalPerformancePublicSurfaceEntry::new(
        "forge_foundational::performance_api::stronger_lane::readiness",
        FoundationalPerformancePublicLane::StrongerLane,
        "production-readiness certification and proof-bearing readiness requirement",
        "plain readiness report or certified bundle proof progression",
    ),
];

pub const fn performance_public_surface_inventory(
) -> &'static [FoundationalPerformancePublicSurfaceEntry] {
    &PERFORMANCE_PUBLIC_SURFACE
}
