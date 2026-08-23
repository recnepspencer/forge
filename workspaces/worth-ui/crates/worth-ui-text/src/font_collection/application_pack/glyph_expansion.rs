use std::collections::BTreeMap;

use harfrust::{FontRef, Tag};

use crate::font_collection::UiFontCollectionAdmissionDenial as Denial;

#[path = "glyph_expansion/context.rs"]
mod context;

#[derive(Default)]
struct LookupPlan {
    replacements: BTreeMap<u16, Box<[u16]>>,
    dependencies: Vec<usize>,
}

pub(super) fn derive(font: &FontRef<'_>) -> Result<usize, Denial> {
    let Some(data) = font.table_data(Tag::from_be_bytes(*b"GSUB")) else {
        return Ok(1);
    };
    derive_from_gsub(data.as_bytes())
}

fn derive_from_gsub(bytes: &[u8]) -> Result<usize, Denial> {
    let script_list = usize::from(u16_at(bytes, 4).ok_or(Denial::MalformedFont)?);
    let feature_list = usize::from(u16_at(bytes, 6).ok_or(Denial::MalformedFont)?);
    let lookup_list = usize::from(u16_at(bytes, 8).ok_or(Denial::MalformedFont)?);
    let plans = lookup_plans(bytes, lookup_list)?;
    let features = feature_lookups(bytes, feature_list)?;
    let mut maximum = 1usize;
    for feature_set in language_feature_sets(bytes, script_list)? {
        let mut roots = feature_set
            .into_iter()
            .map(|index| features.get(index).ok_or(Denial::MalformedFont))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>();
        roots.sort_unstable();
        roots.dedup();
        maximum = maximum.max(simulate_expansion(&plans, &roots)?);
    }
    Ok(maximum)
}

fn lookup_plans(bytes: &[u8], start: usize) -> Result<Vec<LookupPlan>, Denial> {
    let count = usize::from(u16_at(bytes, start).ok_or(Denial::MalformedFont)?);
    (0..count)
        .map(|index| {
            let offset =
                usize::from(u16_at(bytes, start + 2 + index * 2).ok_or(Denial::MalformedFont)?);
            parse_lookup(bytes, start + offset)
        })
        .collect()
}

fn parse_lookup(bytes: &[u8], start: usize) -> Result<LookupPlan, Denial> {
    let lookup_type = u16_at(bytes, start).ok_or(Denial::MalformedFont)?;
    let count = usize::from(u16_at(bytes, start + 4).ok_or(Denial::MalformedFont)?);
    let mut plan = LookupPlan::default();
    let mut dependency_paths = Vec::new();
    for index in 0..count {
        let offset =
            usize::from(u16_at(bytes, start + 6 + index * 2).ok_or(Denial::MalformedFont)?);
        let subtable = start.checked_add(offset).ok_or(Denial::MalformedFont)?;
        let (kind, table) = extension_target(bytes, lookup_type, subtable)?;
        match kind {
            2 => plan
                .replacements
                .extend(multiple_replacements(bytes, table)?),
            5 => dependency_paths.push(context::sequence(bytes, table)?),
            6 => dependency_paths.push(context::chained(bytes, table)?),
            1 | 3 | 4 | 8 => {}
            _ => return Err(Denial::MalformedFont),
        }
    }
    plan.dependencies = context::merge_paths(dependency_paths);
    Ok(plan)
}

fn extension_target(bytes: &[u8], kind: u16, start: usize) -> Result<(u16, usize), Denial> {
    if kind != 7 {
        return Ok((kind, start));
    }
    if u16_at(bytes, start) != Some(1) {
        return Err(Denial::MalformedFont);
    }
    let target = u16_at(bytes, start + 2).ok_or(Denial::MalformedFont)?;
    let offset = usize::try_from(u32_at(bytes, start + 4).ok_or(Denial::MalformedFont)?)
        .map_err(|_| Denial::MalformedFont)?;
    Ok((
        target,
        start.checked_add(offset).ok_or(Denial::MalformedFont)?,
    ))
}

fn multiple_replacements(bytes: &[u8], start: usize) -> Result<BTreeMap<u16, Box<[u16]>>, Denial> {
    if u16_at(bytes, start) != Some(1) {
        return Err(Denial::MalformedFont);
    }
    let coverage_offset = usize::from(u16_at(bytes, start + 2).ok_or(Denial::MalformedFont)?);
    let glyphs = coverage_glyphs(bytes, start + coverage_offset)?;
    let count = usize::from(u16_at(bytes, start + 4).ok_or(Denial::MalformedFont)?);
    if glyphs.len() != count {
        return Err(Denial::MalformedFont);
    }
    glyphs
        .into_iter()
        .enumerate()
        .map(|(index, glyph)| {
            let offset =
                usize::from(u16_at(bytes, start + 6 + index * 2).ok_or(Denial::MalformedFont)?);
            let sequence = start + offset;
            let length = usize::from(u16_at(bytes, sequence).ok_or(Denial::MalformedFont)?);
            let output = (0..length)
                .map(|position| {
                    u16_at(bytes, sequence + 2 + position * 2).ok_or(Denial::MalformedFont)
                })
                .collect::<Result<Box<[_]>, _>>()?;
            Ok((glyph, output))
        })
        .collect()
}

fn coverage_glyphs(bytes: &[u8], start: usize) -> Result<Vec<u16>, Denial> {
    match u16_at(bytes, start).ok_or(Denial::MalformedFont)? {
        1 => {
            let count = usize::from(u16_at(bytes, start + 2).ok_or(Denial::MalformedFont)?);
            (0..count)
                .map(|index| u16_at(bytes, start + 4 + index * 2).ok_or(Denial::MalformedFont))
                .collect()
        }
        2 => coverage_ranges(bytes, start),
        _ => Err(Denial::MalformedFont),
    }
}

fn coverage_ranges(bytes: &[u8], start: usize) -> Result<Vec<u16>, Denial> {
    let count = usize::from(u16_at(bytes, start + 2).ok_or(Denial::MalformedFont)?);
    let mut indexed = Vec::new();
    for index in 0..count {
        let record = start + 4 + index * 6;
        let first = u16_at(bytes, record).ok_or(Denial::MalformedFont)?;
        let last = u16_at(bytes, record + 2).ok_or(Denial::MalformedFont)?;
        let coverage = usize::from(u16_at(bytes, record + 4).ok_or(Denial::MalformedFont)?);
        if first > last {
            return Err(Denial::MalformedFont);
        }
        indexed.extend(
            (first..=last)
                .enumerate()
                .map(|(offset, glyph)| (coverage + offset, glyph)),
        );
    }
    indexed.sort_unstable();
    if indexed
        .iter()
        .enumerate()
        .any(|(expected, (observed, _))| expected != *observed)
    {
        return Err(Denial::MalformedFont);
    }
    Ok(indexed.into_iter().map(|(_, glyph)| glyph).collect())
}

fn simulate_expansion(plans: &[LookupPlan], roots: &[usize]) -> Result<usize, Denial> {
    let mut operations = Vec::new();
    let mut active = vec![false; plans.len()];
    for root in roots {
        append_operations(*root, plans, &mut active, &mut operations)?;
    }
    let inputs = plans
        .iter()
        .flat_map(|plan| plan.replacements.keys().copied())
        .collect::<std::collections::BTreeSet<_>>();
    let mut maximum = 1usize;
    for input in inputs {
        let mut glyphs = vec![input];
        for operation in &operations {
            let replacements = &plans[*operation].replacements;
            glyphs = glyphs
                .into_iter()
                .flat_map(|glyph| {
                    replacements
                        .get(&glyph)
                        .map_or_else(|| vec![glyph], |v| v.to_vec())
                })
                .collect();
            if glyphs.len() > crate::UiGlobalTextProfile::MAX_GLYPHS {
                return Err(Denial::GlyphExpansionCapacityExceeded);
            }
        }
        maximum = maximum.max(glyphs.len());
    }
    Ok(maximum)
}

fn append_operations(
    index: usize,
    plans: &[LookupPlan],
    active: &mut [bool],
    output: &mut Vec<usize>,
) -> Result<(), Denial> {
    let plan = plans.get(index).ok_or(Denial::MalformedFont)?;
    if active[index] {
        return Err(Denial::UnboundedGlyphExpansion);
    }
    active[index] = true;
    if !plan.replacements.is_empty() {
        output.push(index);
    }
    for dependency in &plan.dependencies {
        append_operations(*dependency, plans, active, output)?;
    }
    active[index] = false;
    Ok(())
}

fn feature_lookups(bytes: &[u8], start: usize) -> Result<Vec<Vec<usize>>, Denial> {
    let count = usize::from(u16_at(bytes, start).ok_or(Denial::MalformedFont)?);
    (0..count)
        .map(|index| {
            let offset =
                usize::from(u16_at(bytes, start + 2 + index * 6 + 4).ok_or(Denial::MalformedFont)?);
            let feature = start + offset;
            let count = usize::from(u16_at(bytes, feature + 2).ok_or(Denial::MalformedFont)?);
            (0..count)
                .map(|lookup| {
                    u16_at(bytes, feature + 4 + lookup * 2)
                        .map(usize::from)
                        .ok_or(Denial::MalformedFont)
                })
                .collect()
        })
        .collect()
}

fn language_feature_sets(bytes: &[u8], start: usize) -> Result<Vec<Vec<usize>>, Denial> {
    let count = usize::from(u16_at(bytes, start).ok_or(Denial::MalformedFont)?);
    let mut sets = Vec::new();
    for index in 0..count {
        let offset =
            usize::from(u16_at(bytes, start + 2 + index * 6 + 4).ok_or(Denial::MalformedFont)?);
        let script = start + offset;
        let default = usize::from(u16_at(bytes, script).ok_or(Denial::MalformedFont)?);
        if default != 0 {
            sets.push(language_features(bytes, script + default)?);
        }
        let count = usize::from(u16_at(bytes, script + 2).ok_or(Denial::MalformedFont)?);
        for language in 0..count {
            let offset = usize::from(
                u16_at(bytes, script + 4 + language * 6 + 4).ok_or(Denial::MalformedFont)?,
            );
            sets.push(language_features(bytes, script + offset)?);
        }
    }
    (!sets.is_empty())
        .then_some(sets)
        .ok_or(Denial::MalformedFont)
}

fn language_features(bytes: &[u8], start: usize) -> Result<Vec<usize>, Denial> {
    let required = u16_at(bytes, start + 2).ok_or(Denial::MalformedFont)?;
    let count = usize::from(u16_at(bytes, start + 4).ok_or(Denial::MalformedFont)?);
    let mut features = Vec::with_capacity(count + usize::from(required != u16::MAX));
    if required != u16::MAX {
        features.push(usize::from(required));
    }
    for index in 0..count {
        features.push(usize::from(
            u16_at(bytes, start + 6 + index * 2).ok_or(Denial::MalformedFont)?,
        ));
    }
    features.sort_unstable();
    features.dedup();
    Ok(features)
}

fn u16_at(bytes: &[u8], start: usize) -> Option<u16> {
    Some(u16::from_be_bytes(
        bytes.get(start..start + 2)?.try_into().ok()?,
    ))
}

fn u32_at(bytes: &[u8], start: usize) -> Option<u32> {
    Some(u32::from_be_bytes(
        bytes.get(start..start + 4)?.try_into().ok()?,
    ))
}

#[cfg(test)]
#[path = "glyph_expansion_tests.rs"]
mod tests;
