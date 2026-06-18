use super::error::ForgeQueryBoundaryAuditError;
use super::evidence::{
    derive_boundary_audit_coverage_identity, derive_boundary_audit_finding_identity,
    derive_boundary_audit_report_identity,
};
use super::failure::ForgeQueryBoundaryAuditFailure;
use super::registry_coverage::hard_prohibition_boundary_audit_coverage;
use super::report::ForgeQueryBoundaryAuditReport;
use super::source_set::ForgeQueryBoundaryAuditSourceSet;
use super::syntax_resolution::{
    classify_boundary_audit_source, hard_prohibition_boundary_audit_call_index,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ForgeQueryHardProhibitionBoundaryAudit;

pub fn hard_prohibition_boundary_audit() -> ForgeQueryHardProhibitionBoundaryAudit {
    ForgeQueryHardProhibitionBoundaryAudit
}

impl ForgeQueryHardProhibitionBoundaryAudit {
    pub fn covering_sources(
        self,
        sources: ForgeQueryBoundaryAuditSourceSet,
    ) -> ForgeQueryBoundaryAuditEvaluation {
        ForgeQueryBoundaryAuditEvaluation { sources }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBoundaryAuditEvaluation {
    sources: ForgeQueryBoundaryAuditSourceSet,
}

impl ForgeQueryBoundaryAuditEvaluation {
    pub fn evaluate(&self) -> Result<ForgeQueryBoundaryAuditReport, ForgeQueryBoundaryAuditError> {
        self.sources.validate()?;

        let coverage = hard_prohibition_boundary_audit_coverage();
        let coverage_rows = coverage.rows().to_vec();
        let coverage_identity = derive_boundary_audit_coverage_identity(coverage.rows());
        let call_index = hard_prohibition_boundary_audit_call_index();
        let mut findings = Vec::new();
        let mut parsed_item_count = 0usize;
        let mut visited_call_count = 0usize;

        for source in self.sources.sources() {
            let classification = classify_boundary_audit_source(
                source.label(),
                source.path(),
                source.source(),
                &call_index,
            )?;
            let (source_findings, source_parsed_items, source_visited_calls) =
                classification.into_parts();
            findings.extend(source_findings);
            parsed_item_count += source_parsed_items;
            visited_call_count += source_visited_calls;
        }

        let source_labels = self
            .sources
            .source_labels()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let source_paths = self
            .sources
            .sources()
            .iter()
            .map(|source| source.path().map(str::to_string))
            .collect::<Vec<_>>();
        let source_label_refs = source_labels.iter().map(String::as_str).collect::<Vec<_>>();
        let finding_identities = findings
            .iter()
            .map(derive_boundary_audit_finding_identity)
            .collect::<Vec<_>>();
        let report_identity = derive_boundary_audit_report_identity(
            self.sources.crate_name(),
            &source_label_refs,
            &source_paths,
            &coverage_identity,
            &finding_identities,
            parsed_item_count,
            visited_call_count,
        );

        Ok(ForgeQueryBoundaryAuditReport::sealed(
            self.sources.crate_name().to_string(),
            source_labels,
            source_paths,
            coverage_rows,
            findings,
            coverage_identity,
            finding_identities,
            report_identity,
            parsed_item_count,
            visited_call_count,
        ))
    }

    pub fn assert_clean(&self) -> ForgeQueryBoundaryAuditReport {
        self.try_assert_clean()
            .expect("hard prohibition boundary audit found prohibited seam usage")
    }

    pub fn try_assert_clean(
        &self,
    ) -> Result<ForgeQueryBoundaryAuditReport, ForgeQueryBoundaryAuditFailure> {
        let report = self
            .evaluate()
            .expect("hard prohibition boundary audit source set must be valid Rust");
        report.try_assert_clean()
    }
}
