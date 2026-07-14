use crate::{
    BackgroundEnvelopeEvidenceBundle, BoundedMemoryOperationKind, BoundedMemoryResidencySuite,
    BoundedOperationEnvelopeCounters, CompletedResidencyBoundaryReceipt, LargeStorePressureClass,
    LargeStorePressureEvidenceBundle, ProtectedIntegrityViewEvidence, RoadmapLaneFamily,
    S2AcceptanceSuiteKind, SyntheticCloseoutShortcutAttempt,
    SyntheticCloseoutShortcutRejectionReport,
};
use worth_store_buffer_pool::{AllocationScope, BackgroundWorkClass};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BufferPoolCertificationBundle {
    suite: BoundedMemoryResidencySuite,
    pressure: Vec<LargeStorePressureEvidenceBundle>,
    background: BackgroundEnvelopeEvidenceBundle,
    foundational: CompletedResidencyBoundaryReceipt,
    protected_view: ProtectedIntegrityViewEvidence,
    synthetic_rejections: Vec<SyntheticCloseoutShortcutRejectionReport>,
}

impl BufferPoolCertificationBundle {
    pub fn admit(
        suite: BoundedMemoryResidencySuite,
        pressure: Vec<LargeStorePressureEvidenceBundle>,
        background: BackgroundEnvelopeEvidenceBundle,
        foundational: CompletedResidencyBoundaryReceipt,
        protected_view: ProtectedIntegrityViewEvidence,
        synthetic_rejections: Vec<SyntheticCloseoutShortcutRejectionReport>,
    ) -> Result<Self, BufferPoolCertificationBundleDenial> {
        require_all_pressure_classes(&pressure)?;
        require_buffer_pool_pressure_lanes(&pressure)?;
        require_harness_covers_pressure_classes(suite.harness_evidence(), &pressure)?;
        require_harness_covers_acceptance_suites(suite.harness_evidence())?;
        require_background_classes(&background)?;
        require_foundational_execution(&foundational)?;
        require_protected_view_evidence(protected_view, &foundational)?;
        require_suite_matches_executed_evidence(&suite, &background, &foundational)?;
        require_synthetic_rejections(&synthetic_rejections)?;
        Ok(Self {
            suite,
            pressure,
            background,
            foundational,
            protected_view,
            synthetic_rejections,
        })
    }

    pub const fn suite(&self) -> &BoundedMemoryResidencySuite {
        &self.suite
    }

    pub fn pressure(&self) -> &[LargeStorePressureEvidenceBundle] {
        &self.pressure
    }

    pub const fn background(&self) -> &BackgroundEnvelopeEvidenceBundle {
        &self.background
    }

    pub const fn foundational(&self) -> &CompletedResidencyBoundaryReceipt {
        &self.foundational
    }

    pub const fn protected_view(&self) -> ProtectedIntegrityViewEvidence {
        self.protected_view
    }

    pub fn synthetic_rejections(&self) -> &[SyntheticCloseoutShortcutRejectionReport] {
        &self.synthetic_rejections
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferPoolCertificationBundleDenial {
    MissingPressureClass(LargeStorePressureClass),
    PressureNotBufferPoolHarness(LargeStorePressureClass),
    MissingHarnessPressureClass(LargeStorePressureClass),
    MissingHarnessAcceptanceSuite(S2AcceptanceSuiteKind),
    MissingBackgroundClass(BackgroundWorkClass),
    MissingExecutedResidentCounters,
    MissingExecutedAllocationCounters,
    MissingExecutedCopyCounters,
    ProtectedViewEvidenceMismatch,
    OperationEnvelopeMismatch(BoundedMemoryOperationKind),
    MissingSyntheticRejection(SyntheticCloseoutShortcutAttempt),
}

fn require_all_pressure_classes(
    pressure: &[LargeStorePressureEvidenceBundle],
) -> Result<(), BufferPoolCertificationBundleDenial> {
    for class in LargeStorePressureClass::ALL {
        if !pressure
            .iter()
            .any(|bundle| bundle.pressure_class() == class)
        {
            return Err(BufferPoolCertificationBundleDenial::MissingPressureClass(
                class,
            ));
        }
    }
    Ok(())
}

fn require_buffer_pool_pressure_lanes(
    pressure: &[LargeStorePressureEvidenceBundle],
) -> Result<(), BufferPoolCertificationBundleDenial> {
    for bundle in pressure {
        if bundle.transcript_identity().plan_identity().lane_family()
            != RoadmapLaneFamily::BufferPool
        {
            return Err(
                BufferPoolCertificationBundleDenial::PressureNotBufferPoolHarness(
                    bundle.pressure_class(),
                ),
            );
        }
    }
    Ok(())
}

fn require_harness_covers_pressure_classes(
    harness: &crate::HarnessCloseoutEvidenceReport,
    pressure: &[LargeStorePressureEvidenceBundle],
) -> Result<(), BufferPoolCertificationBundleDenial> {
    for bundle in pressure {
        if !harness.contains_pressure_class(bundle.pressure_class()) {
            return Err(
                BufferPoolCertificationBundleDenial::MissingHarnessPressureClass(
                    bundle.pressure_class(),
                ),
            );
        }
    }
    Ok(())
}

fn require_harness_covers_acceptance_suites(
    harness: &crate::HarnessCloseoutEvidenceReport,
) -> Result<(), BufferPoolCertificationBundleDenial> {
    for suite in S2AcceptanceSuiteKind::ALL {
        let Some(transcript) = harness.transcript_for_acceptance_suite(suite) else {
            return Err(BufferPoolCertificationBundleDenial::MissingHarnessAcceptanceSuite(suite));
        };
        if !transcript.names_required_families() {
            return Err(BufferPoolCertificationBundleDenial::MissingHarnessAcceptanceSuite(suite));
        }
    }
    Ok(())
}

fn require_background_classes(
    background: &BackgroundEnvelopeEvidenceBundle,
) -> Result<(), BufferPoolCertificationBundleDenial> {
    let admitted = background.admitted_classes();
    for class in BackgroundWorkClass::ALL {
        if !admitted.contains(&class) {
            return Err(BufferPoolCertificationBundleDenial::MissingBackgroundClass(
                class,
            ));
        }
    }
    Ok(())
}

fn require_foundational_execution(
    receipt: &CompletedResidencyBoundaryReceipt,
) -> Result<(), BufferPoolCertificationBundleDenial> {
    if receipt
        .resident_memory()
        .counters()
        .resident_bytes()
        .as_bytes()
        == 0
    {
        return Err(BufferPoolCertificationBundleDenial::MissingExecutedResidentCounters);
    }
    if allocation_bytes(receipt) == 0 {
        return Err(BufferPoolCertificationBundleDenial::MissingExecutedAllocationCounters);
    }
    if receipt
        .copy_materialization()
        .counters()
        .zero_copy_admission_count()
        == 0
    {
        return Err(BufferPoolCertificationBundleDenial::MissingExecutedCopyCounters);
    }
    Ok(())
}

fn require_synthetic_rejections(
    reports: &[SyntheticCloseoutShortcutRejectionReport],
) -> Result<(), BufferPoolCertificationBundleDenial> {
    for attempt in required_synthetic_attempts() {
        if !reports.iter().any(|report| {
            report.rejected_attempt() == attempt
                && report.rejected_boundary() == attempt.required_boundary()
        }) {
            return Err(BufferPoolCertificationBundleDenial::MissingSyntheticRejection(attempt));
        }
    }
    Ok(())
}

fn require_protected_view_evidence(
    evidence: ProtectedIntegrityViewEvidence,
    receipt: &CompletedResidencyBoundaryReceipt,
) -> Result<(), BufferPoolCertificationBundleDenial> {
    let resident = receipt.resident_memory().counters();
    let copy = receipt.copy_materialization().counters();
    if evidence.resident_bytes() != resident.resident_bytes().as_bytes()
        || evidence.pinned_pages() != resident.pin_lifecycle().successful_pin_count()
        || u64::from(evidence.protected_view_count()) != copy.zero_copy_admission_count()
    {
        return Err(BufferPoolCertificationBundleDenial::ProtectedViewEvidenceMismatch);
    }
    Ok(())
}

fn require_suite_matches_executed_evidence(
    suite: &BoundedMemoryResidencySuite,
    background: &BackgroundEnvelopeEvidenceBundle,
    foundational: &CompletedResidencyBoundaryReceipt,
) -> Result<(), BufferPoolCertificationBundleDenial> {
    let foreground = expected_foreground_operation_counters(foundational);
    require_operation_counters(suite, BoundedMemoryOperationKind::AdmittedRead, foreground)?;
    require_operation_counters(suite, BoundedMemoryOperationKind::AdmittedWrite, foreground)?;
    require_background_operation_counters(
        suite,
        background,
        BoundedMemoryOperationKind::RecoveryPlanning,
        BackgroundWorkClass::RecoveryPlanning,
    )?;
    require_background_operation_counters(
        suite,
        background,
        BoundedMemoryOperationKind::CompactionPlanning,
        BackgroundWorkClass::CompactionPlanning,
    )?;
    require_background_operation_counters(
        suite,
        background,
        BoundedMemoryOperationKind::LargeRecordStreaming,
        BackgroundWorkClass::LargeRecordStreaming,
    )
}

fn require_background_operation_counters(
    suite: &BoundedMemoryResidencySuite,
    background: &BackgroundEnvelopeEvidenceBundle,
    operation: BoundedMemoryOperationKind,
    work_class: BackgroundWorkClass,
) -> Result<(), BufferPoolCertificationBundleDenial> {
    let evidence = background.envelope_for(work_class).ok_or(
        BufferPoolCertificationBundleDenial::MissingBackgroundClass(work_class),
    )?;
    let counters = evidence.counters();
    require_operation_counters(
        suite,
        operation,
        BoundedOperationEnvelopeCounters::exact(
            counters.resident_bytes_admitted(),
            counters.pinned_pages_admitted() as u64,
            0,
            counters.allocation_bytes_allocated(),
            counters.copied_bytes(),
            0,
        ),
    )
}

fn require_operation_counters(
    suite: &BoundedMemoryResidencySuite,
    operation: BoundedMemoryOperationKind,
    expected: BoundedOperationEnvelopeCounters,
) -> Result<(), BufferPoolCertificationBundleDenial> {
    let Some(report) = suite.report_for(operation) else {
        return Err(BufferPoolCertificationBundleDenial::OperationEnvelopeMismatch(operation));
    };
    if report.counters() == expected {
        Ok(())
    } else {
        Err(BufferPoolCertificationBundleDenial::OperationEnvelopeMismatch(operation))
    }
}

fn expected_foreground_operation_counters(
    receipt: &CompletedResidencyBoundaryReceipt,
) -> BoundedOperationEnvelopeCounters {
    let resident = receipt.resident_memory().counters();
    let copy = receipt.copy_materialization().counters();
    BoundedOperationEnvelopeCounters::exact(
        resident.resident_bytes().as_bytes(),
        resident
            .pin_lifecycle()
            .successful_pin_count()
            .max(resident.pin_lifecycle().active_pinned_pages()),
        resident.dirty_state().dirty_pages().as_pages(),
        receipt
            .allocation()
            .counters()
            .scope(AllocationScope::Foreground)
            .allocated_bytes(),
        copy.copied_bytes(),
        copy.materialized_bytes(),
    )
}

fn required_synthetic_attempts() -> [SyntheticCloseoutShortcutAttempt; 4] {
    [
        SyntheticCloseoutShortcutAttempt::LogsOnlyProof,
        SyntheticCloseoutShortcutAttempt::SameRunSelfComparison,
        SyntheticCloseoutShortcutAttempt::SmallFixtureOnly,
        SyntheticCloseoutShortcutAttempt::TestSupportOwnedOracleMeaning,
    ]
}

pub(crate) fn allocation_bytes(receipt: &CompletedResidencyBoundaryReceipt) -> u64 {
    AllocationScope::ALL
        .into_iter()
        .map(|scope| {
            receipt
                .allocation()
                .counters()
                .scope(scope)
                .allocated_bytes()
        })
        .sum()
}
