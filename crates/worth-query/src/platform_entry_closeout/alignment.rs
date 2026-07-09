use crate::orchestration_inventory::WorthQueryOrchestrationSurfaceInventory;
use crate::public_doc_coverage::{
    WorthQueryPublicDocCoverageAudit, WorthQueryPublicDocCoverageInventory,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPlatformEntryAlignmentAudit {
    name: &'static str,
    digest: String,
    gaps: Vec<String>,
}

impl WorthQueryPlatformEntryAlignmentAudit {
    pub(crate) fn new(name: &'static str, digest: String, mut gaps: Vec<String>) -> Self {
        gaps.sort();
        Self { name, digest, gaps }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn gaps(&self) -> &[String] {
        &self.gaps
    }

    pub fn is_aligned(&self) -> bool {
        self.gaps.is_empty()
    }
}

pub(crate) fn inventory_alignment_audit() -> WorthQueryPlatformEntryAlignmentAudit {
    let inventory = WorthQueryOrchestrationSurfaceInventory::current();
    let docs = WorthQueryPublicDocCoverageInventory::current();
    let mut gaps = Vec::new();

    if docs.source_inventory_digest() != inventory.inventory_digest() {
        gaps.push("public_doc_coverage source inventory digest drifted".to_string());
    }

    for row in inventory.rows() {
        if docs.row_for_public_name(row.public_name()).is_none() {
            gaps.push(format!(
                "missing docs coverage row for {}",
                row.public_name()
            ));
        }
    }

    WorthQueryPlatformEntryAlignmentAudit::new(
        "inventory_alignment",
        inventory.inventory_digest().to_string(),
        gaps,
    )
}

pub(crate) fn docs_coverage_alignment_audit() -> WorthQueryPlatformEntryAlignmentAudit {
    docs_coverage_alignment_audit_from_audit(&WorthQueryPublicDocCoverageAudit::current())
}

pub(crate) fn docs_coverage_alignment_audit_from_audit(
    audit: &WorthQueryPublicDocCoverageAudit,
) -> WorthQueryPlatformEntryAlignmentAudit {
    let mut gaps = Vec::new();
    gaps.extend(
        audit
            .undocumented_public_surfaces()
            .iter()
            .map(|value| format!("undocumented:{value}")),
    );
    gaps.extend(
        audit
            .surfaces_missing_goldens()
            .iter()
            .map(|value| format!("missing_golden:{value}")),
    );
    gaps.extend(
        audit
            .orphan_doc_rows()
            .iter()
            .map(|value| format!("orphan_doc:{value}")),
    );
    gaps.extend(
        audit
            .orphan_golden_rows()
            .iter()
            .map(|value| format!("orphan_golden:{value}")),
    );
    gaps.extend(
        audit
            .readme_discovery_gaps()
            .iter()
            .map(|value| format!("readme_gap:{value}")),
    );
    gaps.extend(
        audit
            .journey_coverage_gaps()
            .iter()
            .map(|value| format!("journey_gap:{value}")),
    );

    WorthQueryPlatformEntryAlignmentAudit::new(
        "docs_coverage_alignment",
        audit.coverage_digest().to_string(),
        gaps,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_alignments_are_green() {
        assert!(inventory_alignment_audit().is_aligned());
        assert!(docs_coverage_alignment_audit().is_aligned());
    }
}
