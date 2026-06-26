use super::source_roots::SourceFirewallRegion;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationSourceFirewallRegionReport {
    region: SourceFirewallRegion,
    scanned_source_count: usize,
    audited_pattern_count: usize,
    violation_count: usize,
}

impl WorthGraphReadDeclarationSourceFirewallRegionReport {
    pub(crate) const fn new(
        region: SourceFirewallRegion,
        scanned_source_count: usize,
        audited_pattern_count: usize,
        violation_count: usize,
    ) -> Self {
        Self {
            region,
            scanned_source_count,
            audited_pattern_count,
            violation_count,
        }
    }

    pub const fn region(&self) -> SourceFirewallRegion {
        self.region
    }

    pub const fn scanned_source_count(&self) -> usize {
        self.scanned_source_count
    }

    pub const fn audited_pattern_count(&self) -> usize {
        self.audited_pattern_count
    }

    pub const fn violation_count(&self) -> usize {
        self.violation_count
    }

    pub fn digest_part(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.region.digest_part(),
            self.scanned_source_count,
            self.audited_pattern_count,
            self.violation_count
        )
    }
}
