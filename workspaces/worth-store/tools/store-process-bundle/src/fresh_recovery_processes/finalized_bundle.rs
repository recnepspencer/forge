use std::path::{Path, PathBuf};

use super::target_directory::{target_parent, FreshProcessCargoTarget};
use super::{build as build_bundle, FreshRecoveryProcessBundle};

#[path = "finalized_bundle/directory.rs"]
mod directory;
#[path = "finalized_bundle/promotion.rs"]
mod promotion;

use directory::{allocate as allocate_directory, remove as remove_directory};
use promotion::promote;

pub const FINALIZED_WRITER_ENV: &str = "WORTH_STORE_PHASE8_FINALIZED_WRITER";
pub const FINALIZED_OBSERVER_ENV: &str = "WORTH_STORE_PHASE8_FINALIZED_OBSERVER";
pub const FINALIZED_RECOVERY_ENV: &str = "WORTH_STORE_PHASE8_FINALIZED_RECOVERY";

pub struct FinalizedFreshRecoveryProcessBundle {
    bundle: Option<FreshRecoveryProcessBundle>,
    directory: PathBuf,
}

pub(super) fn build(
    recipe: super::targets::Recipe,
    workspace: &Path,
    repository: &Path,
) -> Result<FinalizedFreshRecoveryProcessBundle, String> {
    build_at(recipe, workspace, repository, &target_parent(workspace))
}

pub(super) fn build_at(
    recipe: super::targets::Recipe,
    workspace: &Path,
    repository: &Path,
    parent: &Path,
) -> Result<FinalizedFreshRecoveryProcessBundle, String> {
    let scratch = FreshProcessCargoTarget::allocate(parent)?;
    let bundle = match build_bundle(recipe, workspace, repository, &scratch) {
        Ok(bundle) => bundle,
        Err(error) => return finish_build_error(error, scratch),
    };
    let final_directory = match allocate_directory(parent) {
        Ok(directory) => directory,
        Err(error) => return finish_finalization_error(error, None, scratch),
    };
    let promoted = match promote(&bundle, &final_directory) {
        Ok(promoted) => promoted,
        Err(error) => return finish_finalization_error(error, Some(&final_directory), scratch),
    };
    let finalized = FreshRecoveryProcessBundle {
        cargo: bundle.cargo,
        workspace: bundle.workspace,
        repository: bundle.repository,
        recipe: bundle.recipe,
        source: bundle.source,
        feature_roots: bundle.feature_roots,
        feature_graph: bundle.feature_graph,
        writer: promoted.writer,
        observer: promoted.observer,
        recovery: promoted.recovery,
        timings: bundle.timings,
    };
    match scratch.close() {
        Ok(()) => Ok(FinalizedFreshRecoveryProcessBundle {
            bundle: Some(finalized),
            directory: final_directory,
        }),
        Err(close_error) => Err(combine_errors(
            close_error,
            remove_directory(&final_directory),
        )),
    }
}

impl FinalizedFreshRecoveryProcessBundle {
    pub fn bundle(&self) -> &FreshRecoveryProcessBundle {
        self.bundle
            .as_ref()
            .expect("finalized process bundle is available before finish")
    }

    pub fn finish<T>(mut self, result: Result<T, String>) -> Result<T, String> {
        let mut value = None;
        let mut errors = Vec::new();
        match result {
            Ok(item) => value = Some(item),
            Err(error) => errors.push(error),
        }
        if let Some(bundle) = self.bundle.as_ref() {
            if let Err(error) = bundle.verify_source_unchanged() {
                errors.push(format!(
                    "finalized process-bundle source verification failed: {error}"
                ));
            }
            if let Err(error) = bundle.verify_executables_unchanged() {
                errors.push(format!(
                    "finalized process-bundle executable verification failed: {error}"
                ));
            }
        }
        drop(self.bundle.take());
        if let Err(error) = remove_directory(&self.directory) {
            errors.push(error);
        }
        if self.directory.exists() {
            errors.push(format!(
                "finalized process-bundle directory survived close: {}",
                self.directory.display()
            ));
        }
        if errors.is_empty() {
            Ok(value.expect("successful finalized bundle result retains its value"))
        } else {
            Err(errors.join("; "))
        }
    }

    pub fn close(self) -> Result<(), String> {
        self.finish(Ok(()))
    }

    pub fn install_environment(&self, command: &mut std::process::Command) {
        command
            .env(FINALIZED_WRITER_ENV, self.bundle().writer().path())
            .env(FINALIZED_OBSERVER_ENV, self.bundle().observer().path())
            .env(FINALIZED_RECOVERY_ENV, self.bundle().recovery().path());
    }
}

impl Drop for FinalizedFreshRecoveryProcessBundle {
    fn drop(&mut self) {
        if self.directory.exists() {
            let _ = remove_directory(&self.directory);
        }
    }
}

fn finish_build_error(
    build_error: String,
    scratch: FreshProcessCargoTarget,
) -> Result<FinalizedFreshRecoveryProcessBundle, String> {
    Err(combine_errors(build_error, scratch.close()))
}

fn finish_finalization_error(
    error: String,
    directory: Option<&Path>,
    scratch: FreshProcessCargoTarget,
) -> Result<FinalizedFreshRecoveryProcessBundle, String> {
    let cleanup = directory.map_or(Ok(()), remove_directory);
    Err(combine_errors(
        combine_errors(error, cleanup),
        scratch.close(),
    ))
}

fn combine_errors(primary: String, cleanup: Result<(), String>) -> String {
    match cleanup {
        Ok(()) => primary,
        Err(cleanup) => format!("{primary}; cleanup failed: {cleanup}"),
    }
}

#[cfg(test)]
#[path = "finalized_bundle/tests.rs"]
mod tests;
