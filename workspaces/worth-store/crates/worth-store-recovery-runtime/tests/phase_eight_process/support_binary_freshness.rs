use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use worth_store_process_bundle::FreshRecoveryProcessBundle;

#[path = "support_binary_freshness/source_edit.rs"]
mod source_edit;
#[path = "support_binary_freshness/target_directory.rs"]
mod target_directory;

#[test]
fn dependency_only_changes_rebuild_every_production_executable() {
    let process_lane = super::process_lane::acquire().expect("acquire freshness process lane");
    let workspace = workspace_root();
    let repository = repository_root(&workspace);
    let source = repository.join("crates/worth-foundational/src/lib.rs");
    let original = std::fs::read(&source).expect("read freshness source before proof");
    let proof = run_freshness_proof();
    let source_after = std::fs::read(&source);
    let lock_exists = repository.join(".phase-eight-freshness.lock").exists();
    let proof_failure = match proof {
        Err(error) => Some(format!("freshness proof failed: {error}")),
        Ok(()) => match source_after.as_ref() {
            Err(error) => Some(format!("read freshness source after proof: {error}")),
            Ok(bytes) if bytes != &original || lock_exists => Some(format!(
                "MUTANT_PREDICATE:c8-freshness-cleanup-errors-visible source_restored={} lock_released={}",
                bytes == &original,
                !lock_exists
            )),
            Ok(_) => None,
        },
    };
    if let Err(error) = restore_mutation_residue(&repository, &source, &original) {
        panic!("freshness mutation isolation cleanup failed: {error}");
    }
    let lane_failure = process_lane.close().err();
    match (proof_failure, lane_failure) {
        (None, None) => {}
        (Some(proof), None) => panic!("{proof}"),
        (None, Some(lane)) => panic!("freshness process-lane cleanup failed: {lane}"),
        (Some(proof), Some(lane)) => {
            panic!("{proof}; freshness process-lane cleanup failed: {lane}")
        }
    }
}

fn restore_mutation_residue(
    repository: &Path,
    source: &Path,
    original: &[u8],
) -> Result<(), String> {
    std::fs::write(source, original)
        .map_err(|error| format!("restore freshness source bytes: {error}"))?;
    if std::fs::read(source).map_err(|error| format!("verify freshness source bytes: {error}"))?
        != original
    {
        return Err("freshness source bytes differed after isolation cleanup".to_owned());
    }
    let lock = repository.join(".phase-eight-freshness.lock");
    if lock.exists() {
        std::fs::remove_file(&lock)
            .map_err(|error| format!("remove freshness isolation lock: {error}"))?;
    }
    if lock.exists() {
        return Err("freshness isolation lock remained after cleanup".to_owned());
    }
    Ok(())
}

fn run_freshness_proof() -> Result<(), String> {
    let workspace = workspace_root();
    let repository = repository_root(&workspace);
    let source = repository.join("crates/worth-foundational/src/lib.rs");
    let mut edit = source_edit::SourceEdit::acquire(&repository, source)?;
    let target = match target_directory::FreshnessTarget::allocate(&workspace) {
        Ok(target) => target,
        Err(error) => return combine(Err(error), edit.finalize(), Ok(())),
    };
    let proof = catch_unwind(AssertUnwindSafe(|| {
        execute_proof(&workspace, &repository, &mut edit, &target)
    }))
    .map_err(panic_message)
    .and_then(std::convert::identity);
    let source_cleanup = edit.finalize();
    let target_cleanup = target.close();
    combine(proof, source_cleanup, target_cleanup)
}

fn execute_proof(
    workspace: &Path,
    repository: &Path,
    edit: &mut source_edit::SourceEdit,
    target: &target_directory::FreshnessTarget,
) -> Result<(), String> {
    let first = FreshRecoveryProcessBundle::build_production(
        workspace,
        repository,
        target.process_target(),
    )
    .map_err(|error| format!("initial production process bundle: {error}"))?;
    let first_digest = [
        first.writer().digest(),
        first.observer().digest(),
        first.recovery().digest(),
    ];
    edit.install_marker(b"\n// phase-eight dependency-only freshness proof marker\n")?;
    let second = FreshRecoveryProcessBundle::build_production(
        workspace,
        repository,
        target.process_target(),
    )
    .map_err(|error| format!("production process bundle after dependency edit: {error}"))?;
    ensure(
        first.source().digest() != second.source().digest(),
        "dependency edit did not change source closure",
    )?;
    ensure(
        first_digest[0] != second.writer().digest(),
        "writer executable did not rebuild",
    )?;
    ensure(
        first_digest[1] != second.observer().digest(),
        "observer executable did not rebuild",
    )?;
    ensure(
        first_digest[2] != second.recovery().digest(),
        "recovery executable did not rebuild",
    )?;
    assert_fresh_executable(second.writer())?;
    assert_fresh_executable(second.observer())?;
    assert_fresh_executable(second.recovery())?;
    ensure(
        has_dependency_freshness(second.writer())
            || has_dependency_freshness(second.observer())
            || has_dependency_freshness(second.recovery()),
        "dependency edit did not appear as a fresh=false Cargo artifact",
    )?;
    ensure(
        feature_contract(first.writer()) == feature_contract(second.writer()),
        "writer feature contract changed during freshness proof",
    )?;
    ensure(
        feature_contract(first.observer()) == feature_contract(second.observer()),
        "observer feature contract changed during freshness proof",
    )?;
    ensure(
        feature_contract(first.recovery()) == feature_contract(second.recovery()),
        "recovery feature contract changed during freshness proof",
    )
}

fn feature_contract<R>(
    artifact: &worth_store_process_bundle::BoundArtifact<R>,
) -> Vec<(String, String, Vec<String>)> {
    let mut contract = artifact
        .compiler_artifacts()
        .iter()
        .map(|record| {
            (
                record.package().to_owned(),
                record.target().to_owned(),
                record.features().to_vec(),
            )
        })
        .collect::<Vec<_>>();
    contract.sort();
    contract
}

fn assert_fresh_executable<R>(
    artifact: &worth_store_process_bundle::BoundArtifact<R>,
) -> Result<(), String> {
    let target_record = artifact
        .compiler_artifacts()
        .iter()
        .find(|record| record.executable().is_some())
        .ok_or_else(|| "final role compiler artifact was absent".to_owned())?;
    ensure(
        target_record.fresh() == Some(false),
        "final role executable was not rebuilt",
    )
}

fn has_dependency_freshness<R>(artifact: &worth_store_process_bundle::BoundArtifact<R>) -> bool {
    artifact
        .compiler_artifacts()
        .iter()
        .any(|record| record.package() == "worth-foundational" && record.fresh() == Some(false))
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("Phase 8 workspace root")
}

fn repository_root(workspace: &Path) -> PathBuf {
    workspace
        .join("../..")
        .canonicalize()
        .expect("Phase 8 repository root")
}

fn ensure(condition: bool, message: &str) -> Result<(), String> {
    condition.then_some(()).ok_or_else(|| message.to_owned())
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    payload.downcast_ref::<&str>().map_or_else(
        || {
            payload
                .downcast_ref::<String>()
                .cloned()
                .unwrap_or_else(|| "freshness proof panicked with a non-string payload".to_owned())
        },
        |message| (*message).to_owned(),
    )
}

fn combine(
    proof: Result<(), String>,
    source: Result<(), String>,
    target: Result<(), String>,
) -> Result<(), String> {
    let mut errors = Vec::new();
    for (label, result) in [
        ("proof", proof),
        ("source cleanup", source),
        ("target cleanup", target),
    ] {
        if let Err(error) = result {
            errors.push(format!("{label}: {error}"));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}
