use crate::{
    WorthQueryBoundaryAuditError, WorthQueryBoundaryAuditErrorKind, WorthQueryBoundaryAuditSource,
    WorthQueryBoundaryAuditSourceInventory, WorthQueryBoundaryAuditSourceSet,
    WorthQueryEvidenceIdentity,
};

use super::detection::{detect_graph_read_bypass_candidates, mask_comments_and_string_literals};
use super::evidence::{
    derive_graph_read_bypass_candidate_identity, derive_graph_read_bypass_report_identity,
};
use super::finding::WorthQueryGraphReadBypassFinding;
use super::report::{WorthQueryGraphReadBypassCounters, WorthQueryGraphReadBypassReport};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassAudit {
    consumer_name: String,
    source_sets: Vec<WorthQueryBoundaryAuditSourceSet>,
    source_inventory_identities: Vec<WorthQueryEvidenceIdentity>,
}

pub fn graph_read_bypass_audit(consumer_name: impl Into<String>) -> WorthQueryGraphReadBypassAudit {
    WorthQueryGraphReadBypassAudit {
        consumer_name: consumer_name.into(),
        source_sets: Vec::new(),
        source_inventory_identities: Vec::new(),
    }
}

impl WorthQueryGraphReadBypassAudit {
    pub fn required_sources(mut self, sources: WorthQueryBoundaryAuditSourceSet) -> Self {
        self.source_sets.push(sources);
        self
    }

    pub fn required_inventory(
        mut self,
        inventory: &WorthQueryBoundaryAuditSourceInventory,
    ) -> Self {
        self.source_inventory_identities
            .push(inventory.inventory_identity().clone());
        self.source_sets.push(inventory.boundary_sources());
        self
    }

    pub fn evaluate(self) -> Result<WorthQueryGraphReadBypassReport, WorthQueryBoundaryAuditError> {
        validate_request(&self)?;
        let mut audited_source_labels = Vec::new();
        let mut findings = Vec::new();
        let mut evaluated_source_count = 0;
        let mut skipped_empty_source_count = 0;
        for source_set in &self.source_sets {
            validate_source_set_for_graph_read_bypass(source_set)?;
            for source in source_set.sources() {
                if source.source().trim().is_empty() {
                    skipped_empty_source_count += 1;
                    continue;
                }
                evaluated_source_count += 1;
                audited_source_labels.push(source.label().to_string());
                let masked = mask_comments_and_string_literals(source.source());
                for candidate in
                    detect_graph_read_bypass_candidates(source.label(), source.path(), &masked)
                {
                    let identity = derive_graph_read_bypass_candidate_identity(
                        candidate.row,
                        &candidate.source_site,
                    );
                    let finding = WorthQueryGraphReadBypassFinding::sealed(
                        candidate.row.class(),
                        candidate.row.authority_violation(),
                        candidate.row.detection(),
                        candidate.row.detection_key(),
                        candidate.row.replacement_lane(),
                        candidate.source_site,
                        identity,
                    );
                    findings.push(finding);
                }
            }
        }
        sort_findings(&mut findings);
        let finding_identities = findings
            .iter()
            .map(|finding| finding.finding_identity().clone())
            .collect::<Vec<_>>();
        let counters = WorthQueryGraphReadBypassCounters::new(
            evaluated_source_count,
            findings.len(),
            skipped_empty_source_count,
        );
        let report_identity = derive_graph_read_bypass_report_identity(
            &self.consumer_name,
            &audited_source_labels,
            &self.source_inventory_identities,
            &counters,
            &finding_identities,
        );
        Ok(WorthQueryGraphReadBypassReport::sealed(
            self.consumer_name,
            audited_source_labels,
            self.source_inventory_identities,
            findings,
            finding_identities,
            report_identity,
            counters,
        ))
    }
}

fn validate_request(
    audit: &WorthQueryGraphReadBypassAudit,
) -> Result<(), WorthQueryBoundaryAuditError> {
    if audit.consumer_name.trim().is_empty() {
        return Err(WorthQueryBoundaryAuditError::new(
            WorthQueryBoundaryAuditErrorKind::EmptyCrateName,
            "graph-read bypass audit consumer name must not be empty",
        ));
    }
    if audit.source_sets.is_empty() {
        return Err(WorthQueryBoundaryAuditError::new(
            WorthQueryBoundaryAuditErrorKind::MissingRequiredRoot,
            "graph-read bypass audit requires at least one source set",
        ));
    }
    Ok(())
}

fn validate_source_set_for_graph_read_bypass(
    source_set: &WorthQueryBoundaryAuditSourceSet,
) -> Result<(), WorthQueryBoundaryAuditError> {
    if source_set.crate_name().trim().is_empty() {
        return Err(WorthQueryBoundaryAuditError::new(
            WorthQueryBoundaryAuditErrorKind::EmptyCrateName,
            "graph-read bypass audit source set crate name must not be empty",
        ));
    }
    let mut labels = std::collections::BTreeSet::new();
    for source in source_set.sources() {
        validate_source_reference(source)?;
        if !labels.insert(source.label()) {
            return Err(WorthQueryBoundaryAuditError::for_source(
                WorthQueryBoundaryAuditErrorKind::DuplicateSourceLabel,
                source.label(),
                format!(
                    "duplicate graph-read bypass source label `{}`",
                    source.label()
                ),
            ));
        }
    }
    Ok(())
}

fn validate_source_reference(
    source: &WorthQueryBoundaryAuditSource,
) -> Result<(), WorthQueryBoundaryAuditError> {
    if source.label().trim().is_empty() {
        return Err(WorthQueryBoundaryAuditError::new(
            WorthQueryBoundaryAuditErrorKind::EmptySourceLabel,
            "graph-read bypass source label must not be empty",
        ));
    }
    if source.path().is_some_and(|path| path.trim().is_empty()) {
        return Err(WorthQueryBoundaryAuditError::for_source(
            WorthQueryBoundaryAuditErrorKind::EmptySourcePath,
            source.label(),
            format!(
                "graph-read bypass source `{}` path must not be empty",
                source.label()
            ),
        ));
    }
    Ok(())
}

fn sort_findings(findings: &mut [WorthQueryGraphReadBypassFinding]) {
    findings.sort_by(|left, right| {
        left.source_label()
            .cmp(right.source_label())
            .then(left.line().cmp(&right.line()))
            .then(left.column().cmp(&right.column()))
            .then(left.class().cmp(&right.class()))
    });
}
