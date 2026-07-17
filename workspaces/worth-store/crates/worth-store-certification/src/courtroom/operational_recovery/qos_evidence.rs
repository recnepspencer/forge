use sha2::{Digest, Sha256};
use worth_store_budgets::CounterEvidenceStrength;
use worth_store_io_scheduler::{InterferenceCounterName, LatencyEnvelopeAssessmentStatus};

use crate::courtroom::scheduling::S6LatencyInterferenceEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S10OperationalQosDenial {
    EnvelopeDidNotHold(LatencyEnvelopeAssessmentStatus),
    MissingExactCounter(InterferenceCounterName),
}

#[derive(Debug, Clone)]
pub struct S10OperationalQosEvidence {
    source: S6LatencyInterferenceEvidence,
    evidence_identity: [u8; 32],
}

impl S10OperationalQosEvidence {
    pub fn from_interference_evidence(
        source: S6LatencyInterferenceEvidence,
    ) -> Result<Self, S10OperationalQosDenial> {
        if source.status() != LatencyEnvelopeAssessmentStatus::Held {
            return Err(S10OperationalQosDenial::EnvelopeDidNotHold(source.status()));
        }
        for required in required_exact_counters() {
            let exact = source.rows().iter().any(|row| {
                row.name() == required && row.strength() == CounterEvidenceStrength::Exact
            });
            if !exact {
                return Err(S10OperationalQosDenial::MissingExactCounter(required));
            }
        }
        let mut digest = Sha256::new();
        digest.update(b"worth-store-s10-operational-qos-evidence-v1");
        for row in source.rows() {
            digest.update(row.name().as_str().as_bytes());
            digest.update(row.value().to_be_bytes());
            digest.update([row.strength() as u8]);
            digest.update(row.profile_scope().as_bytes());
        }
        Ok(Self {
            source,
            evidence_identity: digest.finalize().into(),
        })
    }

    pub const fn source(&self) -> &S6LatencyInterferenceEvidence {
        &self.source
    }

    pub const fn evidence_identity(&self) -> [u8; 32] {
        self.evidence_identity
    }
}

const fn required_exact_counters() -> [InterferenceCounterName; 5] {
    [
        InterferenceCounterName::QueuePeakDepth,
        InterferenceCounterName::QueueBackpressureEvents,
        InterferenceCounterName::QueueForegroundWaitEvents,
        InterferenceCounterName::QueueViolationEvents,
        InterferenceCounterName::BackgroundYieldEvents,
    ]
}

#[cfg(test)]
mod tests {
    use worth_store_io_scheduler::{
        foreground_reservation::ForegroundIoLaneKind, InterferenceCounterRow, QueueWorkClass,
    };

    use super::*;

    #[test]
    fn operational_qos_rejects_a_bag_missing_required_scheduler_counters() {
        let source = S6LatencyInterferenceEvidence::from_rows_for_test(
            LatencyEnvelopeAssessmentStatus::Held,
            vec![row(InterferenceCounterName::QueuePeakDepth)],
        );
        assert!(matches!(
            S10OperationalQosEvidence::from_interference_evidence(source),
            Err(S10OperationalQosDenial::MissingExactCounter(_))
        ));
    }

    #[test]
    fn operational_qos_accepts_the_exact_scheduler_counter_family() {
        let source = S6LatencyInterferenceEvidence::from_rows_for_test(
            LatencyEnvelopeAssessmentStatus::Held,
            required_exact_counters().into_iter().map(row).collect(),
        );
        let evidence = S10OperationalQosEvidence::from_interference_evidence(source).unwrap();
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
}
