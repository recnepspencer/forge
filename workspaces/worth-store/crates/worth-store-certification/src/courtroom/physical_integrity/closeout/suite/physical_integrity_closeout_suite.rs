use super::physical_integrity_closeout_suite_kind::required_synthetic_attempts;
use crate::{
    CorruptionLocalizationBoundary, ExecutedCorruptionLocalizationEvidence,
    ExecutedIntegrityBoundaryDenialEvidence, IntegrityCloseoutDenialBoundary,
    IntegrityCloseoutEvidenceFamily, IntegrityCloseoutExecutedOutputKind,
    IntegrityCompositionEvidence, IntegrityHarnessTranscriptEvidence,
    IntegrityRecoveryHandoffCloseoutEvidence, PhysicalIntegrityAcceptanceSuite,
    PhysicalIntegrityCloseoutDenial, SyntheticCloseoutShortcutRejectionReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIntegrityCloseoutSuiteEvidence {
    acceptance_suite: PhysicalIntegrityAcceptanceSuite,
    evidence_family: IntegrityCloseoutEvidenceFamily,
    harness: IntegrityHarnessTranscriptEvidence,
    localized_boundaries: Vec<ExecutedCorruptionLocalizationEvidence>,
    denied_boundaries: Vec<ExecutedIntegrityBoundaryDenialEvidence>,
    synthetic_rejections: Vec<SyntheticCloseoutShortcutRejectionReport>,
    recovery_handoff: Option<IntegrityRecoveryHandoffCloseoutEvidence>,
    line_cap_composition: Option<IntegrityCompositionEvidence>,
}

impl PhysicalIntegrityCloseoutSuiteEvidence {
    pub fn corruption_localization(
        harness: IntegrityHarnessTranscriptEvidence,
        localized_boundaries: Vec<ExecutedCorruptionLocalizationEvidence>,
    ) -> Self {
        Self::new(
            PhysicalIntegrityAcceptanceSuite::CorruptionLocalization,
            IntegrityCloseoutEvidenceFamily::CorruptionLocalization,
            harness,
        )
        .with_localized_boundaries(localized_boundaries)
    }

    pub fn boundary_denial(
        harness: IntegrityHarnessTranscriptEvidence,
        denied_boundaries: Vec<ExecutedIntegrityBoundaryDenialEvidence>,
    ) -> Self {
        Self::new(
            PhysicalIntegrityAcceptanceSuite::BoundaryDenial,
            IntegrityCloseoutEvidenceFamily::BoundaryDenial,
            harness,
        )
        .with_denied_boundaries(denied_boundaries)
    }

    pub fn harness_transcript(harness: IntegrityHarnessTranscriptEvidence) -> Self {
        Self::new(
            PhysicalIntegrityAcceptanceSuite::HarnessTranscript,
            IntegrityCloseoutEvidenceFamily::HarnessTranscript,
            harness,
        )
    }

    pub fn synthetic_rejection(
        harness: IntegrityHarnessTranscriptEvidence,
        rejections: Vec<SyntheticCloseoutShortcutRejectionReport>,
    ) -> Self {
        Self::new(
            PhysicalIntegrityAcceptanceSuite::SyntheticShortcutRejection,
            IntegrityCloseoutEvidenceFamily::SyntheticShortcutRejection,
            harness,
        )
        .with_synthetic_rejections(rejections)
    }

    pub fn recovery_handoff(
        harness: IntegrityHarnessTranscriptEvidence,
        handoff: IntegrityRecoveryHandoffCloseoutEvidence,
    ) -> Self {
        Self::new(
            PhysicalIntegrityAcceptanceSuite::RecoveryIntegrityHandoff,
            IntegrityCloseoutEvidenceFamily::RecoveryIntegrityHandoff,
            harness,
        )
        .with_recovery_handoff(handoff)
    }

    pub fn line_cap_composition(
        harness: IntegrityHarnessTranscriptEvidence,
        evidence: IntegrityCompositionEvidence,
    ) -> Self {
        Self::new(
            PhysicalIntegrityAcceptanceSuite::LineCapComposition,
            IntegrityCloseoutEvidenceFamily::LineCapComposition,
            harness,
        )
        .with_line_cap_composition(evidence)
    }

    pub const fn acceptance_suite(&self) -> PhysicalIntegrityAcceptanceSuite {
        self.acceptance_suite
    }

    pub const fn evidence_family(&self) -> IntegrityCloseoutEvidenceFamily {
        self.evidence_family
    }

    pub const fn harness(&self) -> &IntegrityHarnessTranscriptEvidence {
        &self.harness
    }

    pub fn localized_boundaries(&self) -> &[ExecutedCorruptionLocalizationEvidence] {
        &self.localized_boundaries
    }

    pub fn denied_boundaries(&self) -> &[ExecutedIntegrityBoundaryDenialEvidence] {
        &self.denied_boundaries
    }

    pub fn synthetic_rejections(&self) -> &[SyntheticCloseoutShortcutRejectionReport] {
        &self.synthetic_rejections
    }

    pub const fn recovery_handoff_evidence(
        &self,
    ) -> Option<&IntegrityRecoveryHandoffCloseoutEvidence> {
        self.recovery_handoff.as_ref()
    }

    pub const fn line_cap_composition_evidence(&self) -> Option<&IntegrityCompositionEvidence> {
        self.line_cap_composition.as_ref()
    }

    fn new(
        acceptance_suite: PhysicalIntegrityAcceptanceSuite,
        evidence_family: IntegrityCloseoutEvidenceFamily,
        harness: IntegrityHarnessTranscriptEvidence,
    ) -> Self {
        Self {
            acceptance_suite,
            evidence_family,
            harness,
            localized_boundaries: Vec::new(),
            denied_boundaries: Vec::new(),
            synthetic_rejections: Vec::new(),
            recovery_handoff: None,
            line_cap_composition: None,
        }
    }

    fn with_localized_boundaries(
        mut self,
        boundaries: Vec<ExecutedCorruptionLocalizationEvidence>,
    ) -> Self {
        self.localized_boundaries = boundaries;
        self
    }

    fn with_denied_boundaries(
        mut self,
        boundaries: Vec<ExecutedIntegrityBoundaryDenialEvidence>,
    ) -> Self {
        self.denied_boundaries = boundaries;
        self
    }

    fn with_synthetic_rejections(
        mut self,
        rejections: Vec<SyntheticCloseoutShortcutRejectionReport>,
    ) -> Self {
        self.synthetic_rejections = rejections;
        self
    }

    fn with_recovery_handoff(mut self, evidence: IntegrityRecoveryHandoffCloseoutEvidence) -> Self {
        self.recovery_handoff = Some(evidence);
        self
    }

    fn with_line_cap_composition(mut self, evidence: IntegrityCompositionEvidence) -> Self {
        self.line_cap_composition = Some(evidence);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIntegrityCloseoutSuite {
    evidence: Vec<PhysicalIntegrityCloseoutSuiteEvidence>,
}

impl PhysicalIntegrityCloseoutSuite {
    pub fn admit(
        evidence: Vec<PhysicalIntegrityCloseoutSuiteEvidence>,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        let suite = Self { evidence };
        suite.require_complete()?;
        Ok(suite)
    }

    pub fn evidence(&self) -> &[PhysicalIntegrityCloseoutSuiteEvidence] {
        &self.evidence
    }

    pub fn evidence_for(
        &self,
        kind: PhysicalIntegrityAcceptanceSuite,
    ) -> Option<&PhysicalIntegrityCloseoutSuiteEvidence> {
        self.evidence
            .iter()
            .find(|evidence| evidence.acceptance_suite == kind)
    }

    pub fn contains_evidence_family(&self, family: IntegrityCloseoutEvidenceFamily) -> bool {
        self.evidence
            .iter()
            .any(|evidence| evidence.evidence_family == family)
    }

    fn require_complete(&self) -> Result<(), PhysicalIntegrityCloseoutDenial> {
        for kind in PhysicalIntegrityAcceptanceSuite::ALL {
            require_exactly_one_suite(self, kind)?;
        }
        for family in IntegrityCloseoutEvidenceFamily::ALL {
            if !self.contains_evidence_family(family) {
                return Err(PhysicalIntegrityCloseoutDenial::MissingEvidenceFamily(
                    family,
                ));
            }
        }
        require_harnesses_match_evidence(self)?;
        require_localization(self)?;
        require_boundary_denials(self)?;
        require_synthetic_rejections(self)?;
        require_recovery_handoff(self)?;
        require_line_cap_composition(self)
    }
}

fn require_harnesses_match_evidence(
    suite: &PhysicalIntegrityCloseoutSuite,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    for evidence in &suite.evidence {
        if evidence.harness.acceptance_suite() != evidence.acceptance_suite
            || evidence.harness.executed_output().acceptance_suite() != evidence.acceptance_suite
            || evidence.harness.executed_output().output_kind()
                != required_output_kind(evidence.acceptance_suite)
            || IntegrityCloseoutEvidenceFamily::from(evidence.harness.acceptance_suite())
                != evidence.evidence_family
        {
            return Err(PhysicalIntegrityCloseoutDenial::MismatchedHarnessSuite(
                evidence.acceptance_suite,
            ));
        }
    }
    Ok(())
}

const fn required_output_kind(
    suite: PhysicalIntegrityAcceptanceSuite,
) -> IntegrityCloseoutExecutedOutputKind {
    match suite {
        PhysicalIntegrityAcceptanceSuite::CorruptionLocalization => {
            IntegrityCloseoutExecutedOutputKind::CorruptionLocalization
        }
        PhysicalIntegrityAcceptanceSuite::BoundaryDenial => {
            IntegrityCloseoutExecutedOutputKind::BoundaryDenial
        }
        PhysicalIntegrityAcceptanceSuite::HarnessTranscript => {
            IntegrityCloseoutExecutedOutputKind::HarnessTranscript
        }
        PhysicalIntegrityAcceptanceSuite::SyntheticShortcutRejection => {
            IntegrityCloseoutExecutedOutputKind::SyntheticShortcutRejection
        }
        PhysicalIntegrityAcceptanceSuite::RecoveryIntegrityHandoff => {
            IntegrityCloseoutExecutedOutputKind::RecoveryIntegrityHandoff
        }
        PhysicalIntegrityAcceptanceSuite::LineCapComposition => {
            IntegrityCloseoutExecutedOutputKind::LineCapComposition
        }
    }
}

fn require_exactly_one_suite(
    suite: &PhysicalIntegrityCloseoutSuite,
    kind: PhysicalIntegrityAcceptanceSuite,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    let count = suite
        .evidence
        .iter()
        .filter(|evidence| evidence.acceptance_suite == kind)
        .count();
    match count {
        0 => Err(PhysicalIntegrityCloseoutDenial::MissingAcceptanceSuite(
            kind,
        )),
        1 => Ok(()),
        _ => Err(PhysicalIntegrityCloseoutDenial::DuplicateAcceptanceSuite(
            kind,
        )),
    }
}

fn require_localization(
    suite: &PhysicalIntegrityCloseoutSuite,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    let evidence = suite
        .evidence_for(PhysicalIntegrityAcceptanceSuite::CorruptionLocalization)
        .expect("suite presence checked before localization");
    for boundary in CorruptionLocalizationBoundary::ALL {
        if !evidence
            .localized_boundaries
            .iter()
            .any(|row| row.boundary() == boundary)
        {
            return Err(PhysicalIntegrityCloseoutDenial::MissingCorruptionLocalization);
        }
    }
    Ok(())
}

fn require_boundary_denials(
    suite: &PhysicalIntegrityCloseoutSuite,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    let evidence = suite
        .evidence_for(PhysicalIntegrityAcceptanceSuite::BoundaryDenial)
        .expect("suite presence checked before boundary denials");
    for boundary in IntegrityCloseoutDenialBoundary::ALL {
        if !evidence
            .denied_boundaries
            .iter()
            .any(|row| row.boundary() == boundary)
        {
            return Err(PhysicalIntegrityCloseoutDenial::MissingBoundaryDenial(
                boundary,
            ));
        }
    }
    Ok(())
}

fn require_synthetic_rejections(
    suite: &PhysicalIntegrityCloseoutSuite,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    let evidence = suite
        .evidence_for(PhysicalIntegrityAcceptanceSuite::SyntheticShortcutRejection)
        .expect("suite presence checked before synthetic denials");
    for attempt in required_synthetic_attempts() {
        let Some(report) = evidence
            .synthetic_rejections
            .iter()
            .find(|report| report.rejected_attempt() == attempt)
        else {
            return Err(PhysicalIntegrityCloseoutDenial::MissingSyntheticRejection(
                attempt,
            ));
        };
        if report.rejected_boundary() != attempt.required_boundary() {
            return Err(PhysicalIntegrityCloseoutDenial::MissingSyntheticRejection(
                attempt,
            ));
        }
        if report.transcript_identity() != evidence.harness.transcript_identity() {
            return Err(
                PhysicalIntegrityCloseoutDenial::SyntheticRejectionTranscriptMismatch(attempt),
            );
        }
    }
    Ok(())
}

fn require_recovery_handoff(
    suite: &PhysicalIntegrityCloseoutSuite,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    let evidence = suite
        .evidence_for(PhysicalIntegrityAcceptanceSuite::RecoveryIntegrityHandoff)
        .expect("suite presence checked before recovery handoff");
    if evidence.recovery_handoff.is_some() {
        Ok(())
    } else {
        Err(PhysicalIntegrityCloseoutDenial::MissingRecoveryHandoffPayload)
    }
}

fn require_line_cap_composition(
    suite: &PhysicalIntegrityCloseoutSuite,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    let evidence = suite
        .evidence_for(PhysicalIntegrityAcceptanceSuite::LineCapComposition)
        .expect("suite presence checked before line-cap proof");
    let Some(line_cap) = evidence.line_cap_composition.as_ref() else {
        return Err(PhysicalIntegrityCloseoutDenial::MissingLineCapComposition);
    };
    if line_cap.owned_files().is_empty() {
        Err(PhysicalIntegrityCloseoutDenial::MissingOwnedCloseoutFile)
    } else {
        Ok(())
    }
}
