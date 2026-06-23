use std::collections::{BTreeSet, VecDeque};
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::docs_closeout::error::WorthDocsCloseoutError;
use crate::docs_closeout::model::report_context::WorthDocsReportContext;

use super::workspace_scan::TOUCHED_CRATE_EXPECTATIONS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorthDocsGraphEdgeKind {
    CrateMap,
    RelatedDoc,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthDocsGraphEdge {
    from_path: String,
    to_path: String,
    kind: WorthDocsGraphEdgeKind,
}

impl WorthDocsGraphEdge {
    pub fn from_path(&self) -> &str {
        &self.from_path
    }

    pub fn to_path(&self) -> &str {
        &self.to_path
    }

    pub fn kind(&self) -> WorthDocsGraphEdgeKind {
        self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthDocsGraphUnresolvedLink {
    from_path: String,
    raw_target: String,
    attempted_path: String,
}

impl WorthDocsGraphUnresolvedLink {
    pub fn from_path(&self) -> &str {
        &self.from_path
    }

    pub fn raw_target(&self) -> &str {
        &self.raw_target
    }

    pub fn attempted_path(&self) -> &str {
        &self.attempted_path
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthDocsGraph {
    doc_paths: BTreeSet<String>,
    edges: Vec<WorthDocsGraphEdge>,
    unresolved_links: Vec<WorthDocsGraphUnresolvedLink>,
}

impl WorthDocsGraph {
    pub fn edges(&self) -> &[WorthDocsGraphEdge] {
        &self.edges
    }

    pub fn unresolved_links(&self) -> &[WorthDocsGraphUnresolvedLink] {
        &self.unresolved_links
    }

    pub fn has_path(&self, from_path: &str, to_path: &str) -> bool {
        let mut seen = BTreeSet::new();
        let mut queue = VecDeque::from([from_path.to_string()]);
        while let Some(node) = queue.pop_front() {
            if node == to_path {
                return true;
            }
            if !seen.insert(node.clone()) {
                continue;
            }
            for edge in self.edges.iter().filter(|edge| edge.from_path == node) {
                queue.push_back(edge.to_path.clone());
            }
        }
        false
    }
}

pub fn current_worth_docs_graph() -> Result<WorthDocsGraph, WorthDocsCloseoutError> {
    worth_docs_graph_for_root(WorthDocsReportContext::current_workspace().workspace_root())
}

pub fn worth_docs_graph_for_root(
    workspace_root: &Path,
) -> Result<WorthDocsGraph, WorthDocsCloseoutError> {
    let context = WorthDocsReportContext::for_workspace_root(workspace_root);
    let doc_paths = collect_doc_paths(&context)?;
    let mut edges = Vec::new();
    let mut unresolved_links = Vec::new();
    for doc_path in &doc_paths {
        let absolute_path = workspace_root.join(doc_path);
        let markdown = fs::read_to_string(&absolute_path)?;
        let kind = if doc_path.ends_with("/README.md") {
            WorthDocsGraphEdgeKind::CrateMap
        } else {
            WorthDocsGraphEdgeKind::RelatedDoc
        };
        let source_dir = absolute_path
            .parent()
            .expect("markdown file should have parent");
        for link in markdown_links(&markdown) {
            let resolved = normalize_workspace_relative_path(workspace_root, source_dir.join(link));
            if let Some(resolved) = resolved.filter(|resolved| doc_paths.contains(resolved)) {
                edges.push(WorthDocsGraphEdge {
                    from_path: doc_path.clone(),
                    to_path: resolved,
                    kind,
                });
            } else if let Some(attempted_path) =
                normalize_workspace_relative_path(workspace_root, source_dir.join(link))
            {
                unresolved_links.push(WorthDocsGraphUnresolvedLink {
                    from_path: doc_path.clone(),
                    raw_target: link.to_string(),
                    attempted_path,
                });
            }
        }
    }

    Ok(WorthDocsGraph {
        doc_paths,
        edges,
        unresolved_links,
    })
}

fn collect_doc_paths(
    context: &WorthDocsReportContext,
) -> Result<BTreeSet<String>, WorthDocsCloseoutError> {
    let mut doc_paths = BTreeSet::new();
    for expectation in TOUCHED_CRATE_EXPECTATIONS {
        collect_markdown_paths(
            context.workspace_root(),
            &context.crate_docs_dir(expectation.crate_name),
            &mut doc_paths,
        )?;
    }
    Ok(doc_paths)
}

fn collect_markdown_paths(
    workspace_root: &Path,
    current_dir: &Path,
    doc_paths: &mut BTreeSet<String>,
) -> Result<(), WorthDocsCloseoutError> {
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_markdown_paths(workspace_root, &path, doc_paths)?;
            continue;
        }
        if path.extension().and_then(|extension| extension.to_str()) != Some("md") {
            continue;
        }
        if let Some(relative_path) = normalize_workspace_relative_path(workspace_root, path) {
            doc_paths.insert(relative_path);
        }
    }
    Ok(())
}

fn markdown_links(markdown: &str) -> Vec<&str> {
    let mut links = Vec::new();
    let mut rest = markdown;
    while let Some(start) = rest.find("](") {
        let candidate = &rest[start + 2..];
        let Some(end) = candidate.find(')') else {
            break;
        };
        let link = &candidate[..end];
        if !link.contains("://") && link.ends_with(".md") {
            links.push(link);
        }
        rest = &candidate[end + 1..];
    }
    links
}

fn normalize_workspace_relative_path(workspace_root: &Path, path: PathBuf) -> Option<String> {
    let normalized = normalize_path(path);
    normalized
        .strip_prefix(workspace_root)
        .ok()
        .map(|path| path.to_string_lossy().replace('\\', "/"))
}

fn normalize_path(path: PathBuf) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}
