use worth_store_physical_integrity::IntegrityRepairRegion;

use super::resolved_region::ResolvedRepairRegion;

pub(super) fn integrity_regions(
    regions: &[ResolvedRepairRegion],
) -> Result<Vec<IntegrityRepairRegion>, ()> {
    let mut projected = Vec::new();
    projected.try_reserve_exact(regions.len()).map_err(|_| ())?;
    projected.extend(regions.iter().map(ResolvedRepairRegion::integrity));
    Ok(projected)
}

pub(super) fn into_integrity_regions(
    regions: Vec<ResolvedRepairRegion>,
) -> Result<Vec<IntegrityRepairRegion>, ()> {
    let mut projected = Vec::new();
    projected.try_reserve_exact(regions.len()).map_err(|_| ())?;
    projected.extend(regions.into_iter().map(|region| region.integrity()));
    Ok(projected)
}
