use std::marker::PhantomData;

use crate::assumption::NoAssumptionBasis;
use crate::proof::NoProofs;

/// Canonical proof-bearing carrier for statically known phase progression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artifact<P, T, S = NoProofs, A = NoAssumptionBasis> {
    pub(crate) payload: T,
    pub(crate) proofs: S,
    pub(crate) basis: A,
    pub(crate) phase: PhantomData<P>,
}
