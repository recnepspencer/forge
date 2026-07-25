use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

static REPORT_SEQUENCE: AtomicU64 = AtomicU64::new(1);
const REPORT_PUBLICATION_BUDGET: Duration = Duration::from_secs(1);

pub(super) struct CourtroomReportSession {
    report: PathBuf,
    pending: PathBuf,
    published: bool,
}

pub(super) struct PublishedCourtroomReport {
    report: PathBuf,
    elapsed: Duration,
    accepted: bool,
}

impl CourtroomReportSession {
    pub(super) fn begin(report: &Path) -> Result<Self, String> {
        let report = normalize(report)?;
        invalidate_prior(&report)?;
        let pending = pending_path(&report)?;
        Ok(Self {
            report,
            pending,
            published: false,
        })
    }

    pub(super) fn publish(self, encoded: &[u8]) -> Result<PublishedCourtroomReport, String> {
        let started = Instant::now();
        let mut file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.pending)
            .map_err(|error| {
                format!(
                    "cannot create pending courtroom report {}: {error}",
                    self.pending.display()
                )
            })?;
        file.write_all(encoded)
            .map_err(|error| format!("cannot write courtroom report: {error}"))?;
        file.sync_all()
            .map_err(|error| format!("cannot synchronize courtroom report: {error}"))?;
        drop(file);
        std::fs::rename(&self.pending, &self.report).map_err(|error| {
            format!(
                "cannot publish courtroom report {}: {error}",
                self.report.display()
            )
        })?;
        self.finish_publication(started.elapsed())
    }

    fn finish_publication(mut self, elapsed: Duration) -> Result<PublishedCourtroomReport, String> {
        if elapsed > REPORT_PUBLICATION_BUDGET {
            std::fs::remove_file(&self.report)
                .map_err(|error| format!("cannot invalidate slow courtroom report: {error}"))?;
            return Err(format!(
                "courtroom report publication took {}ms; budget is {}ms",
                elapsed.as_millis(),
                REPORT_PUBLICATION_BUDGET.as_millis(),
            ));
        }
        self.published = true;
        Ok(PublishedCourtroomReport {
            report: self.report.clone(),
            elapsed,
            accepted: false,
        })
    }
}

impl PublishedCourtroomReport {
    pub(super) const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub(super) fn accept(mut self) {
        self.accepted = true;
    }
}

impl Drop for PublishedCourtroomReport {
    fn drop(&mut self) {
        if !self.accepted {
            let _ = std::fs::remove_file(&self.report);
        }
    }
}

impl Drop for CourtroomReportSession {
    fn drop(&mut self) {
        if !self.published {
            let _ = std::fs::remove_file(&self.pending);
        }
    }
}

fn normalize(report: &Path) -> Result<PathBuf, String> {
    let absolute = if report.is_absolute() {
        report.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|error| format!("cannot resolve current directory: {error}"))?
            .join(report)
    };
    let parent = absolute
        .parent()
        .ok_or_else(|| "courtroom report has no parent".to_owned())?;
    std::fs::create_dir_all(parent)
        .map_err(|error| format!("cannot create report parent {}: {error}", parent.display()))?;
    let parent = parent
        .canonicalize()
        .map_err(|error| format!("cannot canonicalize report parent: {error}"))?;
    let name = absolute
        .file_name()
        .ok_or_else(|| "courtroom report has no filename".to_owned())?;
    Ok(parent.join(name))
}

fn invalidate_prior(report: &Path) -> Result<(), String> {
    match std::fs::symlink_metadata(report) {
        Ok(metadata) if metadata.is_file() => std::fs::remove_file(report)
            .map_err(|error| format!("cannot invalidate prior report: {error}")),
        Ok(_) => Err(format!(
            "courtroom report path {} is not a file",
            report.display()
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("cannot inspect prior courtroom report: {error}")),
    }
}

fn pending_path(report: &Path) -> Result<PathBuf, String> {
    let parent = report
        .parent()
        .ok_or_else(|| "courtroom report has no parent".to_owned())?;
    let name = report
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| "courtroom report filename must be Unicode".to_owned())?;
    for _ in 0..32 {
        let sequence = REPORT_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let candidate = parent.join(format!(
            "{name}.pending.{}.{}",
            std::process::id(),
            sequence
        ));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }
    Err("cannot allocate a unique pending courtroom report".into())
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::CourtroomReportSession;

    #[test]
    fn publication_invalidates_stale_success_and_publishes_last() {
        let temporary = tempfile::tempdir().unwrap();
        let report = temporary.path().join("courtroom-b.json");
        std::fs::write(&report, b"stale success").unwrap();
        let session = CourtroomReportSession::begin(&report).unwrap();
        assert!(!report.exists());
        session.publish(b"{\"accepted\":true}").unwrap().accept();
        assert_eq!(
            std::fs::read_to_string(report).unwrap(),
            "{\"accepted\":true}"
        );
    }

    #[test]
    fn abandoned_session_leaves_no_success_or_pending_file() {
        let temporary = tempfile::tempdir().unwrap();
        let report = temporary.path().join("courtroom-b.json");
        let pending;
        {
            let session = CourtroomReportSession::begin(&report).unwrap();
            pending = session.pending.clone();
        }
        assert!(!report.exists());
        assert!(!pending.exists());
    }

    #[test]
    fn published_report_is_not_success_until_terminal_acceptance() {
        let temporary = tempfile::tempdir().unwrap();
        let report = temporary.path().join("courtroom-b.json");
        let publication = CourtroomReportSession::begin(&report)
            .unwrap()
            .publish(b"{\"accepted\":true}")
            .unwrap();
        assert!(report.exists());
        drop(publication);
        assert!(!report.exists());
    }

    #[test]
    fn over_budget_publication_invalidates_its_success_artifact() {
        let temporary = tempfile::tempdir().unwrap();
        let report = temporary.path().join("courtroom-b.json");
        let session = CourtroomReportSession::begin(&report).unwrap();
        std::fs::write(&report, b"too slow to certify").unwrap();

        assert!(session
            .finish_publication(Duration::from_millis(1_001))
            .is_err());
        assert!(!report.exists());
    }
}
