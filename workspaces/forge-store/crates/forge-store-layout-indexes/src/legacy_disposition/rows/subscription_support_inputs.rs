//! Historical support surfaces retained as inputs or forbidden aliases at subscription support.

use super::super::{
    LegacyAccessPathBypass as Bypass, LegacySurfaceInventoryRow, LegacySurfaceStage as Stage,
};
use super::row::{forbidden_legacy_root as forbidden, owner_input as input};

pub(super) const ROWS: &[LegacySurfaceInventoryRow] = &[
    input("SubscriptionSupportAccessStructure", Bypass::Admission),
    input(
        "SubscriptionSupportAccessStructureReport",
        Bypass::Admission,
    ),
    input("SupportTrustAccessIndexKind", Bypass::Admission),
    forbidden(
        "Milestone7IndependentReference",
        Stage::ExecutionArtifact,
        Bypass::Execution,
    ),
    forbidden(
        "SupportTrustAccessPath",
        Stage::SelectionArtifact,
        Bypass::Selection,
    ),
    input("SupportTrustAccessStructurePlan", Bypass::Admission),
];
