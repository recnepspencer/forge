use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::corpus_contract::{BoundaryFixture, Corpus, Enforcement, Specimen};

static FIXTURE_ID: AtomicU64 = AtomicU64::new(0);

pub fn run_boundary_check_cases(corpus: &Corpus) {
    for specimen in corpus.rows() {
        let Enforcement::BoundaryCheck(fixture) = specimen.enforcement else {
            continue;
        };
        run_case(corpus, specimen, fixture);
    }
}

fn run_case(corpus: &Corpus, specimen: &Specimen, fixture: BoundaryFixture) {
    let root = fixture_root(specimen.path);
    match fixture {
        BoundaryFixture::QueryAudience { band, facade, item } => {
            crate::query_audience_repository::assemble(&root, band, facade, item);
        }
        BoundaryFixture::Entry { .. } => {
            crate::entry_governed_repository::assemble(&root);
        }
    }
    let legal = run_boundary_check(corpus, &root, true);
    assert!(
        legal.status.success(),
        "legal control failed for {}:\n{}",
        specimen.path,
        String::from_utf8_lossy(&legal.stderr)
    );
    match fixture {
        BoundaryFixture::QueryAudience { band, facade, .. } => {
            crate::query_audience_repository::install_hostile(
                &root, corpus, specimen, band, facade,
            );
        }
        BoundaryFixture::Entry { dependency } => {
            crate::entry_governed_repository::install_hostile(&root, corpus, specimen, dependency);
        }
    }
    let hostile = run_boundary_check(corpus, &root, false);
    let output = format!(
        "{}{}",
        String::from_utf8_lossy(&hostile.stdout),
        String::from_utf8_lossy(&hostile.stderr)
    );
    let _ = fs::remove_dir_all(&root);
    assert!(
        !hostile.status.success(),
        "{} unexpectedly passed",
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

fn run_boundary_check(corpus: &Corpus, root: &Path, update: bool) -> std::process::Output {
    let mut command = Command::new("cargo");
    command
        .args(["run", "--quiet", "--manifest-path"])
        .arg(
            corpus
                .repository_root()
                .join("tools/boundary-check/Cargo.toml"),
        )
        .args(["--", "--root"])
        .arg(root)
        .args(["--config", "tools/boundary-check/config/road1.toml"]);
    if update {
        command.arg("--update-snapshots");
    }
    command.output().expect("run production boundary-check")
}

fn fixture_root(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "worth-cert-adoption-{}-{}-{}",
        label.trim_end_matches(".rs"),
        std::process::id(),
        FIXTURE_ID.fetch_add(1, Ordering::Relaxed)
    ))
}
