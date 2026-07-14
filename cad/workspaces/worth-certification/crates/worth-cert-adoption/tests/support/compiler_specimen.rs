use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::corpus_contract::{CompilerFixture, Corpus, Enforcement, Specimen};

static FIXTURE_ID: AtomicU64 = AtomicU64::new(0);

pub fn run_compiler_cases(corpus: &Corpus) {
    for specimen in corpus
        .rows()
        .iter()
        .filter(|row| matches!(row.enforcement, Enforcement::Rustc(_)))
    {
        run_case(corpus, specimen);
    }
}

fn run_case(corpus: &Corpus, specimen: &Specimen) {
    let Enforcement::Rustc(fixture) = specimen.enforcement else {
        panic!("compiler runner received a non-compiler specimen");
    };
    prepare_fixture_world(corpus, fixture);
    let root = fixture_root(specimen.path);
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("src")).expect("create compiler fixture");
    fs::copy(corpus.specimen_path(specimen), root.join("src/main.rs"))
        .expect("copy compiler specimen");
    install_fixture_dependencies(corpus, &root, fixture);
    fs::write(
        root.join("Cargo.toml"),
        hostile_manifest(corpus, specimen, fixture),
    )
    .expect("write manifest");

    let hostile = cargo_check(&root);
    let output = normalize(&output_text(&hostile), &root, corpus.repository_root());
    let _ = fs::remove_dir_all(&root);
    assert!(
        !hostile.status.success(),
        "{} unexpectedly compiled",
        specimen.path
    );
    for fragment in specimen.fragments {
        assert!(
            output.contains(fragment),
            "{} missing stable fragment {fragment:?}:\n{output}",
            specimen.path
        );
    }
}

fn prepare_fixture_world(corpus: &Corpus, fixture: CompilerFixture) {
    match fixture {
        CompilerFixture::Plain => {}
        CompilerFixture::GovernedAuthorityMismatch => run_forged_substrate_control(corpus),
    }
}

fn install_fixture_dependencies(corpus: &Corpus, root: &Path, fixture: CompilerFixture) {
    match fixture {
        CompilerFixture::Plain => {}
        CompilerFixture::GovernedAuthorityMismatch => {
            install_governed_authority_provider(corpus, root)
        }
    }
}

fn run_forged_substrate_control(corpus: &Corpus) {
    let root = fixture_root("forged_authority_substrate.rs");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("src")).expect("create substrate control fixture");
    fs::write(
        root.join("src/main.rs"),
        include_str!("../fixtures/forged_authority_substrate.rs"),
    )
    .expect("write substrate control source");
    fs::write(root.join("Cargo.toml"), substrate_manifest(corpus))
        .expect("write substrate control manifest");
    let control = cargo_check(&root);
    let output = output_text(&control);
    let _ = fs::remove_dir_all(&root);
    assert!(
        control.status.success(),
        "forged authority must compile against worth-proof alone:\n{output}"
    );
}

fn substrate_manifest(corpus: &Corpus) -> String {
    let root = slash(corpus.repository_root());
    format!(
        r#"[package]
name = "forged-authority-substrate"
version = "0.1.0"
edition = "2021"

[dependencies]
worth-proof = {{ path = "{root}/crates/worth-proof" }}

[workspace]
"#
    )
}

fn hostile_manifest(corpus: &Corpus, specimen: &Specimen, fixture: CompilerFixture) -> String {
    let root = slash(corpus.repository_root());
    let package = specimen.path.trim_end_matches(".rs").replace('_', "-");
    let provider = match fixture {
        CompilerFixture::Plain => "",
        CompilerFixture::GovernedAuthorityMismatch => {
            "worth-cert-governed-authority = { path = \"provider\" }\n"
        }
    };
    format!(
        r#"[package]
name = "{package}"
version = "0.1.0"
edition = "2021"

[dependencies]
worth-proof = {{ path = "{root}/crates/worth-proof" }}
worth-schema-core = {{ path = "{root}/cad/workspaces/worth-contracts/crates/worth-schema-core" }}
{provider}

[workspace]
"#
    )
}

fn install_governed_authority_provider(corpus: &Corpus, fixture: &Path) {
    let provider = fixture.join("provider");
    fs::create_dir_all(provider.join("src")).expect("create governed provider fixture");
    fs::write(
        provider.join("Cargo.toml"),
        format!(
            "[package]\nname = \"worth-cert-governed-authority\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\nworth-proof = {{ path = \"{}/crates/worth-proof\" }}\n",
            slash(corpus.repository_root())
        ),
    )
    .expect("write governed provider manifest");
    fs::write(
        provider.join("src/lib.rs"),
        include_str!("../fixtures/governed_authority_provider.rs"),
    )
    .expect("write governed provider source");
}

fn cargo_check(root: &Path) -> std::process::Output {
    Command::new("cargo")
        .args([
            "check",
            "--quiet",
            "--message-format=short",
            "--manifest-path",
        ])
        .arg(root.join("Cargo.toml"))
        .output()
        .expect("run cargo check")
}

fn output_text(output: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn normalize(output: &str, fixture: &Path, repository: &Path) -> String {
    output
        .replace(&fixture.display().to_string(), "$FIXTURE")
        .replace(&repository.display().to_string(), "$WORKSPACE")
        .replace('\\', "/")
}

fn fixture_root(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "worth-cert-compiler-{}-{}-{}",
        label.trim_end_matches(".rs"),
        std::process::id(),
        FIXTURE_ID.fetch_add(1, Ordering::Relaxed)
    ))
}

fn slash(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
