use std::path::PathBuf;

use super::OfflineMediaReadDenial;

pub(super) fn reject_closure_path_mismatch(
    expected: &[crate::OfflineMediaClosureEntry],
    actual: &[PathBuf],
) -> Result<(), OfflineMediaReadDenial> {
    let mut expected_index = 0;
    let mut actual_index = 0;
    while expected_index < expected.len() && actual_index < actual.len() {
        match expected[expected_index]
            .path()
            .cmp(actual[actual_index].as_path())
        {
            std::cmp::Ordering::Less => {
                return Err(OfflineMediaReadDenial::ContentClosureMissingArtifact {
                    path: expected[expected_index].path().to_path_buf(),
                });
            }
            std::cmp::Ordering::Greater => {
                return Err(OfflineMediaReadDenial::ContentClosureUnexpectedArtifact {
                    path: actual[actual_index].clone(),
                });
            }
            std::cmp::Ordering::Equal => {
                expected_index += 1;
                actual_index += 1;
            }
        }
    }
    if let Some(missing) = expected.get(expected_index) {
        return Err(OfflineMediaReadDenial::ContentClosureMissingArtifact {
            path: missing.path().to_path_buf(),
        });
    }
    if let Some(unexpected) = actual.get(actual_index) {
        return Err(OfflineMediaReadDenial::ContentClosureUnexpectedArtifact {
            path: unexpected.clone(),
        });
    }
    Ok(())
}
