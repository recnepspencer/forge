//! Historical reports whose present owner is the certification lane.

use super::super::LegacySurfaceInventoryRow;
use super::row::certification_report as row;

pub(super) const ROWS: &[LegacySurfaceInventoryRow] = &[
    row("Milestone5ReadPathReport"),
    row("Milestone6AccessStructureClaim"),
    row("Milestone6AccessStructureContract"),
    row("Milestone6AccessStructureVerification"),
    row("Milestone6AccessStructureVerificationPath"),
    row("Milestone6CertificationBundle"),
    row("Milestone6CertificationOrigin"),
    row("Milestone6CertificationSummary"),
    row("Milestone6ComplexityPathStatus"),
    row("Milestone6ComplexitySurface"),
    row("Milestone6CounterContract"),
    row("Milestone6LayoutMaterializationReport"),
    row("Milestone6LayoutReadReport"),
    row("Milestone6PhysicalLayoutReport"),
    row("Milestone7AccessStructureClaim"),
    row("Milestone7AccessStructureContract"),
    row("Milestone7AccessStructureVerification"),
    row("Milestone7AccessStructureVerificationPath"),
    row("Milestone7CertificationBundle"),
    row("Milestone7ComplexityPathStatus"),
    row("Milestone7ComplexitySurface"),
    row("Milestone7CounterContract"),
];
