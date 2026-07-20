#[test]
fn media_authority_and_raw_surfaces_are_compiler_sealed() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/physical_media_authority/supported_media_admission.rs");
    cases.compile_fail("tests/physical_media_authority/media_runtime_authority_is_sealed.rs");
    cases.compile_fail("tests/physical_media_authority/non_authority_values_cannot_promote.rs");
    cases.compile_fail("tests/physical_media_authority/raw_media_surface_is_private.rs");
    cases.compile_fail("tests/physical_media_authority/optional_capabilities_require_handles.rs");
    cases.compile_fail("tests/physical_media_authority/maximal_features_cannot_mint_authority.rs");
}

#[test]
fn ordinary_runtime_sources_preserve_the_physical_media_negative_space() {
    let source_root = canonical_runtime_source_root();
    let forbidden = [
        ("certification authority", "worth_store_certification"),
        ("in-memory physical model", "InMemoryPhysicalFormatModel"),
        (
            "in-memory replay authority",
            "InMemoryPhysicalFormatReplayArtifact",
        ),
        ("persisted heap layout", "PersistedPhysicalLayout"),
        ("raw filesystem effect", "std::fs::"),
    ];

    let mut pending = vec![source_root];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).expect("physical runtime source is readable") {
            let path = entry.expect("source entry is readable").path();
            if path.is_dir() {
                pending.push(path);
                continue;
            }
            if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
                continue;
            }
            let source = std::fs::read_to_string(&path).expect("Rust source is readable");
            for (authority_class, token) in forbidden {
                assert!(
                    !source.contains(token),
                    "{} imports forbidden {} through token `{}`",
                    path.display(),
                    authority_class,
                    token
                );
            }
        }
    }
}

#[test]
fn canonical_media_admission_is_the_only_runtime_backend_constructor() {
    let source_root = canonical_runtime_source_root();
    let admitted_source = source_root.join("media_ownership").join("admission.rs");
    let mut constructor_sites = Vec::new();
    let mut pending = vec![source_root];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                let source = std::fs::read_to_string(&path).unwrap();
                if source.contains("qualify_filesystem_media(request)") {
                    constructor_sites.push(path.clone());
                }
                for forbidden in ["StoreDurabilityRuntime", "InMemoryPhysicalFormatModel"] {
                    assert!(
                        !source.contains(forbidden),
                        "{} reaches {forbidden}",
                        path.display()
                    );
                }
            }
        }
    }
    assert_eq!(constructor_sites, [admitted_source]);

    let backend_source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("worth-store has a crates parent")
        .join("worth-store-physical-backend")
        .join("src")
        .join("filesystem_media")
        .join("admission.rs");
    let backend_source = std::fs::read_to_string(backend_source).unwrap();
    assert!(backend_source.contains("pub(crate) fn qualify("));
    assert!(!backend_source.contains("pub fn qualify("));

    let backend_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("worth-store has a crates parent")
        .join("worth-store-physical-backend")
        .join("src");
    let backend_runtime = backend_root.join("physical_runtime");
    let backend_runtime_sources =
        backend_runtime.exists() && contains_rust_source(&backend_runtime);
    assert!(
        !backend_runtime_sources,
        "the backend must not own a parallel runtime composition tree"
    );
    let backend_lib = std::fs::read_to_string(backend_root.join("lib.rs")).unwrap();
    assert!(
        !backend_lib.contains("physical_runtime"),
        "the backend must not export runtime phase promotion"
    );
}

fn contains_rust_source(directory: &std::path::Path) -> bool {
    std::fs::read_dir(directory)
        .unwrap()
        .filter_map(Result::ok)
        .any(|entry| {
            let path = entry.path();
            if path.is_dir() {
                contains_rust_source(&path)
            } else {
                path.extension().and_then(|value| value.to_str()) == Some("rs")
            }
        })
}

fn canonical_runtime_source_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("physical_runtime")
}
