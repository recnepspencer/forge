#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyAccessPathBypass {
    Declaration,
    Admission,
    Selection,
    Budget,
    Lowering,
    Readiness,
    Execution,
    Readmission,
    DeepImportPrecedent,
    CertificationShortcut,
}
