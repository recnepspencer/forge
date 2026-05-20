#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalProfileFrontDoorFamily {
    DiagnosticRichness,
    SupportPosture,
    CompatibilityPosture,
    AdmissionReadiness,
    RetentionDelivery,
    CertificationPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalProfileFrontDoorConstructionDenial {
    MissingFamily(FoundationalProfileFrontDoorFamily),
    DuplicateFamilyAssignment(FoundationalProfileFrontDoorFamily),
    IllegalComposition(super::super::FoundationalProfileCompositionDenial),
}
