use super::super::evidence::S0StableDigest;
use super::manifest_validation::{is_forbidden_scope, normalize_path, S0ScanScopeRejection};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub enum S0InputFileKind {
    RoadmapDoc,
    CloseoutDoc,
    SourceFile,
    TestFile,
    Workflow,
    ReleaseSurface,
    EvidenceBundle,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct S0InputFileDigest(S0StableDigest);

impl S0InputFileDigest {
    pub fn new(value: impl Into<String>) -> Result<Self, S0ScanScopeRejection> {
        Ok(Self(
            S0StableDigest::new(value).map_err(|_| S0ScanScopeRejection::MissingDigest)?,
        ))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize)]
pub struct S0MatchedInputFile {
    path: String,
    kind: S0InputFileKind,
    digest: S0InputFileDigest,
    byte_count: u64,
}

impl S0MatchedInputFile {
    pub fn new(
        path: impl Into<String>,
        kind: S0InputFileKind,
        digest: S0InputFileDigest,
        byte_count: u64,
    ) -> Result<Self, S0ScanScopeRejection> {
        let path = normalize_path(path)?;
        if is_forbidden_scope(&path) {
            return Err(S0ScanScopeRejection::ForbiddenGeneratedScope);
        }
        Ok(Self {
            path,
            kind,
            digest,
            byte_count,
        })
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn digest(&self) -> &S0InputFileDigest {
        &self.digest
    }

    pub fn byte_count(&self) -> u64 {
        self.byte_count
    }
}
