use std::path::PathBuf;

use super::case::{Phase5LocalityAxis, Phase5LocalityCase};
use super::process_execution;

pub(super) const MODE_ENV: &str = "WORTH_UI_PHASE5_MATRIX_MODE";
pub(super) const LOCAL_CLOSURE_MODE: &str = "local-closure";
pub(super) const SHARD_MODE: &str = "shard";
pub(super) const JOIN_MODE: &str = "join";
pub(super) const CASE_MODE: &str = "case";
pub(super) const SHARD_ENV: &str = "WORTH_UI_PHASE5_MATRIX_SHARD";
pub(super) const JOIN_ENV: &str = "WORTH_UI_PHASE5_MATRIX_JOIN_DIR";
pub(super) const CASE_ENV: &str = "WORTH_UI_PHASE5_MATRIX_CASE";
const CANCEL_ENV: &str = "WORTH_UI_PHASE5_MATRIX_CANCEL_FILE";

pub(super) enum Phase5LocalityExecutionPlan {
    LocalClosure,
    Shard {
        shard_index: usize,
        shard_count: usize,
        deadline: u128,
    },
    Join {
        directory: PathBuf,
    },
    Case {
        case: Phase5LocalityCase,
        deadline: u128,
    },
}

pub(super) fn read() -> Result<Phase5LocalityExecutionPlan, String> {
    let mode = std::env::var(MODE_ENV)
        .map_err(|_| format!("{MODE_ENV} must select an explicit execution mode"))?;
    match mode.as_str() {
        LOCAL_CLOSURE_MODE => local_closure(),
        SHARD_MODE => shard(),
        JOIN_MODE => join(),
        CASE_MODE => case(),
        other => Err(format!("{MODE_ENV} has unknown value {other:?}")),
    }
}

fn local_closure() -> Result<Phase5LocalityExecutionPlan, String> {
    reject_present(SHARD_ENV)?;
    reject_present(JOIN_ENV)?;
    reject_present(CASE_ENV)?;
    reject_present(process_execution::DEADLINE_ENV)?;
    reject_present(CANCEL_ENV)?;
    Ok(Phase5LocalityExecutionPlan::LocalClosure)
}

fn shard() -> Result<Phase5LocalityExecutionPlan, String> {
    reject_present(JOIN_ENV)?;
    reject_present(CASE_ENV)?;
    let (shard_index, shard_count) = parse_shard()?;
    Ok(Phase5LocalityExecutionPlan::Shard {
        shard_index,
        shard_count,
        deadline: deadline()?,
    })
}

fn join() -> Result<Phase5LocalityExecutionPlan, String> {
    reject_present(SHARD_ENV)?;
    reject_present(CASE_ENV)?;
    reject_present(process_execution::DEADLINE_ENV)?;
    reject_present(CANCEL_ENV)?;
    let directory = std::env::var_os(JOIN_ENV)
        .map(PathBuf::from)
        .ok_or_else(|| format!("{JOIN_ENV} is required for join mode"))?;
    Ok(Phase5LocalityExecutionPlan::Join { directory })
}

fn case() -> Result<Phase5LocalityExecutionPlan, String> {
    reject_present(SHARD_ENV)?;
    reject_present(JOIN_ENV)?;
    let filter =
        std::env::var(CASE_ENV).map_err(|_| format!("{CASE_ENV} is required for case mode"))?;
    Ok(Phase5LocalityExecutionPlan::Case {
        case: parse_case(&filter)?,
        deadline: deadline()?,
    })
}

fn parse_shard() -> Result<(usize, usize), String> {
    let value =
        std::env::var(SHARD_ENV).map_err(|_| format!("{SHARD_ENV} is required for shard mode"))?;
    let (index, count) = value
        .split_once('/')
        .ok_or_else(|| format!("{SHARD_ENV} must use INDEX/COUNT"))?;
    let index = index
        .parse::<usize>()
        .map_err(|_| format!("{SHARD_ENV} index is not an integer"))?;
    let count = count
        .parse::<usize>()
        .map_err(|_| format!("{SHARD_ENV} count is not an integer"))?;
    if count == 0 || index >= count {
        return Err(format!("matrix shard {index}/{count} is out of range"));
    }
    Ok((index, count))
}

fn parse_case(filter: &str) -> Result<Phase5LocalityCase, String> {
    let (axis, retained) = filter
        .split_once(':')
        .ok_or_else(|| format!("{CASE_ENV} must use AXIS:RETAINED"))?;
    let axis = Phase5LocalityAxis::ALL
        .into_iter()
        .find(|candidate| candidate.label() == axis)
        .ok_or_else(|| format!("{CASE_ENV} has unknown axis {axis:?}"))?;
    let retained = retained
        .parse::<usize>()
        .map_err(|_| format!("{CASE_ENV} retained size is not an integer"))?;
    if ![1, 32, 2_048, 4_096].contains(&retained) {
        return Err(format!(
            "{CASE_ENV} has unqualified retained size {retained}"
        ));
    }
    Ok(Phase5LocalityCase::new(retained, axis))
}

fn deadline() -> Result<u128, String> {
    if std::env::var_os(process_execution::DEADLINE_ENV).is_some() {
        process_execution::deadline_from_environment()
    } else {
        process_execution::new_deadline()
    }
}

fn reject_present(name: &str) -> Result<(), String> {
    if std::env::var_os(name).is_some() {
        return Err(format!(
            "{name} is not valid in the selected execution mode"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn mode_names_are_distinct() {
        assert_ne!(super::LOCAL_CLOSURE_MODE, super::SHARD_MODE);
        assert_ne!(super::SHARD_MODE, super::JOIN_MODE);
        assert_ne!(super::JOIN_MODE, super::CASE_MODE);
    }
}
