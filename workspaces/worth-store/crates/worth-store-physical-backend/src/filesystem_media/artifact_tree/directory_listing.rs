use super::{
    ArtifactTreeDirectory, ArtifactTreeFailure, ArtifactTreeFailureKind, ArtifactTreeMedia,
};

impl ArtifactTreeMedia<'_> {
    /// Returns confined file names from one artifact directory.
    ///
    /// The caller supplies the retained-entry bound. Directories, non-Unicode
    /// names, and breadth beyond that bound are damaged evidence, not omissions.
    pub fn list_file_names_bounded(
        &self,
        directory: &ArtifactTreeDirectory,
        limit: usize,
    ) -> Result<Box<[String]>, ArtifactTreeFailure> {
        let directory = self.open_directory(directory)?;
        let entries = directory.entries().map_err(|error| {
            ArtifactTreeFailure::io(ArtifactTreeFailureKind::DeniedBeforeEffect, &error)
        })?;
        let mut names = Vec::new();
        for entry in entries {
            if names.len() == limit {
                return Err(ArtifactTreeFailure::structural(
                    ArtifactTreeFailureKind::AccessLimitExceeded,
                ));
            }
            let entry = entry.map_err(|error| {
                ArtifactTreeFailure::io(ArtifactTreeFailureKind::Damaged, &error)
            })?;
            let metadata = entry.metadata().map_err(|error| {
                ArtifactTreeFailure::io(ArtifactTreeFailureKind::Damaged, &error)
            })?;
            if !metadata.is_file() {
                return Err(ArtifactTreeFailure::structural(
                    ArtifactTreeFailureKind::Damaged,
                ));
            }
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| ArtifactTreeFailure::structural(ArtifactTreeFailureKind::Damaged))?;
            super::path::validate_component(&name)
                .map_err(|_| ArtifactTreeFailure::structural(ArtifactTreeFailureKind::Damaged))?;
            names.push(name);
        }
        names.sort_unstable();
        Ok(names.into_boxed_slice())
    }
}
