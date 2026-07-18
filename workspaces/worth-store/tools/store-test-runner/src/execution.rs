use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use crate::plan::{TestExecutionUnit, TestPlan};
use crate::report::{TestRunReport, UnitResult};

pub(crate) fn execute(plan: &TestPlan, target_root: Option<&Path>) -> TestRunReport {
    let started = Instant::now();
    let (results, failure) = if plan.may_run_concurrently() {
        execute_concurrently(plan, target_root)
    } else {
        execute_sequentially(plan, target_root)
    };

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

fn execute_concurrently(
    plan: &TestPlan,
    target_root: Option<&Path>,
) -> (Vec<UnitResult>, Option<String>) {
    let worker_count = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(2)
        .min(4)
        .min(plan.units().len());
    let next = AtomicUsize::new(0);
    let failed = AtomicBool::new(false);
    let results = Mutex::new(Vec::new());
    let failure = Mutex::new(None);

    std::thread::scope(|scope| {
        for _ in 0..worker_count {
            scope.spawn(|| loop {
                if failed.load(Ordering::Acquire) {
                    return;
                }
                let index = next.fetch_add(1, Ordering::AcqRel);
                let Some(unit) = plan.units().get(index) else {
                    return;
                };
                println!("run: {}", unit.identity());
                match execute_unit(unit, target_root) {
                    Ok(result) if result.success => results.lock().unwrap().push(result),
                    Ok(result) => {
                        let message = format!("unit `{}` failed", result.identity);
                        results.lock().unwrap().push(result);
                        let mut first_failure = failure.lock().unwrap();
                        if first_failure.is_none() {
                            *first_failure = Some(message);
                        }
                        failed.store(true, Ordering::Release);
                        return;
                    }
                    Err(error) => {
                        let mut first_failure = failure.lock().unwrap();
                        if first_failure.is_none() {
                            *first_failure = Some(error);
                        }
                        failed.store(true, Ordering::Release);
                        return;
                    }
                }
            });
        }
    });

    let mut results = results.into_inner().unwrap();
    results.sort_by(|left, right| left.identity.cmp(&right.identity));
    (results, failure.into_inner().unwrap())
}

fn execute_unit(
    unit: &TestExecutionUnit,
    target_root: Option<&Path>,
) -> Result<UnitResult, String> {
    if unit.is_filtered() {
        require_matching_test(unit, target_root)?;
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

fn require_matching_test(
    unit: &TestExecutionUnit,
    target_root: Option<&Path>,
) -> Result<(), String> {
    let mut command = command(unit, target_root);
    command.args(["--", "--list"]);
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
    let matches = listed_test_count(&listing);
    if matches == 0 {
        Err(format!(
            "filtered unit `{}` matched zero tests",
            unit.identity()
        ))
    } else {
        Ok(())
    }
}

fn listed_test_count(listing: &str) -> usize {
    listing
        .lines()
        .filter(|line| line.trim_end().ends_with(": test"))
        .count()
}

fn command(unit: &TestExecutionUnit, target_root: Option<&Path>) -> Command {
    let mut command = Command::new(unit.program());
    command.args(unit.arguments()).current_dir(unit.directory());
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
    fn libtest_listing_requires_a_real_test() {
        let empty = "0 tests, 0 benchmarks\n";
        let populated = "module::works: test\n\n1 test, 0 benchmarks\n";
        assert_eq!(listed_test_count(empty), 0);
        assert_eq!(listed_test_count(populated), 1);
    }
}
