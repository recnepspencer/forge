use crate::font_collection::UiFontCollectionAdmissionDenial as Denial;

pub(super) fn sequence(bytes: &[u8], start: usize) -> Result<Vec<usize>, Denial> {
    match u16_at(bytes, start).ok_or(Denial::MalformedFont)? {
        1 => rulesets(bytes, start, 4, 6, sequence_rule),
        2 => rulesets(bytes, start, 6, 8, sequence_rule),
        3 => {
            let glyphs = usize::from(u16_at(bytes, start + 2).ok_or(Denial::MalformedFont)?);
            let records = usize::from(u16_at(bytes, start + 4).ok_or(Denial::MalformedFont)?);
            records_at(bytes, start + 6 + glyphs * 2, records)
        }
        _ => Err(Denial::MalformedFont),
    }
}

pub(super) fn chained(bytes: &[u8], start: usize) -> Result<Vec<usize>, Denial> {
    match u16_at(bytes, start).ok_or(Denial::MalformedFont)? {
        1 => rulesets(bytes, start, 4, 6, chained_rule),
        2 => rulesets(bytes, start, 10, 12, chained_rule),
        3 => chained_format_three(bytes, start),
        _ => Err(Denial::MalformedFont),
    }
}

fn rulesets(
    bytes: &[u8],
    start: usize,
    count_offset: usize,
    offsets_offset: usize,
    rule: fn(&[u8], usize) -> Result<Vec<usize>, Denial>,
) -> Result<Vec<usize>, Denial> {
    let count = usize::from(u16_at(bytes, start + count_offset).ok_or(Denial::MalformedFont)?);
    let mut paths = Vec::new();
    for index in 0..count {
        let offset = usize::from(
            u16_at(bytes, start + offsets_offset + index * 2).ok_or(Denial::MalformedFont)?,
        );
        if offset == 0 {
            continue;
        }
        let set = start + offset;
        let count = usize::from(u16_at(bytes, set).ok_or(Denial::MalformedFont)?);
        for rule_index in 0..count {
            let offset =
                usize::from(u16_at(bytes, set + 2 + rule_index * 2).ok_or(Denial::MalformedFont)?);
            paths.push(rule(bytes, set + offset)?);
        }
    }
    Ok(merge_paths(paths))
}

pub(super) fn merge_paths(paths: Vec<Vec<usize>>) -> Vec<usize> {
    let mut maximum_counts = std::collections::BTreeMap::<usize, usize>::new();
    let mut order = Vec::new();
    for path in paths {
        let mut counts = std::collections::BTreeMap::<usize, usize>::new();
        for dependency in path {
            if !order.contains(&dependency) {
                order.push(dependency);
            }
            *counts.entry(dependency).or_insert(0usize) += 1;
        }
        for (dependency, count) in counts {
            maximum_counts
                .entry(dependency)
                .and_modify(|maximum| *maximum = (*maximum).max(count))
                .or_insert(count);
        }
    }
    order
        .into_iter()
        .flat_map(|dependency| std::iter::repeat_n(dependency, maximum_counts[&dependency]))
        .collect()
}

fn sequence_rule(bytes: &[u8], start: usize) -> Result<Vec<usize>, Denial> {
    let glyphs = usize::from(u16_at(bytes, start).ok_or(Denial::MalformedFont)?);
    let records = usize::from(u16_at(bytes, start + 2).ok_or(Denial::MalformedFont)?);
    records_at(bytes, start + 4 + glyphs.saturating_sub(1) * 2, records)
}

fn chained_rule(bytes: &[u8], start: usize) -> Result<Vec<usize>, Denial> {
    let backtrack = usize::from(u16_at(bytes, start).ok_or(Denial::MalformedFont)?);
    let input_at = start + 2 + backtrack * 2;
    let input = usize::from(u16_at(bytes, input_at).ok_or(Denial::MalformedFont)?);
    let lookahead_at = input_at + 2 + input.saturating_sub(1) * 2;
    let lookahead = usize::from(u16_at(bytes, lookahead_at).ok_or(Denial::MalformedFont)?);
    let records_at_offset = lookahead_at + 2 + lookahead * 2;
    let records = usize::from(u16_at(bytes, records_at_offset).ok_or(Denial::MalformedFont)?);
    records_at(bytes, records_at_offset + 2, records)
}

fn chained_format_three(bytes: &[u8], start: usize) -> Result<Vec<usize>, Denial> {
    let backtrack = usize::from(u16_at(bytes, start + 2).ok_or(Denial::MalformedFont)?);
    let input_at = start + 4 + backtrack * 2;
    let input = usize::from(u16_at(bytes, input_at).ok_or(Denial::MalformedFont)?);
    let lookahead_at = input_at + 2 + input * 2;
    let lookahead = usize::from(u16_at(bytes, lookahead_at).ok_or(Denial::MalformedFont)?);
    let records_at_offset = lookahead_at + 2 + lookahead * 2;
    let records = usize::from(u16_at(bytes, records_at_offset).ok_or(Denial::MalformedFont)?);
    records_at(bytes, records_at_offset + 2, records)
}

fn records_at(bytes: &[u8], start: usize, count: usize) -> Result<Vec<usize>, Denial> {
    (0..count)
        .map(|index| {
            u16_at(bytes, start + index * 4 + 2)
                .map(usize::from)
                .ok_or(Denial::MalformedFont)
        })
        .collect()
}

fn u16_at(bytes: &[u8], start: usize) -> Option<u16> {
    Some(u16::from_be_bytes(
        bytes.get(start..start + 2)?.try_into().ok()?,
    ))
}
