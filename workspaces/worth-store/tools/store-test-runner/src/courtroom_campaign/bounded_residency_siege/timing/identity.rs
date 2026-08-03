#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::courtroom_campaign) enum BoundedResidencySiegePhase {
    MutationEvidence,
    World,
    BinaryBuild,
    SourceInventory,
    PrebuildSourceBinding,
    PostbuildBinaryBinding,
    PostbuildSourceBinding,
    SiegeProducer,
    SiegeServing,
    OfflineObserver,
    FreshReopener,
    DurabilityTerminationCampaign,
    FinalSourceBinding,
    ExecutableVerification,
    RunProvenance,
    OracleVerification,
    CampaignBeforeReport,
    ReportEncoding,
}

impl BoundedResidencySiegePhase {
    pub(super) const BEFORE_REPORT: [Self; 17] = [
        Self::MutationEvidence,
        Self::World,
        Self::BinaryBuild,
        Self::SourceInventory,
        Self::PrebuildSourceBinding,
        Self::PostbuildBinaryBinding,
        Self::PostbuildSourceBinding,
        Self::SiegeProducer,
        Self::SiegeServing,
        Self::OfflineObserver,
        Self::FreshReopener,
        Self::DurabilityTerminationCampaign,
        Self::FinalSourceBinding,
        Self::ExecutableVerification,
        Self::RunProvenance,
        Self::OracleVerification,
        Self::CampaignBeforeReport,
    ];

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::MutationEvidence => "mutation-evidence",
            Self::World => "world",
            Self::BinaryBuild => "binary-build",
            Self::SourceInventory => "source-inventory",
            Self::PrebuildSourceBinding => "prebuild-source-binding",
            Self::PostbuildBinaryBinding => "postbuild-binary-binding",
            Self::PostbuildSourceBinding => "postbuild-source-binding",
            Self::SiegeProducer => "producer",
            Self::SiegeServing => "serving",
            Self::OfflineObserver => "offline-observer",
            Self::FreshReopener => "fresh-reopener",
            Self::DurabilityTerminationCampaign => "durability-termination-campaign",
            Self::FinalSourceBinding => "final-source-binding",
            Self::ExecutableVerification => "executable-verification",
            Self::RunProvenance => "run-provenance",
            Self::OracleVerification => "oracle-verification",
            Self::CampaignBeforeReport => "campaign-before-report",
            Self::ReportEncoding => "report-encoding",
        }
    }
}

pub(super) fn expected_before_report() -> Vec<BoundedResidencySiegePhase> {
    BoundedResidencySiegePhase::BEFORE_REPORT.to_vec()
}

pub(super) fn expected_complete() -> Vec<BoundedResidencySiegePhase> {
    let mut expected = expected_before_report();
    expected.push(BoundedResidencySiegePhase::ReportEncoding);
    expected
}
