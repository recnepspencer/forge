use std::fs;
use std::path::PathBuf;

fn production_rust_files(path: &std::path::Path) -> Vec<PathBuf> {
    fs::read_dir(path)
        .expect("directory exists")
        .filter_map(Result::ok)
        .flat_map(|entry| {
            let path = entry.path();
            if entry.file_type().expect("file type is readable").is_dir() {
                production_rust_files(&path)
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                vec![path]
            } else {
                Vec::new()
            }
        })
        .filter(|file| {
            file.file_name()
                .is_none_or(|name| name != std::ffi::OsStr::new("structure_guard.rs"))
                && !file
                    .components()
                    .any(|component| component.as_os_str() == "public_facade_contracts")
        })
        .collect()
}

#[test]
fn spatial_crate_remains_kernel_dependency_pure() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cargo_toml =
        fs::read_to_string(manifest_dir.join("Cargo.toml")).expect("Cargo.toml is readable");
    let production_dependencies = cargo_toml
        .split("[dev-dependencies]")
        .next()
        .expect("Cargo.toml should declare production dependencies before dev-dependencies");

    assert!(
        !production_dependencies.contains("worth-kernel"),
        "worth-spatial must not depend on worth-kernel"
    );
    assert!(
        !production_dependencies.contains("worth_kernel"),
        "worth-spatial must not depend on worth-kernel under renamed package syntax"
    );

    let source_violations = production_rust_files(&manifest_dir.join("src"))
        .into_iter()
        .filter_map(|file| {
            let text = fs::read_to_string(&file).expect("source file is readable");
            if text.contains("worth_kernel::") || text.contains("worth-kernel") {
                Some(file.display().to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    assert!(
        source_violations.is_empty(),
        "worth-spatial production code must not import or mention worth-kernel: {source_violations:?}"
    );
}
