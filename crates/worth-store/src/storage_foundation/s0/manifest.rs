use super::artifacts::S0_ARTIFACT_SCHEMA_VERSION;
use super::evidence::S0StableDigest;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub enum S0InputFileKind {
    RoadmapDoc,
    CloseoutDoc,
    SourceFile,
    TestFile,
    Workflow,
    ReleaseSurface,
    EvidenceBundle,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct S0DeclaredScanRoot {
    root: String,
    purpose: String,
}

impl S0DeclaredScanRoot {
    pub fn new(
        root: impl Into<String>,
        purpose: impl Into<String>,
    ) -> Result<Self, S0ScanScopeRejection> {
        let root = normalize_path(root)?;
        if root == "." || root == "*" || root.is_empty() {
            return Err(S0ScanScopeRejection::WorkspaceGlobalScope);
        }
        if is_forbidden_scope(&root) {
            return Err(S0ScanScopeRejection::ForbiddenGeneratedScope);
        }
        let purpose = require_non_empty(purpose)?;
        Ok(Self { root, purpose })
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn purpose(&self) -> &str {
        &self.purpose
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0AuditBreadthSummary {
    declared_scan_root_count: u64,
    matched_file_count: u64,
    matched_byte_count: u64,
    unique_file_digest_count: u64,
}

impl S0AuditBreadthSummary {
    pub fn declared_scan_root_count(&self) -> u64 {
        self.declared_scan_root_count
    }

    pub fn matched_file_count(&self) -> u64 {
        self.matched_file_count
    }

    pub fn matched_byte_count(&self) -> u64 {
        self.matched_byte_count
    }

    pub fn unique_file_digest_count(&self) -> u64 {
        self.unique_file_digest_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0ScanCostSurface {
    requested_scan_scope_count: u64,
    admitted_scan_scope_count: u64,
    rejected_scan_scope_count: u64,
    scanned_file_count: u64,
    scanned_byte_count: u64,
}

impl S0ScanCostSurface {
    pub fn requested_scan_scope_count(&self) -> u64 {
        self.requested_scan_scope_count
    }

    pub fn admitted_scan_scope_count(&self) -> u64 {
        self.admitted_scan_scope_count
    }

    pub fn rejected_scan_scope_count(&self) -> u64 {
        self.rejected_scan_scope_count
    }

    pub fn scanned_file_count(&self) -> u64 {
        self.scanned_file_count
    }

    pub fn scanned_byte_count(&self) -> u64 {
        self.scanned_byte_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0AuditInputManifest {
    schema_version: &'static str,
    source_revision: String,
    declared_roots: Vec<S0DeclaredScanRoot>,
    matched_files: Vec<S0MatchedInputFile>,
    breadth_summary: S0AuditBreadthSummary,
    scan_cost: S0ScanCostSurface,
    manifest_digest: S0StableDigest,
}

impl S0AuditInputManifest {
    pub fn new(
        source_revision: impl Into<String>,
        declared_roots: Vec<S0DeclaredScanRoot>,
        matched_files: Vec<S0MatchedInputFile>,
    ) -> Result<Self, S0ScanScopeRejection> {
        let source_revision = require_non_empty(source_revision)?;
        if declared_roots.is_empty() {
            return Err(S0ScanScopeRejection::NoDeclaredScanRoots);
        }
        reject_duplicate_roots(&declared_roots)?;
        reject_duplicate_files(&matched_files)?;
        for file in &matched_files {
            if !declared_roots
                .iter()
                .any(|root| file_is_under_root(file, root))
            {
                return Err(S0ScanScopeRejection::MatchedFileOutsideDeclaredRoots);
            }
        }

        let mut declared_roots = declared_roots;
        let mut matched_files = matched_files;
        declared_roots.sort_by(|left, right| left.root.cmp(&right.root));
        matched_files.sort_by(|left, right| left.path.cmp(&right.path));

        let breadth_summary = breadth_summary(&declared_roots, &matched_files);
        let scan_cost = S0ScanCostSurface {
            requested_scan_scope_count: declared_roots.len() as u64,
            admitted_scan_scope_count: declared_roots.len() as u64,
            rejected_scan_scope_count: 0,
            scanned_file_count: matched_files.len() as u64,
            scanned_byte_count: matched_files
                .iter()
                .map(S0MatchedInputFile::byte_count)
                .sum(),
        };
        let manifest_digest = stable_digest(&S0AuditInputManifestDigestBasis {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            source_revision: &source_revision,
            declared_roots: &declared_roots,
            matched_files: &matched_files,
        })?;

        Ok(Self {
            schema_version: S0_ARTIFACT_SCHEMA_VERSION,
            source_revision,
            declared_roots,
            matched_files,
            breadth_summary,
            scan_cost,
            manifest_digest,
        })
    }

    pub fn witness(&self) -> S0InputManifestWitness {
        S0InputManifestWitness {
            schema_version: self.schema_version,
            source_revision: self.source_revision.clone(),
            manifest_digest: self.manifest_digest.clone(),
        }
    }

    pub fn validate_witness(
        &self,
        witness: &S0InputManifestWitness,
    ) -> Result<(), S0ScanScopeRejection> {
        if witness.schema_version != self.schema_version {
            return Err(S0ScanScopeRejection::StaleSchemaVersion);
        }
        if witness.source_revision != self.source_revision {
            return Err(S0ScanScopeRejection::StaleSourceRevision);
        }
        if witness.manifest_digest != self.manifest_digest {
            return Err(S0ScanScopeRejection::StaleManifestDigest);
        }
        Ok(())
    }

    pub fn source_revision(&self) -> &str {
        &self.source_revision
    }

    pub fn declared_roots(&self) -> &[S0DeclaredScanRoot] {
        &self.declared_roots
    }

    pub fn matched_files(&self) -> &[S0MatchedInputFile] {
        &self.matched_files
    }

    pub fn breadth_summary(&self) -> &S0AuditBreadthSummary {
        &self.breadth_summary
    }

    pub fn scan_cost(&self) -> &S0ScanCostSurface {
        &self.scan_cost
    }

    pub fn manifest_digest(&self) -> &S0StableDigest {
        &self.manifest_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0InputManifestWitness {
    schema_version: &'static str,
    source_revision: String,
    manifest_digest: S0StableDigest,
}

impl S0InputManifestWitness {
    pub fn source_revision(&self) -> &str {
        &self.source_revision
    }

    pub fn manifest_digest(&self) -> &S0StableDigest {
        &self.manifest_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0InputManifestDelta {
    reused_file_count: u64,
    rescanned_file_count: u64,
    added_file_count: u64,
    removed_file_count: u64,
}

impl S0InputManifestDelta {
    pub fn between(previous: &S0AuditInputManifest, current: &S0AuditInputManifest) -> Self {
        let previous_files = previous
            .matched_files()
            .iter()
            .map(|file| (file.path(), file.digest().as_str()))
            .collect::<BTreeMap<_, _>>();
        let current_files = current
            .matched_files()
            .iter()
            .map(|file| (file.path(), file.digest().as_str()))
            .collect::<BTreeMap<_, _>>();

        let reused_file_count = current_files
            .iter()
            .filter(|(path, digest)| previous_files.get(**path) == Some(digest))
            .count() as u64;
        let rescanned_file_count = current_files
            .iter()
            .filter(|(path, digest)| {
                previous_files
                    .get(**path)
                    .is_some_and(|previous_digest| previous_digest != *digest)
            })
            .count() as u64;
        let added_file_count = current_files
            .keys()
            .filter(|path| !previous_files.contains_key(**path))
            .count() as u64;
        let removed_file_count = previous_files
            .keys()
            .filter(|path| !current_files.contains_key(**path))
            .count() as u64;

        Self {
            reused_file_count,
            rescanned_file_count,
            added_file_count,
            removed_file_count,
        }
    }

    pub fn reused_file_count(&self) -> u64 {
        self.reused_file_count
    }

    pub fn rescanned_file_count(&self) -> u64 {
        self.rescanned_file_count
    }

    pub fn added_file_count(&self) -> u64 {
        self.added_file_count
    }

    pub fn removed_file_count(&self) -> u64 {
        self.removed_file_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum S0ScanScopeRejection {
    EmptyRequiredField,
    AbsolutePath,
    ParentTraversal,
    WorkspaceGlobalScope,
    ForbiddenGeneratedScope,
    NoDeclaredScanRoots,
    DuplicateDeclaredRoot,
    DuplicateMatchedFile,
    MatchedFileOutsideDeclaredRoots,
    MissingDigest,
    DigestConstructionFailed,
    StaleSchemaVersion,
    StaleSourceRevision,
    StaleManifestDigest,
}

#[derive(Serialize)]
struct S0AuditInputManifestDigestBasis<'a> {
    schema_version: &'static str,
    source_revision: &'a str,
    declared_roots: &'a [S0DeclaredScanRoot],
    matched_files: &'a [S0MatchedInputFile],
}

fn breadth_summary(
    roots: &[S0DeclaredScanRoot],
    files: &[S0MatchedInputFile],
) -> S0AuditBreadthSummary {
    let unique_file_digest_count = files
        .iter()
        .map(|file| file.digest().as_str())
        .collect::<BTreeSet<_>>()
        .len() as u64;
    S0AuditBreadthSummary {
        declared_scan_root_count: roots.len() as u64,
        matched_file_count: files.len() as u64,
        matched_byte_count: files.iter().map(S0MatchedInputFile::byte_count).sum(),
        unique_file_digest_count,
    }
}

fn reject_duplicate_roots(roots: &[S0DeclaredScanRoot]) -> Result<(), S0ScanScopeRejection> {
    let mut seen = BTreeSet::new();
    if roots.iter().any(|root| !seen.insert(root.root())) {
        return Err(S0ScanScopeRejection::DuplicateDeclaredRoot);
    }
    Ok(())
}

fn reject_duplicate_files(files: &[S0MatchedInputFile]) -> Result<(), S0ScanScopeRejection> {
    let mut seen = BTreeSet::new();
    if files.iter().any(|file| !seen.insert(file.path())) {
        return Err(S0ScanScopeRejection::DuplicateMatchedFile);
    }
    Ok(())
}

fn file_is_under_root(file: &S0MatchedInputFile, root: &S0DeclaredScanRoot) -> bool {
    file.path() == root.root()
        || file
            .path()
            .strip_prefix(root.root())
            .is_some_and(|suffix| suffix.starts_with('/'))
}

fn normalize_path(value: impl Into<String>) -> Result<String, S0ScanScopeRejection> {
    let value = require_non_empty(value)?;
    let normalized = value.replace('\\', "/");
    if normalized.starts_with('/') || normalized.contains(":/") {
        return Err(S0ScanScopeRejection::AbsolutePath);
    }
    if normalized.split('/').any(|part| part == "..") {
        return Err(S0ScanScopeRejection::ParentTraversal);
    }
    Ok(normalized.trim_matches('/').to_string())
}

fn is_forbidden_scope(path: &str) -> bool {
    path.split('/').any(|part| {
        matches!(
            part,
            "target" | ".git" | "node_modules" | "vendor" | "generated" | "dist"
        )
    })
}

fn stable_digest<T: Serialize + ?Sized>(value: &T) -> Result<S0StableDigest, S0ScanScopeRejection> {
    let bytes =
        serde_json::to_vec(value).map_err(|_| S0ScanScopeRejection::DigestConstructionFailed)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    S0StableDigest::new(format!("{:x}", hasher.finalize()))
        .map_err(|_| S0ScanScopeRejection::DigestConstructionFailed)
}

fn require_non_empty(value: impl Into<String>) -> Result<String, S0ScanScopeRejection> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(S0ScanScopeRejection::EmptyRequiredField);
    }
    Ok(value.trim().to_string())
}
