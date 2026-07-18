use super::arguments::{CliCommand, ParsedArguments};

pub(super) fn parse_closeout(arguments: Vec<String>) -> Result<ParsedArguments, String> {
    let action = arguments.first().ok_or_else(|| {
        "closeout requires preservation, mutations, iteration, or assemble".to_owned()
    })?;
    let options = &arguments[1..];
    let command = match action.as_str() {
        "preservation" if options.is_empty() => CliCommand::CloseoutPreservation,
        "mutations" if options.is_empty() => CliCommand::CloseoutMutations,
        "iteration" => CliCommand::CloseoutIteration {
            manifest_path: one_manifest(options, "iteration")?,
        },
        "assemble" => CliCommand::CloseoutAssemble {
            manifest_path: one_manifest(options, "assemble")?,
        },
        _ => return Err(format!("invalid closeout action or options: {action}")),
    };
    Ok(ParsedArguments { command })
}

pub(super) fn parse_artifacts(arguments: Vec<String>) -> Result<ParsedArguments, String> {
    let action = arguments
        .first()
        .ok_or_else(|| "artifacts requires prepare, inspect, plan, or execute".to_owned())?;
    let options = &arguments[1..];
    let command = match action.as_str() {
        "prepare" => {
            let target_root = option_value(options, "--target-root").ok_or_else(|| {
                "artifacts prepare requires --target-root <absolute-path>".to_owned()
            })?;
            if options.len() != 2 {
                return Err(
                    "artifacts prepare accepts only --target-root <absolute-path>".to_owned(),
                );
            }
            CliCommand::ArtifactPrepareRoot { target_root }
        }
        "inspect" => {
            let target_root = option_value(options, "--target-root").ok_or_else(|| {
                "artifacts inspect requires --target-root <absolute-path>".to_owned()
            })?;
            let protected_run = option_value(options, "--protected-run");
            let expected = if protected_run.is_some() { 4 } else { 2 };
            if options.len() != expected {
                return Err(
                    "artifacts inspect accepts --target-root and optional --protected-run"
                        .to_owned(),
                );
            }
            CliCommand::ArtifactInspect {
                target_root,
                protected_run,
            }
        }
        "plan" => {
            let inventory_path = option_value(options, "--inventory")
                .ok_or_else(|| "artifacts plan requires --inventory <path>".to_owned())?;
            let policy = option_value(options, "--policy")
                .ok_or_else(|| "artifacts plan requires --policy bounded-local".to_owned())?;
            if options.len() != 4 {
                return Err(
                    "artifacts plan accepts --inventory <path> --policy bounded-local".to_owned(),
                );
            }
            CliCommand::ArtifactPlan {
                inventory_path,
                policy,
            }
        }
        "execute" => {
            let plan_path = option_value(options, "--plan")
                .ok_or_else(|| "artifacts execute requires --plan <path>".to_owned())?;
            if options.len() != 2 {
                return Err("artifacts execute accepts only --plan <path>".to_owned());
            }
            CliCommand::ArtifactExecute { plan_path }
        }
        _ => return Err(format!("unknown artifacts action: {action}")),
    };
    Ok(ParsedArguments { command })
}

fn one_manifest(options: &[String], action: &str) -> Result<String, String> {
    let path = option_value(options, "--manifest")
        .ok_or_else(|| format!("closeout {action} requires --manifest <path>"))?;
    if options.len() != 2 {
        return Err(format!("closeout {action} accepts only --manifest <path>"));
    }
    Ok(path)
}

fn option_value(arguments: &[String], name: &str) -> Option<String> {
    arguments
        .windows(2)
        .find(|pair| pair[0] == name)
        .map(|pair| pair[1].clone())
}
