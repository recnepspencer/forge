use std::process::Command;

use super::case::Phase5LocalityAxis;
use super::process_execution::{self, DEADLINE_ENV};
use super::RETAINED_SIZES;

const SHARD_COUNT: usize = 16;
const MAXIMUM_PARALLEL_SHARDS: usize = 8;
const EVIDENCE_PREFIX: &str = "WORTH_UI_PHASE5_PRODUCTION_LOCALITY=";

pub(super) fn execute() -> Result<Vec<serde_json::Value>, String> {
    let executable = std::env::current_exe()
        .map_err(|denial| format!("matrix executable identity: {denial}"))?;
    let deadline = process_execution::new_deadline()?;
    let mut rows = Vec::with_capacity(RETAINED_SIZES.len() * Phase5LocalityAxis::ALL.len());
    for wave_start in (0..SHARD_COUNT).step_by(MAXIMUM_PARALLEL_SHARDS) {
        let wave_end = (wave_start + MAXIMUM_PARALLEL_SHARDS).min(SHARD_COUNT);
        let mut workers = Vec::with_capacity(wave_end - wave_start);
        for shard in wave_start..wave_end {
            let executable = executable.clone();
            workers.push(std::thread::spawn(move || {
                let label = format!("shard-{shard}-of-{SHARD_COUNT}");
                let mut command = Command::new(executable);
                command
                    .env(
                        "WORTH_UI_PHASE5_MATRIX_SHARD",
                        format!("{shard}/{SHARD_COUNT}"),
                    )
                    .env(DEADLINE_ENV, deadline.to_string())
                    .env_remove("WORTH_UI_PHASE5_MATRIX_CASE");
                let output = process_execution::run_until(&mut command, deadline, &label)?;
                if !output.status().success() {
                    return Err(format!(
                        "matrix {label} exited {:?}",
                        output.status().code()
                    ));
                }
                parse_shard_rows(output.stdout(), &label)
            }));
        }
        for worker in workers {
            rows.extend(
                worker
                    .join()
                    .map_err(|_| "matrix shard worker panicked".to_owned())??,
            );
        }
    }
    order_and_validate(&mut rows)?;
    Ok(rows)
}

fn parse_shard_rows(bytes: &[u8], label: &str) -> Result<Vec<serde_json::Value>, String> {
    let stdout = std::str::from_utf8(bytes)
        .map_err(|denial| format!("matrix {label} output encoding: {denial}"))?;
    let payloads = stdout
        .lines()
        .filter_map(|line| line.strip_prefix(EVIDENCE_PREFIX))
        .collect::<Vec<_>>();
    let [payload] = payloads.as_slice() else {
        return Err(format!(
            "matrix {label} emitted {} evidence payloads",
            payloads.len()
        ));
    };
    serde_json::from_str(payload)
        .map_err(|denial| format!("matrix {label} evidence encoding: {denial}"))
}

pub(super) fn order_and_validate(rows: &mut [serde_json::Value]) -> Result<(), String> {
    let expected = RETAINED_SIZES.len() * Phase5LocalityAxis::ALL.len();
    if rows.len() != expected {
        return Err(format!(
            "matrix shards emitted {} of {expected} rows",
            rows.len()
        ));
    }
    let mut ordered = rows
        .iter()
        .map(|row| row_ordinal(row).map(|ordinal| (ordinal, row.clone())))
        .collect::<Result<Vec<_>, _>>()?;
    ordered.sort_by_key(|(ordinal, _)| *ordinal);
    for (expected_ordinal, (actual_ordinal, _)) in ordered.iter().enumerate() {
        if *actual_ordinal != expected_ordinal {
            return Err(format!(
                "matrix shards duplicate or omit ordinal {expected_ordinal}"
            ));
        }
    }
    for (destination, (_, source)) in rows.iter_mut().zip(ordered) {
        *destination = source;
    }
    require_hostile_cost_convictions(rows)
}

fn require_hostile_cost_convictions(rows: &[serde_json::Value]) -> Result<(), String> {
    for row in rows {
        let axis = row["axis"]
            .as_str()
            .and_then(|label| {
                Phase5LocalityAxis::ALL
                    .into_iter()
                    .find(|axis| axis.label() == label)
            })
            .ok_or_else(|| "matrix hostile row has unknown axis".to_owned())?;
        let convictions = row["hostile_cost_convictions"]
            .as_array()
            .ok_or_else(|| "matrix hostile convictions are not an array".to_owned())?;
        let observed = convictions
            .iter()
            .map(|conviction| {
                let mutant = conviction["mutant"]
                    .as_str()
                    .ok_or_else(|| "matrix hostile conviction omits mutant".to_owned())?;
                let performed = conviction["performed_work"]
                    .as_u64()
                    .ok_or_else(|| format!("{mutant} omits performed work"))?;
                let wrong = conviction["mutant_work"]
                    .as_u64()
                    .ok_or_else(|| format!("{mutant} omits mutant work"))?;
                let performed_trace = conviction["performed_trace_digest"]
                    .as_str()
                    .filter(|digest| digest.len() == 64)
                    .ok_or_else(|| format!("{mutant} omits its performed trace digest"))?;
                let mutant_trace = conviction["mutant_trace_digest"]
                    .as_str()
                    .filter(|digest| digest.len() == 64)
                    .ok_or_else(|| format!("{mutant} omits its mutant trace digest"))?;
                if conviction["denial"].as_str().is_none() || performed == wrong {
                    return Err(format!("{mutant} conviction has no causal disagreement"));
                }
                if performed_trace == mutant_trace {
                    return Err(format!("{mutant} conviction retained equal traces"));
                }
                Ok(mutant)
            })
            .collect::<Result<Vec<_>, String>>()?;
        let retained = row["retained"]
            .as_u64()
            .and_then(|value| usize::try_from(value).ok())
            .ok_or_else(|| "matrix hostile row omits retained size".to_owned())?;
        let case = super::case::Phase5LocalityCase::new(retained, axis);
        if observed != super::hostile_cost_model::expected_for(case) {
            return Err(format!(
                "matrix hostile placement mismatch for {}: expected {:?}, observed {observed:?}",
                axis.label(),
                super::hostile_cost_model::expected_for(case)
            ));
        }
    }
    Ok(())
}

pub(super) fn row_ordinal(row: &serde_json::Value) -> Result<usize, String> {
    let retained = row["retained"]
        .as_u64()
        .ok_or_else(|| "matrix row omits retained size".to_owned())? as usize;
    let axis = row["axis"]
        .as_str()
        .ok_or_else(|| "matrix row omits axis".to_owned())?;
    let retained_ordinal = RETAINED_SIZES
        .iter()
        .position(|candidate| *candidate == retained)
        .ok_or_else(|| format!("matrix row has unqualified retained size {retained}"))?;
    let axis_ordinal = Phase5LocalityAxis::ALL
        .iter()
        .position(|candidate| candidate.label() == axis)
        .ok_or_else(|| format!("matrix row has unknown axis {axis}"))?;
    Ok(retained_ordinal * Phase5LocalityAxis::ALL.len() + axis_ordinal)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::order_and_validate;

    #[test]
    fn shard_aggregation_orders_every_exact_case_and_rejects_duplicates() {
        let axes = [
            "content",
            "width",
            "paint-value",
            "paint-boundary",
            "dpi",
            "atlas-miss",
            "upload-completion",
            "pin-release",
        ];
        let mut rows = [1_u64, 32, 2_048, 4_096]
            .into_iter()
            .rev()
            .flat_map(|retained| {
                axes.into_iter().rev().map(move |axis| {
                    serde_json::json!({
                        "retained": retained,
                        "axis": axis,
                        "hostile_cost_convictions": super::super::hostile_cost_model::expected_for(
                            super::super::case::Phase5LocalityCase::new(
                                retained as usize,
                                super::super::case::Phase5LocalityAxis::ALL
                                    .into_iter()
                                    .find(|candidate| candidate.label() == axis)
                                    .unwrap(),
                            )
                        ).into_iter().enumerate().map(|(ordinal, mutant)| serde_json::json!({
                            "mutant": mutant,
                            "performed_work": ordinal as u64,
                            "mutant_work": ordinal as u64 + 1,
                            "performed_trace_digest": "11".repeat(32),
                            "mutant_trace_digest": "22".repeat(32),
                            "denial": "fixture-causal-disagreement",
                        })).collect::<Vec<_>>(),
                    })
                })
            })
            .collect::<Vec<_>>();
        order_and_validate(&mut rows).unwrap();
        assert_eq!(rows[0]["retained"], json!(1));
        assert_eq!(rows[0]["axis"], json!("content"));
        rows[1] = rows[0].clone();
        assert!(order_and_validate(&mut rows).is_err());
    }
}
