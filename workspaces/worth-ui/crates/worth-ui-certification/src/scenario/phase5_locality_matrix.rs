mod application;
mod case;
mod dependency_model;
mod execution;
mod execution_mode;
mod joined_rows;
mod oracle;
mod presentation_cost_model;
mod process_execution;
mod report_join;
mod retained_order_reference;
mod shard_admission;
mod shard_contract;
mod shard_orchestration;
mod timings;

use case::{Phase5LocalityAxis, Phase5LocalityCase};
use std::path::Path;

pub(super) const RETAINED_SIZES: [usize; 4] = [1, 32, 2_048, 4_096];

pub fn execute_worker(
    worker_executable: &Path,
    worker_arguments: &[&str],
) -> Result<Vec<serde_json::Value>, String> {
    match execution_mode::read()? {
        execution_mode::Phase5LocalityExecutionPlan::LocalClosure => {
            shard_orchestration::execute(worker_executable, worker_arguments)
        }
        execution_mode::Phase5LocalityExecutionPlan::Shard {
            shard_index,
            shard_count,
            deadline,
        } => execute_shard(
            worker_executable,
            worker_arguments,
            shard_index,
            shard_count,
            deadline,
        ),
        execution_mode::Phase5LocalityExecutionPlan::Join { directory } => {
            report_join::join(&directory)
        }
        execution_mode::Phase5LocalityExecutionPlan::Case { case, deadline } => {
            execute_case(case, deadline)
        }
    }
}

pub fn execute_local_closure(
    worker_executable: &Path,
    worker_arguments: &[&str],
) -> Result<Vec<serde_json::Value>, String> {
    shard_orchestration::execute(worker_executable, worker_arguments)
}

fn execute_case(
    case: Phase5LocalityCase,
    _deadline: u128,
) -> Result<Vec<serde_json::Value>, String> {
    let evidence = execution::execute(case)?;
    Ok(vec![oracle::adjudicate(evidence)?])
}

fn execute_shard(
    worker_executable: &Path,
    worker_arguments: &[&str],
    shard_index: usize,
    shard_count: usize,
    deadline: u128,
) -> Result<Vec<serde_json::Value>, String> {
    shard_contract::validate_shard(shard_index, shard_count)?;
    let total = RETAINED_SIZES.len() * Phase5LocalityAxis::ALL.len();
    let expected = (0..total)
        .filter(|ordinal| ordinal % shard_count == shard_index)
        .count();
    let mut rows = Vec::with_capacity(expected);
    for (retained_ordinal, retained_size) in RETAINED_SIZES.into_iter().enumerate() {
        for (axis_ordinal, axis) in Phase5LocalityAxis::ALL.into_iter().enumerate() {
            let ordinal = retained_ordinal * Phase5LocalityAxis::ALL.len() + axis_ordinal;
            if ordinal % shard_count != shard_index {
                continue;
            }
            let filter = format!("{}:{retained_size}", axis.label());
            let mut command = std::process::Command::new(worker_executable);
            command
                .args(worker_arguments)
                .env(execution_mode::MODE_ENV, execution_mode::CASE_MODE)
                .env(execution_mode::CASE_ENV, &filter)
                .env(process_execution::DEADLINE_ENV, deadline.to_string())
                .env_remove(execution_mode::SHARD_ENV)
                .env_remove(execution_mode::JOIN_ENV);
            let output = process_execution::run_until(&mut command, deadline, &filter)?;
            if !output.status().success() {
                return Err(format!(
                    "matrix child {filter} exited {:?}",
                    output.status().code()
                ));
            }
            let stdout = String::from_utf8(output.stdout().to_vec())
                .map_err(|denial| format!("matrix child {filter} output encoding: {denial}"))?;
            let row = parse_child_row(&stdout, &filter)?;
            rows.push(row);
        }
    }
    assert_eq!(rows.len(), expected);
    Ok(rows)
}

fn parse_child_row(stdout: &str, filter: &str) -> Result<serde_json::Value, String> {
    const PREFIX: &str = "WORTH_UI_PHASE5_PRODUCTION_LOCALITY=";
    let payload = stdout
        .lines()
        .find_map(|line| line.strip_prefix(PREFIX))
        .ok_or_else(|| format!("matrix child {filter} omitted its evidence row"))?;
    let rows: Vec<serde_json::Value> = serde_json::from_str(payload)
        .map_err(|denial| format!("matrix child {filter} evidence encoding: {denial}"))?;
    let [row] = rows.as_slice() else {
        return Err(format!("matrix child {filter} emitted {} rows", rows.len()));
    };
    Ok(row.clone())
}
