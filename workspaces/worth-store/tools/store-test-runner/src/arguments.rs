use std::path::PathBuf;

use crate::mutation_campaign::MutationCampaignScope;
use crate::product::TestProduct;

mod parsing;
mod schedule_lane;

pub(crate) use schedule_lane::CiScheduleLane;

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) struct Arguments {
    pub(crate) product: TestProduct,
    pub(crate) list: bool,
    pub(crate) preflight: bool,
    pub(crate) target_root: Option<PathBuf>,
    pub(crate) report: Option<PathBuf>,
    pub(crate) mutant_report: Option<PathBuf>,
    pub(crate) schedule_seed: Option<u64>,
    pub(crate) ci_schedule_lane: Option<CiScheduleLane>,
    pub(crate) crash_seam: Option<String>,
    pub(crate) mutation_scope: MutationCampaignScope,
    pub(crate) mutant: Option<u8>,
    pub(crate) first_mutant: Option<u8>,
}

impl Arguments {
    pub(crate) fn parse(arguments: impl IntoIterator<Item = String>) -> Result<Self, String> {
        parsing::parse(arguments)
    }
}

pub(super) fn help_requested(arguments: &[String]) -> bool {
    matches!(arguments, [argument] if argument == "-h" || argument == "--help")
}

pub(super) fn usage() -> String {
    "usage: store-test-runner <owner -p PACKAGE|smoke|ui|mutants|courtrooms --courtroom a|b|c|ci --partition LANE|phase-eight-process> \
     [--shard-index N --shard-count N] \
     [--mutation-scope all|physical-work|bounded-residency|c8-closure] \
     [--mutant N|--from-mutant N] [--mutant-report PATH] \
     [--preflight] \
     [--schedule-seed U64 [--crash-seam NAME]|--ci-schedule-lane 0..15] \
     [--list] [--target-root PATH] [--report PATH]"
        .into()
}

#[cfg(test)]
mod tests;
