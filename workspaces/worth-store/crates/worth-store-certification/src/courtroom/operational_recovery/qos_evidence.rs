use sha2::{Digest, Sha256};
use worth_store_budgets::CounterEvidenceStrength;
use worth_store_io_scheduler::{
    BackgroundInterferenceEvidence, InterferenceCounterName, LatencyEnvelopeAssessmentStatus,
};

use crate::courtroom::scheduling::S6LatencyInterferenceEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S10OperationalQosDenial {
    EnvelopeDidNotHold(LatencyEnvelopeAssessmentStatus),
    BackgroundEnvelopeDidNotHold(LatencyEnvelopeAssessmentStatus),
    MissingExactCounter(InterferenceCounterName),
    MissingSampledGauge(InterferenceCounterName),
}

#[derive(Debug, Clone)]
pub struct S10OperationalQosEvidence {
    source: S6LatencyInterferenceEvidence,
    background: BackgroundInterferenceEvidence,
    evidence_identity: [u8; 32],
}

impl S10OperationalQosEvidence {
    pub fn from_interference_evidence(
        source: S6LatencyInterferenceEvidence,
        background: BackgroundInterferenceEvidence,
    ) -> Result<Self, S10OperationalQosDenial> {
        if source.status() != LatencyEnvelopeAssessmentStatus::Held {
            return Err(S10OperationalQosDenial::EnvelopeDidNotHold(source.status()));
        }
        if background.status() != LatencyEnvelopeAssessmentStatus::Held {
            return Err(S10OperationalQosDenial::BackgroundEnvelopeDidNotHold(
                background.status(),
            ));
        }
        for required in required_exact_queue_counters() {
            let exact = source.rows().iter().any(|row| {
                row.name() == required && row.strength() == CounterEvidenceStrength::Exact
            });
            if !exact {
                return Err(S10OperationalQosDenial::MissingExactCounter(required));
            }
        }
        let sampled_peak = source.rows().iter().any(|row| {
            row.name() == InterferenceCounterName::QueuePeakDepth
                && row.strength() == CounterEvidenceStrength::Sampled
        });
        if !sampled_peak {
            return Err(S10OperationalQosDenial::MissingSampledGauge(
                InterferenceCounterName::QueuePeakDepth,
            ));
        }
        let exact_yield = background.counter_rows().iter().any(|row| {
            row.name() == InterferenceCounterName::BackgroundYieldEvents
                && row.strength() == CounterEvidenceStrength::Exact
        });
        if !exact_yield {
            return Err(S10OperationalQosDenial::MissingExactCounter(
                InterferenceCounterName::BackgroundYieldEvents,
            ));
        }
        let mut digest = Sha256::new();
        digest.update(b"worth-store-s10-operational-qos-evidence-v1");
        for row in source.rows() {
            digest.update(row.name().as_str().as_bytes());
            digest.update(row.value().to_be_bytes());
            digest.update([row.strength() as u8]);
            digest.update(row.profile_scope().as_bytes());
        }
        for row in background.counter_rows() {
            digest.update(row.name().as_str().as_bytes());
            digest.update(row.value().to_be_bytes());
            digest.update([row.strength() as u8]);
            digest.update(row.profile_scope().as_bytes());
        }
        Ok(Self {
            source,
            background,
            evidence_identity: digest.finalize().into(),
        })
    }

    pub const fn source(&self) -> &S6LatencyInterferenceEvidence {
        &self.source
    }

    pub const fn background(&self) -> &BackgroundInterferenceEvidence {
        &self.background
    }

    pub const fn evidence_identity(&self) -> [u8; 32] {
        self.evidence_identity
    }
}

const fn required_exact_queue_counters() -> [InterferenceCounterName; 3] {
    [
        InterferenceCounterName::QueueBackpressureEvents,
        InterferenceCounterName::QueueForegroundWaitEvents,
        InterferenceCounterName::QueueViolationEvents,
    ]
}

#[cfg(test)]
mod tests {
    use worth_store_io_scheduler::{
        admit_background_pacing, foreground_reservation::ForegroundIoLaneKind,
        verification_deferred_background_capacity_for_certification_test,
        BackgroundIdleCapacityLeaseRequest, BackgroundInterferenceEvidence,
        BackgroundIoPressureClass, BackgroundResourceBudget, InterferenceCounterRow, QueueSlot,
        QueueWorkClass,
    };

    use super::*;

    #[test]
    fn operational_qos_rejects_a_bag_missing_required_scheduler_counters() {
        let source = S6LatencyInterferenceEvidence::from_rows_for_test(
            LatencyEnvelopeAssessmentStatus::Held,
            vec![sampled_peak()],
        );
        assert!(matches!(
            S10OperationalQosEvidence::from_interference_evidence(source, background()),
            Err(S10OperationalQosDenial::MissingExactCounter(_))
        ));
    }

    #[test]
    fn operational_qos_accepts_the_exact_scheduler_counter_family() {
        let source = S6LatencyInterferenceEvidence::from_rows_for_test(
            LatencyEnvelopeAssessmentStatus::Held,
            required_exact_queue_counters()
                .into_iter()
                .map(row)
                .chain([sampled_peak()])
                .collect(),
        );
        let evidence =
            S10OperationalQosEvidence::from_interference_evidence(source, background()).unwrap();
        assert_ne!(evidence.evidence_identity(), [0; 32]);
    }

    fn row(name: InterferenceCounterName) -> InterferenceCounterRow {
        InterferenceCounterRow::new(
            name,
            0,
            CounterEvidenceStrength::Exact,
            "s10-test-profile",
            QueueWorkClass::Foreground(ForegroundIoLaneKind::PointRead),
            None,
        )
    }

    fn sampled_peak() -> InterferenceCounterRow {
        InterferenceCounterRow::new(
            InterferenceCounterName::QueuePeakDepth,
            1,
            CounterEvidenceStrength::Sampled,
            "s10-test-profile",
            QueueWorkClass::Foreground(ForegroundIoLaneKind::PointRead),
            None,
        )
    }

    fn background() -> BackgroundInterferenceEvidence {
        let requested =
            BackgroundResourceBudget::new().with_queue_slots(QueueSlot::new(1).unwrap());
        let capacity = verification_deferred_background_capacity_for_certification_test(requested);
        let outcome = admit_background_pacing(BackgroundIdleCapacityLeaseRequest::new(capacity));
        BackgroundInterferenceEvidence::from_background_pacing_outcome(
            "s10-test-profile",
            QueueWorkClass::Background(BackgroundIoPressureClass::VerificationPressure),
            outcome,
        )
    }
}
