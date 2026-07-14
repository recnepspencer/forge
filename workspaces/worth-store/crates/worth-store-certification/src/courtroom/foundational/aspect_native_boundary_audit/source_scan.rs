use std::{
    fs,
    path::{Path, PathBuf},
};

use super::AspectNativeBoundaryAuditDenial;

pub(crate) struct AspectNativeBoundarySurfaceCounts {
    pub(crate) current_residue_scan: usize,
    pub(crate) terminal_projection_boundary: usize,
    pub(crate) foundational_adoption: usize,
    pub(crate) public_facade: usize,
    pub(crate) native_harness: usize,
}

pub(crate) fn scan_current_aspect_native_boundary_surfaces(
) -> Result<AspectNativeBoundarySurfaceCounts, AspectNativeBoundaryAuditDenial> {
    let root = repository_root()?;
    let source_files = collect_current_source_files(&root)?;
    Ok(AspectNativeBoundarySurfaceCounts {
        current_residue_scan: scan_for_json_residue_occurrences(&root, &source_files)?,
        terminal_projection_boundary: terminal_projection_boundary_count(&root, &source_files)?,
        foundational_adoption: foundational_adoption_family_count(&root)?,
        public_facade: public_facade_surface_count(&root)?,
        native_harness: native_harness_surface_count(&root)?,
    })
}

fn scan_for_json_residue_occurrences(
    root: &Path,
    source_files: &[PathBuf],
) -> Result<usize, AspectNativeBoundaryAuditDenial> {
    let mut occurrence_count = 0usize;
    for source_file in source_files {
        let source = fs::read_to_string(source_file).map_err(source_read_failed)?;
        let relative_path = repository_relative_path(root, source_file)?;
        occurrence_count += source
            .lines()
            .filter(|line| line_has_json_residue_token(&relative_path, line))
            .count();
    }
    Ok(occurrence_count)
}

fn terminal_projection_boundary_count(
    root: &Path,
    source_files: &[PathBuf],
) -> Result<usize, AspectNativeBoundaryAuditDenial> {
    let mut boundary_count = 0usize;
    for source_file in source_files {
        let relative_path = repository_relative_path(root, source_file)?;
        if !is_terminal_projection_boundary_file(&relative_path) {
            continue;
        }
        let source = fs::read_to_string(source_file).map_err(source_read_failed)?;
        boundary_count += source
            .lines()
            .filter(|line| line_has_json_residue_token(&relative_path, line))
            .count();
    }
    Ok(boundary_count)
}

fn foundational_adoption_family_count(
    root: &Path,
) -> Result<usize, AspectNativeBoundaryAuditDenial> {
    let source = read_workspace_file(
        root,
        "workspaces/worth-store/crates/worth-store-readiness/src/foundational_adoption.rs",
    )?;
    let required_block = source
        .split("pub const fn required_for_physical_format() -> [Self; 6]")
        .nth(1)
        .ok_or(AspectNativeBoundaryAuditDenial::MissingFoundationalAdoption)?;

    Ok([
        "Canonicalization",
        "Diagnostics",
        "Profiles",
        "BoundaryEvidence",
        "ProvenanceReceipts",
        "Performance",
    ]
    .into_iter()
    .filter(|family| required_block.contains(family))
    .count())
}

fn public_facade_surface_count(root: &Path) -> Result<usize, AspectNativeBoundaryAuditDenial> {
    let facade = read_workspace_file(root, "workspaces/worth-store/crates/worth-store/src/lib.rs")?;
    let aspect_native = facade_module_body(&facade, "aspect_native")?;
    let certification = facade_module_body(&facade, "certification")?;
    let terminal_projection = facade_module_body(&facade, "terminal_projection")?;

    require_module_exports(
        aspect_native,
        &[
            "StoreAspectBoundaryFact",
            "StorePhysicalBoundaryWitness",
            "StorePhysicalAuthorityWitness",
        ],
    )?;
    require_module_exports(
        certification,
        &[
            "certify_store_json_residue_inventory",
            "StoreJsonResidueInventory",
            "StoreJsonResidueDenial",
        ],
    )?;
    require_module_exports(
        terminal_projection,
        &[
            "project_store_boundary_fact_to_terminal_json",
            "StoreTerminalJsonProjection",
        ],
    )?;
    Ok(3)
}

fn native_harness_surface_count(root: &Path) -> Result<usize, AspectNativeBoundaryAuditDenial> {
    let source = read_workspace_file(
        root,
        "workspaces/worth-store/crates/worth-store-test-support/src/native_aspect_fixture_authoring.rs",
    )?;
    Ok(["segment_header", "scalar_string"]
        .into_iter()
        .filter(|surface| source.contains(surface))
        .count())
}

fn collect_current_source_files(
    root: &Path,
) -> Result<Vec<PathBuf>, AspectNativeBoundaryAuditDenial> {
    let mut files = Vec::new();
    collect_source_files(&root.join("crates/worth-store"), &mut files)?;
    collect_source_files(&root.join("workspaces/worth-store"), &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_source_files(
    path: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), AspectNativeBoundaryAuditDenial> {
    let metadata = fs::metadata(path).map_err(source_read_failed)?;
    if metadata.is_dir() {
        let mut entries = fs::read_dir(path)
            .map_err(source_read_failed)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(source_read_failed)?;
        entries.sort_by_key(|entry| entry.path());
        for entry in entries {
            let child = entry.path();
            if is_skipped_dir(&child) {
                continue;
            }
            collect_source_files(&child, files)?;
        }
        return Ok(());
    }
    if is_scanned_source_file(path) {
        files.push(path.to_path_buf());
    }
    Ok(())
}

fn line_has_json_residue_token(path: &str, line: &str) -> bool {
    line.contains(&["serde", "json"].join("_"))
        || line.contains(&["json", "!"].join(""))
        || contains_word(line, &["Serial", "ize"].join(""))
        || contains_word(line, &["De", "serialize"].join(""))
        || raw_json_helper_tokens()
            .iter()
            .any(|needle| line.contains(needle))
        || is_terminal_projection_dependency_line(path, line)
}

fn raw_json_helper_tokens() -> [String; 11] {
    [
        ["canonical", "json"].join("_"),
        ["semantic", "json"].join("_"),
        ["stable", "json", "digest"].join("_"),
        ["to", "canonical", "json", "bytes"].join("_"),
        ["validate", "canonical", "json", "bytes"].join("_"),
        ["payload", "json"].join("_"),
        ["deserialize", "json"].join("_"),
        ["deserialize", "optional", "json"].join("_"),
        ["serialize", "optional", "json"].join("_"),
        ["Json", "Document"].join(""),
        ["json", "document"].join("_"),
    ]
}

fn is_terminal_projection_dependency_line(path: &str, line: &str) -> bool {
    path == "workspaces/worth-store/crates/worth-store-aspect-native/Cargo.toml"
        && line.contains(&["serde", "json"].join("_"))
}

fn contains_word(line: &str, word: &str) -> bool {
    let mut search_start = 0;
    while let Some(offset) = line[search_start..].find(word) {
        let start = search_start + offset;
        let end = start + word.len();
        if is_boundary(line, start, end) {
            return true;
        }
        search_start = end;
    }
    false
}

fn is_boundary(line: &str, start: usize, end: usize) -> bool {
    let before = line[..start].chars().next_back();
    let after = line[end..].chars().next();
    !is_word_char(before) && !is_word_char(after)
}

fn is_word_char(character: Option<char>) -> bool {
    character.is_some_and(|value| value.is_ascii_alphanumeric() || value == '_')
}

fn is_terminal_projection_boundary_file(path: &str) -> bool {
    matches!(
        path,
        "workspaces/worth-store/crates/worth-store-aspect-native/Cargo.toml"
            | "workspaces/worth-store/crates/worth-store-aspect-native/src/terminal_json_projection.rs"
    )
}

fn facade_module_body<'a>(
    facade: &'a str,
    module_name: &'static str,
) -> Result<&'a str, AspectNativeBoundaryAuditDenial> {
    let module_header = format!("pub mod {module_name} {{");
    let Some(module_start) = facade.find(&module_header) else {
        return Err(AspectNativeBoundaryAuditDenial::MissingPublicFacadeProof);
    };
    let body_start = module_start + module_header.len();
    let mut depth = 1usize;
    for (offset, character) in facade[body_start..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(&facade[body_start..body_start + offset]);
                }
            }
            _ => {}
        }
    }
    Err(AspectNativeBoundaryAuditDenial::MissingPublicFacadeProof)
}

fn require_module_exports(
    module_body: &str,
    expected_exports: &[&'static str],
) -> Result<(), AspectNativeBoundaryAuditDenial> {
    if expected_exports
        .iter()
        .all(|expected| module_body.contains(expected))
    {
        Ok(())
    } else {
        Err(AspectNativeBoundaryAuditDenial::MissingPublicFacadeProof)
    }
}

fn read_workspace_file(
    root: &Path,
    relative: &str,
) -> Result<String, AspectNativeBoundaryAuditDenial> {
    fs::read_to_string(root.join(relative)).map_err(source_read_failed)
}

fn repository_relative_path(
    root: &Path,
    path: &Path,
) -> Result<String, AspectNativeBoundaryAuditDenial> {
    path.strip_prefix(root)
        .map(|relative| relative.to_string_lossy().replace('\\', "/"))
        .map_err(source_read_failed)
}

fn repository_root() -> Result<PathBuf, AspectNativeBoundaryAuditDenial> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            AspectNativeBoundaryAuditDenial::SourceReadFailed("repository root".to_string())
        })
}

fn is_scanned_source_file(path: &Path) -> bool {
    path.extension().is_some_and(|extension| extension == "rs")
        || path.file_name().is_some_and(|name| name == "Cargo.toml")
}

fn is_skipped_dir(path: &Path) -> bool {
    path.file_name().is_some_and(|name| {
        matches!(
            name.to_string_lossy().as_ref(),
            "target" | ".git" | ".idea" | ".vscode"
        )
    })
}

fn source_read_failed(error: impl ToString) -> AspectNativeBoundaryAuditDenial {
    AspectNativeBoundaryAuditDenial::SourceReadFailed(error.to_string())
}
