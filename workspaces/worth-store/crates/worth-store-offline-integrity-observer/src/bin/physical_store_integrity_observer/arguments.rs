use std::ffi::OsString;
use std::path::PathBuf;

use worth_store_offline_integrity_observer::{
    OfflineIntegrityObservationLimits, OfflineIntegrityReportDestination,
};

pub(super) const HELP: &str = "\
Independent read-only C.9 root-protocol observer\n\n\
Usage:\n  physical_store_integrity_observer observe \\\n    --store-root <closed-or-isolated-store-root> \\\n    --report <path-outside-store-root|-> \\\n    --max-entries <n> --max-bytes <n> --max-open-files <n> \\\n    --max-depth <n> --max-symlinks <n> --max-elapsed-ms <n> \\\n    --max-report-bytes <n>\n\n\
The Phase 3 binary observes current/previous selectors and addressed root manifests.\n\
It never repairs, recovers, quarantines, deletes, or accepts damaged bytes.\n";

pub(super) struct ObserveArguments {
    pub(super) store_root: PathBuf,
    pub(super) report_destination: OfflineIntegrityReportDestination,
    pub(super) limits: OfflineIntegrityObservationLimits,
}

pub(super) enum ArgumentOutcome {
    Help(&'static str),
    Denied(String),
}

pub(super) fn parse(
    arguments: impl IntoIterator<Item = OsString>,
) -> Result<ObserveArguments, ArgumentOutcome> {
    let mut arguments = arguments.into_iter();
    let Some(operation) = arguments.next() else {
        return Err(ArgumentOutcome::Help(HELP));
    };
    if operation == "--help" || operation == "-h" {
        return Err(ArgumentOutcome::Help(HELP));
    }
    if operation != "observe" {
        return Err(ArgumentOutcome::Denied(
            "only the Phase 3 observe operation is available".into(),
        ));
    }
    let mut values = ArgumentValues::default();
    while let Some(flag) = arguments.next() {
        if flag == "--help" || flag == "-h" {
            return Err(ArgumentOutcome::Help(HELP));
        }
        let value = arguments.next().ok_or_else(|| {
            ArgumentOutcome::Denied(format!("{} requires a value", flag.to_string_lossy()))
        })?;
        values.assign(&flag.to_string_lossy(), value)?;
    }
    values.finish()
}

#[derive(Default)]
struct ArgumentValues {
    store_root: Option<PathBuf>,
    report: Option<OsString>,
    maximum_entries: Option<u64>,
    maximum_bytes: Option<u64>,
    maximum_open_files: Option<u32>,
    maximum_depth: Option<u32>,
    maximum_symlinks: Option<u64>,
    maximum_elapsed_milliseconds: Option<u64>,
    maximum_report_bytes: Option<u64>,
}

impl ArgumentValues {
    fn assign(&mut self, flag: &str, value: OsString) -> Result<(), ArgumentOutcome> {
        match flag {
            "--store-root" => set_once(&mut self.store_root, PathBuf::from(value), flag),
            "--report" => set_once(&mut self.report, value, flag),
            "--max-entries" => parse_once(&mut self.maximum_entries, value, flag),
            "--max-bytes" => parse_once(&mut self.maximum_bytes, value, flag),
            "--max-open-files" => parse_once(&mut self.maximum_open_files, value, flag),
            "--max-depth" => parse_once(&mut self.maximum_depth, value, flag),
            "--max-symlinks" => parse_once(&mut self.maximum_symlinks, value, flag),
            "--max-elapsed-ms" => parse_once(&mut self.maximum_elapsed_milliseconds, value, flag),
            "--max-report-bytes" => parse_once(&mut self.maximum_report_bytes, value, flag),
            _ => Err(ArgumentOutcome::Denied(format!("unknown argument {flag}"))),
        }
    }

    fn finish(self) -> Result<ObserveArguments, ArgumentOutcome> {
        let store_root = required(self.store_root, "--store-root")?;
        let report = required(self.report, "--report")?;
        let report_destination = if report == "-" {
            OfflineIntegrityReportDestination::standard_output()
        } else {
            OfflineIntegrityReportDestination::file(PathBuf::from(report)).map_err(|denial| {
                ArgumentOutcome::Denied(format!("invalid --report: {denial:?}"))
            })?
        };
        let limits = OfflineIntegrityObservationLimits::new(
            required(self.maximum_entries, "--max-entries")?,
            required(self.maximum_bytes, "--max-bytes")?,
            required(self.maximum_open_files, "--max-open-files")?,
            required(self.maximum_depth, "--max-depth")?,
            required(self.maximum_symlinks, "--max-symlinks")?,
            required(self.maximum_elapsed_milliseconds, "--max-elapsed-ms")?,
            required(self.maximum_report_bytes, "--max-report-bytes")?,
        )
        .map_err(|denial| {
            ArgumentOutcome::Denied(format!("invalid observation limit: {denial:?}"))
        })?;
        Ok(ObserveArguments {
            store_root,
            report_destination,
            limits,
        })
    }
}

fn required<T>(value: Option<T>, flag: &str) -> Result<T, ArgumentOutcome> {
    value.ok_or_else(|| ArgumentOutcome::Denied(format!("missing required {flag}")))
}

fn set_once<T>(slot: &mut Option<T>, value: T, flag: &str) -> Result<(), ArgumentOutcome> {
    if slot.replace(value).is_some() {
        Err(ArgumentOutcome::Denied(format!("duplicate {flag}")))
    } else {
        Ok(())
    }
}

fn parse_once<T: std::str::FromStr>(
    slot: &mut Option<T>,
    value: OsString,
    flag: &str,
) -> Result<(), ArgumentOutcome> {
    let parsed = value
        .to_str()
        .and_then(|value| value.parse().ok())
        .ok_or_else(|| ArgumentOutcome::Denied(format!("{flag} requires an unsigned integer")))?;
    set_once(slot, parsed, flag)
}
