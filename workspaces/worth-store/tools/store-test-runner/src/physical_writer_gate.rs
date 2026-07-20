use std::path::{Path, PathBuf};

const FORBIDDEN_WRITER_FRAGMENTS: [(&str, &str); 15] = [
    ("std::fs::write", "raw std filesystem write"),
    ("std::fs::rename", "raw std filesystem rename"),
    ("std::fs::remove_", "raw std filesystem deletion"),
    ("std::fs::create_dir", "raw std directory creation"),
    ("OpenOptions", "raw file-open authority"),
    ("File::create", "raw file-create authority"),
    (".write_all(", "raw byte-write authority"),
    (".sync_all(", "raw file-state barrier"),
    (".sync_data(", "raw file-data barrier"),
    (".set_len(", "raw truncation authority"),
    ("StoreDurabilityRuntime", "quarantined durability writer"),
    ("AdmittedWalArtifactStore", "unjoined WAL writer"),
    (
        "InMemoryPhysicalFormatModel",
        "memory physical impersonator",
    ),
    ("windows_sys", "raw platform writer surface"),
    ("libc::write", "raw platform write"),
];

#[test]
fn ordinary_physical_runtime_has_one_media_admission_seam_and_no_writer_bypass() {
    let root = workspace_root().join("crates/worth-store/src/physical_runtime");
    let sources = rust_sources(&root).expect("read worth-store product sources");
    let mut qualify_sites = Vec::new();
    for source in sources {
        let text = std::fs::read_to_string(&source).expect("read product source");
        inspect_product_source(&source, &text).unwrap_or_else(|failure| panic!("{failure}"));
        for (line_index, line) in text.lines().enumerate() {
            if line.contains("qualify_filesystem_media(request)") {
                qualify_sites.push((source.clone(), line_index + 1));
            }
        }
    }
    assert_eq!(
        qualify_sites.len(),
        1,
        "canonical C.4 admission seam drifted"
    );
    assert!(qualify_sites[0]
        .0
        .ends_with("physical_runtime/media_ownership/admission.rs"));

    let owner = workspace_root()
        .join("crates/worth-store-physical-backend/src/filesystem_media/admission.rs");
    let owner = std::fs::read_to_string(owner).expect("read canonical media owner");
    assert!(owner.contains("pub(crate) fn qualify("));
    assert!(!owner.contains("pub fn qualify("));
}

#[test]
fn gate_rejects_each_bypass_family_and_localizes_the_rule() {
    for (source, expected) in [
        ("std::fs::write(root, bytes);", "raw std filesystem write"),
        (
            "FilesystemMediaOwner::admit(root, authority);",
            "direct media-owner construction",
        ),
        (
            "InMemoryPhysicalFormatModel::start_empty_model();",
            "memory physical impersonator",
        ),
    ] {
        let denial = inspect_product_source(Path::new("fixture.rs"), source)
            .expect_err("controlled bypass must be rejected");
        assert!(denial.contains(expected), "wrong denial: {denial}");
    }
}

fn inspect_product_source(path: &Path, source: &str) -> Result<(), String> {
    let code = without_line_comments(source);
    for (fragment, rule) in FORBIDDEN_WRITER_FRAGMENTS {
        if let Some(offset) = code.find(fragment) {
            return Err(localized_denial(path, &code, offset, rule));
        }
    }
    for constructor in [
        "FilesystemMediaOwner::admit",
        "FilesystemMediaOwner::admit_with_schedule",
        "QualifiedFilesystemMedia {",
        "MediaOwnedPhysicalRuntime::new",
    ] {
        if let Some(offset) = code.find(constructor) {
            let allowed = constructor == "MediaOwnedPhysicalRuntime::new"
                && path.ends_with("physical_runtime/media_ownership/admission.rs");
            if !allowed {
                return Err(localized_denial(
                    path,
                    &code,
                    offset,
                    "direct media-owner construction",
                ));
            }
        }
    }
    Ok(())
}

fn without_line_comments(source: &str) -> String {
    let mut code = String::with_capacity(source.len());
    for line in source.lines() {
        code.push_str(line.split_once("//").map_or(line, |(before, _)| before));
        code.push('\n');
    }
    code
}

fn localized_denial(path: &Path, source: &str, offset: usize, rule: &str) -> String {
    let line = source[..offset]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count()
        + 1;
    format!(
        "C.4 physical-writer gate: {rule} at {}:{line}; route ordinary effects through the canonical media-owned runtime",
        path.display()
    )
}

fn rust_sources(root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory)? {
            let path = entry?.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                sources.push(path);
            }
        }
    }
    sources.sort();
    Ok(sources)
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("runner must live under tools/<crate>")
}
