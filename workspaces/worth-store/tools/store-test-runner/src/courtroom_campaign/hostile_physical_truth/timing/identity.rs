use worth_store::physical_runtime::PhysicalWorkHostileTruthScenario;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::courtroom_campaign) enum CampaignPhase {
    MutationEvidence,
    World,
    BinaryBuild,
    SourceInventory,
    PrebuildSourceBinding,
    PostbuildBinaryBinding,
    PostbuildSourceBinding,
    RunProvenance,
    FinalSourceBinding,
    ExecutableVerification,
    CampaignVerification,
    CampaignBeforeReport,
    ReportEncoding,
}

impl CampaignPhase {
    pub(super) const BEFORE_REPORT: [Self; 12] = [
        Self::MutationEvidence,
        Self::World,
        Self::BinaryBuild,
        Self::SourceInventory,
        Self::PrebuildSourceBinding,
        Self::PostbuildBinaryBinding,
        Self::PostbuildSourceBinding,
        Self::RunProvenance,
        Self::FinalSourceBinding,
        Self::ExecutableVerification,
        Self::CampaignVerification,
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
            Self::RunProvenance => "run-provenance",
            Self::FinalSourceBinding => "final-source-binding",
            Self::ExecutableVerification => "executable-verification",
            Self::CampaignVerification => "campaign-verification",
            Self::CampaignBeforeReport => "campaign-before-report",
            Self::ReportEncoding => "report-encoding",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::courtroom_campaign) enum ScenarioStage {
    Seed,
    BaselineObserver,
    Fault,
    PostKillObserver,
    FreshReopener,
}

impl ScenarioStage {
    pub(super) const ALL: [Self; 5] = [
        Self::Seed,
        Self::BaselineObserver,
        Self::Fault,
        Self::PostKillObserver,
        Self::FreshReopener,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Seed => "seed",
            Self::BaselineObserver => "baseline-observer",
            Self::Fault => "fault",
            Self::PostKillObserver => "post-kill-observer",
            Self::FreshReopener => "reopen",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum TimingIdentity {
    Campaign(CampaignPhase),
    Scenario {
        scenario: PhysicalWorkHostileTruthScenario,
        stage: ScenarioStage,
    },
    CaseVerification(PhysicalWorkHostileTruthScenario),
}

impl TimingIdentity {
    pub(super) fn label(self) -> Box<str> {
        match self {
            Self::Campaign(phase) => phase.label().into(),
            Self::Scenario { scenario, stage } => {
                format!("{}:{}", scenario.label(), stage.label()).into()
            }
            Self::CaseVerification(scenario) => {
                format!("case-verification:{}", scenario.label()).into()
            }
        }
    }
}

pub(super) fn expected_before_report() -> Vec<TimingIdentity> {
    let mut expected = CampaignPhase::BEFORE_REPORT
        .into_iter()
        .map(TimingIdentity::Campaign)
        .collect::<Vec<_>>();
    for scenario in PhysicalWorkHostileTruthScenario::ALL {
        expected.extend(
            ScenarioStage::ALL
                .into_iter()
                .map(|stage| TimingIdentity::Scenario { scenario, stage }),
        );
        expected.push(TimingIdentity::CaseVerification(scenario));
    }
    expected
}

pub(super) fn expected_complete() -> Vec<TimingIdentity> {
    let mut expected = expected_before_report();
    expected.push(TimingIdentity::Campaign(CampaignPhase::ReportEncoding));
    expected
}
