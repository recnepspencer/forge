use super::case::Phase5LocalityAxis;
use super::shard_contract::ROW_COUNT;

pub(super) fn order_and_validate(rows: &mut [serde_json::Value]) -> Result<(), String> {
    if rows.len() != ROW_COUNT {
        return Err(format!(
            "matrix shards emitted {} of {ROW_COUNT} rows",
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
    Ok(())
}

pub(super) fn row_ordinal(row: &serde_json::Value) -> Result<usize, String> {
    let retained = row["retained"]
        .as_u64()
        .ok_or_else(|| "matrix row omits retained size".to_owned())? as usize;
    let axis = row["axis"]
        .as_str()
        .ok_or_else(|| "matrix row omits axis".to_owned())?;
    let retained_ordinal = super::RETAINED_SIZES
        .iter()
        .position(|candidate| *candidate == retained)
        .ok_or_else(|| format!("matrix row has unqualified retained size {retained}"))?;
    let axis_ordinal = Phase5LocalityAxis::ALL
        .iter()
        .position(|candidate| candidate.label() == axis)
        .ok_or_else(|| format!("matrix row has unknown axis {axis}"))?;
    Ok(retained_ordinal * Phase5LocalityAxis::ALL.len() + axis_ordinal)
}
