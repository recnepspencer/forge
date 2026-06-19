use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::docs_closeout::error::{WorthDocsCloseoutError, WorthDocsCloseoutErrorKind};
use crate::docs_closeout::model::doc_metadata::{WorthDocKind, WorthDocMetadata};
use crate::docs_closeout::model::report_context::WorthDocsReportContext;

use super::markdown_sections::markdown_headings;

#[derive(Debug, Clone)]
pub struct WorthDocFile {
    pub metadata: WorthDocMetadata,
    pub relative_path: String,
    pub headings: BTreeSet<String>,
    pub markdown: String,
}

#[derive(Debug, Clone)]
pub struct WorthDocsCrateScan {
    pub crate_name: String,
    pub docs_dir: PathBuf,
    pub readme: WorthDocFile,
    pub foundations: Vec<String>,
    pub features: Vec<WorthDocFile>,
    pub boundaries: Vec<WorthDocFile>,
}

#[derive(Debug, Clone)]
pub struct WorthCrateDocExpectation {
    pub crate_name: &'static str,
    pub doc_style: &'static str,
    pub neighbors: &'static [&'static str],
    pub expected_feature_ids: &'static [&'static str],
    pub expected_boundary_ids: &'static [&'static str],
}

pub const TOUCHED_CRATE_EXPECTATIONS: [WorthCrateDocExpectation; 4] = [
    WorthCrateDocExpectation {
        crate_name: "worth-kernel",
        doc_style: "workflow-first,authority-first",
        neighbors: &["worth-spatial", "worth-topo", "worth-geom", "forge-query"],
        expected_feature_ids: &[
            "primitive-construction",
            "shell-with-hole-construction",
            "wire-body-construction",
            "construction-simulation",
            "construction-replay",
            "construction-results-and-diagnostics",
        ],
        expected_boundary_ids: &["kernel-to-spatial", "worth-to-query"],
    },
    WorthCrateDocExpectation {
        crate_name: "worth-spatial",
        doc_style: "semantic-first,authority-first",
        neighbors: &["worth-kernel", "worth-topo", "worth-geom", "forge-query"],
        expected_feature_ids: &[
            "construction-time-birth-bindings",
            "birth-completeness-and-impossibility",
            "birth-truth-artifacts",
        ],
        expected_boundary_ids: &["spatial-to-topo", "spatial-query-proof-posture"],
    },
    WorthCrateDocExpectation {
        crate_name: "worth-topo",
        doc_style: "authority-first",
        neighbors: &["worth-kernel", "worth-spatial", "worth-geom", "forge-query"],
        expected_feature_ids: &[
            "topology-graph-authority",
            "topology-certification-and-parity",
            "topology-workloads-and-seeds",
            "domain-reads",
            "runtime-support",
        ],
        expected_boundary_ids: &["topo-query-runtime-boundary"],
    },
    WorthCrateDocExpectation {
        crate_name: "worth-geom",
        doc_style: "pure-geometry-first",
        neighbors: &["worth-kernel", "worth-spatial", "worth-topo"],
        expected_feature_ids: &[
            "analytic-primitives-and-planes",
            "curve-and-surface-schema",
            "spatial-acceleration-and-matching",
            "boundary-certification-and-intersection",
            "primitive-realization-strategies",
        ],
        expected_boundary_ids: &["geom-to-spatial-authority-boundary"],
    },
];

pub fn scan_all_touched_crates() -> Result<Vec<WorthDocsCrateScan>, WorthDocsCloseoutError> {
    scan_all_touched_crates_for_root(WorthDocsReportContext::current_workspace().workspace_root())
}

pub fn scan_all_touched_crates_for_root(
    workspace_root: &Path,
) -> Result<Vec<WorthDocsCrateScan>, WorthDocsCloseoutError> {
    let context = WorthDocsReportContext::for_workspace_root(workspace_root);
    TOUCHED_CRATE_EXPECTATIONS
        .iter()
        .map(|expectation| scan_crate(&context, expectation))
        .collect::<Result<Vec<_>, _>>()
}

pub fn expectation(crate_name: &str) -> &'static WorthCrateDocExpectation {
    TOUCHED_CRATE_EXPECTATIONS
        .iter()
        .find(|entry| entry.crate_name == crate_name)
        .expect("touched Worth crate expectation should exist")
}

fn scan_crate(
    context: &WorthDocsReportContext,
    expectation: &WorthCrateDocExpectation,
) -> Result<WorthDocsCrateScan, WorthDocsCloseoutError> {
    let docs_dir = context.crate_docs_dir(expectation.crate_name);
    if !docs_dir.exists() {
        return Err(WorthDocsCloseoutError::new(
            WorthDocsCloseoutErrorKind::MissingDoc,
            Some(docs_dir),
            "crate docs directory is missing",
        ));
    }

    let (files, foundations) = scan_doc_files(context.workspace_root(), &docs_dir)?;
    let by_kind = group_files_by_kind(files);
    let readme = by_kind
        .get(&WorthDocKind::CrateReadme)
        .and_then(|rows| rows.first())
        .cloned()
        .ok_or_else(|| {
            WorthDocsCloseoutError::new(
                WorthDocsCloseoutErrorKind::MissingDoc,
                Some(docs_dir.join("README.md")),
                "crate README metadata row is missing",
            )
        })?;

    Ok(WorthDocsCrateScan {
        crate_name: expectation.crate_name.to_string(),
        docs_dir,
        readme,
        foundations,
        features: by_kind
            .get(&WorthDocKind::Feature)
            .cloned()
            .unwrap_or_default(),
        boundaries: by_kind
            .get(&WorthDocKind::Boundary)
            .cloned()
            .unwrap_or_default(),
    })
}

fn group_files_by_kind(files: Vec<WorthDocFile>) -> BTreeMap<WorthDocKind, Vec<WorthDocFile>> {
    let mut by_kind = BTreeMap::new();
    for file in files {
        by_kind
            .entry(file.metadata.kind)
            .or_insert_with(Vec::new)
            .push(file);
    }
    by_kind
}

fn scan_doc_files(
    workspace_root: &Path,
    docs_dir: &Path,
) -> Result<(Vec<WorthDocFile>, Vec<String>), WorthDocsCloseoutError> {
    let mut files = Vec::new();
    let mut foundations = Vec::new();
    collect_markdown_files(
        workspace_root,
        docs_dir,
        docs_dir,
        &mut files,
        &mut foundations,
    )?;
    Ok((files, foundations))
}

fn collect_markdown_files(
    workspace_root: &Path,
    docs_dir: &Path,
    current_dir: &Path,
    files: &mut Vec<WorthDocFile>,
    foundations: &mut Vec<String>,
) -> Result<(), WorthDocsCloseoutError> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_markdown_files(workspace_root, docs_dir, &path, files, foundations)?;
            continue;
        }
        if path.extension().and_then(|extension| extension.to_str()) != Some("md") {
            continue;
        }
        let relative_path = path
            .strip_prefix(docs_dir)
            .expect("doc file should live under crate docs")
            .to_string_lossy()
            .replace('\\', "/");
        if relative_path.starts_with("foundations/") {
            foundations.push(relative_path);
            continue;
        }
        let markdown = fs::read_to_string(&path)?;
        let metadata = WorthDocMetadata::parse(&path, &markdown)?;
        files.push(WorthDocFile {
            headings: markdown_headings(&markdown),
            markdown,
            metadata,
            relative_path,
        });
    }
    files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    foundations.sort();
    Ok(())
}
