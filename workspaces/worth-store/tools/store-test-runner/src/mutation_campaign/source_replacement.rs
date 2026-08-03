use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use super::catalog::ControlledMutation;

const SOURCE_WRITE_RETRY_WINDOW: Duration = Duration::from_secs(2);
const SOURCE_WRITE_RETRY_DELAY: Duration = Duration::from_millis(10);

pub(super) struct InstalledSourceMutation {
    source: PathBuf,
    original: Box<[u8]>,
    mutated: Box<[u8]>,
    restored: bool,
}

struct PreparedSourceMutation {
    source: PathBuf,
    original: Box<[u8]>,
    mutated: Box<[u8]>,
}

pub(super) fn validate_bindings(
    workspace: &Path,
    mutations: &[&ControlledMutation],
) -> Result<(), String> {
    for mutation in mutations {
        prepare(workspace, mutation)?;
    }
    Ok(())
}

impl InstalledSourceMutation {
    pub(super) fn apply(workspace: &Path, mutation: &ControlledMutation) -> Result<Self, String> {
        let prepared = prepare(workspace, mutation)?;
        let mut installed = Self {
            source: prepared.source,
            original: prepared.original,
            mutated: prepared.mutated,
            restored: true,
        };
        installed.install(mutation)?;
        Ok(installed)
    }

    pub(super) fn original(&self) -> &[u8] {
        &self.original
    }

    pub(super) fn mutated(&self) -> &[u8] {
        &self.mutated
    }

    pub(super) fn restore_exact(&mut self, mutation: &ControlledMutation) -> Result<(), String> {
        if self.restored {
            return Ok(());
        }
        write_source(&self.source, &self.original)
            .map_err(|error| format!("cannot restore mutant {} source: {error}", mutation.id))?;
        let restored = std::fs::read(&self.source).map_err(|error| {
            format!("cannot verify mutant {} restoration: {error}", mutation.id)
        })?;
        if restored != self.original.as_ref() {
            return Err(format!(
                "mutant {} source restoration changed original bytes",
                mutation.id
            ));
        }
        self.restored = true;
        Ok(())
    }

    fn install(&mut self, mutation: &ControlledMutation) -> Result<(), String> {
        write_source(&self.source, &self.mutated)
            .map_err(|error| format!("cannot install mutant {}: {error}", mutation.id))?;
        self.restored = false;
        let installed = std::fs::read(&self.source).map_err(|error| {
            format!("cannot verify mutant {} installation: {error}", mutation.id)
        })?;
        if installed != self.mutated.as_ref() {
            return Err(format!(
                "mutant {} installation changed replacement bytes",
                mutation.id
            ));
        }
        Ok(())
    }
}

fn prepare(
    workspace: &Path,
    mutation: &ControlledMutation,
) -> Result<PreparedSourceMutation, String> {
    let source = workspace.join(mutation.source);
    let original = std::fs::read(&source)
        .map_err(|error| format!("cannot read mutation source {}: {error}", source.display()))?;
    let text = std::str::from_utf8(&original)
        .map_err(|_| format!("mutation source {} is not valid UTF-8", source.display()))?;
    let occurrences = mutation.source_occurrences(text);
    if occurrences != 1 {
        return Err(format!(
            "mutant {} requires one exact source seam in {}, found {occurrences}",
            mutation.id,
            source.display()
        ));
    }
    let needle = mutation.source_needle(text);
    let replacement = mutation.source_replacement(text);
    let mutated = text
        .replacen(needle.as_ref(), replacement.as_ref(), 1)
        .into_bytes();
    if mutated == original {
        return Err(format!("mutant {} made no source change", mutation.id));
    }
    Ok(PreparedSourceMutation {
        source,
        original: original.into_boxed_slice(),
        mutated: mutated.into_boxed_slice(),
    })
}

impl Drop for InstalledSourceMutation {
    fn drop(&mut self) {
        if !self.restored {
            let _ = write_source(&self.source, &self.original);
        }
    }
}

fn write_source(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    retry_transient_source_lock(|| std::fs::write(path, bytes))
}

fn retry_transient_source_lock(
    mut write: impl FnMut() -> std::io::Result<()>,
) -> std::io::Result<()> {
    let deadline = Instant::now() + SOURCE_WRITE_RETRY_WINDOW;
    loop {
        match write() {
            Ok(()) => return Ok(()),
            Err(error) if is_transient_source_lock(&error) && Instant::now() < deadline => {
                std::thread::sleep(SOURCE_WRITE_RETRY_DELAY);
            }
            Err(error) => return Err(error),
        }
    }
}

fn is_transient_source_lock(error: &std::io::Error) -> bool {
    matches!(error.raw_os_error(), Some(32 | 33 | 1224))
}

#[cfg(test)]
mod tests {
    use super::{retry_transient_source_lock, validate_bindings, InstalledSourceMutation};
    use crate::mutation_campaign::catalog::{ControlledMutation, MutationTarget};

    #[test]
    fn explicit_restore_reinstalls_exact_original_bytes() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source.rs");
        let original = b"before\r\nexact seam\r\nafter\r\n";
        std::fs::write(&source, original).unwrap();
        let mutation = fixture_mutation();

        let mut installed = InstalledSourceMutation::apply(directory.path(), &mutation).unwrap();
        assert_eq!(
            std::fs::read(&source).unwrap(),
            b"before\r\nmutated seam\r\nafter\r\n"
        );
        installed.restore_exact(&mutation).unwrap();

        assert_eq!(std::fs::read(source).unwrap(), original);
    }

    #[test]
    fn dropping_an_installed_mutation_restores_the_source() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source.rs");
        let original = b"exact seam\n";
        std::fs::write(&source, original).unwrap();
        {
            let _installed =
                InstalledSourceMutation::apply(directory.path(), &fixture_mutation()).unwrap();
            assert_eq!(std::fs::read(&source).unwrap(), b"mutated seam\n");
        }
        assert_eq!(std::fs::read(source).unwrap(), original);
    }

    #[test]
    fn binding_preflight_is_read_only_and_uses_execution_preparation() {
        let directory = tempfile::tempdir().unwrap();
        let source = directory.path().join("source.rs");
        let original = b"before\nexact seam\nafter\n";
        std::fs::write(&source, original).unwrap();
        let mutation = fixture_mutation();

        validate_bindings(directory.path(), &[&mutation]).unwrap();

        assert_eq!(std::fs::read(source).unwrap(), original);
    }

    #[test]
    fn binding_preflight_rejects_a_stale_catalog_seam() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join("source.rs"), b"renamed seam\n").unwrap();
        let mutation = fixture_mutation();

        let error = validate_bindings(directory.path(), &[&mutation]).unwrap_err();

        assert!(error.contains("found 0"), "{error}");
    }

    #[test]
    fn transient_user_mapping_is_retried_before_restoration_fails() {
        let mut attempts = 0;
        let result = retry_transient_source_lock(|| {
            attempts += 1;
            if attempts == 1 {
                Err(std::io::Error::from_raw_os_error(1224))
            } else {
                Ok(())
            }
        });
        if result.is_err() {
            panic!("MUTANT_PREDICATE:source-restoration-transient-lock-unretried");
        }
        assert_eq!(attempts, 2);
    }

    const fn fixture_mutation() -> ControlledMutation {
        ControlledMutation {
            id: 201,
            predicate: "source-restoration",
            source: "source.rs",
            needle: "exact seam",
            replacement: "mutated seam",
            package: "fixture",
            target: MutationTarget::Library,
            selector: "fixture",
        }
    }
}
