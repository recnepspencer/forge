use std::fs;
use std::path::{Path, PathBuf};

pub(super) fn files_containing_any(path: &Path, forbidden_terms: &[&str]) -> Vec<String> {
    rust_files(path)
        .into_iter()
        .filter_map(|file| {
            let text = fs::read_to_string(&file).expect("rust source is readable");
            let matched_terms = forbidden_terms
                .iter()
                .filter(|term| text.contains(**term))
                .copied()
                .collect::<Vec<_>>();
            (!matched_terms.is_empty())
                .then(|| format!("{} contains {matched_terms:?}", src_relative_path(&file)))
        })
        .collect()
}

pub(super) fn production_files_containing_any(
    path: &Path,
    forbidden_terms: &[&str],
) -> Vec<String> {
    rust_files(path)
        .into_iter()
        .filter(|file| !is_test_file(file))
        .filter_map(|file| {
            let text = fs::read_to_string(&file).expect("rust source is readable");
            let matched_terms = forbidden_terms
                .iter()
                .filter(|term| text.contains(**term))
                .copied()
                .collect::<Vec<_>>();
            (!matched_terms.is_empty())
                .then(|| format!("{} contains {matched_terms:?}", src_relative_path(&file)))
        })
        .collect()
}

pub(super) fn domain_structure_closeout_violations(workspace_root: &Path) -> Vec<String> {
    let docs = workspace_root.join("_docs").join("worth");
    let documents = [
        (
            "gate",
            read_doc(&docs, "worth-topo-domain-structure-gate.md"),
            &[
                "**Status:** Closed",
                "worth-topo-domain-structure-closeout.md",
            ][..],
        ),
        (
            "migration map",
            read_doc(&docs, "worth-topo-domain-structure-migration-map.md"),
            &[
                "**Status:** Closed",
                "worth-topo-domain-structure-closeout.md",
                "Owner Decisions Closed Or Deferred",
            ][..],
        ),
        (
            "roadmap",
            read_doc(&docs, "worth_roadmap.md"),
            &[
                "`Worth Topology Domain Structure Gate`: Closed",
                "unblocked",
                "topology-operator expansion",
            ][..],
        ),
        (
            "closeout",
            read_doc(&docs, "worth-topo-domain-structure-closeout.md"),
            &[
                "**Status:** Closed",
                "No legacy facade/export shims",
                "Intentional Deviations",
                "Dense Direct-File Clusters",
                "topology_operators/application/mod.rs",
                "scripts/ci/check_worth_topo_domain_structure.ps1",
                "cargo test -p worth-topo --quiet",
            ][..],
        ),
    ];

    documents
        .into_iter()
        .flat_map(|(name, document, required)| {
            required.iter().filter_map(move |marker| {
                (!document.contains(marker))
                    .then(|| format!("{name} is missing closeout marker `{marker}`"))
            })
        })
        .collect()
}

fn read_doc(docs: &Path, file_name: &str) -> String {
    fs::read_to_string(docs.join(file_name)).expect("domain structure document is readable")
}

pub(super) fn rust_files(path: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_rust_files(path, &mut files);
    files
}

fn collect_rust_files(path: &Path, files: &mut Vec<PathBuf>) {
    if path.is_file() {
        if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path.to_path_buf());
        }
        return;
    }
    for entry in fs::read_dir(path).expect("directory exists") {
        let entry = entry.expect("directory entry is readable");
        let entry_path = entry.path();
        if entry_path.is_dir() {
            collect_rust_files(&entry_path, files);
        } else if entry_path
            .extension()
            .is_some_and(|extension| extension == "rs")
        {
            files.push(entry_path);
        }
    }
}

pub(super) fn is_test_file(path: &Path) -> bool {
    path.components()
        .any(|component| component.as_os_str() == "tests")
        || path.file_name().is_some_and(|name| {
            let name = name.to_string_lossy();
            name.contains("tests") || name == "structure_guard.rs"
        })
}

pub(super) fn src_relative_path(path: &Path) -> String {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    path.strip_prefix(src)
        .expect("file lives under src")
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
