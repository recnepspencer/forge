use forge_signal::facade::{Aspect, AspectVersion};

use crate::boundary::errors::ForgeSignalJsError;
use crate::recipe::model::{AspectSelectionSpec, WasmAspectId};
use crate::runtime::summaries::AspectVersionSummary;

use super::DEFAULT_ASPECT;

pub(super) fn defaulted_produced_aspects(explicit: Option<&[WasmAspectId]>) -> Vec<Aspect> {
    match explicit {
        Some(aspects) if !aspects.is_empty() => {
            normalize_aspects(aspects).unwrap_or_else(|_| vec![DEFAULT_ASPECT])
        }
        _ => vec![DEFAULT_ASPECT],
    }
}

pub(super) fn normalize_aspects(raw: &[WasmAspectId]) -> Result<Vec<Aspect>, ForgeSignalJsError> {
    let mut aspects = raw
        .iter()
        .copied()
        .map(|aspect| {
            Aspect::try_new(aspect).ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!(
                    "aspect `{aspect}` is out of range for forge-signal"
                ))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    aspects.sort_by_key(|aspect| aspect.id());
    aspects.dedup_by_key(|aspect| aspect.id());
    if aspects.is_empty() {
        aspects.push(DEFAULT_ASPECT);
    }
    Ok(aspects)
}

pub(super) fn resolve_selected_aspects(
    selection: Option<&AspectSelectionSpec>,
) -> Result<Vec<Aspect>, ForgeSignalJsError> {
    let Some(selection) = selection else {
        return Ok(vec![DEFAULT_ASPECT]);
    };
    let mut raw = Vec::new();
    if let Some(aspect) = selection.aspect {
        raw.push(aspect);
    }
    if let Some(aspects) = &selection.aspects {
        raw.extend(aspects.iter().copied());
    }
    if raw.is_empty() {
        return Ok(vec![DEFAULT_ASPECT]);
    }
    normalize_aspects(&raw)
}

pub(super) fn resolve_change_aspects(
    aspect: Option<WasmAspectId>,
    aspects: Option<&Vec<WasmAspectId>>,
) -> Result<Vec<Aspect>, ForgeSignalJsError> {
    let selection = AspectSelectionSpec {
        aspect,
        aspects: aspects.cloned(),
    };
    resolve_selected_aspects(Some(&selection))
}

pub(super) fn aspect_mask_from_list(aspects: &[Aspect]) -> forge_signal::facade::AspectMask {
    let mut mask = forge_signal::facade::AspectMask::EMPTY;
    for aspect in aspects {
        mask.insert(*aspect);
    }
    mask
}

pub(super) fn bump_aspects(version: AspectVersion, aspects: &[Aspect]) -> AspectVersion {
    aspects
        .iter()
        .copied()
        .fold(version, |current, aspect| current.bump(aspect))
}

pub(super) fn initial_aspect_version(aspects: &[Aspect]) -> AspectVersion {
    let mut version = AspectVersion::zero();
    for aspect in aspects {
        version = version.with(*aspect, 1);
    }
    version
}

pub(super) fn aspect_versions_summary(
    version: AspectVersion,
    explicit: Option<&[WasmAspectId]>,
) -> Vec<AspectVersionSummary> {
    defaulted_produced_aspects(explicit)
        .into_iter()
        .map(|aspect| AspectVersionSummary {
            aspect: aspect.id(),
            version: version.get(aspect),
        })
        .collect()
}

pub(super) fn aspect_version_from_summary(
    fallback_version: u64,
    aspect_versions: &[AspectVersionSummary],
    explicit: Option<&[WasmAspectId]>,
) -> AspectVersion {
    if aspect_versions.is_empty() {
        return initial_aspect_version(&defaulted_produced_aspects(explicit))
            .with(DEFAULT_ASPECT, fallback_version);
    }

    let mut version = AspectVersion::zero();
    for entry in aspect_versions {
        if let Some(aspect) = Aspect::try_new(entry.aspect) {
            version = version.with(aspect, entry.version);
        }
    }
    version
}

pub(super) fn checked_grid_cells(width: u32, height: u32) -> Result<usize, ForgeSignalJsError> {
    (width as usize)
        .checked_mul(height as usize)
        .ok_or_else(|| {
            ForgeSignalJsError::invalid_input(format!(
                "grid dimensions overflow capacity math: {width}x{height}"
            ))
        })
}

pub(super) fn checked_packed_capacity(
    width: u32,
    height: u32,
    fields_len: usize,
) -> Result<usize, ForgeSignalJsError> {
    checked_grid_cells(width, height)?
        .checked_mul(fields_len)
        .ok_or_else(|| {
            ForgeSignalJsError::invalid_input(format!(
                "packed field capacity overflow for grid {width}x{height} with {fields_len} fields"
            ))
        })
}
