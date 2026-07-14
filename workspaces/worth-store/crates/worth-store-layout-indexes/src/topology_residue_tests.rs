fn rust_sources_below(path: &std::path::Path, found: &mut Vec<std::path::PathBuf>) {
    for entry in std::fs::read_dir(path).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_sources_below(&path, found);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(path);
        }
    }
}

fn production_sources() -> Vec<std::path::PathBuf> {
    let mut sources = Vec::new();
    rust_sources_below(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut sources,
    );
    sources.retain(|path| {
        !path
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .is_some_and(|stem| stem.ends_with("test") || stem.ends_with("tests"))
    });
    sources
}

fn authors_module_behavior(line: &str) -> bool {
    let line = line.trim();
    line.starts_with("impl ")
        || line.starts_with("fn ")
        || line.starts_with("pub fn ")
        || line.starts_with("pub(crate) fn ")
        || line.starts_with("pub(super) fn ")
        || line.starts_with("pub struct ")
        || line.starts_with("pub enum ")
        || line.starts_with("struct ")
        || line.starts_with("enum ")
}

fn exposes_public_field(line: &str) -> bool {
    let line = line.trim();
    line.starts_with("pub ")
        && line.contains(':')
        && !line.starts_with("pub fn ")
        && !line.starts_with("pub const ")
        && !line.starts_with("pub type ")
        && !line.starts_with("pub use ")
        && !line.starts_with("pub mod ")
        && !line.starts_with("pub struct ")
        && !line.starts_with("pub enum ")
        && !line.starts_with("pub trait ")
}

fn contains_forbidden_vocabulary(source: &str) -> bool {
    let forbidden_prefixes = [concat!("S", "8"), concat!("S", "9")];
    let forbidden_module_prefixes = [concat!("s", "8", "_"), concat!("s", "9", "_")];
    forbidden_module_prefixes
        .iter()
        .any(|prefix| source.contains(prefix))
        || source
            .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
            .any(|word| {
                forbidden_prefixes.iter().any(|prefix| {
                    word.strip_prefix(prefix).is_some_and(|suffix| {
                        suffix
                            .chars()
                            .next()
                            .is_some_and(|character| character.is_ascii_alphabetic())
                    })
                }) || word.strip_prefix("Milestone").is_some_and(|suffix| {
                    suffix
                        .chars()
                        .next()
                        .is_some_and(|character| character.is_ascii_digit())
                })
            })
}

#[test]
fn production_vocabulary_contains_no_milestone_identifiers() {
    for path in production_sources() {
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(
            !contains_forbidden_vocabulary(&text),
            "production milestone vocabulary remains in {}",
            path.display()
        );
    }
}

#[test]
fn rust_file_topology_contains_no_roadmap_ordering() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut sources = Vec::new();
    rust_sources_below(&root.join("src"), &mut sources);
    rust_sources_below(&root.join("tests"), &mut sources);

    for path in sources {
        let name = path.file_name().and_then(std::ffi::OsStr::to_str).unwrap();
        let normalized = name.to_ascii_lowercase();
        assert!(
            !normalized.contains("phase")
                && !normalized.contains("milestone")
                && !normalized.starts_with("s8_")
                && !normalized.starts_with("s9_"),
            "Rust file topology retains roadmap ordering in {}",
            path.display()
        );
    }
}

#[test]
fn native_declarations_cannot_become_serde_authority() {
    for path in production_sources() {
        let text = std::fs::read_to_string(&path).unwrap();
        for forbidden in [
            concat!("ser", "de::"),
            concat!("Serial", "ize"),
            concat!("Deserial", "ize"),
            concat!("serde", "_json"),
        ] {
            assert!(
                !text.contains(forbidden),
                "layout production source {} contains serde authority marker {forbidden}",
                path.display()
            );
        }
    }
}

#[test]
fn current_store_authority_cannot_skip_family_and_domain_admission() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for relative in [
        "src/artifact_family/admission.rs",
        "src/keyspace/admission.rs",
        "src/keyspace/request_identity.rs",
        "src/planning/access_request.rs",
        "src/strategy/authority_basis.rs",
    ] {
        let text = std::fs::read_to_string(root.join(relative)).unwrap();
        assert!(
            !text.contains("StoreCurrentAuthorityWitness"),
            "{relative} accepts current Store authority beside copied admission values"
        );
    }
}

#[test]
fn permanent_public_topology_has_no_compatibility_roots() {
    let source_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let compatibility_roots = [
        "layout_bootstrap.rs",
        "layout_customization.rs",
        "layout_families.rs",
        "layout_integrity.rs",
        "layout_maintenance.rs",
        "layout_materialization.rs",
        "layout_migration.rs",
        "layout_observation.rs",
        "layout_rebuild.rs",
        "layout_strategy_admission.rs",
    ];

    for compatibility_root in compatibility_roots {
        assert!(
            !source_root.join(compatibility_root).exists(),
            "compatibility root {compatibility_root} displaced its permanent owner"
        );
    }

    let library = std::fs::read_to_string(source_root.join("lib.rs")).unwrap();
    assert!(!library.contains("pub mod layout_"));
}

#[test]
fn module_and_facade_files_are_composition_only() {
    for path in production_sources() {
        let name = path.file_name().and_then(std::ffi::OsStr::to_str).unwrap();
        if name == "facade.rs" {
            panic!(
                "business-neutral facades must be named for their semantic entrypoint: {}",
                path.display()
            );
        }
        if name != "mod.rs" {
            continue;
        }

        let source = std::fs::read_to_string(&path).unwrap();
        for line in source.lines().map(str::trim) {
            assert!(
                !authors_module_behavior(line),
                "module aggregator {} authors behavior through {line:?}",
                path.display()
            );
        }
    }
}

#[test]
fn production_authority_types_expose_no_public_fields() {
    for path in production_sources() {
        let source = std::fs::read_to_string(&path).unwrap();
        for line in source.lines().map(str::trim) {
            assert!(
                !exposes_public_field(line),
                "production authority field is public in {}: {line}",
                path.display()
            );
        }
    }
}

#[test]
fn production_read_path_has_no_crate_wide_outcome_issuer() {
    let planning_exports = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/planning.rs"),
    )
    .unwrap();
    assert!(
        !planning_exports.contains("SelectedAccessPlanBasis"),
        "generic selected-plan basis escaped the planning owner"
    );

    for path in production_sources() {
        let source = std::fs::read_to_string(&path).unwrap();
        for line in source.lines().map(str::trim) {
            assert!(
                !line.starts_with("pub(crate) fn issue")
                    && !line.starts_with("pub(crate) const fn issue"),
                "crate-wide outcome issuer escapes its owner in {}: {line}",
                path.display()
            );
        }
    }

    for owner in [
        "src/artifact_family/admission.rs",
        "src/keyspace/admission.rs",
        "src/materialization/admission.rs",
        "src/bootstrap/catalog_read_outcome.rs",
        "src/strategy/registry/admission_operation.rs",
    ] {
        let source =
            std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(owner))
                .unwrap();
        assert!(
            !source.contains("pub(crate) fn admit")
                && !source.contains("pub(crate) const fn admit")
                && !source.contains("pub(crate) fn issue")
                && !source.contains("pub(crate) const fn issue"),
            "owner admission file exposes a crate-wide authority constructor: {owner}"
        );
    }

    assert!(
        !std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/strategy/registry/snapshot.rs")
            .exists(),
        "strategy snapshot construction must remain co-located with registry admission"
    );
}

#[test]
fn residue_classifiers_reject_known_bad_mutations() {
    for mutation in [
        "impl LayoutFacade {",
        "pub fn classify(input: Input) -> Outcome {",
        "pub(crate) fn issue(case: Case) -> Outcome {",
        "pub struct ParallelAuthority {",
        "enum HiddenCase {",
    ] {
        assert!(
            authors_module_behavior(mutation),
            "aggregator mutation escaped the behavior classifier: {mutation}"
        );
    }
    for aggregation in [
        "mod admission;",
        "pub use admission::AdmittedRequest;",
        "#[cfg(test)]",
    ] {
        assert!(!authors_module_behavior(aggregation));
    }

    assert!(exposes_public_field("pub authority: AuthorityWitness,"));
    assert!(exposes_public_field("pub case: OwnerCase,"));
    assert!(!exposes_public_field("pub struct OwnerOutcome {"));
    assert!(!exposes_public_field(
        "pub fn authority(&self) -> &AuthorityWitness {"
    ));

    for forbidden in [
        "pub mod layout_integrity;",
        "pub mod layout_maintenance;",
        "pub mod layout_strategy_admission;",
        "pub struct S8OwnerOutcome;",
        "mod s9_transition_inventory;",
    ] {
        assert!(
            forbidden.contains("layout_") || contains_forbidden_vocabulary(forbidden),
            "known-bad vocabulary mutation escaped its classifier: {forbidden}"
        );
    }
}
