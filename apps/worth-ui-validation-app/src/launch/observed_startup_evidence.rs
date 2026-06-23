use std::path::{Path, PathBuf};

use super::{ValidationObservedWorkbenchFiles, ValidationWorkbenchAuthoredInputs};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationObservedStartupEvidence {
    rows: Vec<ValidationObservedStartupRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationObservedStartupRow {
    kind: ValidationObservedStartupFileKind,
    path: PathBuf,
    source_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationObservedStartupFileKind {
    Source,
    Theme,
    Commands,
    CommandProjections,
    Component,
    Appearance,
    Density,
}

impl ValidationObservedStartupEvidence {
    pub fn from_observed_files(
        files: &ValidationObservedWorkbenchFiles,
        authored_inputs: &ValidationWorkbenchAuthoredInputs,
    ) -> Self {
        let mut rows = vec![ValidationObservedStartupRow::new(
            ValidationObservedStartupFileKind::Source,
            files.source_path().to_path_buf(),
            authored_inputs.source().source_digest(),
        )];
        if let Some(theme) = authored_inputs.theme() {
            rows.push(ValidationObservedStartupRow::new(
                ValidationObservedStartupFileKind::Theme,
                theme.source_path().to_path_buf(),
                theme.source_digest(),
            ));
        }
        if let Some(commands) = authored_inputs.commands() {
            rows.push(ValidationObservedStartupRow::new(
                ValidationObservedStartupFileKind::Commands,
                commands.source_path().to_path_buf(),
                commands.source_digest(),
            ));
        }
        if let Some(command_projections) = authored_inputs.command_projections() {
            rows.push(ValidationObservedStartupRow::new(
                ValidationObservedStartupFileKind::CommandProjections,
                command_projections.source_path().to_path_buf(),
                command_projections.source_digest(),
            ));
        }
        if let Some(component) = authored_inputs.component() {
            rows.push(ValidationObservedStartupRow::new(
                ValidationObservedStartupFileKind::Component,
                component.source_path().to_path_buf(),
                component.source_digest(),
            ));
        }
        if let Some(appearance) = authored_inputs.appearance() {
            rows.push(ValidationObservedStartupRow::new(
                ValidationObservedStartupFileKind::Appearance,
                appearance.source_path().to_path_buf(),
                appearance.source_digest(),
            ));
        }
        if let Some(density) = authored_inputs.density() {
            rows.push(ValidationObservedStartupRow::new(
                ValidationObservedStartupFileKind::Density,
                density.source_path().to_path_buf(),
                density.source_digest(),
            ));
        }
        Self { rows }
    }

    pub fn rows(&self) -> &[ValidationObservedStartupRow] {
        &self.rows
    }
}

impl ValidationObservedStartupRow {
    fn new(kind: ValidationObservedStartupFileKind, path: PathBuf, source_digest: u64) -> Self {
        Self {
            kind,
            path,
            source_digest,
        }
    }

    pub fn kind(&self) -> ValidationObservedStartupFileKind {
        self.kind
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn source_digest(&self) -> u64 {
        self.source_digest
    }
}
