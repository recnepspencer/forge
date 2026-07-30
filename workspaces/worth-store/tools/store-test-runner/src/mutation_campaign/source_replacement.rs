use std::path::{Path, PathBuf};

use super::catalog::ControlledMutation;

pub(super) struct InstalledSourceMutation {
    source: PathBuf,
    original: Box<[u8]>,
    mutated: Box<[u8]>,
    restored: bool,
}

impl InstalledSourceMutation {
    pub(super) fn apply(workspace: &Path, mutation: &ControlledMutation) -> Result<Self, String> {
        let source = workspace.join(mutation.source);
        let original = std::fs::read(&source).map_err(|error| {
            format!("cannot read mutation source {}: {error}", source.display())
        })?;
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
        let mut installed = Self {
            source,
            original: original.into_boxed_slice(),
            mutated: mutated.into_boxed_slice(),
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
        std::fs::write(&self.source, &self.original)
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
        std::fs::write(&self.source, &self.mutated)
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

impl Drop for InstalledSourceMutation {
    fn drop(&mut self) {
        if !self.restored {
            let _ = std::fs::write(&self.source, &self.original);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InstalledSourceMutation;
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
