use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

pub(super) struct SourceEdit {
    lock: PathBuf,
    source: PathBuf,
    original: Vec<u8>,
    original_digest: [u8; 32],
}

impl SourceEdit {
    pub(super) fn acquire(repository: &Path, source: PathBuf) -> Result<Self, String> {
        let original = std::fs::read(&source)
            .map_err(|error| format!("read freshness dependency source: {error}"))?;
        let lock = repository.join(".phase-eight-freshness.lock");
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock)
            .map_err(|error| format!("acquire exclusive freshness source lock: {error}"))?;
        use std::io::Write;
        if let Err(error) = file.write_all(std::process::id().to_string().as_bytes()) {
            drop(file);
            return Err(acquisition_cleanup_error(
                &lock,
                format!("write freshness source-lock owner: {error}"),
            ));
        }
        drop(file);
        if !lock.is_file() {
            return Err(acquisition_cleanup_error(
                &lock,
                "freshness source lock disappeared after acquisition".to_owned(),
            ));
        }
        Ok(Self {
            lock,
            source,
            original_digest: digest(&original),
            original,
        })
    }

    pub(super) fn install_marker(&mut self, marker: &[u8]) -> Result<(), String> {
        let mut changed = self.original.clone();
        changed.extend_from_slice(marker);
        std::fs::write(&self.source, &changed)
            .map_err(|error| format!("write dependency-only freshness edit: {error}"))?;
        let installed = std::fs::read(&self.source)
            .map_err(|error| format!("verify dependency-only freshness edit: {error}"))?;
        if installed != changed {
            return Err("freshness edit verification changed the replacement bytes".to_owned());
        }
        Ok(())
    }

    pub(super) fn finalize(self) -> Result<(), String> {
        let restore = std::fs::write(&self.source, &self.original)
            .map_err(|error| format!("restore exact freshness source bytes: {error}"));
        let verification = restore.and_then(|()| {
            let bytes = std::fs::read(&self.source)
                .map_err(|error| format!("read restored freshness source bytes: {error}"))?;
            if bytes != self.original || digest(&bytes) != self.original_digest {
                return Err(
                    "freshness source restoration did not reproduce the original bytes".to_owned(),
                );
            }
            Ok(())
        });
        let lock_removal = std::fs::remove_file(&self.lock)
            .map_err(|error| format!("release freshness source lock: {error}"));
        let lock_verification = lock_removal.and_then(|()| {
            if self.lock.exists() {
                Err("freshness source lock remained after release".to_owned())
            } else {
                Ok(())
            }
        });
        combine(verification, lock_verification)
    }
}

fn acquisition_cleanup_error(lock: &Path, reason: String) -> String {
    match std::fs::remove_file(lock) {
        Ok(()) => reason,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !lock.exists() => reason,
        Err(error) => {
            format!("{reason}; freshness source-lock cleanup after acquisition failure: {error}")
        }
    }
}

fn digest(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

fn combine(first: Result<(), String>, second: Result<(), String>) -> Result<(), String> {
    match (first, second) {
        (Ok(()), Ok(())) => Ok(()),
        (Err(first), Ok(())) | (Ok(()), Err(first)) => Err(first),
        (Err(first), Err(second)) => Err(format!("{first}; {second}")),
    }
}

#[cfg(test)]
mod tests {
    use super::SourceEdit;

    #[test]
    fn finalization_restores_source_and_releases_lock() {
        let repository = tempfile::tempdir().unwrap();
        let source = repository.path().join("source.rs");
        let original = b"original\n";
        std::fs::write(&source, original).unwrap();
        let mut edit = SourceEdit::acquire(repository.path(), source.clone()).unwrap();
        edit.install_marker(b"marker\n").unwrap();
        let lock = repository.path().join(".phase-eight-freshness.lock");

        edit.finalize().unwrap();

        assert_eq!(std::fs::read(source).unwrap(), original);
        assert!(!lock.exists());
    }

    #[test]
    fn finalization_reports_restore_failure_but_still_releases_lock() {
        let repository = tempfile::tempdir().unwrap();
        let source = repository.path().join("source.rs");
        std::fs::write(&source, b"original\n").unwrap();
        let edit = SourceEdit::acquire(repository.path(), source.clone()).unwrap();
        std::fs::remove_file(&source).unwrap();
        std::fs::create_dir(&source).unwrap();
        let lock = repository.path().join(".phase-eight-freshness.lock");

        let error = edit.finalize().unwrap_err();

        assert!(
            error.contains("restore exact freshness source bytes"),
            "{error}"
        );
        assert!(!lock.exists());
    }

    #[test]
    fn finalization_reports_lock_release_failure_after_source_restore() {
        let repository = tempfile::tempdir().unwrap();
        let source = repository.path().join("source.rs");
        let original = b"original\n";
        std::fs::write(&source, original).unwrap();
        let edit = SourceEdit::acquire(repository.path(), source.clone()).unwrap();
        let lock = repository.path().join(".phase-eight-freshness.lock");
        std::fs::remove_file(&lock).unwrap();

        let error = edit.finalize().unwrap_err();

        assert!(error.contains("release freshness source lock"), "{error}");
        assert_eq!(std::fs::read(source).unwrap(), original);
    }
}
