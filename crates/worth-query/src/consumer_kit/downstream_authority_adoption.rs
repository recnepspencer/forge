use std::path::PathBuf;

use crate::consumer_kit::boundary_audit::WorthQueryBoundaryAuditError;
use crate::consumer_kit::consumer_residue::{
    query_consumer_residue_audit, WorthQueryConsumerResidueClass, WorthQueryConsumerResidueReport,
};
use crate::WorthQueryEvidenceIdentity;

const DOWNSTREAM_AUTHORITY_CLASSES: &[WorthQueryConsumerResidueClass] = &[
    WorthQueryConsumerResidueClass::DecomposedProjectionConsumptionAttempt,
    WorthQueryConsumerResidueClass::LocalQueryMeasurementConsumptionIdentity,
    WorthQueryConsumerResidueClass::LocalProjectionContractBinding,
    WorthQueryConsumerResidueClass::LocalQueryBasisDigestCompatibility,
    WorthQueryConsumerResidueClass::LegacyProjectionPrerequisiteAssembly,
    WorthQueryConsumerResidueClass::DirectInternalQueryImport,
];

/// Query-owned audit request for adoption of sealed downstream authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDownstreamAuthorityAdoption {
    consumer_name: String,
    required_roots: Vec<PathBuf>,
}

pub fn downstream_authority_adoption(
    consumer_name: impl Into<String>,
) -> WorthQueryDownstreamAuthorityAdoption {
    WorthQueryDownstreamAuthorityAdoption {
        consumer_name: consumer_name.into(),
        required_roots: Vec::new(),
    }
}

impl WorthQueryDownstreamAuthorityAdoption {
    pub fn required_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.required_roots.push(root.into());
        self
    }

    pub fn evaluate(
        self,
    ) -> Result<WorthQueryDownstreamAuthorityAdoptionProof, WorthQueryBoundaryAuditError> {
        let mut audit = query_consumer_residue_audit(self.consumer_name.clone())
            .with_class_filter(DOWNSTREAM_AUTHORITY_CLASSES.iter().copied());
        for root in self.required_roots {
            audit = audit.required_root(root);
        }
        let report = audit.evaluate()?;
        let manifest = WorthQueryDownstreamAuthorityAdoptionManifest::seal(&report);
        Ok(WorthQueryDownstreamAuthorityAdoptionProof { manifest, report })
    }
}

/// Sealed adoption summary. The report identity binds the complete source
/// inventory, counters, and findings; callers cannot author this manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDownstreamAuthorityAdoptionManifest {
    consumer_name: String,
    audited_source_count: usize,
    prohibited_class_count: usize,
    finding_count: usize,
    source_inventory_digest: String,
    report_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryDownstreamAuthorityAdoptionManifest {
    fn seal(report: &WorthQueryConsumerResidueReport) -> Self {
        Self {
            consumer_name: report.consumer_name().to_string(),
            audited_source_count: report.scanned_file_count(),
            prohibited_class_count: DOWNSTREAM_AUTHORITY_CLASSES.len(),
            finding_count: report.finding_count(),
            source_inventory_digest: report.source_inventory_digest().to_string(),
            report_identity: report.report_identity().clone(),
        }
    }

    pub fn consumer_name(&self) -> &str {
        &self.consumer_name
    }

    pub fn audited_source_count(&self) -> usize {
        self.audited_source_count
    }

    pub fn prohibited_class_count(&self) -> usize {
        self.prohibited_class_count
    }

    pub fn finding_count(&self) -> usize {
        self.finding_count
    }

    pub fn source_inventory_digest(&self) -> &str {
        &self.source_inventory_digest
    }

    pub fn report_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.report_identity
    }

    pub fn adopted(&self) -> bool {
        self.finding_count == 0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDownstreamAuthorityAdoptionProof {
    manifest: WorthQueryDownstreamAuthorityAdoptionManifest,
    report: WorthQueryConsumerResidueReport,
}

impl WorthQueryDownstreamAuthorityAdoptionProof {
    pub fn manifest(&self) -> &WorthQueryDownstreamAuthorityAdoptionManifest {
        &self.manifest
    }

    pub fn residue_report(&self) -> &WorthQueryConsumerResidueReport {
        &self.report
    }

    pub fn assert_adopted(&self) {
        assert!(
            self.manifest.adopted(),
            "downstream authority adoption has residue: {:?}",
            self.report.findings()
        );
    }
}
