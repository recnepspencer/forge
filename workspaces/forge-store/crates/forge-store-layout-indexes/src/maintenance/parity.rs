use crate::maintenance::DerivedIndexRebuildDenied;

use super::identity::{
    declared_counter_shape_parity, DerivedIndexCostEnvelopeParity, DerivedIndexCounterShapeParity,
    DerivedIndexCoverageParity, DerivedIndexIdentityParity, DerivedIndexOrderingParity,
};
use super::rebuild::DerivedIndexRebuildReceipt;

#[derive(Debug, PartialEq, Eq)]
pub struct DerivedIndexParityWitness {
    key_identity: DerivedIndexIdentityParity,
    value_identity: DerivedIndexIdentityParity,
    ordering_identity: DerivedIndexOrderingParity,
    coverage_identity: DerivedIndexCoverageParity,
    cost_envelope: DerivedIndexCostEnvelopeParity,
    counter_shape: DerivedIndexCounterShapeParity,
}

impl DerivedIndexParityWitness {
    const fn new(
        value_identity: DerivedIndexIdentityParity,
        cost_envelope: DerivedIndexCostEnvelopeParity,
        counter_shape: DerivedIndexCounterShapeParity,
    ) -> Self {
        Self {
            key_identity: DerivedIndexIdentityParity::Exact,
            value_identity,
            ordering_identity: DerivedIndexOrderingParity::Exact,
            coverage_identity: DerivedIndexCoverageParity::Exact,
            cost_envelope,
            counter_shape,
        }
    }

    pub const fn key_identity(&self) -> DerivedIndexIdentityParity {
        self.key_identity
    }

    pub const fn value_identity(&self) -> DerivedIndexIdentityParity {
        self.value_identity
    }

    pub const fn ordering_identity(&self) -> DerivedIndexOrderingParity {
        self.ordering_identity
    }

    pub const fn coverage_identity(&self) -> DerivedIndexCoverageParity {
        self.coverage_identity
    }

    pub const fn cost_envelope(&self) -> DerivedIndexCostEnvelopeParity {
        self.cost_envelope
    }

    pub const fn counter_shape(&self) -> DerivedIndexCounterShapeParity {
        self.counter_shape
    }

    pub const fn parity_holds(&self) -> bool {
        true
    }
}

pub(crate) fn verify_parity(
    receipt: DerivedIndexRebuildReceipt,
) -> Result<DerivedIndexParityWitness, DerivedIndexRebuildDenied> {
    let plan = receipt.plan();
    let source_authority = plan.source_authority();
    let authoritative = source_authority.parity_basis();
    let rebuilt = receipt.rebuilt_basis();

    if authoritative.coverage() != plan.rebuild_scope().authority_coverage() {
        return Err(DerivedIndexRebuildDenied::ParityCoverageMismatch {
            expected: plan.rebuild_scope().authority_coverage().clone(),
            actual: authoritative.coverage().clone(),
        });
    }
    if rebuilt.coverage() != plan.rebuild_scope().authority_coverage() {
        return Err(DerivedIndexRebuildDenied::ParityCoverageMismatch {
            expected: plan.rebuild_scope().authority_coverage().clone(),
            actual: rebuilt.coverage().clone(),
        });
    }
    if authoritative.unique_keys() != rebuilt.unique_keys() {
        return Err(DerivedIndexRebuildDenied::ParityKeyIdentityMismatch);
    }
    if authoritative
        .ordered_rows()
        .iter()
        .map(|row| row.key())
        .ne(rebuilt.ordered_rows().iter().map(|row| row.key()))
    {
        return Err(DerivedIndexRebuildDenied::ParityOrderingMismatch);
    }
    let value_identity = match source_authority.value_identity_parity() {
        exact @ DerivedIndexIdentityParity::Exact => {
            if authoritative
                .ordered_rows()
                .iter()
                .map(|row| row.value_fingerprint())
                .ne(rebuilt
                    .ordered_rows()
                    .iter()
                    .map(|row| row.value_fingerprint()))
            {
                return Err(DerivedIndexRebuildDenied::ParityValueIdentityMismatch);
            }
            exact
        }
        parity => parity,
    };
    let cost_envelope = match source_authority.cost_envelope_parity() {
        matched @ DerivedIndexCostEnvelopeParity::DeclaredEnvelopeMatched => {
            if !authoritative.cost_envelope_compliant() || !rebuilt.cost_envelope_compliant() {
                return Err(DerivedIndexRebuildDenied::ParityCostEnvelopeMismatch);
            }
            matched
        }
        parity => parity,
    };

    let counter_shape = declared_counter_shape_parity(plan.request().strategy_family());
    if matches!(
        counter_shape,
        DerivedIndexCounterShapeParity::ExactDeterministicPhysicalShape
    ) && authoritative.counter_shape() != rebuilt.counter_shape()
    {
        return Err(DerivedIndexRebuildDenied::ParityCounterShapeMismatch);
    }

    Ok(DerivedIndexParityWitness::new(
        value_identity,
        cost_envelope,
        counter_shape,
    ))
}
