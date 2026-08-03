use worth_store::physical_runtime::{
    ExternalPhysicalRecordLocator, PhysicalRecordId, ServingPhysicalRuntime,
};

use super::super::super::{
    configuration::BoundedResidencyConfiguration,
    read_pressure::{
        media_observation::{metadata_reads, positioned_reads},
        work_accounting::ReadWorkIdentitySpan,
    },
};
use super::record_copy::{self, CopyTotals, VerifiedRecordCopy};

pub(super) struct MeasuredRead {
    pub(super) copy: VerifiedRecordCopy,
    pub(super) positioned_effects: u64,
    pub(super) metadata_effects: u64,
}

pub(super) struct ReadPressureSchedule {
    pub(super) cold: MeasuredRead,
    pub(super) hot: MeasuredRead,
    pub(super) refault: MeasuredRead,
    pub(super) work_span: ReadWorkIdentitySpan,
    pub(super) copies: CopyTotals,
    pub(super) read_work: u64,
}

#[derive(Clone, Copy)]
enum EffectExpectation {
    Required,
    Forbidden,
}

#[derive(Clone, Copy)]
struct ReadCase {
    locator: ExternalPhysicalRecordLocator,
    ordinal: usize,
    label: &'static str,
    expectation: EffectExpectation,
}

#[derive(Default)]
struct ScheduleTotals {
    work_span: ReadWorkIdentitySpan,
    copies: CopyTotals,
    read_work: u64,
}

impl ScheduleTotals {
    fn observe(
        &mut self,
        serving: &ServingPhysicalRuntime,
        read: VerifiedRecordCopy,
    ) -> Result<(), String> {
        self.work_span.observe(serving, read.observation)?;
        self.copies.observe(read);
        self.read_work = self
            .read_work
            .saturating_add(read.observation.physical_work_count());
        Ok(())
    }
}

pub(super) fn execute(
    serving: &ServingPhysicalRuntime,
    records: &[PhysicalRecordId],
    configuration: BoundedResidencyConfiguration,
) -> Result<ReadPressureSchedule, String> {
    let store = serving.store_identity();
    let first = ExternalPhysicalRecordLocator::new(store, records[0]);
    let mut totals = ScheduleTotals::default();

    let cold = measure(
        serving,
        configuration,
        ReadCase {
            locator: first,
            ordinal: 0,
            label: "cold",
            expectation: EffectExpectation::Required,
        },
    )?;
    totals.observe(serving, cold.copy)?;
    let hot = measure(
        serving,
        configuration,
        ReadCase {
            locator: first,
            ordinal: 0,
            label: "hot",
            expectation: EffectExpectation::Forbidden,
        },
    )?;
    totals.observe(serving, hot.copy)?;

    for (ordinal, record) in records.iter().copied().enumerate().skip(1) {
        let locator = ExternalPhysicalRecordLocator::new(store, record);
        let read = record_copy::read(serving, locator, configuration, ordinal)?;
        totals.observe(serving, read)?;
    }

    let refault = measure(
        serving,
        configuration,
        ReadCase {
            locator: first,
            ordinal: 0,
            label: "refault",
            expectation: EffectExpectation::Required,
        },
    )?;
    totals.observe(serving, refault.copy)?;
    Ok(ReadPressureSchedule {
        cold,
        hot,
        refault,
        work_span: totals.work_span,
        copies: totals.copies,
        read_work: totals.read_work,
    })
}

fn measure(
    serving: &ServingPhysicalRuntime,
    configuration: BoundedResidencyConfiguration,
    case: ReadCase,
) -> Result<MeasuredRead, String> {
    let positioned_before = positioned_reads(serving);
    let metadata_before = metadata_reads(serving);
    let copy = record_copy::read(serving, case.locator, configuration, case.ordinal)?;
    let positioned_effects = delta(
        positioned_reads(serving),
        positioned_before,
        &format!("{} positioned reads", case.label),
    )?;
    let metadata_effects = delta(
        metadata_reads(serving),
        metadata_before,
        &format!("{} metadata reads", case.label),
    )?;
    let work = copy.observation.physical_work_count();
    match case.expectation {
        EffectExpectation::Required if positioned_effects == 0 || work == 0 => {
            return Err(format!(
                "C.6 {} read did not enter canonical physical work",
                case.label
            ));
        }
        EffectExpectation::Forbidden
            if positioned_effects != 0 || metadata_effects != 0 || work != 0 =>
        {
            return Err(format!(
                "C.6 {} read repeated physical work or a media effect",
                case.label
            ));
        }
        EffectExpectation::Required | EffectExpectation::Forbidden => {}
    }
    Ok(MeasuredRead {
        copy,
        positioned_effects,
        metadata_effects,
    })
}

fn delta(after: u64, before: u64, label: &str) -> Result<u64, String> {
    after
        .checked_sub(before)
        .ok_or_else(|| format!("C.6 {label} counter regressed"))
}
