use crate::facade::*;

use super::truth_snapshot::FintechTruthSnapshot;

#[derive(Debug, Default)]
pub(super) struct FintechTruthMismatch {
    pub fields: Vec<String>,
}

impl FintechTruthMismatch {
    pub(super) fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

pub(super) fn compare_exact(
    left: &FintechTruthSnapshot,
    right: &FintechTruthSnapshot,
) -> FintechTruthMismatch {
    let mut mismatch = FintechTruthMismatch::default();

    if left.primary_market != right.primary_market {
        mismatch.fields.push("primary_market".to_string());
    }
    if left.primary_threshold != right.primary_threshold {
        mismatch.fields.push("primary_threshold".to_string());
    }
    if left.primary_audit != right.primary_audit {
        mismatch.fields.push("primary_audit".to_string());
    }
    if left.rates_partition != right.rates_partition {
        mismatch.fields.push("rates_partition".to_string());
    }
    if left.credit_partition != right.credit_partition {
        mismatch.fields.push("credit_partition".to_string());
    }
    if left.rates_bucket_zero != right.rates_bucket_zero {
        mismatch.fields.push("rates_bucket_zero".to_string());
    }
    if left.coarse_partition_book != right.coarse_partition_book {
        mismatch.fields.push("coarse_partition_book".to_string());
    }
    if left.bucket_aggregates != right.bucket_aggregates {
        mismatch.fields.push("bucket_aggregates".to_string());
    }
    if left.scenario_aggregates != right.scenario_aggregates {
        mismatch.fields.push("scenario_aggregates".to_string());
    }
    if left.branch_heads != right.branch_heads {
        mismatch.fields.push("branch_heads".to_string());
    }

    for (alias, left_replay) in &left.replays {
        match right.replays.get(alias) {
            Some(right_replay) => {
                if !compare_replay_slices(left_replay, right_replay)
                    .mismatches
                    .is_empty()
                {
                    mismatch.fields.push(format!("replay:{alias}"));
                }
            }
            None => mismatch.fields.push(format!("replay_missing:{alias}")),
        }
    }

    for (alias, left_lineage) in &left.lineages {
        match right.lineages.get(alias) {
            Some(right_lineage) => {
                if !compare_lineage_records(left_lineage, right_lineage)
                    .mismatches
                    .is_empty()
                {
                    mismatch.fields.push(format!("lineage:{alias}"));
                }
            }
            None => mismatch.fields.push(format!("lineage_missing:{alias}")),
        }
    }

    mismatch
}
