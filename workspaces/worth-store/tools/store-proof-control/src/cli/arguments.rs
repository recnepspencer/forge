use super::stage_arguments::{parse_artifacts, parse_closeout};
use crate::selection::{RequestedSourceEdit, StoreProofMode, StoreProofRequest};

pub enum CliCommand {
    Baseline {
        observe_artifacts: bool,
    },
    AuditExecutableListing,
    Validate,
    SealProofAuthority,
    SealProofBehaviorAuthority,
    SealScenarioAuthority,
    InternalObserve {
        request_path: String,
    },
    CiAggregate {
        evidence_root: String,
    },
    ArtifactInspect {
        target_root: String,
        protected_run: Option<String>,
    },
    ArtifactPrepareRoot {
        target_root: String,
    },
    ArtifactPlan {
        inventory_path: String,
        policy: String,
    },
    ArtifactExecute {
        plan_path: String,
    },
    CloseoutPreservation,
    CloseoutMutations,
    CloseoutIteration {
        manifest_path: String,
    },
    CloseoutAssemble {
        manifest_path: String,
    },
    Proof {
        request: StoreProofRequest,
        preflight_bundle: Option<String>,
    },
}

pub struct ParsedArguments {
    pub command: CliCommand,
}

impl ParsedArguments {
    pub fn parse(mut arguments: impl Iterator<Item = String>) -> Result<Self, String> {
        let command = arguments.next().ok_or_else(usage)?;
        if command == "baseline" {
            return Ok(Self {
                command: CliCommand::Baseline {
                    observe_artifacts: arguments.any(|argument| argument == "--observe-artifacts"),
                },
            });
        }
        if command == "validate" {
            return Ok(Self {
                command: CliCommand::Validate,
            });
        }
        if command == "audit-executable-listing" {
            return Ok(Self {
                command: CliCommand::AuditExecutableListing,
            });
        }
        if command == "seal-scenario-authority" {
            return Ok(Self {
                command: CliCommand::SealScenarioAuthority,
            });
        }
        if command == "seal-proof-authority" {
            return Ok(Self {
                command: CliCommand::SealProofAuthority,
            });
        }
        if command == "seal-proof-behavior-authority" {
            return Ok(Self {
                command: CliCommand::SealProofBehaviorAuthority,
            });
        }
        if command == "internal-observe" {
            let remaining: Vec<_> = arguments.collect();
            let request_path = option_value(&remaining, "--request")
                .ok_or_else(|| "internal-observe requires --request <path>".to_owned())?;
            if remaining.len() != 2 {
                return Err("internal-observe accepts only --request <path>".to_owned());
            }
            return Ok(Self {
                command: CliCommand::InternalObserve { request_path },
            });
        }
        if command == "ci-aggregate" {
            let remaining: Vec<_> = arguments.collect();
            let evidence_root = option_value(&remaining, "--evidence-root")
                .ok_or_else(|| "ci-aggregate requires --evidence-root <path>".to_owned())?;
            if remaining.len() != 2 {
                return Err("ci-aggregate accepts only --evidence-root <path>".to_owned());
            }
            return Ok(Self {
                command: CliCommand::CiAggregate { evidence_root },
            });
        }
        if command == "artifacts" {
            return parse_artifacts(arguments.collect());
        }
        if command == "closeout" {
            return parse_closeout(arguments.collect());
        }
        let mode = parse_mode(&command).ok_or_else(usage)?;
        let remaining: Vec<_> = arguments.collect();
        let package =
            option_value(&remaining, "-p").or_else(|| option_value(&remaining, "--package"));
        let partition = option_value(&remaining, "--partition");
        let proof_profile = option_value(&remaining, "--profile");
        let scenario_identity = option_value(&remaining, "--scenario");
        let seed = option_value(&remaining, "--seed")
            .map(|value| {
                value.parse::<u64>().map_err(|_| {
                    format!("--seed requires an unsigned 64-bit integer, got {value:?}")
                })
            })
            .transpose()?;
        let backend = option_value(&remaining, "--backend");
        let preflight_bundle = option_value(&remaining, "--preflight-bundle");
        let target_root = option_value(&remaining, "--target-root");
        let source_edit = source_edit(&remaining)?;
        let shard_index = parsed_usize_option(&remaining, "--shard-index")?;
        let shard_count = parsed_usize_option(&remaining, "--shard-count")?;
        let plan_only = remaining.iter().any(|argument| argument == "--plan-only");
        reject_unknown_options(&remaining)?;
        Ok(Self {
            command: CliCommand::Proof {
                request: StoreProofRequest::new(
                    mode,
                    package,
                    partition,
                    proof_profile,
                    scenario_identity,
                    plan_only,
                )
                .with_seed(seed)
                .with_backend(backend)
                .with_shard(shard_index, shard_count)
                .with_target_root(target_root)
                .with_source_edit(source_edit),
                preflight_bundle,
            },
        })
    }
}

fn source_edit(arguments: &[String]) -> Result<Option<RequestedSourceEdit>, String> {
    let source_path = option_value(arguments, "--edit-source");
    let original_sha256 = option_value(arguments, "--edit-original-sha256");
    let purpose = option_value(arguments, "--edit-purpose");
    let description = option_value(arguments, "--edit-description");
    match (source_path, original_sha256, purpose, description) {
        (None, None, None, None) => Ok(None),
        (Some(source_path), Some(original_sha256), Some(purpose), Some(description)) => Ok(Some(
            RequestedSourceEdit::new(source_path, original_sha256, purpose, description),
        )),
        _ => Err("source edit evidence requires --edit-source, --edit-original-sha256, --edit-purpose, and --edit-description together".to_owned()),
    }
}

fn parse_mode(value: &str) -> Option<StoreProofMode> {
    match value {
        "owner" => Some(StoreProofMode::Owner),
        "smoke" => Some(StoreProofMode::Smoke),
        "ui" => Some(StoreProofMode::Ui),
        "ci" => Some(StoreProofMode::Ci),
        "soak" => Some(StoreProofMode::Soak),
        "release" => Some(StoreProofMode::Release),
        "hardware" => Some(StoreProofMode::Hardware),
        _ => None,
    }
}

fn option_value(arguments: &[String], name: &str) -> Option<String> {
    arguments
        .windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}

fn reject_unknown_options(arguments: &[String]) -> Result<(), String> {
    let value_options = [
        "-p",
        "--package",
        "--partition",
        "--profile",
        "--scenario",
        "--seed",
        "--backend",
        "--preflight-bundle",
        "--target-root",
        "--edit-source",
        "--edit-original-sha256",
        "--edit-purpose",
        "--edit-description",
        "--shard-index",
        "--shard-count",
    ];
    let mut index = 0;
    while index < arguments.len() {
        let argument = &arguments[index];
        if value_options.contains(&argument.as_str()) {
            if index + 1 >= arguments.len() {
                return Err(format!("missing value after {argument}"));
            }
            index += 2;
            continue;
        }
        if argument == "--plan-only" {
            index += 1;
            continue;
        }
        return Err(format!("unknown proof-control argument: {argument}"));
    }
    Ok(())
}

fn parsed_usize_option(arguments: &[String], name: &str) -> Result<Option<usize>, String> {
    option_value(arguments, name)
        .map(|value| {
            value
                .parse::<usize>()
                .map_err(|_| format!("{name} requires an unsigned integer, got {value:?}"))
        })
        .transpose()
}

fn usage() -> String {
    "usage: store-proof-control <baseline|audit-executable-listing|validate|seal-proof-authority|seal-proof-behavior-authority|seal-scenario-authority|artifacts|closeout|owner|smoke|ui|ci|soak|release|hardware>"
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::{CliCommand, ParsedArguments};

    #[test]
    fn documented_seed_and_backend_options_are_plan_bound() {
        let soak = ParsedArguments::parse(
            ["soak", "--profile", "checkpoint-heavy", "--seed", "42"]
                .into_iter()
                .map(str::to_owned),
        )
        .unwrap();
        let CliCommand::Proof { request: soak, .. } = soak.command else {
            panic!("soak parsed as a non-proof command");
        };
        assert_eq!(soak.proof_profile(), Some("checkpoint-heavy"));
        assert_eq!(soak.seed(), Some(42));

        let release = ParsedArguments::parse(
            [
                "release",
                "--backend",
                "windows-file",
                "--preflight-bundle",
                ".store-proof/evidence/preflight/complete.json",
            ]
            .into_iter()
            .map(str::to_owned),
        )
        .unwrap();
        let CliCommand::Proof {
            request: release,
            preflight_bundle,
        } = release.command
        else {
            panic!("release parsed as a non-proof command");
        };
        assert_eq!(release.backend(), Some("windows-file"));
        assert_eq!(
            preflight_bundle.as_deref(),
            Some(".store-proof/evidence/preflight/complete.json")
        );
    }

    #[test]
    fn malformed_seed_is_rejected_before_selection() {
        let denial = ParsedArguments::parse(
            ["soak", "--profile", "checkpoint-heavy", "--seed", "random"]
                .into_iter()
                .map(str::to_owned),
        )
        .err()
        .unwrap();
        assert!(denial.contains("unsigned 64-bit integer"));
    }

    #[test]
    fn artifact_lifecycle_commands_require_explicit_stage_inputs() {
        let inspect = ParsedArguments::parse(
            [
                "artifacts",
                "inspect",
                "--target-root",
                "C:/workspace/target",
            ]
            .into_iter()
            .map(str::to_owned),
        )
        .unwrap();
        assert!(matches!(
            inspect.command,
            CliCommand::ArtifactInspect { .. }
        ));

        let execute = ParsedArguments::parse(
            ["artifacts", "execute", "--plan", "cleanup.json"]
                .into_iter()
                .map(str::to_owned),
        )
        .unwrap();
        assert!(matches!(
            execute.command,
            CliCommand::ArtifactExecute { .. }
        ));
        let denial = ParsedArguments::parse(
            [
                "artifacts",
                "execute",
                "--target-root",
                "C:/workspace/target",
            ]
            .into_iter()
            .map(str::to_owned),
        )
        .err()
        .unwrap();
        assert!(denial.contains("requires --plan"));
    }

    #[test]
    fn closeout_stages_and_iteration_target_roots_are_explicit() {
        let preservation =
            ParsedArguments::parse(["closeout", "preservation"].into_iter().map(str::to_owned))
                .unwrap();
        assert!(matches!(
            preservation.command,
            CliCommand::CloseoutPreservation
        ));

        let iteration = ParsedArguments::parse(
            ["closeout", "iteration", "--manifest", "iteration.json"]
                .into_iter()
                .map(str::to_owned),
        )
        .unwrap();
        assert!(matches!(
            iteration.command,
            CliCommand::CloseoutIteration { .. }
        ));

        let proof = ParsedArguments::parse(
            [
                "smoke",
                "--target-root",
                "C:/workspace/.store-proof/clean-target",
                "--plan-only",
            ]
            .into_iter()
            .map(str::to_owned),
        )
        .unwrap();
        let CliCommand::Proof { request, .. } = proof.command else {
            panic!("smoke parsed as a non-proof command");
        };
        assert_eq!(
            request.target_root(),
            Some("C:/workspace/.store-proof/clean-target")
        );
    }
}
