use super::case::{Phase5LocalityAxis, Phase5LocalityCase};
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
    require_hostile_cost_convictions(rows)
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
        let case = Phase5LocalityCase::new(retained, axis);
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
