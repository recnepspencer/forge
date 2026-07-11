use super::counter_evidence::S8StrategyCounterEvidence;
use super::declaration::S8StrategyDeclaration;
use crate::access_shape::{S8AccessShapeDetail, S8PrefixBasis, S8RangeBasis};

pub(super) fn derive_strategy_counter_evidence(
    declaration: S8StrategyDeclaration,
) -> S8StrategyCounterEvidence {
    let point_lookup = super::admission::planned_counter_envelope_for(
        declaration.family(),
        S8AccessShapeDetail::PointLookup,
    );
    let range_lookup = super::admission::planned_counter_envelope_for(
        declaration.family(),
        S8AccessShapeDetail::RangeLookup(S8RangeBasis::CanonicalRangeBounds),
    );
    let prefix_lookup = super::admission::planned_counter_envelope_for(
        declaration.family(),
        S8AccessShapeDetail::PrefixLookup(S8PrefixBasis::CanonicalPrefixBounds),
    );
    let publication =
        super::admission::planned_publication_counter_snapshot_for(declaration.family());
    let recovery = super::admission::planned_recovery_counter_snapshot_for(declaration.family());
    let aggregate = declaration
        .planned_counter_envelope()
        .map(|envelope| envelope.aggregate_profile())
        .or_else(|| {
            derive_shape_specific_aggregate_profile(point_lookup, range_lookup, prefix_lookup)
        })
        .unwrap_or_else(zero_counter_profile);

    S8StrategyCounterEvidence::new(
        point_lookup,
        range_lookup,
        prefix_lookup,
        publication,
        recovery,
        aggregate,
    )
}

const fn derive_shape_specific_aggregate_profile(
    point_lookup: Option<crate::budget::S8PlannedCounterEnvelope>,
    range_lookup: Option<crate::budget::S8PlannedCounterEnvelope>,
    prefix_lookup: Option<crate::budget::S8PlannedCounterEnvelope>,
) -> Option<super::S8StrategyCounterProfile> {
    match (point_lookup, range_lookup, prefix_lookup) {
        (Some(point), Some(range), Some(prefix)) => Some(max_counter_profile(
            point.aggregate_profile(),
            range.aggregate_profile(),
            prefix.aggregate_profile(),
        )),
        _ => None,
    }
}

const fn max_counter_profile(
    point: super::S8StrategyCounterProfile,
    range: super::S8StrategyCounterProfile,
    prefix: super::S8StrategyCounterProfile,
) -> super::S8StrategyCounterProfile {
    super::S8StrategyCounterProfile::new(
        max3(
            point.point_lookups(),
            range.point_lookups(),
            prefix.point_lookups(),
        ),
        max3(
            point.range_lookups(),
            range.range_lookups(),
            prefix.range_lookups(),
        ),
        max3(
            point.wal_replays(),
            range.wal_replays(),
            prefix.wal_replays(),
        ),
        max3(
            point.publications(),
            range.publications(),
            prefix.publications(),
        ),
        max3(
            point.maintenance_reads(),
            range.maintenance_reads(),
            prefix.maintenance_reads(),
        ),
    )
}

const fn max3(first: u16, second: u16, third: u16) -> u16 {
    let first_second = if first > second { first } else { second };
    if first_second > third {
        first_second
    } else {
        third
    }
}

const fn zero_counter_profile() -> super::S8StrategyCounterProfile {
    super::S8StrategyCounterProfile::new(0, 0, 0, 0, 0)
}
