use crate::catalog::PhysicalArtifactFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutCorruptionClassification {
    DerivedProjectionRebuildToParity,
    AuthoritativeSourceQuarantineRequired { family: PhysicalArtifactFamily },
}
