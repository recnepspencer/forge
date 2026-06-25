use super::firewall_region::WorthGraphReadAccessHardDeletionSourceRegion;
#[cfg(test)]
use super::firewall_region_row::WorthGraphReadAccessHardDeletionSourceFirewallRegionRow;
#[cfg(test)]
use super::firewall_report::WorthGraphReadAccessHardDeletionSourceFirewallReport;
use super::firewall_violation::WorthGraphReadAccessHardDeletionSourceFirewallViolation;
use super::forbidden_pattern::HARD_DELETION_FORBIDDEN_PATTERNS;

#[cfg(test)]
pub(crate) fn scan_source(
    source_path: &str,
    source_text: &str,
) -> Result<
    WorthGraphReadAccessHardDeletionSourceFirewallReport,
    WorthGraphReadAccessHardDeletionSourceFirewallViolation,
> {
    scan_source_text(
        source_path,
        WorthGraphReadAccessHardDeletionSourceRegion::StandaloneTestInput,
        source_text,
    )?;
    Ok(WorthGraphReadAccessHardDeletionSourceFirewallReport::new(
        vec![
            WorthGraphReadAccessHardDeletionSourceFirewallRegionRow::new(
                WorthGraphReadAccessHardDeletionSourceRegion::StandaloneTestInput,
                source_path.to_string(),
                1,
            ),
        ],
        HARD_DELETION_FORBIDDEN_PATTERNS.len(),
        0,
    ))
}

#[cfg(test)]
pub(crate) fn scan_source_for_region(
    source_path: &str,
    region: WorthGraphReadAccessHardDeletionSourceRegion,
    source_text: &str,
) -> Result<
    WorthGraphReadAccessHardDeletionSourceFirewallReport,
    WorthGraphReadAccessHardDeletionSourceFirewallViolation,
> {
    scan_source_text(source_path, region, source_text)?;
    Ok(WorthGraphReadAccessHardDeletionSourceFirewallReport::new(
        vec![
            WorthGraphReadAccessHardDeletionSourceFirewallRegionRow::new(
                region,
                source_path.to_string(),
                1,
            ),
        ],
        HARD_DELETION_FORBIDDEN_PATTERNS.len(),
        0,
    ))
}

pub(crate) fn scan_source_text(
    source_path: &str,
    region: WorthGraphReadAccessHardDeletionSourceRegion,
    source_text: &str,
) -> Result<(), WorthGraphReadAccessHardDeletionSourceFirewallViolation> {
    for pattern in HARD_DELETION_FORBIDDEN_PATTERNS {
        if pattern.applies_to(region) && source_text.contains(pattern.needle()) {
            return Err(
                WorthGraphReadAccessHardDeletionSourceFirewallViolation::new(
                    source_path,
                    pattern.label(),
                ),
            );
        }
    }
    Ok(())
}
