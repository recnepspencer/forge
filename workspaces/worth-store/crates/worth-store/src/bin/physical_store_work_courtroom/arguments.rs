use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum WriteScenario {
    SeedPriorTruth,
    BeforeBackendDispatch,
    DuringShortWrite,
    AfterExactWriteBeforeSchedulerSettlement,
    DuringRootPublication,
}

impl WriteScenario {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::SeedPriorTruth => "seed-prior-truth",
            Self::BeforeBackendDispatch => "before-backend-dispatch",
            Self::DuringShortWrite => "during-short-write",
            Self::AfterExactWriteBeforeSchedulerSettlement => {
                "after-exact-write-before-scheduler-settlement"
            }
            Self::DuringRootPublication => "during-root-publication",
        }
    }
}

pub(super) struct WriteInvocation {
    pub(super) root: PathBuf,
    pub(super) configuration: PathBuf,
    pub(super) oracle: PathBuf,
    pub(super) scenario: WriteScenario,
}

pub(super) struct ReopenInvocation {
    pub(super) root: PathBuf,
    pub(super) configuration: PathBuf,
}

pub(super) struct ShutdownInvocation {
    pub(super) root: PathBuf,
    pub(super) configuration: PathBuf,
}

pub(super) struct BoundedResidencyProducerInvocation {
    pub(super) root: PathBuf,
    pub(super) configuration: PathBuf,
}

pub(super) struct BoundedResidencyServingInvocation {
    pub(super) root: PathBuf,
    pub(super) configuration: PathBuf,
    pub(super) schedule: crate::bounded_residency::schedule::BoundedResidencySchedulePlan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum C7CrashSeamInvocation {
    BeforeWalAppend,
    DuringWalAppendPrefix,
    AfterWalWriteBeforeBarrier,
    AfterWalBarrierBeforeDataDispatch,
    DuringDataWritePrefix,
    AfterDataSettlementBeforeRootPublication,
    AfterRootReplacementBeforeNamespaceDurability,
    AfterPhysicalDurabilityBeforeAcknowledgment,
}

impl C7CrashSeamInvocation {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::BeforeWalAppend => "before-wal-append",
            Self::DuringWalAppendPrefix => "during-wal-append-prefix",
            Self::AfterWalWriteBeforeBarrier => "after-wal-write-before-barrier",
            Self::AfterWalBarrierBeforeDataDispatch => "after-wal-barrier-before-data-dispatch",
            Self::DuringDataWritePrefix => "during-data-write-prefix",
            Self::AfterDataSettlementBeforeRootPublication => {
                "after-data-settlement-before-root-publication"
            }
            Self::AfterRootReplacementBeforeNamespaceDurability => {
                "after-root-replacement-before-namespace-durability"
            }
            Self::AfterPhysicalDurabilityBeforeAcknowledgment => {
                "after-physical-durability-before-acknowledgment"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum C7DurabilityCheckpointOrderInvocation {
    CheckpointBeforeTarget,
    TargetSealedBeforeCheckpoint,
}

pub(super) struct C7CrashInvocation {
    pub(super) root: PathBuf,
    pub(super) configuration: PathBuf,
    pub(super) seam: C7CrashSeamInvocation,
    pub(super) checkpoint_order: C7DurabilityCheckpointOrderInvocation,
}

pub(super) enum CourtroomInvocation {
    Write(WriteInvocation),
    Reopen(ReopenInvocation),
    Shutdown(ShutdownInvocation),
    BoundedResidencyProducer(BoundedResidencyProducerInvocation),
    BoundedResidencyServing(BoundedResidencyServingInvocation),
    C7Crash(C7CrashInvocation),
}

struct CourtroomPaths {
    root: PathBuf,
    configuration: PathBuf,
}

#[derive(Default)]
struct OptionalExecutionArguments {
    oracle: Option<PathBuf>,
    scenario: Option<WriteScenario>,
    crash_seam: Option<C7CrashSeamInvocation>,
    schedule_plan: Option<String>,
}

struct ParsedCourtroomArguments {
    mode: String,
    paths: CourtroomPaths,
    optional: OptionalExecutionArguments,
}

impl CourtroomInvocation {
    pub(super) fn parse(arguments: impl IntoIterator<Item = OsString>) -> Result<Self, String> {
        ParsedCourtroomArguments::parse(arguments)?.into_invocation()
    }
}

impl ParsedCourtroomArguments {
    fn parse(arguments: impl IntoIterator<Item = OsString>) -> Result<Self, String> {
        let mut arguments = arguments.into_iter();
        let mode = text(arguments.next(), "missing courtroom mode")?;
        let mut root = None;
        let mut configuration = None;
        let mut optional = OptionalExecutionArguments::default();
        while let Some(option) = arguments.next() {
            let option = text(Some(option), "non-Unicode option")?;
            let value = arguments
                .next()
                .ok_or_else(|| format!("{option} requires a value"))?;
            match option.as_str() {
                "--root" => root = Some(PathBuf::from(value)),
                "--configuration" => configuration = Some(PathBuf::from(value)),
                "--oracle" => optional.oracle = Some(PathBuf::from(value)),
                "--scenario" => {
                    optional.scenario =
                        Some(parse_scenario(&text(Some(value), "non-Unicode scenario")?)?)
                }
                "--crash-seam" => {
                    optional.crash_seam = Some(parse_c7_crash_seam(&text(
                        Some(value),
                        "non-Unicode crash seam",
                    )?)?)
                }
                "--schedule-plan" => {
                    optional.schedule_plan = Some(text(Some(value), "non-Unicode schedule plan")?)
                }
                _ => return Err(format!("unknown courtroom option `{option}`")),
            }
        }
        Ok(Self {
            mode,
            paths: CourtroomPaths {
                root: root.ok_or_else(|| "--root is required".to_owned())?,
                configuration: configuration
                    .ok_or_else(|| "--configuration is required".to_owned())?,
            },
            optional,
        })
    }

    fn into_invocation(self) -> Result<CourtroomInvocation, String> {
        let Self {
            mode,
            paths,
            optional,
        } = self;
        match mode.as_str() {
            "write" => write(paths, optional).map(CourtroomInvocation::Write),
            "reopen" => simple(paths, optional, "reopen")
                .map(ReopenInvocation::from)
                .map(CourtroomInvocation::Reopen),
            "shutdown" => simple(paths, optional, "shutdown")
                .map(ShutdownInvocation::from)
                .map(CourtroomInvocation::Shutdown),
            "bounded-residency-producer" => simple(paths, optional, "bounded-residency-producer")
                .map(BoundedResidencyProducerInvocation::from)
                .map(CourtroomInvocation::BoundedResidencyProducer),
            "bounded-residency-serving" => {
                serving(paths, optional).map(CourtroomInvocation::BoundedResidencyServing)
            }
            "c7-crash" => c7_termination(paths, optional).map(CourtroomInvocation::C7Crash),
            _ => Err(format!("unknown courtroom mode `{mode}`")),
        }
    }
}

fn write(
    paths: CourtroomPaths,
    optional: OptionalExecutionArguments,
) -> Result<WriteInvocation, String> {
    if optional.schedule_plan.is_some() || optional.crash_seam.is_some() {
        return Err("write does not accept a schedule plan or crash seam".to_owned());
    }
    Ok(WriteInvocation {
        root: paths.root,
        configuration: paths.configuration,
        oracle: optional
            .oracle
            .ok_or_else(|| "write requires --oracle".to_owned())?,
        scenario: optional
            .scenario
            .ok_or_else(|| "write requires --scenario".to_owned())?,
    })
}

fn simple(
    paths: CourtroomPaths,
    optional: OptionalExecutionArguments,
    mode: &str,
) -> Result<CourtroomPaths, String> {
    deny_unexpected(&optional, mode)?;
    Ok(paths)
}

impl From<CourtroomPaths> for ReopenInvocation {
    fn from(paths: CourtroomPaths) -> Self {
        Self {
            root: paths.root,
            configuration: paths.configuration,
        }
    }
}

impl From<CourtroomPaths> for ShutdownInvocation {
    fn from(paths: CourtroomPaths) -> Self {
        Self {
            root: paths.root,
            configuration: paths.configuration,
        }
    }
}

impl From<CourtroomPaths> for BoundedResidencyProducerInvocation {
    fn from(paths: CourtroomPaths) -> Self {
        Self {
            root: paths.root,
            configuration: paths.configuration,
        }
    }
}

fn serving(
    paths: CourtroomPaths,
    optional: OptionalExecutionArguments,
) -> Result<BoundedResidencyServingInvocation, String> {
    if optional.oracle.is_some() || optional.scenario.is_some() || optional.crash_seam.is_some() {
        return Err("bounded-residency-serving accepts no oracle or scenario".to_owned());
    }
    Ok(BoundedResidencyServingInvocation {
        root: paths.root,
        configuration: paths.configuration,
        schedule: crate::bounded_residency::schedule::BoundedResidencySchedulePlan::parse(
            &optional
                .schedule_plan
                .ok_or_else(|| "bounded-residency-serving requires --schedule-plan".to_owned())?,
        )?,
    })
}

fn c7_termination(
    paths: CourtroomPaths,
    optional: OptionalExecutionArguments,
) -> Result<C7CrashInvocation, String> {
    if optional.oracle.is_some() || optional.scenario.is_some() {
        return Err("c7-crash accepts no oracle or write scenario".to_owned());
    }
    Ok(C7CrashInvocation {
        root: paths.root,
        configuration: paths.configuration,
        seam: optional
            .crash_seam
            .ok_or_else(|| "c7-crash requires --crash-seam".to_owned())?,
        checkpoint_order: parse_c7_checkpoint_order(
            &optional
                .schedule_plan
                .ok_or_else(|| "c7-crash requires --schedule-plan".to_owned())?,
        )?,
    })
}

fn parse_c7_checkpoint_order(value: &str) -> Result<C7DurabilityCheckpointOrderInvocation, String> {
    match value {
        "durability-checkpoint-order=checkpoint-before-target" => {
            Ok(C7DurabilityCheckpointOrderInvocation::CheckpointBeforeTarget)
        }
        "durability-checkpoint-order=target-sealed-before-checkpoint" => {
            Ok(C7DurabilityCheckpointOrderInvocation::TargetSealedBeforeCheckpoint)
        }
        _ => Err(format!("unknown C7 checkpoint schedule decision `{value}`")),
    }
}

fn parse_scenario(value: &str) -> Result<WriteScenario, String> {
    match value {
        "seed-prior-truth" => Ok(WriteScenario::SeedPriorTruth),
        "before-backend-dispatch" => Ok(WriteScenario::BeforeBackendDispatch),
        "during-short-write" => Ok(WriteScenario::DuringShortWrite),
        "after-exact-write-before-scheduler-settlement" => {
            Ok(WriteScenario::AfterExactWriteBeforeSchedulerSettlement)
        }
        "during-root-publication" => Ok(WriteScenario::DuringRootPublication),
        _ => Err(format!("unknown write scenario `{value}`")),
    }
}

fn parse_c7_crash_seam(value: &str) -> Result<C7CrashSeamInvocation, String> {
    match value {
        "before-wal-append" => Ok(C7CrashSeamInvocation::BeforeWalAppend),
        "during-wal-append-prefix" => Ok(C7CrashSeamInvocation::DuringWalAppendPrefix),
        "after-wal-write-before-barrier" => Ok(C7CrashSeamInvocation::AfterWalWriteBeforeBarrier),
        "after-wal-barrier-before-data-dispatch" => {
            Ok(C7CrashSeamInvocation::AfterWalBarrierBeforeDataDispatch)
        }
        "during-data-write-prefix" => Ok(C7CrashSeamInvocation::DuringDataWritePrefix),
        "after-data-settlement-before-root-publication" => {
            Ok(C7CrashSeamInvocation::AfterDataSettlementBeforeRootPublication)
        }
        "after-root-replacement-before-namespace-durability" => {
            Ok(C7CrashSeamInvocation::AfterRootReplacementBeforeNamespaceDurability)
        }
        "after-physical-durability-before-acknowledgment" => {
            Ok(C7CrashSeamInvocation::AfterPhysicalDurabilityBeforeAcknowledgment)
        }
        _ => Err(format!("unknown C7 crash seam `{value}`")),
    }
}

fn deny_unexpected(optional: &OptionalExecutionArguments, mode: &str) -> Result<(), String> {
    if optional.oracle.is_some()
        || optional.scenario.is_some()
        || optional.schedule_plan.is_some()
        || optional.crash_seam.is_some()
    {
        return Err(format!("{mode} received an unsupported execution option"));
    }
    Ok(())
}

fn text(value: Option<OsString>, failure: &str) -> Result<String, String> {
    value
        .ok_or_else(|| failure.to_owned())?
        .into_string()
        .map_err(|_| failure.to_owned())
}

#[cfg(test)]
#[path = "arguments/tests.rs"]
mod tests;
