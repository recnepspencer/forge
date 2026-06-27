use forge_store_physical_format::PhysicalRecordSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalBoundaryLocalization {
    PageHeader,
    PageBody,
    FrameHeader,
    FrameBody,
    LengthField,
    SlotDirectory,
    SlotState(PhysicalRecordSlot),
    ExtentBoundary,
    AmbiguousBoundary,
}
