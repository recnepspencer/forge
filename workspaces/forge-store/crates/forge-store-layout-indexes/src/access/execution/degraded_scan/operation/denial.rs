#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DegradedExactScanExecutionDenied {
    ArtifactFamily(crate::ArtifactFamilyDenial),
    KeyDomain(crate::ArtifactFamilyDenial),
    ConcreteKey(crate::ArtifactFamilyDenial),
    Materialization(crate::MaterializationDenial),
    Shape(crate::access::shape::AccessShapeUnsupportedDenial),
    RequestAdmission(crate::PhysicalAccessRequestAdmissionDenied),
    Selection(crate::AccessPlanSelectionDenied),
    UnexpectedSelectedOperation,
    Stale(crate::StaleLayoutMaterialization),
    Physical(crate::PhysicalDegradedExecutionDenial),
}
