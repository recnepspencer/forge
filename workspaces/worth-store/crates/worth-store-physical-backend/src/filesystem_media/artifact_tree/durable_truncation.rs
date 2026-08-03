use super::{ArtifactTreeFailure, ArtifactTreeFailureKind, ArtifactTreeFile, ArtifactTreeMedia};
use crate::filesystem_media::artifact_tree_effects::{
    artifact_file_length, begin, synchronize_file,
};
use crate::filesystem_media::MediaOperationRole;

impl ArtifactTreeMedia<'_> {
    /// Shortens one confined artifact and synchronizes its resulting file state.
    ///
    /// Successful return is the durability boundary for the shorter state.
    ///
    /// The retained length must be strictly smaller than the observed length;
    /// this operation cannot extend or silently accept a stale repair plan.
    pub fn truncate_file_durably(
        &self,
        artifact: &ArtifactTreeFile,
        retained_bytes: u64,
    ) -> Result<(), ArtifactTreeFailure> {
        let _coordination = self
            .owner
            .begin_artifact_mutation(vec![artifact.coordination_key()])
            .map_err(|_| denied())?;
        let directory = self.open_directory(&artifact.directory)?;
        let file = self.open_mutable_file(&directory, &artifact.file_name)?;
        let observed_bytes = artifact_file_length(self.owner, &file)?;
        if retained_bytes >= observed_bytes {
            return Err(ArtifactTreeFailure::structural(
                ArtifactTreeFailureKind::AccessLimitExceeded,
            ));
        }
        let sequence_file = file
            .try_clone()
            .map(cap_std::fs::File::into_std)
            .map_err(|error| {
                ArtifactTreeFailure::io(ArtifactTreeFailureKind::DeniedBeforeEffect, &error)
            })?;
        let sequence = self
            .owner
            .mutation_sequence_for(&sequence_file)
            .map_err(|error| {
                ArtifactTreeFailure::io(ArtifactTreeFailureKind::DeniedBeforeEffect, &error)
            })?;
        let _sequence = sequence.lock();
        let attempt = begin(self.owner, MediaOperationRole::Truncate, 0);
        if let Some(error) = attempt.fail_before_error() {
            attempt.denied();
            return Err(ArtifactTreeFailure::io(
                ArtifactTreeFailureKind::DeniedBeforeEffect,
                &error,
            ));
        }
        match file.set_len(retained_bytes) {
            Ok(()) if attempt.effect_observation_is_indeterminate() => {
                attempt.indeterminate(0);
                Err(ArtifactTreeFailure::structural(
                    ArtifactTreeFailureKind::IndeterminateEffect,
                ))
            }
            Ok(()) => {
                attempt.completed(0);
                synchronize_file(self.owner, &file)
            }
            Err(error) => {
                attempt.indeterminate(0);
                Err(ArtifactTreeFailure::io(
                    ArtifactTreeFailureKind::IndeterminateEffect,
                    &error,
                ))
            }
        }
    }
}

const fn denied() -> ArtifactTreeFailure {
    ArtifactTreeFailure::structural(ArtifactTreeFailureKind::DeniedBeforeEffect)
}
