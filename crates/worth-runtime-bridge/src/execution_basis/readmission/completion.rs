use crate::execution_basis::{BridgeBoundExecutionBasis, BridgeYieldedExecutionBasis};

use super::BridgeExecutionBasisReadmissionCounters;

#[must_use = "returned yielded authority and its owner counters must remain joined"]
pub struct BridgeExecutionBasisReadmissionYielded {
    yielded: BridgeYieldedExecutionBasis,
    counters: BridgeExecutionBasisReadmissionCounters,
}

#[must_use = "committed execution authority and its owner counters must remain joined"]
pub struct BridgeExecutionBasisReadmissionCommitted {
    basis: BridgeBoundExecutionBasis,
    counters: BridgeExecutionBasisReadmissionCounters,
}

impl BridgeExecutionBasisReadmissionYielded {
    pub(super) const fn new(
        yielded: BridgeYieldedExecutionBasis,
        counters: BridgeExecutionBasisReadmissionCounters,
    ) -> Self {
        Self { yielded, counters }
    }

    pub const fn counters(&self) -> BridgeExecutionBasisReadmissionCounters {
        self.counters
    }

    pub fn into_parts(
        self,
    ) -> (
        BridgeYieldedExecutionBasis,
        BridgeExecutionBasisReadmissionCounters,
    ) {
        (self.yielded, self.counters)
    }
}

impl BridgeExecutionBasisReadmissionCommitted {
    pub(super) const fn new(
        basis: BridgeBoundExecutionBasis,
        counters: BridgeExecutionBasisReadmissionCounters,
    ) -> Self {
        Self { basis, counters }
    }

    pub const fn counters(&self) -> BridgeExecutionBasisReadmissionCounters {
        self.counters
    }

    pub fn into_parts(
        self,
    ) -> (
        BridgeBoundExecutionBasis,
        BridgeExecutionBasisReadmissionCounters,
    ) {
        (self.basis, self.counters)
    }
}
