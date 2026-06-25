use super::super::stable_digest;
use super::firewall_region::WorthGraphReadAccessHardDeletionSourceRegion;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessHardDeletionSourceFirewallRegionRow {
    region: String,
    root_identity: String,
    scanned_source_count: usize,
    row_digest: String,
}

impl WorthGraphReadAccessHardDeletionSourceFirewallRegionRow {
    pub(crate) fn new(
        region: WorthGraphReadAccessHardDeletionSourceRegion,
        root_identity: String,
        scanned_source_count: usize,
    ) -> Self {
        let region = region.as_str().to_string();
        let row_digest = stable_digest(&[
            "worth_graph_read_access_hard_deletion_source_firewall_region_row_v1".to_string(),
            format!("region:{region}"),
            format!("root:{root_identity}"),
            format!("scanned:{scanned_source_count}"),
        ]);
        Self {
            region,
            root_identity,
            scanned_source_count,
            row_digest,
        }
    }

    pub fn region(&self) -> &str {
        &self.region
    }

    pub fn root_identity(&self) -> &str {
        &self.root_identity
    }

    pub const fn scanned_source_count(&self) -> usize {
        self.scanned_source_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
