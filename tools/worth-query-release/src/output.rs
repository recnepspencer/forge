//! Refuse-overwrite staging for the envelope and its descriptive report.

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::denial::WorthQueryReleaseCeremonyError as Error;

pub(crate) fn write_release_outputs(
    envelope_path: &Path,
    envelope_bytes: &[u8],
    report_path: &Path,
    report_bytes: &[u8],
) -> Result<(), Error> {
    if envelope_path == report_path {
        return Err(Error::OutputPathConflict);
    }
    require_absent(envelope_path)?;
    require_absent(report_path)?;
    let envelope = StagedOutput::create(envelope_path, envelope_bytes)?;
    let report = match StagedOutput::create(report_path, report_bytes) {
        Ok(report) => report,
        Err(error) => {
            envelope.cleanup();
            return Err(error);
        }
    };
    if let Err(error) = report.commit() {
        report.cleanup();
        envelope.cleanup();
        return Err(error);
    }
    if let Err(error) = envelope.commit() {
        envelope.cleanup();
        let _ = fs::remove_file(report_path);
        return Err(error);
    }
    Ok(())
}

pub(crate) fn write_new_output(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    require_absent(path)?;
    let output = StagedOutput::create(path, bytes)?;
    if let Err(error) = output.commit() {
        output.cleanup();
        return Err(error);
    }
    Ok(())
}

struct StagedOutput {
    temporary: PathBuf,
    destination: PathBuf,
}

impl StagedOutput {
    fn create(destination: &Path, bytes: &[u8]) -> Result<Self, Error> {
        let temporary = temporary_path(destination)?;
        let mut file = create_new(&temporary)?;
        if let Err(error) = file.write_all(bytes).and_then(|()| file.sync_all()) {
            let _ = fs::remove_file(&temporary);
            return Err(Error::OutputWrite {
                path: temporary,
                error,
            });
        }
        Ok(Self {
            temporary,
            destination: destination.to_owned(),
        })
    }

    fn commit(&self) -> Result<(), Error> {
        fs::hard_link(&self.temporary, &self.destination).map_err(|error| Error::OutputWrite {
            path: self.destination.clone(),
            error,
        })?;
        let _ = fs::remove_file(&self.temporary);
        Ok(())
    }

    fn cleanup(&self) {
        let _ = fs::remove_file(&self.temporary);
    }
}

fn require_absent(path: &Path) -> Result<(), Error> {
    match path.try_exists() {
        Ok(false) => Ok(()),
        Ok(true) => Err(Error::OutputAlreadyExists {
            path: path.to_owned(),
        }),
        Err(error) => Err(Error::OutputWrite {
            path: path.to_owned(),
            error,
        }),
    }
}

fn temporary_path(destination: &Path) -> Result<PathBuf, Error> {
    let Some(file_name) = destination.file_name().and_then(|name| name.to_str()) else {
        return Err(Error::OutputWrite {
            path: destination.to_owned(),
            error: std::io::Error::new(std::io::ErrorKind::InvalidInput, "output has no file name"),
        });
    };
    Ok(destination.with_file_name(format!(
        ".{file_name}.{}.worth-query-release.tmp",
        std::process::id()
    )))
}

fn create_new(path: &Path) -> Result<File, Error> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Error::OutputWrite {
            path: path.to_owned(),
            error,
        })
}
