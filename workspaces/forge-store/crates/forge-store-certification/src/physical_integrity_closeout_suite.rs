use crate::{
    PhysicalIntegrityCloseoutDenial, S3AcceptanceSuiteKind, S3CloseoutDenialBoundary,
    S3CloseoutEvidenceFamily, S3CloseoutExecutedOutputKind, S3CorruptionLocalizationBoundary,
    S3ExecutedBoundaryDenialEvidence, S3ExecutedCorruptionLocalizationEvidence,
    S3HarnessTranscriptEvidence, S3LineCapCompositionEvidence, S3S4HandoffCloseoutEvidence,
    SyntheticCloseoutShortcutAttempt, SyntheticCloseoutShortcutRejectionReport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalIntegrityCloseoutSuiteEvidence {
    acceptance_suite: S3AcceptanceSuiteKind,
    evidence_family: S3CloseoutEvidenceFamily,
    harness: S3HarnessTranscriptEvidence,
    localized_boundaries: Vec<S3ExecutedCorruptionLocalizationEvidence>,
    denied_boundaries: Vec<S3ExecutedBoundaryDenialEvidence>,
    synthetic_rejections: Vec<SyntheticCloseoutShortcutRejectionReport>,
    s4_handoff: Option<S3S4HandoffCloseoutEvidence>,
    line_cap_composition: Option<S3LineCapCompositionEvidence>,
}

impl PhysicalIntegrityCloseoutSuiteEvidence {
    pub fn corruption_localization(
        harness: S3HarnessTranscriptEvidence,
        localized_boundaries: Vec<S3ExecutedCorruptionLocalizationEvidence>,
    ) -> Self {
        Self::new(
            S3AcceptanceSuiteKind::CorruptionLocalization,
            S3CloseoutEvidenceFamily::CorruptionLocalization,
            harness,
        )
        .with_localized_boundaries(localized_boundaries)
    }

    pub fn boundary_denial(
        harness: S3HarnessTranscriptEvidence,
        denied_boundaries: Vec<S3ExecutedBoundaryDenialEvidence>,
    ) -> Self {
        Self::new(
            S3AcceptanceSuiteKind::BoundaryDenial,
            S3CloseoutEvidenceFamily::BoundaryDenial,
            harness,
        )
        .with_denied_boundaries(denied_boundaries)
    }

    pub fn harness_transcript(harness: S3HarnessTranscriptEvidence) -> Self {
        Self::new(
            S3AcceptanceSuiteKind::HarnessTranscript,
            S3CloseoutEvidenceFamily::HarnessTranscript,
            harness,
        )
    }

    pub fn synthetic_rejection(
        harness: S3HarnessTranscriptEvidence,
        rejections: Vec<SyntheticCloseoutShortcutRejectionReport>,
    ) -> Self {
        Self::new(
            S3AcceptanceSuiteKind::SyntheticShortcutRejection,
            S3CloseoutEvidenceFamily::SyntheticShortcutRejection,
            harness,
        )
        .with_synthetic_rejections(rejections)
    }

    pub fn s4_handoff(
        harness: S3HarnessTranscriptEvidence,
        handoff: S3S4HandoffCloseoutEvidence,
    ) -> Self {
        Self::new(
            S3AcceptanceSuiteKind::S4IntegrityHandoff,
            S3CloseoutEvidenceFamily::S4IntegrityHandoff,
            harness,
        )
        .with_s4_handoff(handoff)
    }

    pub fn line_cap_composition(
        harness: S3HarnessTranscriptEvidence,
        evidence: S3LineCapCompositionEvidence,
    ) -> Self {
        Self::new(
            S3AcceptanceSuiteKind::LineCapComposition,
            S3CloseoutEvidenceFamily::LineCapComposition,
            harness,
        )
        .with_line_cap_composition(evidence)
    }

    pub const fn acceptance_suite(&self) -> S3AcceptanceSuiteKind {
        self.acceptance_suite
    }

    pub const fn evidence_family(&self) -> S3CloseoutEvidenceFamily {
        self.evidence_family
    }

    pub const fn harness(&self) -> &S3HarnessTranscriptEvidence {
        &self.harness
    }

    pub fn localized_boundaries(&self) -> &[S3ExecutedCorruptionLocalizationEvidence] {
        &self.localized_boundaries
    }

    pub fn denied_boundaries(&self) -> &[S3ExecutedBoundaryDenialEvidence] {
        &self.denied_boundaries
    }

    pub fn synthetic_rejections(&self) -> &[SyntheticCloseoutShortcutRejectionReport] {
        &self.synthetic_rejections
    }

    pub const fn s4_handoff_evidence(&self) -> Option<&S3S4HandoffCloseoutEvidence> {
        self.s4_handoff.as_ref()
    }

    pub const fn line_cap_composition_evidence(&self) -> Option<&S3LineCapCompositionEvidence> {
        self.line_cap_composition.as_ref()
    }

    fn new(
        acceptance_suite: S3AcceptanceSuiteKind,
        evidence_family: S3CloseoutEvidenceFamily,
        harness: S3HarnessTranscriptEvidence,
    ) -> Self {
        Self {
            acceptance_suite,
            evidence_family,
            harness,
            localized_boundaries: Vec::new(),
            denied_boundaries: Vec::new(),
            synthetic_rejections: Vec::new(),
            s4_handoff: None,
            line_cap_composition: None,
        }
    }

    fn with_localized_boundaries(
        mut self,
        boundaries: Vec<S3ExecutedCorruptionLocalizationEvidence>,
    ) -> Self {
        self.localized_boundaries = boundaries;
        self
    }

    fn with_denied_boundaries(mut self, boundaries: Vec<S3ExecutedBoundaryDenialEvidence>) -> Self {
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

    fn with_s4_handoff(mut self, evidence: S3S4HandoffCloseoutEvidence) -> Self {
        self.s4_handoff = Some(evidence);
        self
    }

    fn with_line_cap_composition(mut self, evidence: S3LineCapCompositionEvidence) -> Self {
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
        kind: S3AcceptanceSuiteKind,
    ) -> Option<&PhysicalIntegrityCloseoutSuiteEvidence> {
        self.evidence
            .iter()
            .find(|evidence| evidence.acceptance_suite == kind)
    }

    pub fn contains_evidence_family(&self, family: S3CloseoutEvidenceFamily) -> bool {
        self.evidence
            .iter()
            .any(|evidence| evidence.evidence_family == family)
    }

    fn require_complete(&self) -> Result<(), PhysicalIntegrityCloseoutDenial> {
        for kind in S3AcceptanceSuiteKind::ALL {
            require_exactly_one_suite(self, kind)?;
        }
        for family in S3CloseoutEvidenceFamily::ALL {
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
        require_s4_handoff(self)?;
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
            || S3CloseoutEvidenceFamily::from(evidence.harness.acceptance_suite())
                != evidence.evidence_family
        {
            return Err(PhysicalIntegrityCloseoutDenial::MismatchedHarnessSuite(
                evidence.acceptance_suite,
            ));
        }
    }
    Ok(())
}

const fn required_output_kind(suite: S3AcceptanceSuiteKind) -> S3CloseoutExecutedOutputKind {
    match suite {
        S3AcceptanceSuiteKind::CorruptionLocalization => {
            S3CloseoutExecutedOutputKind::CorruptionLocalization
        }
        S3AcceptanceSuiteKind::BoundaryDenial => S3CloseoutExecutedOutputKind::BoundaryDenial,
        S3AcceptanceSuiteKind::HarnessTranscript => S3CloseoutExecutedOutputKind::HarnessTranscript,
        S3AcceptanceSuiteKind::SyntheticShortcutRejection => {
            S3CloseoutExecutedOutputKind::SyntheticShortcutRejection
        }
        S3AcceptanceSuiteKind::S4IntegrityHandoff => {
            S3CloseoutExecutedOutputKind::S4IntegrityHandoff
        }
        S3AcceptanceSuiteKind::LineCapComposition => {
            S3CloseoutExecutedOutputKind::LineCapComposition
        }
    }
}

fn require_exactly_one_suite(
    suite: &PhysicalIntegrityCloseoutSuite,
    kind: S3AcceptanceSuiteKind,
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
        .evidence_for(S3AcceptanceSuiteKind::CorruptionLocalization)
        .expect("suite presence checked before localization");
    for boundary in S3CorruptionLocalizationBoundary::ALL {
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
        .evidence_for(S3AcceptanceSuiteKind::BoundaryDenial)
        .expect("suite presence checked before boundary denials");
    for boundary in S3CloseoutDenialBoundary::ALL {
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
        .evidence_for(S3AcceptanceSuiteKind::SyntheticShortcutRejection)
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

fn require_s4_handoff(
    suite: &PhysicalIntegrityCloseoutSuite,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    let evidence = suite
        .evidence_for(S3AcceptanceSuiteKind::S4IntegrityHandoff)
        .expect("suite presence checked before S.4 handoff");
    if evidence.s4_handoff.is_some() {
        Ok(())
    } else {
        Err(PhysicalIntegrityCloseoutDenial::MissingS4HandoffPayload)
    }
}

fn require_line_cap_composition(
    suite: &PhysicalIntegrityCloseoutSuite,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    let evidence = suite
        .evidence_for(S3AcceptanceSuiteKind::LineCapComposition)
        .expect("suite presence checked before line-cap proof");
    let Some(line_cap) = evidence.line_cap_composition.as_ref() else {
        return Err(PhysicalIntegrityCloseoutDenial::MissingLineCapComposition);
    };
    if line_cap.owned_files().is_empty() {
        Err(PhysicalIntegrityCloseoutDenial::MissingS3OwnedCloseoutFile)
    } else {
        Ok(())
    }
}

fn required_synthetic_attempts() -> [SyntheticCloseoutShortcutAttempt; 7] {
    [
        SyntheticCloseoutShortcutAttempt::LogsOnlyProof,
        SyntheticCloseoutShortcutAttempt::SameRunSelfComparison,
        SyntheticCloseoutShortcutAttempt::ExpectedErrorsOnly,
        SyntheticCloseoutShortcutAttempt::InMemoryOnlyBuffers,
        SyntheticCloseoutShortcutAttempt::SmallFixtureOnly,
        SyntheticCloseoutShortcutAttempt::FixtureLabelsOnly,
        SyntheticCloseoutShortcutAttempt::TestSupportOwnedOracleMeaning,
    ]
}
