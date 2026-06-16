use std::collections::{BTreeMap, BTreeSet};

use super::carrier_set::PlanarBooleanSplitSourceEdgeCarrierSet;
use super::counters::PlanarBooleanSplitSourceEdgeCarrierCounters;
use super::denial::{
    PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial,
    PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind,
};
use super::input::PlanarBooleanSplitSourceEdgeCarrierRecoveryInput;
use super::recovered_carrier::PlanarBooleanSplitSourceEdgeCarrier;
use super::validation::{denial, validate_carrier_provenance, validate_recovery_input};

pub(crate) fn recover_source_edge_carriers(
    input: PlanarBooleanSplitSourceEdgeCarrierRecoveryInput<'_>,
) -> Result<PlanarBooleanSplitSourceEdgeCarrierSet, PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial>
{
    validate_recovery_input(&input)?;
    let scope = input.scope_admission();
    let ledger = input.event_ledger();
    let mut duplicate_carrier_references_collapsed = 0;
    let mut carriers_by_identity = BTreeMap::new();

    for carrier in ledger.segment_carriers() {
        validate_carrier_provenance(carrier, ledger.event_ledger_identity())?;
        let recovered = PlanarBooleanSplitSourceEdgeCarrier::from_segment_carrier(
            scope.scope_admission_identity(),
            ledger.event_ledger_identity(),
            carrier,
        );
        if let Some(existing) =
            carriers_by_identity.insert(recovered.carrier_identity().to_string(), recovered.clone())
        {
            if existing != recovered {
                return Err(denial(
                    PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::DuplicateCarrierIdentityWithConflictingSourceBinding,
                    recovered.carrier_identity(),
                    "duplicate split carrier identity has conflicting topology binding",
                ));
            }
            duplicate_carrier_references_collapsed += 1;
        }
    }

    let point_refs = validate_point_references(&carriers_by_identity, input.event_ledger())?;
    let interval_refs = validate_interval_references(&carriers_by_identity, input.event_ledger())?;
    let group_refs = validate_group_references(&carriers_by_identity, input.event_ledger())?;

    let mut carriers = carriers_by_identity.into_values().collect::<Vec<_>>();
    carriers.sort_by(|left, right| {
        left.operand_side()
            .query_key()
            .cmp(right.operand_side().query_key())
            .then_with(|| {
                left.source_edge_identity()
                    .cmp(right.source_edge_identity())
            })
            .then_with(|| left.carrier_identity().cmp(right.carrier_identity()))
            .then_with(|| {
                left.recovered_carrier_identity()
                    .cmp(right.recovered_carrier_identity())
            })
    });
    let distinct_source_edge_count = carriers
        .iter()
        .map(|carrier| {
            format!(
                "{}:{}",
                carrier.operand_side().query_key(),
                carrier.source_edge_identity()
            )
        })
        .collect::<BTreeSet<_>>()
        .len();
    let counters = PlanarBooleanSplitSourceEdgeCarrierCounters::new(
        carriers.len(),
        distinct_source_edge_count,
        point_refs,
        interval_refs,
        group_refs,
        duplicate_carrier_references_collapsed,
        carriers.len(),
    );

    Ok(PlanarBooleanSplitSourceEdgeCarrierSet::new(
        scope.scope_admission_identity().to_string(),
        scope.split_request_identity().to_string(),
        ledger.event_ledger_identity().to_string(),
        ledger.segment_carrier_set_identity().to_string(),
        scope.candidate_index_product_identity().to_string(),
        scope.query_index_plan_digest().to_string(),
        carriers,
        counters,
    ))
}

fn validate_point_references(
    carriers: &BTreeMap<String, PlanarBooleanSplitSourceEdgeCarrier>,
    ledger: &crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerReceipt,
) -> Result<usize, PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial> {
    let mut inspected = 0;
    for event in ledger.point_events() {
        for carrier_identity in event.participating_carrier_identities() {
            inspected += 1;
            require_carrier(
                carriers,
                carrier_identity,
                event.event_identity(),
                PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::UnknownPointEventCarrierReference,
            )?;
        }
    }
    Ok(inspected)
}

fn validate_interval_references(
    carriers: &BTreeMap<String, PlanarBooleanSplitSourceEdgeCarrier>,
    ledger: &crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerReceipt,
) -> Result<usize, PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial> {
    let mut inspected = 0;
    for event in ledger.interval_events() {
        for carrier_identity in [
            event.left_carrier_identity(),
            event.right_carrier_identity(),
        ] {
            inspected += 1;
            require_carrier(
                carriers,
                carrier_identity,
                event.event_identity(),
                PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::UnknownIntervalEventCarrierReference,
            )?;
        }
    }
    Ok(inspected)
}

fn validate_group_references(
    carriers: &BTreeMap<String, PlanarBooleanSplitSourceEdgeCarrier>,
    ledger: &crate::workload_platform::planar_boolean_events::PlanarBooleanEventLedgerReceipt,
) -> Result<usize, PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial> {
    let mut inspected = 0;
    for group in ledger.event_groups() {
        for carrier_identity in group.participating_carrier_identities() {
            inspected += 1;
            require_carrier(
                carriers,
                carrier_identity,
                group.group_identity(),
                PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::UnknownGroupedCarrierReference,
            )?;
        }
    }
    Ok(inspected)
}

fn require_carrier(
    carriers: &BTreeMap<String, PlanarBooleanSplitSourceEdgeCarrier>,
    carrier_identity: &str,
    evidence_identity: &str,
    kind: PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind,
) -> Result<(), PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial> {
    if carriers.contains_key(carrier_identity) {
        Ok(())
    } else {
        Err(denial(
            kind,
            evidence_identity,
            format!(
                "event references carrier `{carrier_identity}` outside recovered split carriers"
            ),
        ))
    }
}
