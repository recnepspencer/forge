use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::artifact_walk::ObservedRecoveryArtifact;
use super::{
    RecoveryObserverCounters, RecoveryObserverLimits, RecoveryObserverObservationDenial,
    RecoveryObserverObservationFailure,
};

pub(super) fn observe_file(
    root: &Path,
    path: PathBuf,
    limits: RecoveryObserverLimits,
    counters: &mut RecoveryObserverCounters,
) -> Result<ObservedRecoveryArtifact, RecoveryObserverObservationFailure> {
    let declared = path
        .metadata()
        .map_err(|error| media_failure(error, *counters, &path))?
        .len();
    let projected = counters
        .bytes_read()
        .checked_add(declared)
        .ok_or_else(|| byte_limit_failure(u64::MAX, limits, *counters, &path))?;
    if projected > limits.maximum_bytes() {
        return Err(byte_limit_failure(projected, limits, *counters, &path));
    }
    let mut file =
        std::fs::File::open(&path).map_err(|error| media_failure(error, *counters, &path))?;
    counters
        .record_file_opened()
        .ok_or_else(|| byte_limit_failure(u64::MAX, limits, *counters, &path))?;
    let mut digest = Sha256::new();
    let mut observed = 0_u64;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|error| media_failure(error, *counters, &path))?;
        if count == 0 {
            break;
        }
        let count = count as u64;
        let total = counters
            .bytes_read()
            .checked_add(count)
            .ok_or_else(|| byte_limit_failure(u64::MAX, limits, *counters, &path))?;
        if total > limits.maximum_bytes() {
            return Err(byte_limit_failure(total, limits, *counters, &path));
        }
        counters
            .record_bytes_read(count)
            .ok_or_else(|| byte_limit_failure(u64::MAX, limits, *counters, &path))?;
        observed = observed
            .checked_add(count)
            .ok_or_else(|| byte_limit_failure(u64::MAX, limits, *counters, &path))?;
        digest.update(&buffer[..count as usize]);
    }
    if observed != declared {
        return Err(RecoveryObserverObservationFailure::at_path(
            RecoveryObserverObservationDenial::ArtifactChanged,
            *counters,
            &path,
        ));
    }
    let relative = path
        .strip_prefix(root)
        .map_err(|_| artifact_changed(*counters, &path))?
        .to_str()
        .ok_or_else(|| {
            RecoveryObserverObservationFailure::at_path(
                RecoveryObserverObservationDenial::NonUnicodePath,
                *counters,
                &path,
            )
        })?
        .replace('\\', "/")
        .into_boxed_str();
    counters.record_artifact_observed().ok_or_else(|| {
        RecoveryObserverObservationFailure::at_path(
            RecoveryObserverObservationDenial::ArtifactChanged,
            *counters,
            &path,
        )
    })?;
    Ok(ObservedRecoveryArtifact {
        path: relative,
        byte_length: observed,
        digest: digest.finalize().into(),
    })
}

fn byte_limit_failure(
    observed: u64,
    limits: RecoveryObserverLimits,
    counters: RecoveryObserverCounters,
    path: &Path,
) -> RecoveryObserverObservationFailure {
    RecoveryObserverObservationFailure::at_path(
        RecoveryObserverObservationDenial::ByteLimit {
            observed,
            admitted: limits.maximum_bytes(),
        },
        counters,
        path,
    )
}

fn media_failure(
    error: std::io::Error,
    counters: RecoveryObserverCounters,
    path: &Path,
) -> RecoveryObserverObservationFailure {
    RecoveryObserverObservationFailure::at_path(
        RecoveryObserverObservationDenial::Media(error.kind()),
        counters,
        path,
    )
}

fn artifact_changed(
    counters: RecoveryObserverCounters,
    path: &Path,
) -> RecoveryObserverObservationFailure {
    RecoveryObserverObservationFailure::at_path(
        RecoveryObserverObservationDenial::ArtifactChanged,
        counters,
        path,
    )
}
