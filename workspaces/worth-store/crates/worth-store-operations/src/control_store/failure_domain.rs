use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfiguredFailureDomainId(String);

impl ConfiguredFailureDomainId {
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        if value.trim().is_empty() || value.len() > 256 {
            None
        } else {
            Some(Self(value))
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalControlLocation {
    path: PathBuf,
    failure_domain: ConfiguredFailureDomainId,
}

impl OperationalControlLocation {
    pub fn new(path: impl Into<PathBuf>, failure_domain: ConfiguredFailureDomainId) -> Self {
        Self {
            path: path.into(),
            failure_domain,
        }
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub const fn failure_domain(&self) -> &ConfiguredFailureDomainId {
        &self.failure_domain
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectedOperationalMediaLocation {
    path: PathBuf,
    failure_domain: ConfiguredFailureDomainId,
    role: ProtectedOperationalMediaRole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtectedOperationalMediaRole {
    Source,
    BackupTarget,
}

impl ProtectedOperationalMediaLocation {
    pub fn source(path: impl Into<PathBuf>, failure_domain: ConfiguredFailureDomainId) -> Self {
        Self {
            path: path.into(),
            failure_domain,
            role: ProtectedOperationalMediaRole::Source,
        }
    }
    pub fn backup_target(
        path: impl Into<PathBuf>,
        failure_domain: ConfiguredFailureDomainId,
    ) -> Self {
        Self {
            path: path.into(),
            failure_domain,
            role: ProtectedOperationalMediaRole::BackupTarget,
        }
    }
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub const fn failure_domain(&self) -> &ConfiguredFailureDomainId {
        &self.failure_domain
    }
    pub const fn role(&self) -> ProtectedOperationalMediaRole {
        self.role
    }
}
