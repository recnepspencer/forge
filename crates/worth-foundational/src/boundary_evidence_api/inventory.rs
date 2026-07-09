#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoundaryEvidencePublicLane {
    CommonPath,
    LowerLane,
    StrongerLane,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundaryEvidencePublicSurfaceEntry {
    path: &'static str,
    lane: BoundaryEvidencePublicLane,
    teaches: &'static str,
    does_not_hide: &'static str,
}

impl BoundaryEvidencePublicSurfaceEntry {
    const fn new(
        path: &'static str,
        lane: BoundaryEvidencePublicLane,
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

    pub const fn lane(&self) -> BoundaryEvidencePublicLane {
        self.lane
    }

    pub const fn teaches(&self) -> &'static str {
        self.teaches
    }

    pub const fn does_not_hide(&self) -> &'static str {
        self.does_not_hide
    }
}

const BOUNDARY_EVIDENCE_PUBLIC_SURFACE: [BoundaryEvidencePublicSurfaceEntry; 10] = [
    BoundaryEvidencePublicSurfaceEntry::new(
        "worth_foundational::boundary_evidence_api::common_path",
        BoundaryEvidencePublicLane::CommonPath,
        "common-path lineage, provenance, receipt, support, and attachment entrypoints",
        "lower-lane inspection or stronger proof-bearing readmission and readiness",
    ),
    BoundaryEvidencePublicSurfaceEntry::new(
        "worth_foundational::boundary_evidence_api::lower_lane::primitives",
        BoundaryEvidencePublicLane::LowerLane,
        "inspectable primitive category, locality, role, and legality vocabulary",
        "common-path ergonomics or stronger readiness certification",
    ),
    BoundaryEvidencePublicSurfaceEntry::new(
        "worth_foundational::boundary_evidence_api::lower_lane::provenance",
        BoundaryEvidencePublicLane::LowerLane,
        "inspectable provenance layering, source-basis, and freshness vocabulary",
        "lineage attestation or stronger support/current-basis readmission",
    ),
    BoundaryEvidencePublicSurfaceEntry::new(
        "worth_foundational::boundary_evidence_api::lower_lane::receipts",
        BoundaryEvidencePublicLane::LowerLane,
        "inspectable planning, executed, and closeout receipt vocabulary",
        "lineage continuity claims or stronger readiness certification",
    ),
    BoundaryEvidencePublicSurfaceEntry::new(
        "worth_foundational::boundary_evidence_api::lower_lane::lineage",
        BoundaryEvidencePublicLane::LowerLane,
        "inspectable continuity, divergence, promotion, and partiality vocabulary",
        "provenance construction or stronger readiness certification",
    ),
    BoundaryEvidencePublicSurfaceEntry::new(
        "worth_foundational::boundary_evidence_api::lower_lane::support",
        BoundaryEvidencePublicLane::LowerLane,
        "inspectable support-truth, recovery posture, basis disclosure, and residual debt vocabulary",
        "authority truth or stronger readmission",
    ),
    BoundaryEvidencePublicSurfaceEntry::new(
        "worth_foundational::boundary_evidence_api::lower_lane::attachments",
        BoundaryEvidencePublicLane::LowerLane,
        "inspectable attachment targets, materialization, canonical basis, digest, and bundle vocabulary",
        "proof-bearing readmission or readiness certification",
    ),
    BoundaryEvidencePublicSurfaceEntry::new(
        "worth_foundational::boundary_evidence_api::stronger_lane",
        BoundaryEvidencePublicLane::StrongerLane,
        "grouped stronger lane for readmission and readiness boundaries",
        "common-path authoring or lower-lane inspection",
    ),
    BoundaryEvidencePublicSurfaceEntry::new(
        "worth_foundational::boundary_evidence_api::stronger_lane::readmission",
        BoundaryEvidencePublicLane::StrongerLane,
        "current-basis and support-basis trust-boundary readmission surfaces",
        "plain materialized bundles or lower-lane attachments",
    ),
    BoundaryEvidencePublicSurfaceEntry::new(
        "worth_foundational::boundary_evidence_api::stronger_lane::readiness",
        BoundaryEvidencePublicLane::StrongerLane,
        "production-readiness certification and proof-bearing readiness requirement",
        "plain readiness report or lower-lane inspection",
    ),
];

pub const fn boundary_evidence_public_surface_inventory(
) -> &'static [BoundaryEvidencePublicSurfaceEntry] {
    &BOUNDARY_EVIDENCE_PUBLIC_SURFACE
}
