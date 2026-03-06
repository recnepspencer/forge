//! Domain-free effect routing contract for Tier-0 scheduling.

use crate::data::dirty_set::BatchedDirtySet;

/// Maps domain effects to dirty-domain impacts.
pub trait EffectMapping {
    type Domain: Copy + Ord;
    type Effect;
    type Impact: Copy + Ord;

    /// Route one effect into the batched dirty set.
    fn route(effect: &Self::Effect, sink: &mut BatchedDirtySet<Self::Domain, Self::Impact>);
}
