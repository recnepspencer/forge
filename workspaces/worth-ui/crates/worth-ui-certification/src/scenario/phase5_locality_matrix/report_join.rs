//! Non-reexecuting exact join for independently executed Phase 5 shard reports.

use std::path::Path;

use super::{joined_rows, shard_contract};

pub(super) fn join(directory: &Path) -> Result<Vec<serde_json::Value>, String> {
    let actual_reports = std::fs::read_dir(directory)
        .map_err(|denial| format!("matrix join directory: {denial}"))?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .path()
                .extension()
                .is_some_and(|value| value == "jsonl")
        })
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect::<std::collections::BTreeSet<_>>();
    let expected_reports = (0..shard_contract::SHARD_COUNT)
        .map(shard_contract::report_name)
        .collect::<std::collections::BTreeSet<_>>();
    if actual_reports != expected_reports {
        return Err(format!(
            "matrix join report set mismatch: expected {expected_reports:?}, observed {actual_reports:?}"
        ));
    }

    let mut rows = Vec::with_capacity(shard_contract::ROW_COUNT);
    for shard in 0..shard_contract::SHARD_COUNT {
        let report = directory.join(shard_contract::report_name(shard));
        let contents = std::fs::read_to_string(&report)
            .map_err(|denial| format!("matrix join report {}: {denial}", report.display()))?;
        let payloads = contents
            .lines()
            .filter_map(|line| line.strip_prefix("WORTH_UI_PHASE5_PRODUCTION_LOCALITY="))
            .collect::<Vec<_>>();
        let [payload] = payloads.as_slice() else {
            return Err(format!(
                "matrix join shard {shard} emitted {} evidence payloads",
                payloads.len()
            ));
        };
        let shard_rows: Vec<serde_json::Value> = serde_json::from_str(payload)
            .map_err(|denial| format!("matrix join shard {shard} evidence: {denial}"))?;
        if shard_rows.len() != shard_contract::expected_rows(shard) {
            return Err(format!(
                "matrix join shard {shard} emitted {} rows instead of {}",
                shard_rows.len(),
                shard_contract::expected_rows(shard)
            ));
        }
        for row in &shard_rows {
            let ordinal = joined_rows::row_ordinal(row)?;
            if ordinal % shard_contract::SHARD_COUNT != shard {
                return Err(format!(
                    "matrix join shard {shard} carried foreign ordinal {ordinal}"
                ));
            }
        }
        rows.extend(shard_rows);
    }
    joined_rows::order_and_validate(&mut rows)?;
    Ok(rows)
}
