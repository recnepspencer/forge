use std::path::{Path, PathBuf};

#[path = "raw_media_owner_gate.rs"]
mod raw_media_owner_gate;

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

const FORBIDDEN_RECORD_SUBSTITUTIONS: [(&str, &str); 10] = [
    (
        "InMemoryPhysicalFormatModel",
        "heap model on the production record path",
    ),
    (
        "InMemoryPhysicalFormatReplayArtifact",
        "replay artifact on the ordinary record path",
    ),
    (
        "PersistedPhysicalLayout",
        "predecoded whole-layout substitution",
    ),
    (
        "PhysicalReference",
        "legacy location-shaped reference on the durable record path",
    ),
    (
        "walk_current_durable_record_manifest",
        "offline verifier on the production path",
    ),
    (
        "OfflineManifestCodec",
        "offline codec on the production path",
    ),
    ("std::fs::", "direct filesystem access on the record path"),
    ("create_or_open", "ambiguous initialize/open facade"),
    ("collect_all", "whole-store collection helper"),
    ("read_to_end", "unbounded record materialization"),
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
        (
            "let media = QualifiedFilesystemMedia { owner, profile };",
            "direct media-owner construction",
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
            let allowed = (constructor == "MediaOwnedPhysicalRuntime::new"
                && path.ends_with("physical_runtime/media_ownership/admission.rs"))
                || (constructor == "QualifiedFilesystemMedia {"
                    && occurrence_is_return_type(&code, offset));
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

#[test]
fn c5_record_path_has_no_heap_replay_offline_or_raw_filesystem_substitute() {
    let root = workspace_root().join("crates/worth-store/src/physical_runtime/record_serving");
    for source in rust_sources(&root).expect("read C.5 product sources") {
        let text = std::fs::read_to_string(&source).expect("read C.5 source");
        inspect_record_source(&source, &text).unwrap_or_else(|failure| panic!("{failure}"));
    }
    let observer = workspace_root()
        .join("crates/worth-store-offline-verifier/src/bin/physical_store_offline_observer.rs");
    let observer = std::fs::read_to_string(observer).expect("read C.5 offline observer");
    assert!(!observer.contains("worth_store::"));
    assert!(!observer.contains("ServingPhysicalRuntime"));
    let observer_manifest = std::fs::read_to_string(
        workspace_root().join("crates/worth-store-offline-verifier/Cargo.toml"),
    )
    .expect("read offline verifier manifest");
    assert!(
        !observer_manifest.lines().any(|line| {
            line.trim_start().starts_with("worth-store =")
                || line.trim_start().starts_with("worth_store =")
        }),
        "the separately linked offline observer must not depend on the Store runtime crate"
    );

    let frame_ports = root.join("residency/candidate_frame_residency.rs");
    let frame_ports = std::fs::read_to_string(frame_ports).expect("read C.6 frame port seam");
    inspect_candidate_publication_port(&frame_ports).unwrap_or_else(|failure| panic!("{failure}"));

    let publication = root.join("publication/director/execution.rs");
    let publication = std::fs::read_to_string(publication).expect("read Store publication owner");
    assert!(
        publication.contains("publication_progression::execute_prepared_root("),
        "Store must retain current-truth publication after the C.6 candidate seam"
    );
}

#[test]
fn c5_gate_rejects_all_catalogued_substitution_mutants() {
    for (fragment, rule) in FORBIDDEN_RECORD_SUBSTITUTIONS {
        let denial = inspect_record_source(Path::new("controlled_mutant.rs"), fragment)
            .expect_err("catalogued C.5 substitution must fail");
        assert!(denial.contains(rule), "wrong C.5 denial: {denial}");
    }
}

#[test]
fn c6_candidate_port_cannot_acquire_current_truth_authority() {
    let mutant = r#"
        pub(super) trait CandidateFramePublicationPort {
            fn submit(
                &self,
                media: &QualifiedFilesystemMedia,
                candidate: CandidateFrameSet,
            ) -> Result<PublishedRecordBatch, RecordAppendError> {
                publication_progression::execute(media, candidate.into_plan())
            }
        }
    "#;
    let denial = inspect_candidate_publication_port(mutant)
        .expect_err("a C.6 port with media and publication authority must be rejected");
    assert!(denial.contains("physical media"));
}

fn inspect_record_source(path: &Path, source: &str) -> Result<(), String> {
    let code = without_line_comments(source);
    for (fragment, rule) in FORBIDDEN_RECORD_SUBSTITUTIONS {
        if let Some(offset) = code.find(fragment) {
            return Err(localized_denial(path, &code, offset, rule));
        }
    }
    Ok(())
}

fn inspect_candidate_publication_port(source: &str) -> Result<(), String> {
    let start = source
        .find("trait CandidateFramePublicationPort")
        .ok_or_else(|| "C.6 candidate publication port is missing".to_owned())?;
    let candidate_tail = &source[start..];
    if candidate_tail.contains("QualifiedFilesystemMedia") {
        return Err(
            "C.5/C.6 boundary: CandidateFramePublicationPort acquired physical media authority"
                .to_owned(),
        );
    }
    let contract = trait_contract(source, "CandidateFramePublicationPort")?;
    for (fragment, authority) in [
        ("publication_progression", "publication progression"),
        ("RecordArtifactFile", "artifact naming"),
        ("MediaCounterSnapshot", "media-effect evidence"),
        ("ArtifactTreeFailure", "backend failure"),
        ("FnMut", "Store-owned write callback"),
        ("replace_catalog", "catalog replacement"),
    ] {
        if contract.contains(fragment) {
            return Err(format!(
                "C.5/C.6 boundary: CandidateFramePublicationPort acquired {authority} authority"
            ));
        }
    }
    if !contract.contains("fn begin(") || contract.contains("fn submit(") {
        return Err(
            "C.5/C.6 boundary: candidate port must begin residency without submitting publication"
                .to_owned(),
        );
    }
    let residency = trait_contract(source, "CandidateFrameResidencySession")?;
    if !residency.contains("fn retain(")
        || !residency.contains("Result<Box<dyn ResidentCandidateFrame>, RecordAppendDenial>")
        || residency.contains("FnMut")
        || residency.contains("ArtifactTreeFailure")
    {
        return Err(
            "C.5/C.6 boundary: residency must own each frame for the duration of Store's physical write"
                .to_owned(),
        );
    }
    let resident = trait_contract(source, "ResidentCandidateFrame")?;
    if !resident.contains("fn role(&self) -> CandidateFrameRole;")
        || !resident.contains("fn coordinate(&self) -> CandidateFrameCoordinate;")
        || !resident.contains("fn bytes(&self) -> &[u8];")
        || !resident.contains("fn publish_clean(")
        || !resident.contains("Result<CandidateFrameWriteCompletion, RecordAppendDenial>")
        || resident.contains("FnMut")
        || resident.contains("ArtifactTreeFailure")
    {
        return Err(
            "C.5/C.6 boundary: the resident guard must expose bytes to Store and release ownership without acquiring write authority"
                .to_owned(),
        );
    }
    Ok(())
}

fn trait_contract<'source>(source: &'source str, name: &str) -> Result<&'source str, String> {
    let marker = format!("trait {name}");
    let start = source
        .find(&marker)
        .ok_or_else(|| format!("C.6 `{name}` contract is missing"))?;
    let tail = &source[start..];
    let end = tail
        .find("\n}")
        .ok_or_else(|| format!("C.6 `{name}` contract is malformed"))?
        + 2;
    Ok(&tail[..end])
}

fn occurrence_is_return_type(source: &str, offset: usize) -> bool {
    source[..offset]
        .rsplit_once('\n')
        .map_or(&source[..offset], |(_, line)| line)
        .contains("->")
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
