use std::process::Command;
use std::time::{Duration, Instant};

use worth_store::physical_runtime::PhysicalWorkProcessEvidence;
use worth_store_recovery_runtime::{RecoveryReportEnvelope, RecoveryReportOutcome};

use super::super::{
    binary_binding::BuiltCourtroomExecutables, process_execution, schedule::C7DurabilityCrashSeam,
    world::BoundedResidencySiegeWorld,
};

const RECOVERY_TIMEOUT: Duration = Duration::from_secs(300);
const RECOVERY_PROFILE: &str = "c8-phase8-fate-coverage-v1";

#[derive(Clone, Copy)]
pub(in crate::courtroom_campaign::bounded_residency_siege) struct C8RecoveryRuntimeMarker {
    store: [u8; 16],
    runtime: u64,
    root_generation: u64,
}

pub(in crate::courtroom_campaign::bounded_residency_siege) struct C8RecoveryEvidence {
    process: PhysicalWorkProcessEvidence,
    marker: C8RecoveryRuntimeMarker,
    report: RecoveryReportEnvelope,
    elapsed: Duration,
}

impl C8RecoveryEvidence {
    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn process(
        &self,
    ) -> &PhysicalWorkProcessEvidence {
        &self.process
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn marker(
        &self,
    ) -> C8RecoveryRuntimeMarker {
        self.marker
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn report(
        &self,
    ) -> &RecoveryReportEnvelope {
        &self.report
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn elapsed(&self) -> Duration {
        self.elapsed
    }
}

impl C8RecoveryRuntimeMarker {
    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn store(self) -> [u8; 16] {
        self.store
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn runtime(self) -> u64 {
        self.runtime
    }

    pub(in crate::courtroom_campaign::bounded_residency_siege) const fn root_generation(
        self,
    ) -> u64 {
        self.root_generation
    }
}

pub(super) fn execute(
    world: &BoundedResidencySiegeWorld,
    binaries: &BuiltCourtroomExecutables,
    seam: C7DurabilityCrashSeam,
) -> Result<C8RecoveryEvidence, String> {
    let report_path = world
        .root()
        .join(format!("c8-recovery-report-{}.bin", seam.label()));
    let mut command = Command::new(binaries.recovery().path());
    command
        .arg(world.store())
        .arg(format!("--bounded-profile={RECOVERY_PROFILE}"))
        .arg(format!("--report={}", report_path.display()));
    let started = Instant::now();
    let process = process_execution::run_success_allowing_stderr(
        &mut command,
        RECOVERY_TIMEOUT,
        "C8 physical recovery",
    )?;
    let elapsed = started.elapsed();
    let marker = parse_runtime_marker(process.stderr())?;
    let report = read_report(&report_path)?;
    validate_process_report(&marker, &report)?;
    let process = process.evidence(&format!("c7:{}:c8-recovery", seam.label()))?;
    Ok(C8RecoveryEvidence {
        process,
        marker,
        report,
        elapsed,
    })
}

fn read_report(path: &std::path::Path) -> Result<RecoveryReportEnvelope, String> {
    let encoded = std::fs::read(path).map_err(|error| {
        format!(
            "C8 recovery report {} could not be read: {error}",
            path.display()
        )
    })?;
    RecoveryReportEnvelope::decode(&encoded).map_err(|denial| {
        format!(
            "C8 recovery report {} was denied: {denial:?}",
            path.display()
        )
    })
}

fn validate_process_report(
    marker: &C8RecoveryRuntimeMarker,
    report: &RecoveryReportEnvelope,
) -> Result<(), String> {
    if report.outcome() != RecoveryReportOutcome::Recovered {
        return Err(format!(
            "C8 recovery process did not report Recovered: {:?}",
            report.outcome()
        ));
    }
    if report.store_identity() != Some(marker.store) {
        return Err("C8 recovery marker and report crossed Store identity".to_owned());
    }
    if report.root_generation() != Some(marker.root_generation) {
        return Err("C8 recovery marker and report crossed root generation".to_owned());
    }
    if report.denial_cause().is_some() {
        return Err("Recovered C8 report carried a denial cause".to_owned());
    }
    Ok(())
}

fn parse_runtime_marker(stderr: &str) -> Result<C8RecoveryRuntimeMarker, String> {
    let lines = stderr
        .lines()
        .filter(|line| line.starts_with("C8_RECOVERY_RUNTIME "))
        .collect::<Vec<_>>();
    let [line] = lines.as_slice() else {
        return Err(format!(
            "C8 recovery process must emit one runtime marker, found {}",
            lines.len()
        ));
    };
    let fields = line.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 4 {
        return Err(format!("malformed C8 recovery runtime marker `{line}`"));
    }
    let runtime = nonzero_number(fields[2], "C8 recovery runtime")?;
    let root_generation = nonzero_number(fields[3], "C8 recovery root generation")?;
    Ok(C8RecoveryRuntimeMarker {
        store: fixed_hex(fields[1])?,
        runtime,
        root_generation,
    })
}

fn fixed_hex(encoded: &str) -> Result<[u8; 16], String> {
    if encoded.len() != 32 || !encoded.is_ascii() {
        return Err("C8 recovery Store marker must contain 16 hexadecimal bytes".to_owned());
    }
    let mut bytes = [0_u8; 16];
    for (index, byte) in bytes.iter_mut().enumerate() {
        let offset = index * 2;
        *byte = u8::from_str_radix(&encoded[offset..offset + 2], 16)
            .map_err(|_| "C8 recovery Store marker contains non-hexadecimal data".to_owned())?;
    }
    Ok(bytes)
}

fn nonzero_number(encoded: &str, label: &str) -> Result<u64, String> {
    let value = encoded
        .parse::<u64>()
        .map_err(|_| format!("{label} is not a valid number"))?;
    if value == 0 {
        return Err(format!("{label} cannot be zero"));
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::parse_runtime_marker;

    #[test]
    fn runtime_marker_requires_one_nonzero_typed_identity() {
        let marker =
            parse_runtime_marker("C8_RECOVERY_RUNTIME 000102030405060708090a0b0c0d0e0f 7 9\n")
                .unwrap();
        assert_eq!(marker.runtime(), 7);
        assert_eq!(marker.root_generation(), 9);
        assert_eq!(
            marker.store(),
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
    }

    #[test]
    fn runtime_marker_rejects_duplicates_and_malformed_fields() {
        for stderr in [
            "C8_RECOVERY_RUNTIME 000102030405060708090a0b0c0d0e0f 7 9\nC8_RECOVERY_RUNTIME 000102030405060708090a0b0c0d0e0f 8 9\n",
            "C8_RECOVERY_RUNTIME foreign 7 9\n",
            "C8_RECOVERY_RUNTIME 000102030405060708090a0b0c0d0e0f 0 9\n",
        ] {
            assert!(parse_runtime_marker(stderr).is_err());
        }
    }
}
