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

pub(super) struct BoundedResidencyInvocation {
    pub(super) root: PathBuf,
    pub(super) configuration: PathBuf,
    pub(super) oracle: PathBuf,
}

pub(super) enum CourtroomInvocation {
    Write(WriteInvocation),
    Reopen(ReopenInvocation),
    Shutdown(ShutdownInvocation),
    BoundedResidency(BoundedResidencyInvocation),
}

impl CourtroomInvocation {
    pub(super) fn parse(arguments: impl IntoIterator<Item = OsString>) -> Result<Self, String> {
        let mut arguments = arguments.into_iter();
        let mode = text(arguments.next(), "missing courtroom mode")?;
        let mut root = None;
        let mut configuration = None;
        let mut oracle = None;
        let mut scenario = None;
        while let Some(option) = arguments.next() {
            let option = text(Some(option), "non-Unicode option")?;
            let value = arguments
                .next()
                .ok_or_else(|| format!("{option} requires a value"))?;
            match option.as_str() {
                "--root" => root = Some(PathBuf::from(value)),
                "--configuration" => configuration = Some(PathBuf::from(value)),
                "--oracle" => oracle = Some(PathBuf::from(value)),
                "--scenario" => {
                    scenario = Some(parse_scenario(&text(Some(value), "non-Unicode scenario")?)?)
                }
                _ => return Err(format!("unknown courtroom option `{option}`")),
            }
        }
        let root = root.ok_or_else(|| "--root is required".to_owned())?;
        let configuration =
            configuration.ok_or_else(|| "--configuration is required".to_owned())?;
        match mode.as_str() {
            "write" => Ok(Self::Write(WriteInvocation {
                root,
                configuration,
                oracle: oracle.ok_or_else(|| "write requires --oracle".to_owned())?,
                scenario: scenario.ok_or_else(|| "write requires --scenario".to_owned())?,
            })),
            "reopen" => {
                deny_unexpected(oracle, scenario, "reopen")?;
                Ok(Self::Reopen(ReopenInvocation {
                    root,
                    configuration,
                }))
            }
            "shutdown" => {
                deny_unexpected(oracle, scenario, "shutdown")?;
                Ok(Self::Shutdown(ShutdownInvocation {
                    root,
                    configuration,
                }))
            }
            "bounded-residency" => {
                if scenario.is_some() {
                    return Err("bounded-residency does not accept --scenario".to_owned());
                }
                Ok(Self::BoundedResidency(BoundedResidencyInvocation {
                    root,
                    configuration,
                    oracle: oracle
                        .ok_or_else(|| "bounded-residency requires --oracle".to_owned())?,
                }))
            }
            _ => Err(format!("unknown courtroom mode `{mode}`")),
        }
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

fn deny_unexpected(
    oracle: Option<PathBuf>,
    scenario: Option<WriteScenario>,
    mode: &str,
) -> Result<(), String> {
    if oracle.is_some() || scenario.is_some() {
        return Err(format!("{mode} accepts only root and configuration"));
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
mod tests {
    use super::{CourtroomInvocation, WriteScenario};

    #[test]
    fn reopen_cannot_receive_oracle_or_scenario_state() {
        for extra in [["--oracle", "oracle"], ["--scenario", "seed-prior-truth"]] {
            let arguments = [
                "reopen",
                "--root",
                "root",
                "--configuration",
                "configuration",
                extra[0],
                extra[1],
            ]
            .into_iter()
            .map(Into::into);
            assert!(CourtroomInvocation::parse(arguments).is_err());
        }
    }

    #[test]
    fn write_scenario_is_typed_at_the_process_boundary() {
        let parsed = CourtroomInvocation::parse(
            [
                "write",
                "--root",
                "root",
                "--configuration",
                "configuration",
                "--oracle",
                "oracle",
                "--scenario",
                "before-backend-dispatch",
            ]
            .into_iter()
            .map(Into::into),
        )
        .unwrap();
        let CourtroomInvocation::Write(invocation) = parsed else {
            panic!("write invocation must retain its mode");
        };
        assert_eq!(invocation.scenario, WriteScenario::BeforeBackendDispatch);
    }

    #[test]
    fn bounded_residency_requires_its_parent_declared_oracle() {
        let denied = CourtroomInvocation::parse(
            [
                "bounded-residency",
                "--root",
                "root",
                "--configuration",
                "configuration",
            ]
            .into_iter()
            .map(Into::into),
        );
        assert!(denied.is_err());

        let admitted = CourtroomInvocation::parse(
            [
                "bounded-residency",
                "--root",
                "root",
                "--configuration",
                "configuration",
                "--oracle",
                "oracle",
            ]
            .into_iter()
            .map(Into::into),
        )
        .unwrap();
        assert!(matches!(admitted, CourtroomInvocation::BoundedResidency(_)));
    }
}
