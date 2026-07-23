use std::io::Write;

use super::{ArtifactTreeFailure, ArtifactTreeFailureKind};
use crate::filesystem_media::{FilesystemMediaOwner, MediaOperationIdentity, MediaOperationRole};

pub(super) enum ExactWriteEffect {
    Completed(MediaOperationIdentity),
    DeniedBeforeEffect(ArtifactTreeFailure),
    Indeterminate {
        failure: ArtifactTreeFailure,
        completed_bytes: u64,
        operation: MediaOperationIdentity,
    },
}

pub(super) fn execute(
    owner: &FilesystemMediaOwner,
    file: &mut cap_std::fs::File,
    bytes: &[u8],
) -> ExactWriteEffect {
    let Some(operation) = owner.issue_operation_identity() else {
        return ExactWriteEffect::DeniedBeforeEffect(ArtifactTreeFailure::structural(
            ArtifactTreeFailureKind::DeniedBeforeEffect,
        ));
    };
    let requested = bytes.len() as u64;
    let attempt = super::super::artifact_tree_effects::begin(
        owner,
        MediaOperationRole::PositionedWrite,
        requested,
    );
    if let Some(error) = attempt.fail_before_error() {
        attempt.denied();
        return ExactWriteEffect::DeniedBeforeEffect(ArtifactTreeFailure::io(
            ArtifactTreeFailureKind::DeniedBeforeEffect,
            &error,
        ));
    }
    let limit = attempt.transfer_limit(requested) as usize;
    let completed = match write_prefix(file, &bytes[..limit]) {
        Ok(completed) => completed,
        Err((error, completed_bytes)) => {
            attempt.indeterminate(completed_bytes);
            return ExactWriteEffect::Indeterminate {
                failure: ArtifactTreeFailure::io(
                    ArtifactTreeFailureKind::IndeterminateEffect,
                    &error,
                ),
                completed_bytes,
                operation,
            };
        }
    };
    if limit != bytes.len() {
        attempt.partial(completed);
        return ExactWriteEffect::Indeterminate {
            failure: ArtifactTreeFailure::structural(ArtifactTreeFailureKind::PartialWrite {
                completed_bytes: completed,
            }),
            completed_bytes: completed,
            operation,
        };
    }
    if attempt.effect_observation_is_indeterminate() {
        attempt.indeterminate(completed);
        return ExactWriteEffect::Indeterminate {
            failure: ArtifactTreeFailure::structural(ArtifactTreeFailureKind::IndeterminateEffect),
            completed_bytes: completed,
            operation,
        };
    }
    attempt.completed(completed);
    ExactWriteEffect::Completed(operation)
}

fn write_prefix(file: &mut impl Write, bytes: &[u8]) -> Result<u64, (std::io::Error, u64)> {
    let mut completed = 0_usize;
    while completed < bytes.len() {
        match file.write(&bytes[completed..]) {
            Ok(0) => {
                return Err((
                    std::io::Error::new(
                        std::io::ErrorKind::WriteZero,
                        "filesystem write made no progress",
                    ),
                    completed as u64,
                ));
            }
            Ok(written) => completed += written,
            Err(error) if error.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(error) => return Err((error, completed as u64)),
        }
    }
    Ok(completed as u64)
}

#[cfg(test)]
mod tests {
    use super::write_prefix;
    use std::io::Write;

    struct PrefixThenError {
        bytes: Vec<u8>,
        prefix: usize,
    }

    impl Write for PrefixThenError {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if self.bytes.len() == self.prefix {
                return Err(std::io::Error::other("injected writer failure"));
            }
            let count = (self.prefix - self.bytes.len()).min(bytes.len());
            self.bytes.extend_from_slice(&bytes[..count]);
            Ok(count)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn writer_error_retains_the_exact_confirmed_prefix() {
        let mut writer = PrefixThenError {
            bytes: Vec::new(),
            prefix: 3,
        };
        let (_, completed) = write_prefix(&mut writer, b"abcdef").unwrap_err();
        assert_eq!(completed, 3);
        assert_eq!(writer.bytes, b"abc");
    }
}
