use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

use crate::plan::{TestExecutionUnit, TestPlan};
use crate::report::{TestRunReport, UnitResult};

pub(crate) fn execute(plan: &TestPlan, target_root: Option<&Path>) -> TestRunReport {
    let started = Instant::now();
    let (results, failure) = execute_sequentially(plan, target_root);

    TestRunReport {
        product: plan.product_name(),
        revision: git_revision(plan),
        elapsed_ms: started.elapsed().as_millis(),
        success: failure.is_none() && results.len() == plan.units().len(),
        failure,
        units: results,
    }
}

fn execute_sequentially(
    plan: &TestPlan,
    target_root: Option<&Path>,
) -> (Vec<UnitResult>, Option<String>) {
    let mut results = Vec::new();

    for unit in plan.units() {
        println!("run: {}", unit.identity());
        match execute_unit(unit, target_root) {
            Ok(result) if result.success => results.push(result),
            Ok(result) => {
                let failure = Some(format!("unit `{}` failed", result.identity));
                results.push(result);
                return (results, failure);
            }
            Err(error) => return (results, Some(error)),
        }
    }
    (results, None)
}

fn execute_unit(
    unit: &TestExecutionUnit,
    target_root: Option<&Path>,
) -> Result<UnitResult, String> {
    if let Some(expected) = unit.expected_test_count() {
        require_matching_tests(unit, target_root, expected)?;
    }
    let started = Instant::now();
    let success = run(unit, target_root)?;
    Ok(UnitResult {
        identity: unit.identity().into(),
        command: unit.display_command(),
        elapsed_ms: started.elapsed().as_millis(),
        success,
    })
}

fn run(unit: &TestExecutionUnit, target_root: Option<&Path>) -> Result<bool, String> {
    let mut command = command(unit, target_root);
    command
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    command
        .status()
        .map(|status| status.success())
        .map_err(|error| format!("failed to start `{}`: {error}", unit.display_command()))
}

fn require_matching_tests(
    unit: &TestExecutionUnit,
    target_root: Option<&Path>,
    expected: usize,
) -> Result<(), String> {
    let mut arguments = unit.arguments().to_vec();
    let nextest = arguments.starts_with(&["nextest".into(), "run".into()]);
    if nextest {
        arguments[1] = "list".into();
        arguments.retain(|argument| argument != "--no-fail-fast");
        arguments.extend(["--message-format".into(), "oneline".into()]);
    } else {
        if arguments.iter().any(|argument| argument == "--") {
            arguments.push("--list".into());
        } else {
            arguments.extend(["--".into(), "--list".into()]);
        }
    }
    let mut command = command_with_arguments(unit, &arguments, target_root);
    let output = command.output().map_err(|error| {
        format!(
            "failed to list filtered unit `{}`: {error}",
            unit.identity()
        )
    })?;
    if !output.status.success() {
        return Err(format!(
            "listing filtered unit `{}` failed:\n{}",
            unit.identity(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let listing = String::from_utf8_lossy(&output.stdout);
    let matches = listed_test_count(&listing, nextest);
    if matches != expected {
        Err(format!(
            "filtered unit `{}` must match exactly {expected} tests, matched {matches}",
            unit.identity()
        ))
    } else {
        Ok(())
    }
}

fn listed_test_count(listing: &str, nextest: bool) -> usize {
    listing
        .lines()
        .filter(|line| {
            if nextest {
                !line.trim().is_empty()
            } else {
                line.trim_end().ends_with(": test")
            }
        })
        .count()
}

fn command(unit: &TestExecutionUnit, target_root: Option<&Path>) -> Command {
    command_with_arguments(unit, unit.arguments(), target_root)
}

fn command_with_arguments(
    unit: &TestExecutionUnit,
    arguments: &[String],
    target_root: Option<&Path>,
) -> Command {
    let mut command = Command::new(unit.program());
    command.args(arguments).current_dir(unit.directory());
    if unit.program() == "cargo" {
        if let Some(root) = target_root {
            command.env("CARGO_TARGET_DIR", root);
        }
    }
    command
}

fn git_revision(plan: &TestPlan) -> Option<String> {
    let directory = plan.units().first()?.directory();
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(directory)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::listed_test_count;

    #[test]
    fn machine_readable_listing_requires_the_expected_tests() {
        let empty = "0 tests, 0 benchmarks\n";
        let populated = "module::works: test\n\n1 test, 0 benchmarks\n";
        assert_eq!(listed_test_count(empty, false), 0);
        assert_eq!(listed_test_count(populated, false), 1);
        assert_eq!(listed_test_count("package::binary module::test\n", true), 1);
    }
}
