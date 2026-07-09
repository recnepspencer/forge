use crate::application::WorthQueryMilestoneClosureStatus;
use crate::WorthQueryEvidenceIdentity;

use super::evidence::consumer_kit_family_closure_identity;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryConsumerKitFamilyName {
    EvidenceReportKit,
    HardProhibitionRegistry,
    BoundaryAudit,
    SupportSnapshot,
    SupportPinning,
    InMemoryTestBackend,
    ConsumerResidueAudit,
}

impl WorthQueryConsumerKitFamilyName {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceReportKit => "evidence-report-kit",
            Self::HardProhibitionRegistry => "hard-prohibition-registry",
            Self::BoundaryAudit => "boundary-audit",
            Self::SupportSnapshot => "support-snapshot",
            Self::SupportPinning => "support-pinning",
            Self::InMemoryTestBackend => "in-memory-test-backend",
            Self::ConsumerResidueAudit => "consumer-residue-audit",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerKitFamilyClosureRow {
    family_name: WorthQueryConsumerKitFamilyName,
    status: WorthQueryMilestoneClosureStatus,
    evidence_label: &'static str,
    evidence_digest: String,
    evidence_source_paths: Vec<&'static str>,
    closure_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryConsumerKitFamilyClosureRow {
    pub(crate) fn closed(
        family_name: WorthQueryConsumerKitFamilyName,
        evidence_label: &'static str,
        evidence_digest: impl Into<String>,
        evidence_source_paths: impl IntoIterator<Item = &'static str>,
    ) -> Self {
        Self::new(
            family_name,
            WorthQueryMilestoneClosureStatus::Closed,
            evidence_label,
            evidence_digest,
            evidence_source_paths,
        )
    }

    pub(crate) fn new(
        family_name: WorthQueryConsumerKitFamilyName,
        status: WorthQueryMilestoneClosureStatus,
        evidence_label: &'static str,
        evidence_digest: impl Into<String>,
        evidence_source_paths: impl IntoIterator<Item = &'static str>,
    ) -> Self {
        let evidence_digest = evidence_digest.into();
        let evidence_source_paths = evidence_source_paths.into_iter().collect::<Vec<_>>();
        let closure_identity = consumer_kit_family_closure_identity(
            family_name,
            status,
            evidence_label,
            &evidence_digest,
            &evidence_source_paths,
        );
        Self {
            family_name,
            status,
            evidence_label,
            evidence_digest,
            evidence_source_paths,
            closure_identity,
        }
    }

    pub fn family_name(&self) -> WorthQueryConsumerKitFamilyName {
        self.family_name
    }

    pub fn status(&self) -> WorthQueryMilestoneClosureStatus {
        self.status
    }

    pub fn evidence_label(&self) -> &'static str {
        self.evidence_label
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn evidence_source_paths(&self) -> &[&'static str] {
        &self.evidence_source_paths
    }

    pub fn closure_digest(&self) -> &str {
        self.closure_identity.as_str()
    }

    pub fn closure_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.closure_identity
    }
}
