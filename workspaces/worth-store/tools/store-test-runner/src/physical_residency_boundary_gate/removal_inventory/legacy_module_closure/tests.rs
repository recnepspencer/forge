use std::fs;

use tempfile::tempdir;

use super::discover;

#[test]
fn identifier_free_descendants_of_legacy_directory_modules_are_discovered() {
    let root = tempdir().unwrap();
    let source = root.path().join("crates/example/src/lib.rs");
    write(
        &source,
        "#[cfg(feature = \"legacy-s2-models\")]\nmod legacy;\n",
    );
    let module = root.path().join("crates/example/src/legacy/mod.rs");
    let hidden = root.path().join("crates/example/src/legacy/hidden.rs");
    write(&module, "mod hidden;\n");
    write(&hidden, "pub struct InnocentLookingType;\n");

    let closure = discover(root.path(), &[source]).unwrap();

    assert!(closure.contains_key(&module));
    assert_eq!(
        closure.get(&hidden).unwrap(),
        &std::collections::BTreeSet::from(["legacy-s2-module-closure".to_owned()])
    );
}

#[test]
fn file_style_legacy_modules_include_their_descendant_directory() {
    let root = tempdir().unwrap();
    let source = root.path().join("crates/example/src/lib.rs");
    write(
        &source,
        "#[cfg(feature = \"legacy-certification-models\")]\nmod legacy;\n",
    );
    let module = root.path().join("crates/example/src/legacy.rs");
    let hidden = root.path().join("crates/example/src/legacy/hidden.rs");
    write(&module, "mod hidden;\n");
    write(&hidden, "pub struct CopiedFixture;\n");

    let closure = discover(root.path(), &[source]).unwrap();

    assert!(closure.contains_key(&module));
    assert!(closure.contains_key(&hidden));
}

#[test]
fn multiline_legacy_cfg_attributes_cannot_hide_identifier_free_descendants() {
    let root = tempdir().unwrap();
    let source = root.path().join("crates/example/src/lib.rs");
    write(
        &source,
        "#[cfg(\n    any(\n        test,\n        feature = \"legacy-s2-models\",\n    )\n)]\nmod legacy;\n",
    );
    let module = root.path().join("crates/example/src/legacy/mod.rs");
    let hidden = root.path().join("crates/example/src/legacy/hidden.rs");
    write(&module, "mod hidden;\n");
    write(&hidden, "pub struct InnocentLookingType;\n");

    let closure = discover(root.path(), &[source]).unwrap();

    assert!(closure.contains_key(&module));
    assert!(closure.contains_key(&hidden));
}

#[test]
fn ordinary_modules_are_not_classified_as_legacy_closure() {
    let root = tempdir().unwrap();
    let source = root.path().join("crates/example/src/lib.rs");
    let ordinary = root.path().join("crates/example/src/ordinary.rs");
    write(&source, "mod ordinary;\n");
    write(&ordinary, "pub struct CanonicalOwner;\n");

    assert!(discover(root.path(), &[source]).unwrap().is_empty());
}

#[test]
fn cfg_attr_does_not_reclassify_a_canonical_module_as_legacy() {
    let root = tempdir().unwrap();
    let source = root.path().join("crates/example/src/lib.rs");
    let ordinary = root.path().join("crates/example/src/ordinary.rs");
    write(
        &source,
        "#[cfg_attr(feature = \"legacy-s2-models\", derive(Debug))]\nmod ordinary;\n",
    );
    write(&ordinary, "pub struct CanonicalOwner;\n");

    assert!(discover(root.path(), &[source]).unwrap().is_empty());
}

#[test]
fn path_overrides_include_their_identifier_free_descendant_directory() {
    let root = tempdir().unwrap();
    let source = root.path().join("crates/example/src/lib.rs");
    write(
        &source,
        "#[cfg(feature = \"legacy-s2-models\")]\n#[path = \"alternate/legacy_root.rs\"]\nmod legacy;\n",
    );
    let module = root
        .path()
        .join("crates/example/src/alternate/legacy_root.rs");
    let hidden = root
        .path()
        .join("crates/example/src/alternate/legacy_root/hidden.rs");
    write(&module, "mod hidden;\n");
    write(&hidden, "pub struct InnocentLookingType;\n");

    let closure = discover(root.path(), &[source]).unwrap();

    assert!(closure.contains_key(&module));
    assert!(closure.contains_key(&hidden));
}

#[test]
fn path_overrides_cannot_escape_the_workspace() {
    let root = tempdir().unwrap();
    let workspace = root.path().join("workspace");
    let source = workspace.join("crates/example/src/lib.rs");
    write(
        &source,
        "#[cfg(feature = \"legacy-s2-models\")]\n#[path = \"../../../../outside.rs\"]\nmod legacy;\n",
    );
    write(
        &root.path().join("outside.rs"),
        "pub struct EscapedLegacy;\n",
    );

    let denial = discover(&workspace, &[source]).expect_err("module escape must be denied");

    assert!(denial.contains("escapes Store workspace"), "{denial}");
}

fn write(path: &std::path::Path, contents: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, contents).unwrap();
}
