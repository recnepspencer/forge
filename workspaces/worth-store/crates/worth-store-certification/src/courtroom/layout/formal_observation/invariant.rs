#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutFormalInvariant {
    BTreeSelectedReferenceBoundToStableExecution,
    LsmMembershipRolesAreCanonical,
    LsmMembershipSequenceIsStrict,
    LsmTombstoneSurvivesReplacementFrontier,
    LsmActivationBindsCompactionFrontier,
    PhysicalCompactionPublishesNewerRoot,
    OwnerCaseCoverageIsExact,
}
