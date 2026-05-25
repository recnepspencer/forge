use crate::application::{
    ForgeQueryCapabilityFamily, ForgeQueryCapabilityStatus, ForgeQuerySupportMatrix,
    ForgeQuerySupportReport, ForgeQuerySupportSectionPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainEntrySupportSnapshot {
    report: ForgeQuerySupportReport,
    snapshot_digest: String,
}

impl ForgeQueryDomainEntrySupportSnapshot {
    pub(crate) fn from_support_report(report: ForgeQuerySupportReport) -> Self {
        let snapshot_digest = report.report_digest().to_string();
        Self {
            report,
            snapshot_digest,
        }
    }

    pub fn support_matrix(&self) -> &ForgeQuerySupportMatrix {
        self.report.support_matrix()
    }

    pub fn admitted_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        self.report.admitted_capability_families()
    }

    pub fn deferred_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        self.report.deferred_capability_families()
    }

    pub fn unsupported_capability_families(&self) -> &[ForgeQueryCapabilityFamily] {
        self.report.unsupported_capability_families()
    }

    pub fn section_postures(&self) -> &[ForgeQuerySupportSectionPosture] {
        self.report.section_postures()
    }

    pub fn validated_config_digest(&self) -> &str {
        self.report.validated_config_digest()
    }

    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    pub fn capability_status(
        &self,
        family: ForgeQueryCapabilityFamily,
    ) -> Option<ForgeQueryCapabilityStatus> {
        self.support_matrix()
            .descriptor(family)
            .map(|descriptor| descriptor.status())
    }
}
