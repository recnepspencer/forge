use std::path::PathBuf;

use crate::classification::CiTestLane;
use crate::mutation_campaign::MutationCampaignScope;
use crate::product::{CourtroomSelection, TestProduct};

use super::{Arguments, CiScheduleLane};

pub(super) fn parse(arguments: impl IntoIterator<Item = String>) -> Result<Arguments, String> {
    let parsed = ParsedArguments::collect(arguments)?;
    let product = parsed.product()?;
    parsed.validate(&product)?;
    Ok(parsed.finish(product))
}

struct ParsedArguments {
    command: String,
    package: Option<String>,
    partition: Option<CiTestLane>,
    shard_index: Option<usize>,
    shard_count: Option<usize>,
    list: bool,
    preflight: bool,
    target_root: Option<PathBuf>,
    report: Option<PathBuf>,
    mutant_report: Option<PathBuf>,
    schedule_seed: Option<u64>,
    ci_schedule_lane: Option<CiScheduleLane>,
    crash_seam: Option<String>,
    mutation_scope: Option<MutationCampaignScope>,
    mutant: Option<u8>,
    first_mutant: Option<u8>,
    courtroom: Option<CourtroomSelection>,
}

impl ParsedArguments {
    fn collect(arguments: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut arguments = arguments.into_iter();
        let command = arguments.next().ok_or_else(super::usage)?;
        let mut parsed = Self::new(command);
        while let Some(option) = arguments.next() {
            parsed.accept_option(option, &mut arguments)?;
        }
        Ok(parsed)
    }

    fn new(command: String) -> Self {
        Self {
            command,
            package: None,
            partition: None,
            shard_index: None,
            shard_count: None,
            list: false,
            preflight: false,
            target_root: None,
            report: None,
            mutant_report: None,
            schedule_seed: None,
            ci_schedule_lane: None,
            crash_seam: None,
            mutation_scope: None,
            mutant: None,
            first_mutant: None,
            courtroom: None,
        }
    }

    fn accept_option(
        &mut self,
        option: String,
        arguments: &mut impl Iterator<Item = String>,
    ) -> Result<(), String> {
        match option.as_str() {
            "-p" | "--package" => self.package = Some(value(arguments, &option)?),
            "--partition" => {
                self.partition = Some(value(arguments, &option)?.parse::<CiTestLane>()?)
            }
            "--shard-index" => self.shard_index = Some(number(arguments, &option)?),
            "--shard-count" => self.shard_count = Some(number(arguments, &option)?),
            "--target-root" => self.target_root = Some(path(arguments, &option)?),
            "--report" => self.report = Some(path(arguments, &option)?),
            "--mutant-report" => self.mutant_report = Some(path(arguments, &option)?),
            "--schedule-seed" => self.schedule_seed = Some(u64_number(arguments, &option)?),
            "--ci-schedule-lane" => {
                self.ci_schedule_lane = Some(CiScheduleLane::parse(&value(arguments, &option)?)?)
            }
            "--crash-seam" => self.crash_seam = Some(value(arguments, &option)?),
            "--mutation-scope" => {
                self.mutation_scope =
                    Some(MutationCampaignScope::parse(&value(arguments, &option)?)?)
            }
            "--courtroom" => {
                self.courtroom = Some(CourtroomSelection::parse(&value(arguments, &option)?)?)
            }
            "--mutant" => self.mutant = Some(mutant_id(arguments, &option)?),
            "--from-mutant" => self.first_mutant = Some(mutant_id(arguments, &option)?),
            "--list" => self.list = true,
            "--preflight" => self.preflight = true,
            unknown => return Err(format!("unknown argument `{unknown}`\n{}", super::usage())),
        }
        Ok(())
    }

    fn product(&self) -> Result<TestProduct, String> {
        match self.command.as_str() {
            "owner" => Ok(TestProduct::Owner {
                package: self
                    .package
                    .clone()
                    .ok_or_else(|| "owner requires -p <package>".to_owned())?,
            }),
            "smoke" => Ok(TestProduct::Smoke),
            "ui" => Ok(TestProduct::Ui),
            "mutants" => Ok(TestProduct::Mutants),
            "courtrooms" => Ok(TestProduct::Courtrooms {
                courtroom: self
                    .courtroom
                    .ok_or_else(|| "courtrooms requires --courtroom <a|b|c>".to_owned())?,
            }),
            "ci" => Ok(TestProduct::Ci {
                lane: self
                    .partition
                    .ok_or_else(|| "ci requires --partition <lane>".to_owned())?,
                shard: shard(self.shard_index, self.shard_count)?,
            }),
            unknown => Err(format!("unknown command `{unknown}`\n{}", super::usage())),
        }
    }

    fn validate(&self, product: &TestProduct) -> Result<(), String> {
        self.validate_command_options(product)?;
        self.validate_mutation_options(product)?;
        self.validate_courtroom_options(product)
    }

    fn validate_command_options(&self, product: &TestProduct) -> Result<(), String> {
        if !matches!(product, TestProduct::Owner { .. }) && self.package.is_some() {
            return Err("-p/--package is valid only for owner".into());
        }
        if !matches!(product, TestProduct::Ci { .. })
            && (self.partition.is_some()
                || self.shard_index.is_some()
                || self.shard_count.is_some())
        {
            return Err("partition and shard arguments are valid only for ci".into());
        }
        Ok(())
    }

    fn validate_mutation_options(&self, product: &TestProduct) -> Result<(), String> {
        if !matches!(product, TestProduct::Mutants)
            && (self.mutant.is_some()
                || self.first_mutant.is_some()
                || self.mutation_scope.is_some()
                || self.preflight)
        {
            return Err("mutation scope and selectors are valid only for mutants".into());
        }
        if self.mutant.is_some() && self.first_mutant.is_some() {
            return Err("--mutant and --from-mutant are mutually exclusive".into());
        }
        if self.preflight
            && (self.list
                || self.mutant.is_some()
                || self.first_mutant.is_some()
                || self.report.is_some())
        {
            return Err(
                "--preflight checks the complete selected scope and rejects listing, reports, and execution selectors"
                    .into(),
            );
        }
        if let Some(id) = self.mutant.or(self.first_mutant) {
            let scope = self.mutation_scope.unwrap_or(MutationCampaignScope::All);
            if !scope.contains(id) {
                return Err(format!(
                    "mutation selectors require an id in the selected mutation scope, got `{id}`"
                ));
            }
        }
        if matches!(product, TestProduct::Mutants) && self.report.is_some() {
            if !matches!(
                self.mutation_scope,
                Some(
                    MutationCampaignScope::PhysicalWork
                        | MutationCampaignScope::BoundedResidency
                        | MutationCampaignScope::C8Closure
                )
            ) {
                return Err("mutation reports require a bounded mutation scope".into());
            }
            if self.list || self.mutant.is_some() || self.first_mutant.is_some() {
                return Err("mutation reports require the complete executing campaign".into());
            }
        }
        Ok(())
    }

    fn validate_courtroom_options(&self, product: &TestProduct) -> Result<(), String> {
        if !matches!(product, TestProduct::Courtrooms { .. }) && self.courtroom.is_some() {
            return Err("--courtroom is valid only for courtrooms".into());
        }
        if !matches!(product, TestProduct::Courtrooms { .. }) && self.mutant_report.is_some() {
            return Err("--mutant-report is valid only for courtrooms".into());
        }
        if (self.schedule_seed.is_some()
            || self.ci_schedule_lane.is_some()
            || self.crash_seam.is_some())
            && !matches!(
                product,
                TestProduct::Courtrooms {
                    courtroom: CourtroomSelection::C
                }
            )
        {
            return Err("schedule selection is valid only for Courtroom C".into());
        }
        if self.schedule_seed.is_some() && self.ci_schedule_lane.is_some() {
            return Err("--schedule-seed and --ci-schedule-lane are mutually exclusive".into());
        }
        if self.ci_schedule_lane.is_some() && self.crash_seam.is_some() {
            return Err(
                "--ci-schedule-lane derives its crash seam and rejects --crash-seam".into(),
            );
        }
        if matches!(product, TestProduct::Courtrooms { .. }) {
            self.validate_courtroom_execution()?;
        }
        Ok(())
    }

    fn validate_courtroom_execution(&self) -> Result<(), String> {
        if self.list {
            if self.report.is_some()
                || self.mutant_report.is_some()
                || self.target_root.is_some()
                || self.schedule_seed.is_some()
                || self.ci_schedule_lane.is_some()
                || self.crash_seam.is_some()
            {
                return Err(
                    "courtroom --list does not accept execution or report arguments".into(),
                );
            }
        } else {
            if self.report.is_none() {
                return Err("executing courtrooms requires --report <path>".into());
            }
            if self.mutant_report.is_none() {
                return Err("executing courtrooms requires --mutant-report <path>".into());
            }
        }
        Ok(())
    }

    fn finish(self, product: TestProduct) -> Arguments {
        Arguments {
            product,
            list: self.list,
            preflight: self.preflight,
            target_root: self.target_root,
            report: self.report,
            mutant_report: self.mutant_report,
            schedule_seed: self.schedule_seed,
            ci_schedule_lane: self.ci_schedule_lane,
            crash_seam: self.crash_seam,
            mutation_scope: self.mutation_scope.unwrap_or(MutationCampaignScope::All),
            mutant: self.mutant,
            first_mutant: self.first_mutant,
        }
    }
}

fn value(arguments: &mut impl Iterator<Item = String>, option: &str) -> Result<String, String> {
    arguments
        .next()
        .ok_or_else(|| format!("{option} requires a value"))
}

fn number(arguments: &mut impl Iterator<Item = String>, option: &str) -> Result<usize, String> {
    let raw = value(arguments, option)?;
    raw.parse()
        .map_err(|_| format!("{option} requires a non-negative integer, got `{raw}`"))
}

fn u64_number(arguments: &mut impl Iterator<Item = String>, option: &str) -> Result<u64, String> {
    let raw = value(arguments, option)?;
    raw.parse()
        .map_err(|_| format!("{option} requires an unsigned 64-bit integer, got `{raw}`"))
}

fn mutant_id(arguments: &mut impl Iterator<Item = String>, option: &str) -> Result<u8, String> {
    let raw = value(arguments, option)?;
    raw.parse().map_err(|_| {
        format!(
            "{option} requires an integer from 1 through {}, got `{raw}`",
            crate::mutation_campaign::maximum_id()
        )
    })
}

fn path(arguments: &mut impl Iterator<Item = String>, option: &str) -> Result<PathBuf, String> {
    value(arguments, option).map(PathBuf::from)
}

fn shard(index: Option<usize>, count: Option<usize>) -> Result<Option<(usize, usize)>, String> {
    match (index, count) {
        (None, None) => Ok(None),
        (Some(index), Some(count)) if count > 0 && index < count => Ok(Some((index, count))),
        (Some(_), Some(0)) => Err("--shard-count must be greater than zero".into()),
        (Some(index), Some(count)) => Err(format!(
            "--shard-index {index} is outside shard count {count}"
        )),
        _ => Err("--shard-index and --shard-count must be supplied together".into()),
    }
}
