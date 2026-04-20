use crate::failure::StoreErrorKind;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportArtifactFamily {
    SchemaSupport,
    LineageSupport,
    CursorSupport,
    EmbeddedCheckpoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportArtifactRecoveryDisposition {
    RetainClean,
    RequireRebuild,
    RequireQuarantine,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportArtifactRecoveryEntry {
    pub(crate) family: SupportArtifactFamily,
    pub(crate) scope_identity: String,
    pub(crate) related_commit_id: Option<u64>,
    pub(crate) disposition: SupportArtifactRecoveryDisposition,
    pub(crate) kind: StoreErrorKind,
    pub(crate) reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportArtifactRecoveryReport {
    pub(crate) entries: Vec<SupportArtifactRecoveryEntry>,
}

impl SupportArtifactRecoveryEntry {
    pub fn family(&self) -> SupportArtifactFamily { self.family }
    pub fn scope_identity(&self) -> &str { &self.scope_identity }
    pub fn disposition(&self) -> SupportArtifactRecoveryDisposition { self.disposition }
    pub fn related_commit_id(&self) -> Option<u64> { self.related_commit_id }
    pub fn kind(&self) -> &StoreErrorKind { &self.kind }
    pub fn reason(&self) -> &str { &self.reason }
}

impl SupportArtifactRecoveryReport {
    pub fn empty() -> Self { Self { entries: Vec::new() } }
    pub fn entries(&self) -> &[SupportArtifactRecoveryEntry] { &self.entries }
    pub fn is_clean(&self) -> bool { self.entries.is_empty() }
    pub fn rebuilds(&self) -> Vec<&SupportArtifactRecoveryEntry> {
        self.entries.iter().filter(|entry| matches!(entry.disposition, SupportArtifactRecoveryDisposition::RequireRebuild)).collect()
    }
    pub fn quarantines(&self) -> Vec<&SupportArtifactRecoveryEntry> {
        self.entries.iter().filter(|entry| matches!(entry.disposition, SupportArtifactRecoveryDisposition::RequireQuarantine)).collect()
    }
}
