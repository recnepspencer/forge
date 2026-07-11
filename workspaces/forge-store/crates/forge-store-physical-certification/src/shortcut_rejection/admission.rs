use crate::{
    FaultDeliveryDenial, ForbiddenShortcutKind, OracleDenial, PhysicalEvidenceBundleDenial,
    PhysicalScenarioDefinitionDenial, SimulationHarnessBoundaryDenial, SimulationPlanDenial,
    TerminalProjectionOnlyEvidenceDenied, TranscriptReplayDenial,
};

use super::{ShortcutRejectionBoundary, SyntheticHarnessShortcutDenialReceipt};

pub fn shortcut_denial_from_scenario_denial(
    denial: PhysicalScenarioDefinitionDenial,
) -> Option<SyntheticHarnessShortcutDenialReceipt> {
    let (shortcut, boundary) = match denial {
        PhysicalScenarioDefinitionDenial::JsonScenarioAuthorityDenied => (
            ForbiddenShortcutKind::JsonScenarioAuthority,
            ShortcutRejectionBoundary::ScenarioJsonAuthority,
        ),
        PhysicalScenarioDefinitionDenial::TerminalProjectionScenarioDenied => (
            ForbiddenShortcutKind::TerminalProjectionAuthority,
            ShortcutRejectionBoundary::ScenarioTerminalProjection,
        ),
        PhysicalScenarioDefinitionDenial::RawStringScenarioAuthorityDenied => (
            ForbiddenShortcutKind::JsonScenarioAuthority,
            ShortcutRejectionBoundary::ScenarioRawStringAuthority,
        ),
        PhysicalScenarioDefinitionDenial::CopiedDigestScenarioAuthorityDenied => (
            ForbiddenShortcutKind::CopiedDigestAuthority,
            ShortcutRejectionBoundary::ScenarioCopiedDigest,
        ),
        PhysicalScenarioDefinitionDenial::FixtureLabelScenarioAuthorityDenied => (
            ForbiddenShortcutKind::FixtureLabelAuthority,
            ShortcutRejectionBoundary::ScenarioFixtureLabel,
        ),
        PhysicalScenarioDefinitionDenial::ProofProgressionSkipped => (
            ForbiddenShortcutKind::SkippedProofProgression,
            ShortcutRejectionBoundary::ScenarioProofProgressionSkipped,
        ),
        _ => return None,
    };
    Some(SyntheticHarnessShortcutDenialReceipt::from_store_denial(
        shortcut, boundary,
    ))
}

pub fn shortcut_denial_from_fault_delivery_denial(
    denial: FaultDeliveryDenial,
) -> Option<SyntheticHarnessShortcutDenialReceipt> {
    match denial {
        FaultDeliveryDenial::PrivateMutationDenied => {
            Some(SyntheticHarnessShortcutDenialReceipt::from_store_denial(
                ForbiddenShortcutKind::PrivateMutation,
                ShortcutRejectionBoundary::FaultDeliveryPrivateMutation,
            ))
        }
        _ => None,
    }
}

pub fn shortcut_denial_from_oracle_denial(
    denial: OracleDenial,
) -> Option<SyntheticHarnessShortcutDenialReceipt> {
    let (shortcut, boundary) = match denial {
        OracleDenial::TestSupportOracleDenied => (
            ForbiddenShortcutKind::TestSupportVerdictAuthority,
            ShortcutRejectionBoundary::OracleTestSupportVerdict,
        ),
        OracleDenial::LogOnlyEvidenceDenied => (
            ForbiddenShortcutKind::LogsAsProof,
            ShortcutRejectionBoundary::HarnessBoundaryLogOutput,
        ),
        OracleDenial::SameRunSelfComparisonDenied => (
            ForbiddenShortcutKind::SameRunSelfComparison,
            ShortcutRejectionBoundary::HarnessBoundarySameRunSelfComparison,
        ),
        OracleDenial::FixtureLabelOracleDenied => (
            ForbiddenShortcutKind::FixtureLabelAuthority,
            ShortcutRejectionBoundary::OracleFixtureLabel,
        ),
        _ => return None,
    };
    Some(SyntheticHarnessShortcutDenialReceipt::from_store_denial(
        shortcut, boundary,
    ))
}

pub fn shortcut_denial_from_evidence_bundle_denial(
    denial: PhysicalEvidenceBundleDenial,
) -> Option<SyntheticHarnessShortcutDenialReceipt> {
    let (shortcut, boundary) = match denial {
        PhysicalEvidenceBundleDenial::LooseLogDenied => (
            ForbiddenShortcutKind::LogsAsProof,
            ShortcutRejectionBoundary::EvidenceLooseLog,
        ),
        PhysicalEvidenceBundleDenial::TerminalJsonDenied => (
            ForbiddenShortcutKind::TerminalProjectionAuthority,
            ShortcutRejectionBoundary::EvidenceTerminalProjection,
        ),
        PhysicalEvidenceBundleDenial::SameRunSelfComparisonDenied => (
            ForbiddenShortcutKind::SameRunSelfComparison,
            ShortcutRejectionBoundary::EvidenceSameRunSelfComparison,
        ),
        _ => return None,
    };
    Some(SyntheticHarnessShortcutDenialReceipt::from_store_denial(
        shortcut, boundary,
    ))
}

pub fn shortcut_denial_from_terminal_projection_denial(
    denial: TerminalProjectionOnlyEvidenceDenied,
) -> SyntheticHarnessShortcutDenialReceipt {
    match denial {
        TerminalProjectionOnlyEvidenceDenied::TerminalJsonProjection => {
            SyntheticHarnessShortcutDenialReceipt::from_store_denial(
                ForbiddenShortcutKind::TerminalProjectionAuthority,
                ShortcutRejectionBoundary::EvidenceTerminalProjection,
            )
        }
    }
}

pub fn shortcut_denial_from_transcript_denial(
    denial: TranscriptReplayDenial,
) -> Option<SyntheticHarnessShortcutDenialReceipt> {
    let (shortcut, boundary) = match denial {
        TranscriptReplayDenial::LooseLogDenied => (
            ForbiddenShortcutKind::LogsAsProof,
            ShortcutRejectionBoundary::EvidenceLooseLogTranscript,
        ),
        TranscriptReplayDenial::TerminalJsonDenied => (
            ForbiddenShortcutKind::TerminalProjectionAuthority,
            ShortcutRejectionBoundary::EvidenceTerminalJsonTranscript,
        ),
        TranscriptReplayDenial::CopiedTranscriptFieldsDenied => (
            ForbiddenShortcutKind::CopiedDigestAuthority,
            ShortcutRejectionBoundary::TranscriptCopiedFields,
        ),
        TranscriptReplayDenial::SameRunSelfComparisonDenied => (
            ForbiddenShortcutKind::SameRunSelfComparison,
            ShortcutRejectionBoundary::EvidenceSameRunTranscript,
        ),
        _ => return None,
    };
    Some(SyntheticHarnessShortcutDenialReceipt::from_store_denial(
        shortcut, boundary,
    ))
}

pub fn shortcut_denial_from_plan_denial(
    denial: SimulationPlanDenial,
) -> Option<SyntheticHarnessShortcutDenialReceipt> {
    match denial {
        SimulationPlanDenial::ProofProgressionSkipped => {
            Some(SyntheticHarnessShortcutDenialReceipt::from_store_denial(
                ForbiddenShortcutKind::SkippedProofProgression,
                ShortcutRejectionBoundary::PlanProofProgressionSkipped,
            ))
        }
        _ => None,
    }
}

pub fn shortcut_denial_from_harness_boundary_denial(
    denial: SimulationHarnessBoundaryDenial,
) -> Option<SyntheticHarnessShortcutDenialReceipt> {
    let (shortcut, boundary) = match denial {
        SimulationHarnessBoundaryDenial::CopiedS4ReportCannotAdmitEntry => (
            ForbiddenShortcutKind::CopiedDigestAuthority,
            ShortcutRejectionBoundary::HarnessBoundaryCopiedS4Report,
        ),
        SimulationHarnessBoundaryDenial::LogOutputCannotAdmitEntry => (
            ForbiddenShortcutKind::LogsAsProof,
            ShortcutRejectionBoundary::HarnessBoundaryLogOutput,
        ),
        SimulationHarnessBoundaryDenial::SameRunSelfComparisonCannotAdmitEntry => (
            ForbiddenShortcutKind::SameRunSelfComparison,
            ShortcutRejectionBoundary::HarnessBoundarySameRunSelfComparison,
        ),
        SimulationHarnessBoundaryDenial::TerminalProjectionCannotAdmitEntry => (
            ForbiddenShortcutKind::TerminalProjectionAuthority,
            ShortcutRejectionBoundary::HarnessBoundaryTerminalProjection,
        ),
        SimulationHarnessBoundaryDenial::TestSupportMechanicsCannotOwnCertificationMeaning => (
            ForbiddenShortcutKind::TestSupportVerdictAuthority,
            ShortcutRejectionBoundary::HarnessBoundaryTestSupportMeaning,
        ),
        SimulationHarnessBoundaryDenial::ProofProgressionSkipped => (
            ForbiddenShortcutKind::SkippedProofProgression,
            ShortcutRejectionBoundary::HarnessBoundaryProofProgressionSkipped,
        ),
        _ => return None,
    };
    Some(SyntheticHarnessShortcutDenialReceipt::from_store_denial(
        shortcut, boundary,
    ))
}
