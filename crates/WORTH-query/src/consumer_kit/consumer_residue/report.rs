use crate::WorthQueryEvidenceIdentity;

use super::finding::WorthQueryConsumerResidueFinding;
use super::inventory::WorthQueryConsumerResidueSourceInventory;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerResidueReport {
    consumer_name: String,
    audited_roots: Vec<String>,
    findings: Vec<WorthQueryConsumerResidueFinding>,
    finding_identities: Vec<WorthQueryEvidenceIdentity>,
    report_identity: WorthQueryEvidenceIdentity,
    source_inventory: WorthQueryConsumerResidueSourceInventory,
    scanned_file_count: usize,
    parsed_item_count: usize,
    visited_node_count: usize,
}

impl WorthQueryConsumerResidueReport {
    pub(crate) fn sealed(
        consumer_name: String,
        audited_roots: Vec<String>,
        findings: Vec<WorthQueryConsumerResidueFinding>,
        finding_identities: Vec<WorthQueryEvidenceIdentity>,
        report_identity: WorthQueryEvidenceIdentity,
        source_inventory: WorthQueryConsumerResidueSourceInventory,
        counters: WorthQueryConsumerResidueReportCounters,
    ) -> Self {
        Self {
            consumer_name,
            audited_roots,
            findings,
            finding_identities,
            report_identity,
            source_inventory,
            scanned_file_count: counters.scanned_file_count,
            parsed_item_count: counters.parsed_item_count,
            visited_node_count: counters.visited_node_count,
        }
    }

    pub fn consumer_name(&self) -> &str {
        &self.consumer_name
    }

    pub fn audited_roots(&self) -> &[String] {
        &self.audited_roots
    }

    pub fn findings(&self) -> &[WorthQueryConsumerResidueFinding] {
        &self.findings
    }

    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    pub fn finding_identities(&self) -> &[WorthQueryEvidenceIdentity] {
        &self.finding_identities
    }

    pub fn report_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.report_identity
    }

    pub fn source_inventory(&self) -> &WorthQueryConsumerResidueSourceInventory {
        &self.source_inventory
    }

    pub fn audited_source_paths(&self) -> &[String] {
        self.source_inventory.audited_source_paths()
    }

    pub fn source_inventory_digest(&self) -> &str {
        self.source_inventory.inventory_digest()
    }

    pub fn skipped_non_rust_file_count(&self) -> usize {
        self.source_inventory.skipped_non_rust_file_count()
    }

    pub fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub fn parsed_item_count(&self) -> usize {
        self.parsed_item_count
    }

    pub fn visited_node_count(&self) -> usize {
        self.visited_node_count
    }

    pub fn assert_clean(&self) {
        assert!(
            self.findings.is_empty(),
            "Query consumer residue audit found forbidden residue: {:?}",
            self.findings
        );
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthQueryConsumerResidueReportCounters {
    pub(crate) scanned_file_count: usize,
    pub(crate) parsed_item_count: usize,
    pub(crate) visited_node_count: usize,
    pub(crate) skipped_non_rust_file_count: usize,
}
