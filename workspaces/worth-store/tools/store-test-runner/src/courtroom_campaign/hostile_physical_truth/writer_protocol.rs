use std::num::NonZeroU32;

use super::process_execution::CapturedProcess;
use worth_store::physical_runtime::{
    PhysicalWorkFilesystemProfileEvidence, PhysicalWorkHostileTruthScenario,
};

pub(super) struct SeedObservation {
    process: NonZeroU32,
    generation: u64,
    records: u64,
    filesystem: PhysicalWorkFilesystemProfileEvidence,
}

impl SeedObservation {
    pub(super) const fn process(&self) -> NonZeroU32 {
        self.process
    }

    pub(super) const fn generation(&self) -> u64 {
        self.generation
    }

    pub(super) const fn records(&self) -> u64 {
        self.records
    }

    pub(super) const fn filesystem(&self) -> &PhysicalWorkFilesystemProfileEvidence {
        &self.filesystem
    }
}

pub(super) fn parse_seed(process: &CapturedProcess) -> Result<SeedObservation, String> {
    let marker = exactly_one(process.stdout(), "C5_1_COURTROOM_SEEDED ")?;
    let fields = marker.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 4 {
        return Err(format!("malformed seed marker `{marker}`"));
    }
    let reported = parse_process(fields[1], "seed")?;
    if reported != process.process() {
        return Err("seed marker reported a foreign process identity".into());
    }
    Ok(SeedObservation {
        process: reported,
        generation: number(fields[2], "seed generation")?,
        records: number(fields[3], "seed record count")?,
        filesystem: super::super::filesystem_profile_protocol::parse(process.stdout())?,
    })
}

pub(super) struct CheckpointObservation {
    process: NonZeroU32,
    checkpoint: Box<str>,
    detail: Box<str>,
}

impl CheckpointObservation {
    pub(super) const fn process(&self) -> NonZeroU32 {
        self.process
    }

    pub(super) fn checkpoint(&self) -> &str {
        &self.checkpoint
    }

    pub(super) fn detail(&self) -> &str {
        &self.detail
    }

    pub(super) fn schedule(&self, scenario: PhysicalWorkHostileTruthScenario) -> String {
        format!(
            "{}:{}:{}",
            scenario.label(),
            self.checkpoint(),
            self.detail()
        )
    }
}

pub(super) fn parse_checkpoint(
    process: &CapturedProcess,
    marker: &str,
    scenario: PhysicalWorkHostileTruthScenario,
) -> Result<CheckpointObservation, String> {
    let fields = marker.split_whitespace().collect::<Vec<_>>();
    if fields.len() != 5
        || fields[0] != "C5_1_COURTROOM_CHECKPOINT"
        || fields[1] != scenario.label()
    {
        return Err(format!("malformed {} marker `{marker}`", scenario.label()));
    }
    let reported = parse_process(fields[3], "checkpoint")?;
    if reported != process.process() {
        return Err(format!(
            "{} checkpoint reported a foreign process identity",
            scenario.label()
        ));
    }
    let observation = CheckpointObservation {
        process: reported,
        checkpoint: fields[2].into(),
        detail: fields[4].into(),
    };
    validate_checkpoint(scenario, &observation)?;
    Ok(observation)
}

fn validate_checkpoint(
    scenario: PhysicalWorkHostileTruthScenario,
    observation: &CheckpointObservation,
) -> Result<(), String> {
    let expected = match scenario {
        PhysicalWorkHostileTruthScenario::BeforeBackendDispatch => "BeforeBackendDispatch",
        PhysicalWorkHostileTruthScenario::DuringShortWrite => "MediaEffect",
        PhysicalWorkHostileTruthScenario::DuringRootPublication => {
            "AfterCatalogReplacementBeforeSchedulerSettlement"
        }
        PhysicalWorkHostileTruthScenario::AfterExactWriteBeforeSchedulerSettlement => {
            "AfterExactWriteBeforeSchedulerSettlement"
        }
        PhysicalWorkHostileTruthScenario::DuringShutdown => "SignalDisposed",
    };
    if observation.checkpoint() != expected {
        return Err(format!(
            "{} reached `{}` instead of `{expected}`",
            scenario.label(),
            observation.checkpoint()
        ));
    }
    match scenario {
        PhysicalWorkHostileTruthScenario::DuringShortWrite
            if !valid_media_detail(observation.detail(), "positioned_write", true) =>
        {
            Err("short-write checkpoint did not bind a positioned write".into())
        }
        PhysicalWorkHostileTruthScenario::DuringRootPublication if observation.detail() != "-" => {
            Err("publication checkpoint carried unexpected media detail".into())
        }
        _ => Ok(()),
    }
}

fn valid_media_detail(detail: &str, role: &str, partial_transfer: bool) -> bool {
    let fields = detail.split(':').collect::<Vec<_>>();
    let [actual_role, role_ordinal, identified_ordinal, requested_bytes] = fields.as_slice() else {
        return false;
    };
    let Ok(requested_bytes) = requested_bytes.parse::<u64>() else {
        return false;
    };
    *actual_role == role
        && *role_ordinal == "1"
        && *identified_ordinal == "1"
        && (!partial_transfer || requested_bytes > 1)
}

fn exactly_one<'lines>(lines: &'lines [String], prefix: &str) -> Result<&'lines str, String> {
    let matching = lines
        .iter()
        .filter(|line| line.starts_with(prefix))
        .collect::<Vec<_>>();
    match matching.as_slice() {
        [line] => Ok(line),
        _ => Err(format!(
            "expected one `{prefix}` marker, found {}",
            matching.len()
        )),
    }
}

fn parse_process(encoded: &str, label: &str) -> Result<NonZeroU32, String> {
    NonZeroU32::new(number(encoded, label)?)
        .ok_or_else(|| format!("{label} process identity cannot be zero"))
}

fn number<T>(encoded: &str, label: &str) -> Result<T, String>
where
    T: std::str::FromStr,
{
    encoded
        .parse()
        .map_err(|_| format!("{label} is not a valid number"))
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::{parse_checkpoint, CapturedProcess, PhysicalWorkHostileTruthScenario};

    #[test]
    fn canonical_short_write_media_checkpoint_is_accepted() {
        let process = NonZeroU32::new(41).unwrap();
        let captured = CapturedProcess::for_test(process);
        let scenario = PhysicalWorkHostileTruthScenario::DuringShortWrite;
        let marker = format!(
            "C5_1_COURTROOM_CHECKPOINT {} MediaEffect {} positioned_write:1:1:4096",
            scenario.label(),
            process.get(),
        );
        assert!(parse_checkpoint(&captured, &marker, scenario).is_ok());
    }

    #[test]
    fn canonical_catalog_replacement_checkpoint_is_accepted() {
        let process = NonZeroU32::new(41).unwrap();
        let captured = CapturedProcess::for_test(process);
        let scenario = PhysicalWorkHostileTruthScenario::DuringRootPublication;
        let marker = format!(
            "C5_1_COURTROOM_CHECKPOINT {} \
             AfterCatalogReplacementBeforeSchedulerSettlement {} -",
            scenario.label(),
            process.get(),
        );
        assert!(parse_checkpoint(&captured, &marker, scenario).is_ok());

        let old_media_marker = format!(
            "C5_1_COURTROOM_CHECKPOINT {} MediaEffect {} atomic_replace:1:1:0",
            scenario.label(),
            process.get(),
        );
        assert!(parse_checkpoint(&captured, &old_media_marker, scenario).is_err());
    }

    #[test]
    fn lookalike_or_foreign_media_checkpoint_protocol_is_rejected() {
        let process = NonZeroU32::new(41).unwrap();
        let captured = CapturedProcess::for_test(process);
        let scenario = PhysicalWorkHostileTruthScenario::DuringShortWrite;
        for detail in [
            "positioned-write:1:1:4096",
            "positioned_read:1:1:4096",
            "positioned_write:1:1:1",
            "positioned_write:1:any:4096",
        ] {
            let marker = format!(
                "C5_1_COURTROOM_CHECKPOINT {} MediaEffect {} {detail}",
                scenario.label(),
                process.get(),
            );
            assert!(parse_checkpoint(&captured, &marker, scenario).is_err());
        }
    }
}
