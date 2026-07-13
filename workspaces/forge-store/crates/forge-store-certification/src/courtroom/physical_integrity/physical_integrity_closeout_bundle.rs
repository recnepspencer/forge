use crate::{
    scenario::physical_integrity::physical_integrity_closeout_harness_runner::{
        physical_integrity_closeout_suite_plan_and_transcript,
        run_physical_integrity_closeout_harness,
    },
    PhysicalIntegrityCloseoutDenial, PhysicalIntegrityCloseoutReport,
    PhysicalIntegrityCloseoutSuite, PhysicalIntegrityCloseoutSuiteEvidence, S3AcceptanceSuiteKind,
    S3CloseoutHarnessExecutionEvidence, S3ExecutedBoundaryDenialEvidence,
    S3ExecutedCorruptionLocalizationEvidence, S3LineCapCompositionEvidence,
    S3S4HandoffCloseoutEvidence, SyntheticCloseoutShortcutAttempt, SyntheticCloseoutShortcutInput,
    SyntheticCloseoutShortcutRejectionReport,
};
use forge_store_readiness::PhysicalIntegrityReadiness;
use forge_store_recovery_physics::AdmittedRecoveryIntegrityInput;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PhysicalIntegrityCloseoutRequest {
    suite: PhysicalIntegrityCloseoutSuite,
    physical_integrity_readiness: PhysicalIntegrityReadiness,
    recovery_handoff: AdmittedRecoveryIntegrityInput,
}

impl PhysicalIntegrityCloseoutRequest {
    pub(crate) fn new(
        suite: PhysicalIntegrityCloseoutSuite,
        physical_integrity_readiness: PhysicalIntegrityReadiness,
        recovery_handoff: AdmittedRecoveryIntegrityInput,
    ) -> Self {
        Self {
            suite,
            physical_integrity_readiness,
            recovery_handoff,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PhysicalIntegrityCertificationBundle {
    suite: PhysicalIntegrityCloseoutSuite,
    physical_integrity_readiness: PhysicalIntegrityReadiness,
    recovery_handoff: AdmittedRecoveryIntegrityInput,
    report: PhysicalIntegrityCloseoutReport,
}

impl PhysicalIntegrityCertificationBundle {
    pub(crate) fn close(
        request: PhysicalIntegrityCloseoutRequest,
    ) -> Result<Self, PhysicalIntegrityCloseoutDenial> {
        require_physical_integrity_readiness_scope(&request.physical_integrity_readiness)?;
        require_recovery_handoff(&request.recovery_handoff)?;
        require_recovery_handoff_suite_evidence(&request)?;
        let report = closeout_report(&request);
        Ok(Self {
            suite: request.suite,
            physical_integrity_readiness: request.physical_integrity_readiness,
            recovery_handoff: request.recovery_handoff,
            report,
        })
    }

    pub const fn suite(&self) -> &PhysicalIntegrityCloseoutSuite {
        &self.suite
    }

    pub const fn physical_integrity_readiness(&self) -> &PhysicalIntegrityReadiness {
        &self.physical_integrity_readiness
    }

    pub const fn recovery_handoff(&self) -> &AdmittedRecoveryIntegrityInput {
        &self.recovery_handoff
    }

    pub const fn report(&self) -> &PhysicalIntegrityCloseoutReport {
        &self.report
    }
}

pub fn close_physical_integrity_from_executed_evidence(
    physical_integrity_readiness: PhysicalIntegrityReadiness,
    recovery_handoff: AdmittedRecoveryIntegrityInput,
    localized_boundaries: Vec<S3ExecutedCorruptionLocalizationEvidence>,
    denied_boundaries: Vec<S3ExecutedBoundaryDenialEvidence>,
    line_cap_composition: S3LineCapCompositionEvidence,
) -> Result<PhysicalIntegrityCertificationBundle, PhysicalIntegrityCloseoutDenial> {
    let suite = PhysicalIntegrityCloseoutSuite::admit(executed_closeout_suite_evidence(
        &recovery_handoff,
        localized_boundaries,
        denied_boundaries,
        line_cap_composition,
    )?)?;
    PhysicalIntegrityCertificationBundle::close(PhysicalIntegrityCloseoutRequest::new(
        suite,
        physical_integrity_readiness,
        recovery_handoff,
    ))
}

fn executed_closeout_suite_evidence(
    s4_readiness: &AdmittedRecoveryIntegrityInput,
    localized_boundaries: Vec<S3ExecutedCorruptionLocalizationEvidence>,
    denied_boundaries: Vec<S3ExecutedBoundaryDenialEvidence>,
    line_cap_composition: S3LineCapCompositionEvidence,
) -> Result<Vec<PhysicalIntegrityCloseoutSuiteEvidence>, PhysicalIntegrityCloseoutDenial> {
    let synthetic_rejections = executed_synthetic_shortcut_rejections()?;
    let recovery_handoff = S3S4HandoffCloseoutEvidence::from_readiness(s4_readiness);
    Ok(vec![
        PhysicalIntegrityCloseoutSuiteEvidence::corruption_localization(
            physical_integrity_suite_harness(
                S3AcceptanceSuiteKind::CorruptionLocalization,
                S3CloseoutHarnessExecutionEvidence::corruption_localization(&localized_boundaries),
            )?,
            localized_boundaries,
        ),
        PhysicalIntegrityCloseoutSuiteEvidence::boundary_denial(
            physical_integrity_suite_harness(
                S3AcceptanceSuiteKind::BoundaryDenial,
                S3CloseoutHarnessExecutionEvidence::boundary_denial(&denied_boundaries),
            )?,
            denied_boundaries,
        ),
        PhysicalIntegrityCloseoutSuiteEvidence::harness_transcript(
            physical_integrity_suite_harness(
                S3AcceptanceSuiteKind::HarnessTranscript,
                S3CloseoutHarnessExecutionEvidence::harness_transcript(1),
            )?,
        ),
        PhysicalIntegrityCloseoutSuiteEvidence::synthetic_rejection(
            physical_integrity_suite_harness(
                S3AcceptanceSuiteKind::SyntheticShortcutRejection,
                S3CloseoutHarnessExecutionEvidence::synthetic_rejection(&synthetic_rejections),
            )?,
            synthetic_rejections,
        ),
        PhysicalIntegrityCloseoutSuiteEvidence::recovery_handoff(
            physical_integrity_suite_harness(
                S3AcceptanceSuiteKind::S4IntegrityHandoff,
                S3CloseoutHarnessExecutionEvidence::recovery_handoff(&recovery_handoff),
            )?,
            recovery_handoff,
        ),
        PhysicalIntegrityCloseoutSuiteEvidence::line_cap_composition(
            physical_integrity_suite_harness(
                S3AcceptanceSuiteKind::LineCapComposition,
                S3CloseoutHarnessExecutionEvidence::line_cap_composition(&line_cap_composition),
            )?,
            line_cap_composition,
        ),
    ])
}

fn physical_integrity_suite_harness(
    suite: S3AcceptanceSuiteKind,
    execution: S3CloseoutHarnessExecutionEvidence,
) -> Result<crate::S3HarnessTranscriptEvidence, PhysicalIntegrityCloseoutDenial> {
    Ok(run_physical_integrity_closeout_harness(suite, execution)?
        .harness()
        .clone())
}

fn executed_synthetic_shortcut_rejections(
) -> Result<Vec<SyntheticCloseoutShortcutRejectionReport>, PhysicalIntegrityCloseoutDenial> {
    let (_, transcript) = physical_integrity_closeout_suite_plan_and_transcript(
        S3AcceptanceSuiteKind::SyntheticShortcutRejection,
    )?;
    let mut reports = Vec::new();
    for attempt in required_synthetic_attempts() {
        let input = SyntheticCloseoutShortcutInput::from_transcript(attempt, &transcript);
        let report = match SyntheticCloseoutShortcutRejectionReport::attempt_shortcut_certification(
            input,
            &transcript,
        ) {
            Ok(()) => {
                return Err(PhysicalIntegrityCloseoutDenial::MissingSyntheticRejection(
                    attempt,
                ));
            }
            Err(denial) => {
                SyntheticCloseoutShortcutRejectionReport::from_failed_shortcut_attempt(denial)
            }
        };
        reports.push(report);
    }
    Ok(reports)
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

fn require_physical_integrity_readiness_scope(
    readiness: &PhysicalIntegrityReadiness,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    if readiness.payload().claims_later_sequence_semantics() {
        Err(PhysicalIntegrityCloseoutDenial::S3ReadinessClaimsLaterSequence)
    } else {
        Ok(())
    }
}

fn require_recovery_handoff(
    handoff: &AdmittedRecoveryIntegrityInput,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    let payload = handoff.payload();
    if !payload.proves_no_raw_bytes_crossed() || !handoff.proves_no_raw_bytes_crossed() {
        return Err(PhysicalIntegrityCloseoutDenial::S4HandoffContainsRawBytes);
    }
    if payload.claims_recovery() || handoff.claims_recovery() {
        return Err(PhysicalIntegrityCloseoutDenial::S4HandoffClaimsRecovery);
    }
    if payload.page_frames().is_empty()
        || payload.wal_frames().is_empty()
        || payload.checkpoint_records().is_empty()
    {
        return Err(PhysicalIntegrityCloseoutDenial::MissingS4HandoffPayload);
    }
    Ok(())
}

fn require_recovery_handoff_suite_evidence(
    request: &PhysicalIntegrityCloseoutRequest,
) -> Result<(), PhysicalIntegrityCloseoutDenial> {
    let Some(evidence) = request
        .suite
        .evidence_for(S3AcceptanceSuiteKind::S4IntegrityHandoff)
        .and_then(|suite| suite.recovery_handoff_evidence())
    else {
        return Err(PhysicalIntegrityCloseoutDenial::MissingS4HandoffPayload);
    };
    if evidence.matches_readiness(&request.recovery_handoff) {
        Ok(())
    } else {
        Err(PhysicalIntegrityCloseoutDenial::S4HandoffEvidenceMismatch)
    }
}

fn closeout_report(request: &PhysicalIntegrityCloseoutRequest) -> PhysicalIntegrityCloseoutReport {
    PhysicalIntegrityCloseoutReport::from_closeout(
        &request.suite,
        request.recovery_handoff.payload().identity().clone(),
        request.recovery_handoff.counters(),
        request.recovery_handoff.proves_no_raw_bytes_crossed(),
        request.recovery_handoff.claims_recovery(),
        request
            .physical_integrity_readiness
            .payload()
            .claims_later_sequence_semantics(),
    )
}
