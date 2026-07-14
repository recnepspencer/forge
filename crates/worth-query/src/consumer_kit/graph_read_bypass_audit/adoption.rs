use crate::WorthQueryEvidenceIdentity;

use super::graph_read_bypass_digest;
use super::report::{
    WorthQueryGraphReadBypassReport, WorthQueryGraphReadBypassReportResidueCertification,
};
use super::residue::WorthQueryGraphReadBypassResidueManifest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadBypassAdoptionErrorKind {
    BlankConsumerName,
    MissingAuditReport,
    MissingResidueManifest,
    MissingSourceInventoryProof,
    ResidueCertificationFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassAdoptionError {
    kind: WorthQueryGraphReadBypassAdoptionErrorKind,
    message: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassAdoption {
    consumer_name: String,
    report: Option<WorthQueryGraphReadBypassReport>,
    residue_manifest: Option<WorthQueryGraphReadBypassResidueManifest>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassAdoptionProof {
    manifest: WorthQueryGraphReadBypassAdoptionManifest,
    report: WorthQueryGraphReadBypassReport,
    residue_manifest: WorthQueryGraphReadBypassResidueManifest,
    residue_certification: WorthQueryGraphReadBypassReportResidueCertification,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassAdoptionManifest {
    consumer_name: String,
    report_identity: WorthQueryEvidenceIdentity,
    residue_manifest_digest: String,
    residue_certification_identity: WorthQueryEvidenceIdentity,
    unclassified_finding_count: usize,
    manifest_digest: String,
}

pub fn graph_read_bypass_adoption(
    consumer_name: impl Into<String>,
) -> WorthQueryGraphReadBypassAdoption {
    WorthQueryGraphReadBypassAdoption {
        consumer_name: consumer_name.into(),
        report: None,
        residue_manifest: None,
    }
}

impl WorthQueryGraphReadBypassAdoption {
    pub fn audit_report(mut self, report: WorthQueryGraphReadBypassReport) -> Self {
        self.report = Some(report);
        self
    }

    pub fn residue_manifest(mut self, manifest: WorthQueryGraphReadBypassResidueManifest) -> Self {
        self.residue_manifest = Some(manifest);
        self
    }

    pub fn certify(
        self,
    ) -> Result<WorthQueryGraphReadBypassAdoptionProof, WorthQueryGraphReadBypassAdoptionError>
    {
        let consumer_name = required_consumer_name(self.consumer_name)?;
        let report = required_audit_report(self.report)?;
        require_source_inventory_proof(&report)?;
        let residue_manifest = required_residue_manifest(self.residue_manifest)?;
        let residue_certification = certify_report_residue(&report, &residue_manifest)?;
        let manifest = adoption_manifest_for_certified_report(
            consumer_name,
            &report,
            &residue_manifest,
            &residue_certification,
        );
        Ok(WorthQueryGraphReadBypassAdoptionProof {
            manifest,
            report,
            residue_manifest,
            residue_certification,
        })
    }
}

impl WorthQueryGraphReadBypassAdoptionProof {
    pub fn manifest(&self) -> &WorthQueryGraphReadBypassAdoptionManifest {
        &self.manifest
    }

    pub fn report(&self) -> &WorthQueryGraphReadBypassReport {
        &self.report
    }

    pub fn residue_manifest(&self) -> &WorthQueryGraphReadBypassResidueManifest {
        &self.residue_manifest
    }

    pub fn residue_certification(&self) -> &WorthQueryGraphReadBypassReportResidueCertification {
        &self.residue_certification
    }

    pub fn has_no_unclassified_findings(&self) -> bool {
        self.manifest.unclassified_finding_count == 0
    }

    pub fn unclassified_finding_count(&self) -> usize {
        self.manifest.unclassified_finding_count
    }
}

impl WorthQueryGraphReadBypassAdoptionManifest {
    fn sealed(
        consumer_name: String,
        report_identity: WorthQueryEvidenceIdentity,
        residue_manifest_digest: String,
        residue_certification_identity: WorthQueryEvidenceIdentity,
        unclassified_finding_count: usize,
    ) -> Self {
        let unclassified_finding_count_text = unclassified_finding_count.to_string();
        let manifest_digest = graph_read_bypass_digest(
            "adoption-manifest",
            [
                consumer_name.as_str(),
                report_identity.terminal_projection_for_reporting(),
                residue_manifest_digest.as_str(),
                residue_certification_identity.terminal_projection_for_reporting(),
                unclassified_finding_count_text.as_str(),
            ],
        );
        Self {
            consumer_name,
            report_identity,
            residue_manifest_digest,
            residue_certification_identity,
            unclassified_finding_count,
            manifest_digest,
        }
    }

    pub fn consumer_name(&self) -> &str {
        &self.consumer_name
    }

    pub fn report_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.report_identity
    }

    pub fn residue_manifest_digest(&self) -> &str {
        &self.residue_manifest_digest
    }

    pub fn residue_certification_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.residue_certification_identity
    }

    pub fn unclassified_finding_count(&self) -> usize {
        self.unclassified_finding_count
    }

    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }
}

impl WorthQueryGraphReadBypassAdoptionError {
    fn new(kind: WorthQueryGraphReadBypassAdoptionErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> WorthQueryGraphReadBypassAdoptionErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for WorthQueryGraphReadBypassAdoptionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.kind.as_str(), self.message)
    }
}

impl std::error::Error for WorthQueryGraphReadBypassAdoptionError {}

impl WorthQueryGraphReadBypassAdoptionErrorKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlankConsumerName => "blank-consumer-name",
            Self::MissingAuditReport => "missing-audit-report",
            Self::MissingResidueManifest => "missing-residue-manifest",
            Self::MissingSourceInventoryProof => "missing-source-inventory-proof",
            Self::ResidueCertificationFailed => "residue-certification-failed",
        }
    }
}

fn required_consumer_name(
    consumer_name: String,
) -> Result<String, WorthQueryGraphReadBypassAdoptionError> {
    let consumer_name = consumer_name.trim().to_string();
    if consumer_name.is_empty() {
        Err(WorthQueryGraphReadBypassAdoptionError::new(
            WorthQueryGraphReadBypassAdoptionErrorKind::BlankConsumerName,
            "graph-read bypass adoption requires a consumer name",
        ))
    } else {
        Ok(consumer_name)
    }
}

fn required_audit_report(
    report: Option<WorthQueryGraphReadBypassReport>,
) -> Result<WorthQueryGraphReadBypassReport, WorthQueryGraphReadBypassAdoptionError> {
    report.ok_or_else(|| {
        WorthQueryGraphReadBypassAdoptionError::new(
            WorthQueryGraphReadBypassAdoptionErrorKind::MissingAuditReport,
            "graph-read bypass adoption requires an evaluated audit report",
        )
    })
}

fn require_source_inventory_proof(
    report: &WorthQueryGraphReadBypassReport,
) -> Result<(), WorthQueryGraphReadBypassAdoptionError> {
    if report.source_inventory_identities().is_empty() {
        Err(WorthQueryGraphReadBypassAdoptionError::new(
            WorthQueryGraphReadBypassAdoptionErrorKind::MissingSourceInventoryProof,
            "graph-read bypass adoption requires a report derived from a source inventory",
        ))
    } else {
        Ok(())
    }
}

fn required_residue_manifest(
    residue_manifest: Option<WorthQueryGraphReadBypassResidueManifest>,
) -> Result<WorthQueryGraphReadBypassResidueManifest, WorthQueryGraphReadBypassAdoptionError> {
    residue_manifest.ok_or_else(|| {
        WorthQueryGraphReadBypassAdoptionError::new(
            WorthQueryGraphReadBypassAdoptionErrorKind::MissingResidueManifest,
            "graph-read bypass adoption requires an explicit residue manifest",
        )
    })
}

fn certify_report_residue(
    report: &WorthQueryGraphReadBypassReport,
    residue_manifest: &WorthQueryGraphReadBypassResidueManifest,
) -> Result<
    WorthQueryGraphReadBypassReportResidueCertification,
    WorthQueryGraphReadBypassAdoptionError,
> {
    report
        .certify_with_residue(residue_manifest)
        .map_err(|error| {
            WorthQueryGraphReadBypassAdoptionError::new(
                WorthQueryGraphReadBypassAdoptionErrorKind::ResidueCertificationFailed,
                error.message(),
            )
        })
}

fn adoption_manifest_for_certified_report(
    consumer_name: String,
    report: &WorthQueryGraphReadBypassReport,
    residue_manifest: &WorthQueryGraphReadBypassResidueManifest,
    residue_certification: &WorthQueryGraphReadBypassReportResidueCertification,
) -> WorthQueryGraphReadBypassAdoptionManifest {
    WorthQueryGraphReadBypassAdoptionManifest::sealed(
        consumer_name,
        report.report_identity().clone(),
        residue_manifest.manifest_digest().to_string(),
        residue_certification.certification_identity().clone(),
        0,
    )
}
