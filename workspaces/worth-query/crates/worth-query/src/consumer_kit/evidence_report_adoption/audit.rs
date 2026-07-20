use super::error::WorthQueryEvidenceReportAdoptionError;
use super::evidence::{
    derive_adoption_finding_identity, derive_adoption_report_identity,
    derive_adoption_residue_identity,
};
use super::report::{
    WorthQueryEvidenceReportAdoptionReport, WorthQueryEvidenceReportAdoptionResidueRow,
};
use super::source_set::{
    WorthQueryEvidenceReportAdoptionResidueClassification,
    WorthQueryEvidenceReportAdoptionSourceSet,
};
use super::syntax::classify_evidence_report_adoption_source;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryEvidenceReportAdoptionAudit;

pub fn evidence_report_adoption_audit() -> WorthQueryEvidenceReportAdoptionAudit {
    WorthQueryEvidenceReportAdoptionAudit
}

impl WorthQueryEvidenceReportAdoptionAudit {
    pub fn covering_sources(
        self,
        sources: WorthQueryEvidenceReportAdoptionSourceSet,
    ) -> WorthQueryEvidenceReportAdoptionEvaluation {
        WorthQueryEvidenceReportAdoptionEvaluation { sources }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryEvidenceReportAdoptionEvaluation {
    sources: WorthQueryEvidenceReportAdoptionSourceSet,
}

impl WorthQueryEvidenceReportAdoptionEvaluation {
    pub fn evaluate(
        &self,
    ) -> Result<WorthQueryEvidenceReportAdoptionReport, WorthQueryEvidenceReportAdoptionError> {
        self.sources.validate()?;

        let mut residue_rows = Vec::new();
        let mut findings = Vec::new();
        let mut parsed_item_count = 0usize;
        let mut visited_site_count = 0usize;

        for source in self.sources.sources() {
            let source_classification =
                classify_evidence_report_adoption_source(source.label(), source.source())?;
            parsed_item_count += source_classification.parsed_item_count();
            visited_site_count += source_classification.residue_sites().len();
            findings.extend(source_classification.into_findings(
                source.label(),
                source.path(),
                source.classification(),
            ));

            if source.classification()
                == WorthQueryEvidenceReportAdoptionResidueClassification::DefendedDomainArtifactIdentity
            {
                for (symbol, usage_count) in source_classification.symbol_usage_counts() {
                    let row_identity = derive_adoption_residue_identity(
                        source.label(),
                        source.path(),
                        &symbol,
                        source.classification(),
                        usage_count,
                    );
                    residue_rows.push(WorthQueryEvidenceReportAdoptionResidueRow::sealed(
                        source.label().to_string(),
                        source.path().map(str::to_string),
                        symbol,
                        source.classification(),
                        usage_count,
                        row_identity,
                    ));
                }
            }
        }

        let source_labels = self
            .sources
            .sources()
            .iter()
            .map(|source| source.label().to_string())
            .collect::<Vec<_>>();
        let finding_identities = findings
            .iter()
            .map(derive_adoption_finding_identity)
            .collect::<Vec<_>>();
        let report_identity = derive_adoption_report_identity(
            self.sources.crate_name(),
            &source_labels,
            &residue_rows,
            &finding_identities,
            parsed_item_count,
            visited_site_count,
        );

        Ok(WorthQueryEvidenceReportAdoptionReport::sealed(
            self.sources.crate_name().to_string(),
            source_labels,
            residue_rows,
            findings,
            finding_identities,
            report_identity,
            parsed_item_count,
            visited_site_count,
        ))
    }
}
