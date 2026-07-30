use super::UiHostObservationBatch;
use std::collections::{BTreeSet, VecDeque};
use std::sync::Mutex;

pub const UI_HOST_OBSERVATION_DRAIN_BATCH_LIMIT: usize = 16;
pub const UI_HOST_OBSERVATION_DRAIN_REPORT_LIMIT: usize = 256;
pub const UI_HOST_OBSERVATION_DRAIN_BYTE_LIMIT: usize = 64 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostObservationDrainDenial {
    BatchCapacityExceeded,
    ReportCapacityExceeded,
    ByteCapacityExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostObservationRetentionDenial {
    ReleasedSession,
    Capacity(UiHostObservationDrainDenial),
}

/// Adapter-issued, mechanically bounded raw observation transfer.
///
/// Construction checks storage represented by the actual reports rather than
/// trusting the batch's unvalidated canonical byte and report claims.
pub struct UiHostObservationDrain {
    batches: Box<[UiHostObservationBatch]>,
}

/// Standard adapter-owned bounded retention for raw host reports.
///
/// The adapter owns this value. Runtime and application callers can request a
/// drain only through the adapter contract and never receive a cloneable queue.
pub struct UiHostObservationRetention {
    state: Mutex<UiHostObservationRetentionState>,
}

struct UiHostObservationRetentionState {
    released_sessions: BTreeSet<u64>,
    batches: VecDeque<UiHostObservationBatch>,
    reports: usize,
    bytes: usize,
}

impl UiHostObservationDrain {
    pub fn bounded(
        batches: Vec<UiHostObservationBatch>,
    ) -> Result<Self, UiHostObservationDrainDenial> {
        if batches.len() > UI_HOST_OBSERVATION_DRAIN_BATCH_LIMIT {
            return Err(UiHostObservationDrainDenial::BatchCapacityExceeded);
        }
        let (reports, bytes) = measure_batches(&batches)?;
        if reports > UI_HOST_OBSERVATION_DRAIN_REPORT_LIMIT {
            return Err(UiHostObservationDrainDenial::ReportCapacityExceeded);
        }
        if bytes > UI_HOST_OBSERVATION_DRAIN_BYTE_LIMIT {
            return Err(UiHostObservationDrainDenial::ByteCapacityExceeded);
        }
        Ok(Self {
            batches: batches.into_boxed_slice(),
        })
    }

    pub fn empty() -> Self {
        Self {
            batches: Box::new([]),
        }
    }

    pub fn into_batches(self) -> Box<[UiHostObservationBatch]> {
        self.batches
    }
}

impl Default for UiHostObservationRetention {
    fn default() -> Self {
        Self {
            state: Mutex::new(UiHostObservationRetentionState {
                released_sessions: BTreeSet::new(),
                batches: VecDeque::new(),
                reports: 0,
                bytes: 0,
            }),
        }
    }
}

impl UiHostObservationRetention {
    pub fn retain(
        &self,
        batch: UiHostObservationBatch,
    ) -> Result<(), UiHostObservationRetentionDenial> {
        let (reports, bytes) = measure_batches(std::slice::from_ref(&batch))
            .map_err(UiHostObservationRetentionDenial::Capacity)?;
        let mut state = self
            .state
            .lock()
            .expect("host observation retention poisoned");
        if state
            .released_sessions
            .contains(&batch.canonical_core().host_session())
        {
            return Err(UiHostObservationRetentionDenial::ReleasedSession);
        }
        if state.batches.len() == UI_HOST_OBSERVATION_DRAIN_BATCH_LIMIT {
            return Err(UiHostObservationRetentionDenial::Capacity(
                UiHostObservationDrainDenial::BatchCapacityExceeded,
            ));
        }
        let next_reports = state.reports.checked_add(reports).ok_or(
            UiHostObservationRetentionDenial::Capacity(
                UiHostObservationDrainDenial::ReportCapacityExceeded,
            ),
        )?;
        let next_bytes =
            state
                .bytes
                .checked_add(bytes)
                .ok_or(UiHostObservationRetentionDenial::Capacity(
                    UiHostObservationDrainDenial::ByteCapacityExceeded,
                ))?;
        if next_reports > UI_HOST_OBSERVATION_DRAIN_REPORT_LIMIT {
            return Err(UiHostObservationRetentionDenial::Capacity(
                UiHostObservationDrainDenial::ReportCapacityExceeded,
            ));
        }
        if next_bytes > UI_HOST_OBSERVATION_DRAIN_BYTE_LIMIT {
            return Err(UiHostObservationRetentionDenial::Capacity(
                UiHostObservationDrainDenial::ByteCapacityExceeded,
            ));
        }
        state.batches.push_back(batch);
        state.reports = next_reports;
        state.bytes = next_bytes;
        Ok(())
    }

    pub fn pending_batch_count(&self) -> usize {
        self.state
            .lock()
            .expect("host observation retention poisoned")
            .batches
            .len()
    }

    pub fn pending_batch_count_for(&self, host_session_identity: u64) -> usize {
        self.state
            .lock()
            .expect("host observation retention poisoned")
            .batches
            .iter()
            .filter(|batch| batch.canonical_core().host_session() == host_session_identity)
            .count()
    }

    pub fn drain(&self, host_session_identity: u64) -> UiHostObservationDrain {
        let mut state = self
            .state
            .lock()
            .expect("host observation retention poisoned");
        let batches = take_session_batches(&mut state, host_session_identity);
        UiHostObservationDrain {
            batches: batches.into_boxed_slice(),
        }
    }

    pub fn release_session(&self, host_session_identity: u64) {
        let mut state = self
            .state
            .lock()
            .expect("host observation retention poisoned");
        take_session_batches(&mut state, host_session_identity);
        state.released_sessions.insert(host_session_identity);
    }
}

fn take_session_batches(
    state: &mut UiHostObservationRetentionState,
    host_session_identity: u64,
) -> Vec<UiHostObservationBatch> {
    let mut retained = VecDeque::with_capacity(state.batches.len());
    let mut drained = Vec::new();
    while let Some(batch) = state.batches.pop_front() {
        if batch.canonical_core().host_session() == host_session_identity {
            let (reports, bytes) = measure_batches(std::slice::from_ref(&batch))
                .expect("retained host observation remains mechanically measurable");
            state.reports -= reports;
            state.bytes -= bytes;
            drained.push(batch);
        } else {
            retained.push_back(batch);
        }
    }
    state.batches = retained;
    drained
}

fn measure_batches(
    batches: &[UiHostObservationBatch],
) -> Result<(usize, usize), UiHostObservationDrainDenial> {
    let mut reports = 0usize;
    let mut bytes = 0usize;
    for batch in batches {
        reports = reports
            .checked_add(batch.reports().len())
            .ok_or(UiHostObservationDrainDenial::ReportCapacityExceeded)?;
        bytes = batch.reports().iter().try_fold(bytes, |total, report| {
            total
                .checked_add(report.encoded_len())
                .ok_or(UiHostObservationDrainDenial::ByteCapacityExceeded)
        })?;
    }
    Ok((reports, bytes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        UiHostObservationBatchInput, UiHostObservationLoss, UiHostObservationPayload,
        UiHostObservationReport, UiHostObservationSequence, UiHostObservationSequenceRange,
        UiHostObservationTimeBasis, UiHostProtocolContract, UiHostProtocolNegotiation,
        UiMountedFrameIdentity, UiSurfaceBindingGeneration,
    };

    #[test]
    fn adapter_retention_is_bounded_drained_and_terminalized_per_session() {
        let retention = UiHostObservationRetention::default();
        for sequence in 1..=UI_HOST_OBSERVATION_DRAIN_BATCH_LIMIT {
            retention
                .retain(batch(1, sequence as u64, "x"))
                .expect("canonical adapter retention capacity");
        }
        assert_eq!(
            retention.retain(batch(1, 17, "x")),
            Err(UiHostObservationRetentionDenial::Capacity(
                UiHostObservationDrainDenial::BatchCapacityExceeded
            ))
        );
        assert_eq!(retention.drain(1).into_batches().len(), 16);
        assert_eq!(retention.pending_batch_count(), 0);
        retention.release_session(1);
        assert_eq!(
            retention.retain(batch(1, 18, "x")),
            Err(UiHostObservationRetentionDenial::ReleasedSession)
        );
        retention
            .retain(batch(2, 1, "x"))
            .expect("a distinct host session remains usable");
        assert_eq!(retention.pending_batch_count_for(2), 1);
        assert_eq!(retention.drain(2).into_batches().len(), 1);
    }

    #[test]
    fn drain_bounds_measure_actual_reports_not_untrusted_core_claims() {
        let report = report(1, "x".repeat(UI_HOST_OBSERVATION_DRAIN_BYTE_LIMIT + 1));
        let baseline = batch(1, 1, "x");
        let core = baseline.canonical_core();
        let forged = UiHostObservationBatch::from_untrusted_parts(
            crate::UiHostObservationCanonicalCore::from_untrusted(
                crate::UiHostObservationCanonicalCoreInput {
                    protocol: core.protocol(),
                    host_session: core.host_session(),
                    binding: core.binding(),
                    frame: core.frame(),
                    sequences: core.sequences(),
                    report_count: core.report_count(),
                    byte_count: 0,
                    loss: core.loss(),
                },
            ),
            vec![report],
            baseline.integrity(),
        );
        assert!(matches!(
            UiHostObservationDrain::bounded(vec![forged]),
            Err(UiHostObservationDrainDenial::ByteCapacityExceeded)
        ));
    }

    fn batch(host_session: u64, sequence: u64, text: &str) -> UiHostObservationBatch {
        batch_from_reports(host_session, sequence, vec![report(sequence, text.into())])
    }

    fn batch_from_reports(
        host_session: u64,
        sequence: u64,
        reports: Vec<UiHostObservationReport>,
    ) -> UiHostObservationBatch {
        let protocol = match UiHostProtocolContract::current().negotiate() {
            UiHostProtocolNegotiation::Compatible(agreement) => agreement,
            UiHostProtocolNegotiation::Incompatible(_) => unreachable!(),
        };
        UiHostObservationBatch::new(UiHostObservationBatchInput {
            protocol,
            host_session,
            binding: UiSurfaceBindingGeneration::mint_unbound().unwrap(),
            frame: UiMountedFrameIdentity::mint_unbound().unwrap(),
            sequences: UiHostObservationSequenceRange::new(
                UiHostObservationSequence::new(sequence),
                UiHostObservationSequence::new(sequence),
            ),
            loss: UiHostObservationLoss::Complete,
            reports,
        })
        .expect("focused drain fixture is structurally valid")
    }

    fn report(sequence: u64, text: String) -> UiHostObservationReport {
        UiHostObservationReport::new(
            UiHostObservationSequence::new(sequence),
            UiHostObservationTimeBasis::HostMonotonicTick(sequence),
            UiHostObservationPayload::TextInput {
                revision: sequence,
                text: text.into_boxed_str(),
            },
        )
    }
}
