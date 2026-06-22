use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitSourceEdgeCarrierSet;
use crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerReceipt;

use super::carrier_event_index::PlanarBooleanSplitEventParticipationIndex;
use super::carrier_event_row::PlanarBooleanSplitEventParticipationRow;
use super::counters::PlanarBooleanSplitEventParticipationCounters;
use super::denial::{
    PlanarBooleanSplitEventParticipationDenial, PlanarBooleanSplitEventParticipationDenialKind,
};
use super::identity::participation_index_identity;

pub(crate) fn build_participation_index(
    recovered_carriers: &PlanarBooleanSplitSourceEdgeCarrierSet,
    ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<PlanarBooleanSplitEventParticipationIndex, PlanarBooleanSplitEventParticipationDenial> {
    require_matching_recovered_carrier_authority(recovered_carriers, ledger)?;
    require_matching_segment_carrier_set_authority(recovered_carriers, ledger)?;
    require_recovered_carrier_rows(recovered_carriers, ledger)?;
    let mut rows = carrier_rows(recovered_carriers);
    attach_point_event_references(&mut rows, ledger)?;
    attach_interval_event_references(&mut rows, ledger)?;
    attach_validated_event_group_references(&mut rows, ledger)?;
    let indexed = canonical_index_rows(rows, ledger.event_ledger_identity());
    let counters = participation_counters(
        ledger,
        indexed.rows.len(),
        indexed.duplicate_references_collapsed,
    );
    let index_identity = participation_index_identity(
        ledger.event_ledger_identity(),
        recovered_carriers.carrier_set_identity(),
        &indexed.rows,
    );
    Ok(PlanarBooleanSplitEventParticipationIndex::new(
        index_identity,
        ledger.event_ledger_identity().to_string(),
        recovered_carriers.carrier_set_identity().to_string(),
        indexed.rows,
        ledger
            .point_events()
            .iter()
            .map(|event| (event.event_identity().to_string(), event.clone()))
            .collect(),
        ledger
            .interval_events()
            .iter()
            .map(|event| (event.event_identity().to_string(), event.clone()))
            .collect(),
        counters,
    ))
}

fn require_matching_recovered_carrier_authority(
    recovered_carriers: &PlanarBooleanSplitSourceEdgeCarrierSet,
    ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanSplitEventParticipationDenial> {
    if recovered_carriers.event_ledger_identity() == ledger.event_ledger_identity() {
        Ok(())
    } else {
        Err(denial(
            PlanarBooleanSplitEventParticipationDenialKind::MissingCarrierRows,
            ledger.event_ledger_identity(),
            "split event participation indexing requires recovered carriers for the same event ledger",
        ))
    }
}

fn require_matching_segment_carrier_set_authority(
    recovered_carriers: &PlanarBooleanSplitSourceEdgeCarrierSet,
    ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanSplitEventParticipationDenial> {
    if recovered_carriers.segment_carrier_set_identity() == ledger.segment_carrier_set_identity() {
        Ok(())
    } else {
        Err(denial(
            PlanarBooleanSplitEventParticipationDenialKind::CarrierSetIdentityMismatch,
            ledger.segment_carrier_set_identity(),
            "split event participation indexing requires recovered carriers for the ledger carrier set",
        ))
    }
}

fn require_recovered_carrier_rows(
    recovered_carriers: &PlanarBooleanSplitSourceEdgeCarrierSet,
    ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanSplitEventParticipationDenial> {
    if recovered_carriers.carriers().is_empty() {
        Err(denial(
            PlanarBooleanSplitEventParticipationDenialKind::MissingCarrierRows,
            ledger.event_ledger_identity(),
            "split event participation indexing requires recovered split carrier rows",
        ))
    } else {
        Ok(())
    }
}

struct RowBuild {
    carrier_identity: String,
    source_edge_identity: String,
    start_source_endpoint_identity: String,
    start_projected_endpoint_fact_identity: String,
    end_source_endpoint_identity: String,
    end_projected_endpoint_fact_identity: String,
    point_event_identities: Vec<String>,
    interval_event_identities: Vec<String>,
    event_group_identities: Vec<String>,
}

impl RowBuild {
    fn into_index_row(
        self,
        event_ledger_identity: &str,
    ) -> PlanarBooleanSplitEventParticipationRow {
        PlanarBooleanSplitEventParticipationRow::new(
            event_ledger_identity,
            self.carrier_identity,
            self.source_edge_identity,
            self.start_source_endpoint_identity,
            self.start_projected_endpoint_fact_identity,
            self.end_source_endpoint_identity,
            self.end_projected_endpoint_fact_identity,
            self.point_event_identities,
            self.interval_event_identities,
            self.event_group_identities,
        )
    }
}

struct CanonicalIndexRows {
    rows: Vec<PlanarBooleanSplitEventParticipationRow>,
    duplicate_references_collapsed: usize,
}

fn attach_point_event_references(
    rows: &mut BTreeMap<String, RowBuild>,
    ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanSplitEventParticipationDenial> {
    for point in ledger.point_events() {
        for carrier in point.participating_carrier_identities() {
            require_row(rows, carrier, ledger.event_ledger_identity())?
                .point_event_identities
                .push(point.event_identity().to_string());
        }
    }
    Ok(())
}

fn attach_interval_event_references(
    rows: &mut BTreeMap<String, RowBuild>,
    ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanSplitEventParticipationDenial> {
    for interval in ledger.interval_events() {
        for carrier in [
            interval.left_carrier_identity(),
            interval.right_carrier_identity(),
        ] {
            require_row(rows, carrier, ledger.event_ledger_identity())?
                .interval_event_identities
                .push(interval.event_identity().to_string());
        }
    }
    Ok(())
}

fn attach_validated_event_group_references(
    rows: &mut BTreeMap<String, RowBuild>,
    ledger: &PlanarBooleanEventLedgerReceipt,
) -> Result<(), PlanarBooleanSplitEventParticipationDenial> {
    let point_ids = ledger_point_event_identities(ledger);
    let interval_ids = ledger_interval_event_identities(ledger);
    for group in ledger.event_groups() {
        require_grouped_point_events_in_ledger(
            group.point_event_identities(),
            &point_ids,
            group.group_identity(),
        )?;
        require_grouped_interval_events_in_ledger(
            group.interval_event_identities(),
            &interval_ids,
            group.group_identity(),
        )?;
        for carrier in group.participating_carrier_identities() {
            require_row(rows, carrier, ledger.event_ledger_identity())?
                .event_group_identities
                .push(group.group_identity().to_string());
        }
    }
    Ok(())
}

fn ledger_point_event_identities(ledger: &PlanarBooleanEventLedgerReceipt) -> BTreeSet<String> {
    ledger
        .point_events()
        .iter()
        .map(|event| event.event_identity().to_string())
        .collect()
}

fn ledger_interval_event_identities(ledger: &PlanarBooleanEventLedgerReceipt) -> BTreeSet<String> {
    ledger
        .interval_events()
        .iter()
        .map(|event| event.event_identity().to_string())
        .collect()
}

fn require_grouped_point_events_in_ledger(
    point_event_identities: &[String],
    point_ids: &BTreeSet<String>,
    group_identity: &str,
) -> Result<(), PlanarBooleanSplitEventParticipationDenial> {
    for point_id in point_event_identities {
        if !point_ids.contains(point_id) {
            return Err(orphan_reference_denial(
                PlanarBooleanSplitEventParticipationDenialKind::UnknownGroupedPointEvent,
                group_identity,
                "event group references a point event outside the event ledger",
            ));
        }
    }
    Ok(())
}

fn require_grouped_interval_events_in_ledger(
    interval_event_identities: &[String],
    interval_ids: &BTreeSet<String>,
    group_identity: &str,
) -> Result<(), PlanarBooleanSplitEventParticipationDenial> {
    for interval_id in interval_event_identities {
        if !interval_ids.contains(interval_id) {
            return Err(orphan_reference_denial(
                PlanarBooleanSplitEventParticipationDenialKind::UnknownGroupedIntervalEvent,
                group_identity,
                "event group references an interval event outside the event ledger",
            ));
        }
    }
    Ok(())
}

fn canonical_index_rows(
    rows: BTreeMap<String, RowBuild>,
    event_ledger_identity: &str,
) -> CanonicalIndexRows {
    let mut duplicate_references_collapsed = 0;
    let mut rows = rows
        .into_values()
        .map(|row| {
            let point_before = row.point_event_identities.len();
            let interval_before = row.interval_event_identities.len();
            let group_before = row.event_group_identities.len();
            let row = row.into_index_row(event_ledger_identity);
            duplicate_references_collapsed += point_before - row.point_event_identities().len();
            duplicate_references_collapsed +=
                interval_before - row.interval_event_identities().len();
            duplicate_references_collapsed += group_before - row.event_group_identities().len();
            row
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.carrier_identity()
            .cmp(right.carrier_identity())
            .then_with(|| {
                left.source_edge_identity()
                    .cmp(right.source_edge_identity())
            })
            .then_with(|| {
                left.participation_row_identity()
                    .cmp(right.participation_row_identity())
            })
    });
    CanonicalIndexRows {
        rows,
        duplicate_references_collapsed,
    }
}

fn participation_counters(
    ledger: &PlanarBooleanEventLedgerReceipt,
    indexed_row_count: usize,
    duplicate_references_collapsed: usize,
) -> PlanarBooleanSplitEventParticipationCounters {
    PlanarBooleanSplitEventParticipationCounters::new(
        indexed_row_count,
        ledger
            .point_events()
            .iter()
            .map(|event| event.participating_carrier_identities().len())
            .sum(),
        ledger.interval_events().len().saturating_mul(2),
        ledger
            .event_groups()
            .iter()
            .map(|group| group.participating_carrier_identities().len())
            .sum(),
        0,
        duplicate_references_collapsed,
    )
}

fn carrier_rows(
    recovered_carriers: &PlanarBooleanSplitSourceEdgeCarrierSet,
) -> BTreeMap<String, RowBuild> {
    recovered_carriers
        .carriers()
        .iter()
        .map(|carrier| {
            (
                carrier.carrier_identity().to_string(),
                RowBuild {
                    carrier_identity: carrier.carrier_identity().to_string(),
                    source_edge_identity: carrier.source_edge_identity().to_string(),
                    start_source_endpoint_identity: carrier
                        .start_source_endpoint_identity()
                        .to_string(),
                    start_projected_endpoint_fact_identity: carrier
                        .start_projected_endpoint_fact_identity()
                        .to_string(),
                    end_source_endpoint_identity: carrier
                        .end_source_endpoint_identity()
                        .to_string(),
                    end_projected_endpoint_fact_identity: carrier
                        .end_projected_endpoint_fact_identity()
                        .to_string(),
                    point_event_identities: Vec::new(),
                    interval_event_identities: Vec::new(),
                    event_group_identities: Vec::new(),
                },
            )
        })
        .collect()
}

fn require_row<'a>(
    rows: &'a mut BTreeMap<String, RowBuild>,
    carrier_identity: &str,
    event_ledger_identity: &str,
) -> Result<&'a mut RowBuild, PlanarBooleanSplitEventParticipationDenial> {
    rows.get_mut(carrier_identity).ok_or_else(|| {
        orphan_reference_denial(
            PlanarBooleanSplitEventParticipationDenialKind::UnknownCarrierReference,
            event_ledger_identity,
            "event ledger references a carrier missing from split participation index rows",
        )
    })
}

fn denial(
    kind: PlanarBooleanSplitEventParticipationDenialKind,
    evidence_identity: impl Into<String>,
    human_reason: impl Into<String>,
) -> PlanarBooleanSplitEventParticipationDenial {
    PlanarBooleanSplitEventParticipationDenial::new(kind, evidence_identity, human_reason)
}

fn orphan_reference_denial(
    kind: PlanarBooleanSplitEventParticipationDenialKind,
    evidence_identity: impl Into<String>,
    human_reason: impl Into<String>,
) -> PlanarBooleanSplitEventParticipationDenial {
    PlanarBooleanSplitEventParticipationDenial::with_rejected_orphan_reference(
        kind,
        evidence_identity,
        human_reason,
    )
}
