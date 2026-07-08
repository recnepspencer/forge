use crate::maintenance::S8DerivedIndexRebuildDenied;

use super::identity::{
    declared_counter_shape_parity, S8DerivedIndexCostEnvelopeParity,
    S8DerivedIndexCounterShapeParity, S8DerivedIndexCoverageParity, S8DerivedIndexIdentityParity,
    S8DerivedIndexOrderingParity,
};
use super::rebuild::S8DerivedIndexRebuildReceipt;

#[derive(Debug, PartialEq, Eq)]
pub struct S8DerivedIndexParityWitness {
    key_identity: S8DerivedIndexIdentityParity,
    value_identity: S8DerivedIndexIdentityParity,
    ordering_identity: S8DerivedIndexOrderingParity,
    coverage_identity: S8DerivedIndexCoverageParity,
    cost_envelope: S8DerivedIndexCostEnvelopeParity,
    counter_shape: S8DerivedIndexCounterShapeParity,
}

impl S8DerivedIndexParityWitness {
    const fn new(
        value_identity: S8DerivedIndexIdentityParity,
        cost_envelope: S8DerivedIndexCostEnvelopeParity,
        counter_shape: S8DerivedIndexCounterShapeParity,
    ) -> Self {
        Self {
            key_identity: S8DerivedIndexIdentityParity::Exact,
            value_identity,
            ordering_identity: S8DerivedIndexOrderingParity::Exact,
            coverage_identity: S8DerivedIndexCoverageParity::Exact,
            cost_envelope,
            counter_shape,
        }
    }

    pub const fn key_identity(&self) -> S8DerivedIndexIdentityParity {
        self.key_identity
    }

    pub const fn value_identity(&self) -> S8DerivedIndexIdentityParity {
        self.value_identity
    }

    pub const fn ordering_identity(&self) -> S8DerivedIndexOrderingParity {
        self.ordering_identity
    }

    pub const fn coverage_identity(&self) -> S8DerivedIndexCoverageParity {
        self.coverage_identity
    }

    pub const fn cost_envelope(&self) -> S8DerivedIndexCostEnvelopeParity {
        self.cost_envelope
    }

    pub const fn counter_shape(&self) -> S8DerivedIndexCounterShapeParity {
        self.counter_shape
    }

    pub const fn parity_holds(&self) -> bool {
        true
    }
}

pub(crate) fn verify_parity(
    receipt: S8DerivedIndexRebuildReceipt,
) -> Result<S8DerivedIndexParityWitness, S8DerivedIndexRebuildDenied> {
    let plan = receipt.plan();
    let source_authority = plan.source_authority();
    let authoritative = source_authority.parity_basis();
    let rebuilt = receipt.rebuilt_basis();

    if authoritative.coverage() != plan.rebuild_scope().authority_coverage() {
        return Err(S8DerivedIndexRebuildDenied::ParityCoverageMismatch {
            expected: plan.rebuild_scope().authority_coverage(),
            actual: authoritative.coverage(),
        });
    }
    if rebuilt.coverage() != plan.rebuild_scope().authority_coverage() {
        return Err(S8DerivedIndexRebuildDenied::ParityCoverageMismatch {
            expected: plan.rebuild_scope().authority_coverage(),
            actual: rebuilt.coverage(),
        });
    }
    if authoritative.unique_keys() != rebuilt.unique_keys() {
        return Err(S8DerivedIndexRebuildDenied::ParityKeyIdentityMismatch);
    }
    if authoritative
        .ordered_rows()
        .iter()
        .map(|row| row.key())
        .ne(rebuilt.ordered_rows().iter().map(|row| row.key()))
    {
        return Err(S8DerivedIndexRebuildDenied::ParityOrderingMismatch);
    }
    let value_identity = match source_authority.value_identity_parity() {
        exact @ S8DerivedIndexIdentityParity::Exact => {
            if authoritative
                .ordered_rows()
                .iter()
                .map(|row| row.value_fingerprint())
                .ne(rebuilt
                    .ordered_rows()
                    .iter()
                    .map(|row| row.value_fingerprint()))
            {
                return Err(S8DerivedIndexRebuildDenied::ParityValueIdentityMismatch);
            }
            exact
        }
        parity => parity,
    };
    let cost_envelope = match source_authority.cost_envelope_parity() {
        matched @ S8DerivedIndexCostEnvelopeParity::DeclaredEnvelopeMatched => {
            if !authoritative.cost_envelope_compliant() || !rebuilt.cost_envelope_compliant() {
                return Err(S8DerivedIndexRebuildDenied::ParityCostEnvelopeMismatch);
            }
            matched
        }
        parity => parity,
    };

    let counter_shape = declared_counter_shape_parity(plan.request().strategy_family());
    if matches!(
        counter_shape,
        S8DerivedIndexCounterShapeParity::ExactDeterministicPhysicalShape
    ) && authoritative.counter_shape() != rebuilt.counter_shape()
    {
        return Err(S8DerivedIndexRebuildDenied::ParityCounterShapeMismatch);
    }

    Ok(S8DerivedIndexParityWitness::new(
        value_identity,
        cost_envelope,
        counter_shape,
    ))
}
