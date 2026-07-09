use crate::{WorthQueryBoundaryAuditSourceSite, WorthQueryEvidenceIdentity};

use super::registry::{
    WorthQueryGraphReadBypassAuthorityViolation, WorthQueryGraphReadBypassClass,
    WorthQueryGraphReadBypassDetection,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassFinding {
    class: WorthQueryGraphReadBypassClass,
    authority_violation: WorthQueryGraphReadBypassAuthorityViolation,
    detection: WorthQueryGraphReadBypassDetection,
    detection_key: &'static str,
    replacement_lane: &'static str,
    source_site: WorthQueryBoundaryAuditSourceSite,
    finding_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphReadBypassFinding {
    pub(crate) fn sealed(
        class: WorthQueryGraphReadBypassClass,
        authority_violation: WorthQueryGraphReadBypassAuthorityViolation,
        detection: WorthQueryGraphReadBypassDetection,
        detection_key: &'static str,
        replacement_lane: &'static str,
        source_site: WorthQueryBoundaryAuditSourceSite,
        finding_identity: WorthQueryEvidenceIdentity,
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

    pub fn class(&self) -> WorthQueryGraphReadBypassClass {
        self.class
    }

    pub fn authority_violation(&self) -> WorthQueryGraphReadBypassAuthorityViolation {
        self.authority_violation
    }

    pub fn detection(&self) -> WorthQueryGraphReadBypassDetection {
        self.detection
    }

    pub fn detection_key(&self) -> &'static str {
        self.detection_key
    }

    pub fn replacement_lane(&self) -> &'static str {
        self.replacement_lane
    }

    pub fn source_site(&self) -> &WorthQueryBoundaryAuditSourceSite {
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

    pub fn finding_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.finding_identity
    }
}
