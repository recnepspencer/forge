//! Bounded intake for untrusted release-ceremony files.

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::denial::WorthQueryReleaseCeremonyError as Error;

pub(crate) fn read_bounded_input(path: &Path, maximum: u64) -> Result<Vec<u8>, Error> {
    let file = File::open(path).map_err(|error| Error::InputRead {
        path: path.to_owned(),
        error,
    })?;
    let declared = file
        .metadata()
        .map_err(|error| Error::InputRead {
            path: path.to_owned(),
            error,
        })?
        .len();
    if declared > maximum {
        return Err(Error::InputByteBudgetExceeded {
            path: path.to_owned(),
            maximum,
        });
    }
    let mut bytes = Vec::with_capacity(usize::try_from(declared).unwrap_or(0));
    file.take(maximum.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| Error::InputRead {
            path: path.to_owned(),
            error,
        })?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > maximum {
        return Err(Error::InputByteBudgetExceeded {
            path: path.to_owned(),
            maximum,
        });
    }
    Ok(bytes)
}
