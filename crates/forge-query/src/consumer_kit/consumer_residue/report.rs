use crate::ForgeQueryEvidenceIdentity;

use super::finding::ForgeQueryConsumerResidueFinding;
use super::inventory::ForgeQueryConsumerResidueSourceInventory;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryConsumerResidueReport {
    consumer_name: String,
    audited_roots: Vec<String>,
    findings: Vec<ForgeQueryConsumerResidueFinding>,
    finding_identities: Vec<ForgeQueryEvidenceIdentity>,
    report_identity: ForgeQueryEvidenceIdentity,
    source_inventory: ForgeQueryConsumerResidueSourceInventory,
    scanned_file_count: usize,
    parsed_item_count: usize,
    visited_node_count: usize,
}

impl ForgeQueryConsumerResidueReport {
    pub(crate) fn sealed(
        consumer_name: String,
        audited_roots: Vec<String>,
        findings: Vec<ForgeQueryConsumerResidueFinding>,
        finding_identities: Vec<ForgeQueryEvidenceIdentity>,
        report_identity: ForgeQueryEvidenceIdentity,
        source_inventory: ForgeQueryConsumerResidueSourceInventory,
        counters: ForgeQueryConsumerResidueReportCounters,
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

    pub fn findings(&self) -> &[ForgeQueryConsumerResidueFinding] {
        &self.findings
    }

    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    pub fn finding_identities(&self) -> &[ForgeQueryEvidenceIdentity] {
        &self.finding_identities
    }

    pub fn report_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.report_identity
    }

    pub fn source_inventory(&self) -> &ForgeQueryConsumerResidueSourceInventory {
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
pub(crate) struct ForgeQueryConsumerResidueReportCounters {
    pub(crate) scanned_file_count: usize,
    pub(crate) parsed_item_count: usize,
    pub(crate) visited_node_count: usize,
    pub(crate) skipped_non_rust_file_count: usize,
}
