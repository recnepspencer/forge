use crate::{ForgeQueryBoundaryAuditSourceSite, ForgeQueryEvidenceIdentity};

use super::registry::{
    ForgeQueryGraphReadBypassAuthorityViolation, ForgeQueryGraphReadBypassClass,
    ForgeQueryGraphReadBypassDetection,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadBypassFinding {
    class: ForgeQueryGraphReadBypassClass,
    authority_violation: ForgeQueryGraphReadBypassAuthorityViolation,
    detection: ForgeQueryGraphReadBypassDetection,
    detection_key: &'static str,
    replacement_lane: &'static str,
    source_site: ForgeQueryBoundaryAuditSourceSite,
    finding_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphReadBypassFinding {
    pub(crate) fn sealed(
        class: ForgeQueryGraphReadBypassClass,
        authority_violation: ForgeQueryGraphReadBypassAuthorityViolation,
        detection: ForgeQueryGraphReadBypassDetection,
        detection_key: &'static str,
        replacement_lane: &'static str,
        source_site: ForgeQueryBoundaryAuditSourceSite,
        finding_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        Self {
            class,
            authority_violation,
            detection,
            detection_key,
            replacement_lane,
            source_site,
            finding_identity,
        }
    }

    pub fn class(&self) -> ForgeQueryGraphReadBypassClass {
        self.class
    }

    pub fn authority_violation(&self) -> ForgeQueryGraphReadBypassAuthorityViolation {
        self.authority_violation
    }

    pub fn detection(&self) -> ForgeQueryGraphReadBypassDetection {
        self.detection
    }

    pub fn detection_key(&self) -> &'static str {
        self.detection_key
    }

    pub fn replacement_lane(&self) -> &'static str {
        self.replacement_lane
    }

    pub fn source_site(&self) -> &ForgeQueryBoundaryAuditSourceSite {
        &self.source_site
    }

    pub fn source_label(&self) -> &str {
        self.source_site.source_label()
    }

    pub fn source_path(&self) -> Option<&str> {
        self.source_site.source_path()
    }

    pub fn line(&self) -> usize {
        self.source_site.line()
    }

    pub fn column(&self) -> usize {
        self.source_site.column()
    }

    pub fn finding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.finding_identity
    }
}
