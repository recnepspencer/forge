use super::super::errors::{
    WorthGraphReadAccessAdmissionPostureError, WorthGraphReadAccessAdmissionPostureErrorKind,
};
use super::super::posture_record::WorthGraphReadAdmissionPostureRecord;
use super::super::query_admission_projection::WorthGraphReadAdmissionCapabilityGapKind;
use super::super::stable_identity_digest::stable_digest;
use super::cap_ledger::admission_gap_cap_ledger_row;
use super::gap_family_counter::WorthGraphReadAdmissionGapFamilyCounter;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAdmissionGapCapReport {
    gap_count: usize,
    gap_family_caps: Vec<WorthGraphReadAdmissionGapFamilyCounter>,
    report_digest: String,
}

pub(crate) fn cap_report_from_posture_records(
    records: &[WorthGraphReadAdmissionPostureRecord],
) -> Result<WorthGraphReadAdmissionGapCapReport, WorthGraphReadAccessAdmissionPostureError> {
    let mut gap_kinds = records
        .iter()
        .filter_map(|record| record.posture_outcome().admission_gap())
        .map(|gap| gap.kind())
        .collect::<Vec<_>>();
    gap_kinds.sort_by_key(|kind| kind.as_str());
    gap_kinds.dedup();

    let counters = gap_kinds
        .iter()
        .map(|kind| counter_for_gap_kind(records, *kind))
        .collect::<Vec<_>>();
    if counters.iter().any(|counter| !counter.is_within_cap()) {
        return Err(WorthGraphReadAccessAdmissionPostureError::new(
            WorthGraphReadAccessAdmissionPostureErrorKind::CapabilityGapCapExceeded,
        ));
    }
    let gap_count = records
        .iter()
        .filter(|record| record.posture_outcome().admission_gap().is_some())
        .count();
    let mut digest_parts = vec![
        "worth_graph_read_admission_gap_cap_report_v1".to_string(),
        format!("gap_count:{gap_count}"),
        format!("family_count:{}", counters.len()),
    ];
    digest_parts.extend(
        counters
            .iter()
            .map(WorthGraphReadAdmissionGapFamilyCounter::digest_part),
    );
    Ok(WorthGraphReadAdmissionGapCapReport {
        gap_count,
        gap_family_caps: counters,
        report_digest: stable_digest(&digest_parts),
    })
}

impl WorthGraphReadAdmissionGapCapReport {
    pub const fn gap_count(&self) -> usize {
        self.gap_count
    }

    pub const fn capped_gap_family_count(&self) -> usize {
        self.gap_family_caps.len()
    }

    pub fn gap_family_caps(&self) -> &[WorthGraphReadAdmissionGapFamilyCounter] {
        &self.gap_family_caps
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

fn counter_for_gap_kind(
    records: &[WorthGraphReadAdmissionPostureRecord],
    kind: WorthGraphReadAdmissionCapabilityGapKind,
) -> WorthGraphReadAdmissionGapFamilyCounter {
    let matching_gaps = records
        .iter()
        .filter_map(|record| record.posture_outcome().admission_gap())
        .filter(|gap| gap.kind() == kind)
        .collect::<Vec<_>>();
    let Some(ledger_row) = admission_gap_cap_ledger_row(kind) else {
        return WorthGraphReadAdmissionGapFamilyCounter::new(
            kind,
            matching_gaps.len(),
            0,
            "missing_ledger_row",
        );
    };
    WorthGraphReadAdmissionGapFamilyCounter::new(
        kind,
        matching_gaps.len(),
        ledger_row.must_not_exceed_count(),
        ledger_row.digest_part(),
    )
}
