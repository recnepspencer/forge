use super::super::artifacts::S0_ARTIFACT_SCHEMA_VERSION;
use super::super::evidence::S0StableDigest;
use super::manifest_validation::{
    file_is_under_root, reject_duplicate_files, reject_duplicate_roots, require_non_empty,
    stable_digest, S0ScanScopeRejection,
};
use super::matched_input::S0MatchedInputFile;
use super::scan_scope::S0DeclaredScanRoot;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0AuditInputManifest {
    pub(super) schema_version: &'static str,
    pub(super) source_revision: String,
    declared_roots: Vec<S0DeclaredScanRoot>,
    matched_files: Vec<S0MatchedInputFile>,
    breadth_summary: S0AuditBreadthSummary,
    scan_cost: S0ScanCostSurface,
    pub(super) manifest_digest: S0StableDigest,
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
        reject_duplicate_roots(declared_roots.iter().map(S0DeclaredScanRoot::root))?;
        reject_duplicate_files(matched_files.iter().map(S0MatchedInputFile::path))?;
        for file in &matched_files {
            if !declared_roots
                .iter()
                .any(|root| file_is_under_root(file.path(), root.root()))
            {
                return Err(S0ScanScopeRejection::MatchedFileOutsideDeclaredRoots);
            }
        }

        let mut declared_roots = declared_roots;
        let mut matched_files = matched_files;
        declared_roots.sort_by(|left, right| left.root().cmp(right.root()));
        matched_files.sort_by(|left, right| left.path().cmp(right.path()));

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
        .collect::<std::collections::BTreeSet<_>>()
        .len() as u64;
    S0AuditBreadthSummary {
        declared_scan_root_count: roots.len() as u64,
        matched_file_count: files.len() as u64,
        matched_byte_count: files.iter().map(S0MatchedInputFile::byte_count).sum(),
        unique_file_digest_count,
    }
}
