use super::{S8AdmittedLayoutStrategy, S8LayoutStrategyFamily, S8StrategyCounterProfile};
use crate::access_shape::{S8AccessShapeDetail, S8PrefixBasis, S8RangeBasis};
use crate::budget::S8PlannedCounterEnvelope;
use crate::execution::S8AccessPathCounterSnapshot;

impl S8AdmittedLayoutStrategy {
    pub const fn declared_counter_profile(&self) -> S8StrategyCounterProfile {
        match self.planned_counter_envelope() {
            Some(envelope) => envelope.aggregate_profile(),
            None => self
                .invariants
                .suite()
                .counter_evidence()
                .aggregate_profile(),
        }
    }

    pub const fn planned_counter_envelope(&self) -> Option<S8PlannedCounterEnvelope> {
        if family_requires_shape_specific_lookup_envelope(self.declaration.family()) {
            None
        } else {
            self.declaration.planned_counter_envelope()
        }
    }

    pub const fn planned_counter_envelope_for(
        &self,
        detail: S8AccessShapeDetail,
    ) -> Option<S8PlannedCounterEnvelope> {
        planned_counter_envelope_for(self.declaration.family(), detail)
    }
}

pub(crate) const fn planned_counter_envelope_for(
    family: S8LayoutStrategyFamily,
    detail: S8AccessShapeDetail,
) -> Option<S8PlannedCounterEnvelope> {
    match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => match detail {
            S8AccessShapeDetail::PointLookup => Some(baseline_btree_point_counter_envelope()),
            S8AccessShapeDetail::RangeLookup(S8RangeBasis::CanonicalRangeBounds) => {
                Some(baseline_btree_range_counter_envelope())
            }
            S8AccessShapeDetail::PrefixLookup(S8PrefixBasis::CanonicalPrefixBounds) => {
                Some(baseline_btree_prefix_counter_envelope())
            }
            _ => None,
        },
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => match detail {
            S8AccessShapeDetail::PointLookup => declared_strategy_counter_envelope(family),
            _ => None,
        },
        _ => None,
    }
}

pub(super) const fn declared_strategy_counter_envelope(
    family: S8LayoutStrategyFamily,
) -> Option<S8PlannedCounterEnvelope> {
    match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => None,
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => Some(S8PlannedCounterEnvelope::new(
            S8AccessPathCounterSnapshot::exact(
                1, 1, 0, 0, 0, 2, 2, 2, 1, 0, 0, 0, 8_192, 0, 0, 2, 0,
            ),
            S8AccessPathCounterSnapshot::exact(
                0, 0, 0, 2, 2, 4, 0, 0, 0, 0, 0, 4, 16_384, 8_192, 2, 4, 2,
            ),
            S8AccessPathCounterSnapshot::exact(
                0, 0, 1, 0, 1, 2, 0, 0, 0, 0, 0, 1, 8_192, 0, 0, 2, 0,
            ),
        )),
        _ => None,
    }
}

pub(super) const fn planned_publication_counter_snapshot_for(
    family: S8LayoutStrategyFamily,
) -> S8AccessPathCounterSnapshot {
    match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => baseline_btree_publication_snapshot(),
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => {
            declared_strategy_counter_envelope(family)
                .expect("LSM strategy declares planned counter envelope")
                .publication()
        }
        _ => zero_counter_snapshot(),
    }
}

pub(super) const fn planned_recovery_counter_snapshot_for(
    family: S8LayoutStrategyFamily,
) -> S8AccessPathCounterSnapshot {
    match family {
        S8LayoutStrategyFamily::BaselineBTreeRange => baseline_btree_recovery_snapshot(),
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => {
            declared_strategy_counter_envelope(family)
                .expect("LSM strategy declares planned counter envelope")
                .recovery()
        }
        _ => zero_counter_snapshot(),
    }
}

pub(super) const fn family_requires_shape_specific_lookup_envelope(
    family: S8LayoutStrategyFamily,
) -> bool {
    matches!(family, S8LayoutStrategyFamily::BaselineBTreeRange)
}

const fn baseline_btree_point_counter_envelope() -> S8PlannedCounterEnvelope {
    S8PlannedCounterEnvelope::new(
        S8AccessPathCounterSnapshot::exact(1, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 0, 8_192, 0, 0, 2, 0),
        baseline_btree_publication_snapshot(),
        baseline_btree_recovery_snapshot(),
    )
}

const fn baseline_btree_range_counter_envelope() -> S8PlannedCounterEnvelope {
    S8PlannedCounterEnvelope::new(
        S8AccessPathCounterSnapshot::exact(0, 1, 0, 0, 0, 2, 2, 2, 1, 0, 0, 0, 8_192, 0, 0, 2, 0),
        baseline_btree_publication_snapshot(),
        baseline_btree_recovery_snapshot(),
    )
}

const fn baseline_btree_prefix_counter_envelope() -> S8PlannedCounterEnvelope {
    S8PlannedCounterEnvelope::new(
        S8AccessPathCounterSnapshot::exact(0, 1, 0, 0, 0, 2, 2, 2, 0, 1, 0, 0, 8_192, 0, 0, 2, 0),
        baseline_btree_publication_snapshot(),
        baseline_btree_recovery_snapshot(),
    )
}

pub(super) const fn baseline_btree_publication_snapshot() -> S8AccessPathCounterSnapshot {
    S8AccessPathCounterSnapshot::exact(0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 4_096, 4_096, 1, 1, 1)
}

pub(super) const fn baseline_btree_recovery_snapshot() -> S8AccessPathCounterSnapshot {
    S8AccessPathCounterSnapshot::exact(0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 4_096, 0, 0, 1, 0)
}

const fn zero_counter_snapshot() -> S8AccessPathCounterSnapshot {
    S8AccessPathCounterSnapshot::exact(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
}
