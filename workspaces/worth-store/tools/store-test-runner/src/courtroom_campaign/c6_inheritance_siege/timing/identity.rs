#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::courtroom_campaign) enum SiegePhase {
    MutationEvidence,
    World,
    BinaryBuild,
    SourceInventory,
    PrebuildSourceBinding,
    PostbuildBinaryBinding,
    PostbuildSourceBinding,
    SiegeWriter,
    OfflineObserver,
    FreshReopener,
    FinalSourceBinding,
    ExecutableVerification,
    RunProvenance,
    OracleVerification,
    CampaignBeforeReport,
    ReportEncoding,
}

impl SiegePhase {
    pub(super) const BEFORE_REPORT: [Self; 15] = [
        Self::MutationEvidence,
        Self::World,
        Self::BinaryBuild,
        Self::SourceInventory,
        Self::PrebuildSourceBinding,
        Self::PostbuildBinaryBinding,
        Self::PostbuildSourceBinding,
        Self::SiegeWriter,
        Self::OfflineObserver,
        Self::FreshReopener,
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
            Self::SiegeWriter => "siege-writer",
            Self::OfflineObserver => "offline-observer",
            Self::FreshReopener => "fresh-reopener",
            Self::FinalSourceBinding => "final-source-binding",
            Self::ExecutableVerification => "executable-verification",
            Self::RunProvenance => "run-provenance",
            Self::OracleVerification => "oracle-verification",
            Self::CampaignBeforeReport => "campaign-before-report",
            Self::ReportEncoding => "report-encoding",
        }
    }
}

pub(super) fn expected_before_report() -> Vec<SiegePhase> {
    SiegePhase::BEFORE_REPORT.to_vec()
}

pub(super) fn expected_complete() -> Vec<SiegePhase> {
    let mut expected = expected_before_report();
    expected.push(SiegePhase::ReportEncoding);
    expected
}
