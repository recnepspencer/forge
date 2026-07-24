use worth_ui_host_contract::{
    UiHostObservationCoalescingIdentity, UiHostObservationFamily, UiHostObservationReport,
    UiHostObservationSequence, UiHostObservationSequenceRange,
};

use super::basis_admission::UiBasisAdmittedObservationBatch;
use super::progression::validate_sequence_progression;
use super::state::{
    UiHostObservationBatchFingerprint, UiHostObservationPartition, UiRetainedHostObservationReport,
};
use super::{
    UiDuplicateHostObservationBatch, UiHostObservationBatchDisposition,
    UiHostObservationDisposition, UiHostObservationFrameRelation, UiHostObservationReportDenial,
    UiHostObservationReportOutcome, UiHostObservationReportValidation,
    UiValidatedHostObservationBatch, UiValidatedHostObservationReport,
};

impl UiHostObservationReportValidation {
    pub(super) fn retain_covered_batch(
        &mut self,
        admitted: UiBasisAdmittedObservationBatch,
    ) -> Result<UiHostObservationReportOutcome, UiHostObservationReportDenial> {
        let (batch, relation) = admitted.into_parts();
        let core = batch.core();
        let binding = core.binding();
        let mut partition = self
            .partitions
            .get(&binding)
            .cloned()
            .unwrap_or_else(UiHostObservationPartition::empty);
        if partition.duplicate(core, batch.integrity()) {
            return Ok(UiHostObservationReportOutcome::Duplicate(
                UiDuplicateHostObservationBatch::new(core.sequences(), batch.integrity()),
            ));
        }
        validate_sequence_progression(partition.last_sequence, core.sequences())?;
        let previous = UiHostObservationRetentionBasis::from_partition(&partition);
        let validated = retain_batch_reports(
            &mut partition,
            batch.reports(),
            relation,
            host_coalescing_admission(&batch),
        );
        let family = capacity_family(batch.disposition(), batch.reports());
        let admitted = self.admit_retention_capacity(&partition, previous, family)?;
        partition.last_sequence = Some(core.sequences().last());
        partition.remember_batch(UiHostObservationBatchFingerprint {
            sequences: core.sequences(),
            integrity: batch.integrity(),
        });
        self.commit_partition(binding, partition, admitted);
        Ok(UiHostObservationReportOutcome::Validated(
            UiValidatedHostObservationBatch::new(core, relation, batch.disposition(), validated),
        ))
    }

    fn admit_retention_capacity(
        &self,
        partition: &UiHostObservationPartition,
        previous: UiHostObservationRetentionBasis,
        family: UiHostObservationFamily,
    ) -> Result<UiAdmittedHostObservationRetention, UiHostObservationReportDenial> {
        enforce_local_capacity(partition, self.capacity, family)?;
        let global_reports = self.global_reports - previous.report_count + partition.reports.len();
        let global_bytes = self.global_bytes - previous.byte_count + partition.byte_count;
        if global_reports > self.capacity.global_reports()
            || global_bytes > self.capacity.global_bytes()
        {
            return Err(UiHostObservationReportDenial::GlobalCapacityExceeded(
                family,
            ));
        }
        Ok(UiAdmittedHostObservationRetention {
            global_reports,
            global_bytes,
        })
    }

    fn commit_partition(
        &mut self,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
        partition: UiHostObservationPartition,
        admitted: UiAdmittedHostObservationRetention,
    ) {
        self.global_reports = admitted.global_reports;
        self.global_bytes = admitted.global_bytes;
        self.partitions.insert(binding, partition);
    }
}

#[derive(Clone, Copy)]
struct UiHostObservationRetentionBasis {
    report_count: usize,
    byte_count: usize,
}

#[derive(Clone, Copy)]
struct UiAdmittedHostObservationRetention {
    global_reports: usize,
    global_bytes: usize,
}

#[derive(Clone, Copy)]
struct UiHostCoalescingAdmission {
    survivor: UiHostObservationSequence,
    replaced: UiHostObservationSequenceRange,
}

impl UiHostObservationRetentionBasis {
    fn from_partition(partition: &UiHostObservationPartition) -> Self {
        Self {
            report_count: partition.reports.len(),
            byte_count: partition.byte_count,
        }
    }
}

fn capacity_family(
    disposition: UiHostObservationBatchDisposition,
    reports: &[UiHostObservationReport],
) -> UiHostObservationFamily {
    match disposition {
        UiHostObservationBatchDisposition::Complete => reports
            .last()
            .expect("exact complete coverage contains a report")
            .family(),
        UiHostObservationBatchDisposition::Coalesced { family, .. }
        | UiHostObservationBatchDisposition::Overflow { family, .. } => family,
    }
}

fn host_coalescing_admission(
    batch: &super::sequence_coverage::UiSequenceCoveredObservationBatch,
) -> Option<UiHostCoalescingAdmission> {
    match (batch.disposition(), batch.host_survivor()) {
        (UiHostObservationBatchDisposition::Coalesced { replaced, .. }, Some(survivor)) => {
            Some(UiHostCoalescingAdmission { survivor, replaced })
        }
        _ => None,
    }
}

fn retain_batch_reports(
    partition: &mut UiHostObservationPartition,
    reports: &[UiHostObservationReport],
    relation: UiHostObservationFrameRelation,
    host_coalescing: Option<UiHostCoalescingAdmission>,
) -> Vec<UiValidatedHostObservationReport> {
    reports
        .iter()
        .cloned()
        .map(|report| {
            let replaced = host_coalescing
                .filter(|admission| admission.survivor == report.sequence())
                .map(|admission| admission.replaced);
            retain_report(partition, report, relation, replaced)
        })
        .collect()
}

fn retain_report(
    partition: &mut UiHostObservationPartition,
    report: UiHostObservationReport,
    relation: UiHostObservationFrameRelation,
    host_replaced: Option<UiHostObservationSequenceRange>,
) -> UiValidatedHostObservationReport {
    let coalescing_identity = report.payload().coalescing_identity();
    let replaced = partition.reports.back().and_then(|previous| {
        (coalescing_identity.is_some()
            && previous.coalescing_identity == coalescing_identity
            && previous.relation == relation)
            .then(|| previous.replaced_range())
    });
    let disposition = if let Some(replaced) = replaced {
        let previous = partition
            .reports
            .pop_back()
            .expect("coalescing candidate is retained");
        partition.byte_count -= previous.encoded_len;
        let first = host_replaced
            .map(|range| range.first().min(replaced.first()))
            .unwrap_or(replaced.first());
        let last = host_replaced
            .map(|range| range.last().max(previous.report.report().sequence()))
            .unwrap_or(previous.report.report().sequence());
        UiHostObservationDisposition::Coalesced {
            replaced: UiHostObservationSequenceRange::new(first, last),
        }
    } else {
        host_replaced
            .map(|replaced| UiHostObservationDisposition::Coalesced { replaced })
            .unwrap_or(UiHostObservationDisposition::Retained)
    };
    let validated = UiValidatedHostObservationReport::new(report, disposition);
    push_retained(partition, validated.clone(), relation, coalescing_identity);
    validated
}

fn push_retained(
    partition: &mut UiHostObservationPartition,
    report: UiValidatedHostObservationReport,
    relation: UiHostObservationFrameRelation,
    coalescing_identity: Option<UiHostObservationCoalescingIdentity>,
) {
    let encoded_len = report.report().encoded_len();
    partition.byte_count += encoded_len;
    partition
        .reports
        .push_back(UiRetainedHostObservationReport {
            report,
            relation,
            coalescing_identity,
            encoded_len,
        });
}

fn enforce_local_capacity(
    partition: &UiHostObservationPartition,
    capacity: super::UiHostObservationCapacity,
    family: UiHostObservationFamily,
) -> Result<(), UiHostObservationReportDenial> {
    if partition.reports.len() > capacity.local_reports()
        || partition.byte_count > capacity.local_bytes()
    {
        return Err(UiHostObservationReportDenial::LocalCapacityExceeded(family));
    }
    Ok(())
}
