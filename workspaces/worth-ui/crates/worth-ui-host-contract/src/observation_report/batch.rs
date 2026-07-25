use super::{
    UiHostObservationCoalescingIdentity, UiHostObservationFamily, UiHostObservationReport,
    UiHostObservationSequenceRange,
};

pub const UI_HOST_OBSERVATION_BATCH_REPORT_LIMIT: usize = 256;
pub const UI_HOST_OBSERVATION_BATCH_BYTE_LIMIT: usize = 64 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostObservationLoss {
    Complete,
    Coalesced {
        family: UiHostObservationFamily,
        replaced: UiHostObservationSequenceRange,
        survivor: UiHostObservationCoalescingIdentity,
    },
    Overflow {
        family: UiHostObservationFamily,
        affected: UiHostObservationSequenceRange,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostObservationCanonicalCore {
    protocol: crate::UiHostProtocolAgreement,
    host_session: u64,
    binding: crate::UiSurfaceBindingGeneration,
    frame: crate::UiMountedFrameIdentity,
    sequences: UiHostObservationSequenceRange,
    report_count: usize,
    byte_count: usize,
    loss: UiHostObservationLoss,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostObservationCanonicalCoreInput {
    pub protocol: crate::UiHostProtocolAgreement,
    pub host_session: u64,
    pub binding: crate::UiSurfaceBindingGeneration,
    pub frame: crate::UiMountedFrameIdentity,
    pub sequences: UiHostObservationSequenceRange,
    pub report_count: usize,
    pub byte_count: usize,
    pub loss: UiHostObservationLoss,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostObservationBatchConstructionDenial {
    Empty,
    SequenceRangeUnordered,
    ReportSequenceOutsideRange,
    ReportSequenceNotStrictlyIncreasing,
    ReportCountExceeded,
    ByteCountExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiHostObservationBatch {
    core: UiHostObservationCanonicalCore,
    reports: Box<[UiHostObservationReport]>,
    integrity: super::UiHostObservationIntegrity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiHostObservationBatchInput {
    pub protocol: crate::UiHostProtocolAgreement,
    pub host_session: u64,
    pub binding: crate::UiSurfaceBindingGeneration,
    pub frame: crate::UiMountedFrameIdentity,
    pub sequences: UiHostObservationSequenceRange,
    pub loss: UiHostObservationLoss,
    pub reports: Vec<UiHostObservationReport>,
}

impl UiHostObservationCanonicalCore {
    pub const fn from_untrusted(input: UiHostObservationCanonicalCoreInput) -> Self {
        Self {
            protocol: input.protocol,
            host_session: input.host_session,
            binding: input.binding,
            frame: input.frame,
            sequences: input.sequences,
            report_count: input.report_count,
            byte_count: input.byte_count,
            loss: input.loss,
        }
    }

    pub const fn protocol(self) -> crate::UiHostProtocolAgreement {
        self.protocol
    }

    pub const fn host_session(self) -> u64 {
        self.host_session
    }

    pub const fn binding(self) -> crate::UiSurfaceBindingGeneration {
        self.binding
    }

    pub const fn frame(self) -> crate::UiMountedFrameIdentity {
        self.frame
    }

    pub const fn sequences(self) -> UiHostObservationSequenceRange {
        self.sequences
    }

    pub const fn report_count(self) -> usize {
        self.report_count
    }

    pub const fn byte_count(self) -> usize {
        self.byte_count
    }

    pub const fn loss(self) -> UiHostObservationLoss {
        self.loss
    }
}

impl UiHostObservationBatch {
    pub fn new(
        input: UiHostObservationBatchInput,
    ) -> Result<Self, UiHostObservationBatchConstructionDenial> {
        let byte_count = input
            .reports
            .iter()
            .map(UiHostObservationReport::encoded_len)
            .sum();
        let core =
            UiHostObservationCanonicalCore::from_untrusted(UiHostObservationCanonicalCoreInput {
                protocol: input.protocol,
                host_session: input.host_session,
                binding: input.binding,
                frame: input.frame,
                sequences: input.sequences,
                report_count: input.reports.len(),
                byte_count,
                loss: input.loss,
            });
        validate_shape(core, &input.reports)?;
        let integrity = super::UiHostObservationIntegrity::derive(core, &input.reports);
        Ok(Self {
            core,
            reports: input.reports.into_boxed_slice(),
            integrity,
        })
    }

    pub fn from_untrusted_parts(
        core: UiHostObservationCanonicalCore,
        reports: Vec<UiHostObservationReport>,
        integrity: super::UiHostObservationIntegrity,
    ) -> Self {
        Self {
            core,
            reports: reports.into_boxed_slice(),
            integrity,
        }
    }

    pub const fn canonical_core(&self) -> UiHostObservationCanonicalCore {
        self.core
    }

    pub fn reports(&self) -> &[UiHostObservationReport] {
        &self.reports
    }

    pub const fn integrity(&self) -> super::UiHostObservationIntegrity {
        self.integrity
    }

    pub fn validate_shape(&self) -> Result<(), UiHostObservationBatchConstructionDenial> {
        validate_shape(self.core, &self.reports)
    }
}

fn validate_shape(
    core: UiHostObservationCanonicalCore,
    reports: &[UiHostObservationReport],
) -> Result<(), UiHostObservationBatchConstructionDenial> {
    let complete_overflow = matches!(
        core.loss,
        UiHostObservationLoss::Overflow { affected, .. } if affected == core.sequences
    );
    if reports.is_empty() && !complete_overflow {
        return Err(UiHostObservationBatchConstructionDenial::Empty);
    }
    if reports.len() > UI_HOST_OBSERVATION_BATCH_REPORT_LIMIT {
        return Err(UiHostObservationBatchConstructionDenial::ReportCountExceeded);
    }
    if !core.sequences.is_ordered() {
        return Err(UiHostObservationBatchConstructionDenial::SequenceRangeUnordered);
    }
    if core.report_count != reports.len() {
        return Err(UiHostObservationBatchConstructionDenial::ReportCountExceeded);
    }
    let byte_count = reports
        .iter()
        .map(UiHostObservationReport::encoded_len)
        .sum::<usize>();
    if byte_count != core.byte_count || byte_count > UI_HOST_OBSERVATION_BATCH_BYTE_LIMIT {
        return Err(UiHostObservationBatchConstructionDenial::ByteCountExceeded);
    }
    if reports
        .iter()
        .any(|report| !core.sequences.contains(report.sequence()))
    {
        return Err(UiHostObservationBatchConstructionDenial::ReportSequenceOutsideRange);
    }
    if reports
        .windows(2)
        .any(|pair| pair[0].sequence() >= pair[1].sequence())
    {
        return Err(UiHostObservationBatchConstructionDenial::ReportSequenceNotStrictlyIncreasing);
    }
    Ok(())
}
