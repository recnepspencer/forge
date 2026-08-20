mod application;
mod case;
mod ci_join;
mod dependency_model;
mod execution;
mod hostile_cost_model;
mod oracle;
mod presentation_cost_model;
mod process_execution;
mod retained_order_reference;
mod shard_orchestration;
mod timings;

use case::{Phase5LocalityAxis, Phase5LocalityCase};

pub use hostile_cost_model::expected_for_label as cost_hostile_cases_for_axis;
pub use hostile_cost_model::ALL as COST_HOSTILE_CASES;

const RETAINED_SIZES: [usize; 4] = [1, 32, 2_048, 4_096];

pub fn execute() -> Result<Vec<serde_json::Value>, String> {
    if let Some(directory) = std::env::var_os("WORTH_UI_PHASE5_MATRIX_JOIN_DIR") {
        return ci_join::join(std::path::Path::new(&directory));
    }
    let development_filter = std::env::var("WORTH_UI_PHASE5_MATRIX_CASE").ok();
    if development_filter.is_none() {
        return execute_isolated_matrix();
    }
    let mut rows = Vec::with_capacity(1);
    for retained_size in RETAINED_SIZES {
        for axis in Phase5LocalityAxis::ALL {
            if development_filter
                .as_deref()
                .is_some_and(|filter| filter != format!("{}:{retained_size}", axis.label()))
            {
                continue;
            }
            let case = Phase5LocalityCase::new(retained_size, axis);
            let evidence = execution::execute(case)?;
            rows.push(oracle::adjudicate(evidence)?);
        }
    }
    assert_eq!(rows.len(), 1, "development filter must name one matrix row");
    Ok(rows)
}

fn execute_isolated_matrix() -> Result<Vec<serde_json::Value>, String> {
    let Some((shard_index, shard_count)) = requested_shard()? else {
        return shard_orchestration::execute();
    };
    execute_shard(shard_index, shard_count)
}

fn execute_shard(shard_index: usize, shard_count: usize) -> Result<Vec<serde_json::Value>, String> {
    let executable = std::env::current_exe()
        .map_err(|denial| format!("matrix executable identity: {denial}"))?;
    let deadline = process_execution::deadline_from_environment()?;
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
            let mut command = std::process::Command::new(&executable);
            command.env("WORTH_UI_PHASE5_MATRIX_CASE", &filter);
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

fn requested_shard() -> Result<Option<(usize, usize)>, String> {
    let Some(value) = std::env::var("WORTH_UI_PHASE5_MATRIX_SHARD").ok() else {
        return Ok(None);
    };
    let (index, count) = value
        .split_once('/')
        .ok_or_else(|| "matrix shard must use INDEX/COUNT".to_owned())?;
    let index = index
        .parse::<usize>()
        .map_err(|_| "matrix shard index is not an integer".to_owned())?;
    let count = count
        .parse::<usize>()
        .map_err(|_| "matrix shard count is not an integer".to_owned())?;
    if count == 0 || index >= count {
        return Err(format!("matrix shard {index}/{count} is out of range"));
    }
    Ok(Some((index, count)))
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
