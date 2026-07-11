use crate::courtroom::source_tree::store_crate_source;
use std::{fs, path::PathBuf};

#[test]
fn raw_value_cannot_satisfy_store_authority() {
    let crate_source = aspect_native_source("lib.rs");
    assert!(crate_source.contains("```compile_fail"));
    assert!(crate_source.contains("StoreAspectBoundaryFact::from_raw_value(raw)"));
    assert!(crate_source.contains("StoreAspectAuthorityInput::new(raw_struct)"));

    for source in aspect_native_authority_sources() {
        let contents = fs::read_to_string(&source).unwrap();
        assert!(
            !contents.contains("from_raw_value"),
            "{} exposes raw value authority",
            source.display()
        );
        assert!(
            !contents.contains("from_unvalidated_struct"),
            "{} exposes unvalidated struct authority",
            source.display()
        );
        assert!(
            !contents.contains("from_terminal_projection"),
            "{} exposes terminal projection authority",
            source.display()
        );
    }
}

#[test]
fn string_and_projection_text_have_no_store_authority_constructors() {
    for source in aspect_native_authority_sources() {
        let contents = fs::read_to_string(&source).unwrap();
        assert!(
            !contents.contains("impl From<String> for StoreAspectIdentity"),
            "{} promotes String into identity",
            source.display()
        );
        assert!(
            !contents.contains("impl From<&str> for StoreAspectIdentity"),
            "{} promotes &str into identity",
            source.display()
        );
        assert!(
            !contents.contains("terminal_projection"),
            "{} names terminal projection inside Phase 2 authority",
            source.display()
        );
    }
}

fn aspect_native_source(file: &str) -> String {
    fs::read_to_string(aspect_native_root().join(file)).unwrap()
}

fn aspect_native_authority_sources() -> Vec<PathBuf> {
    [
        "authority/authoritative_state.rs",
        "authority/authoritative_patch.rs",
        "authority/identity_authority.rs",
        "value_admission.rs",
        "contract_admission.rs",
        "physical_witness.rs",
    ]
    .into_iter()
    .map(|file| aspect_native_root().join(file))
    .collect()
}

fn aspect_native_root() -> PathBuf {
    store_crate_source("forge-store-aspect-native")
}
