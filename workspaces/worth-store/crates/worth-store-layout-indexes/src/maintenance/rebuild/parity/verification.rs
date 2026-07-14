use super::super::{DerivedIndexCandidateReadmissionReceipt, DerivedIndexParityBasis};
use super::identity::declared_counter_shape_parity;
use super::identity::{
    DerivedIndexCostEnvelopeParity, DerivedIndexCounterShapeParity, DerivedIndexCoverageParity,
    DerivedIndexIdentityParity, DerivedIndexOrderingParity,
};
use super::DerivedIndexParityCounterSnapshot;

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

fn verify_parity(
    receipt: DerivedIndexCandidateReadmissionReceipt,
) -> (
    Result<DerivedIndexParityWitness, DerivedIndexParityDenied>,
    DerivedIndexParityCounterSnapshot,
) {
    let (execution, rebuilt) = receipt.into_parts();
    let plan = execution.plan();
    let source_authority = plan.source_authority();
    let authoritative = source_authority.authoritative_parity_basis(plan.request().key_domain());
    let rebuilt = &rebuilt;

    let mut counters = DerivedIndexParityCounterSnapshot::from_authoritative_basis(&authoritative);
    if let Err(denial) = verify_coverage_identity(&authoritative, rebuilt, &mut counters) {
        return (Err(denial), counters);
    }
    if let Err(denial) = verify_key_sequence(&authoritative, rebuilt, &mut counters) {
        return (Err(denial), counters);
    }
    if let Err(denial) = verify_value_identity(&authoritative, rebuilt, &mut counters) {
        return (Err(denial), counters);
    }

    let counter_shape = declared_counter_shape_parity(plan.request().strategy_family());
    if let Err(denial) = verify_counter_shape(&authoritative, rebuilt, counter_shape, &mut counters)
    {
        return (Err(denial), counters);
    }

    (
        Ok(DerivedIndexParityWitness::new(
            source_authority.value_identity_parity(),
            source_authority.cost_envelope_parity(),
            counter_shape,
        )),
        counters,
    )
}

fn verify_coverage_identity(
    authoritative: &DerivedIndexParityBasis,
    rebuilt: &DerivedIndexParityBasis,
    counters: &mut DerivedIndexParityCounterSnapshot,
) -> Result<(), DerivedIndexParityDenied> {
    counters.record_coverage();
    if authoritative.coverage() != rebuilt.coverage() {
        return Err(DerivedIndexParityDenied::CoverageMismatch);
    }
    Ok(())
}

fn verify_key_sequence(
    authoritative: &DerivedIndexParityBasis,
    rebuilt: &DerivedIndexParityBasis,
    counters: &mut DerivedIndexParityCounterSnapshot,
) -> Result<(), DerivedIndexParityDenied> {
    if authoritative.row_count() != rebuilt.row_count() {
        return Err(DerivedIndexParityDenied::KeySequenceMismatch);
    }
    for (expected, actual) in authoritative
        .ordered_rows()
        .iter()
        .zip(rebuilt.ordered_rows())
    {
        counters.record_key(expected.key().as_bytes(), actual.key().as_bytes());
        if expected.key() != actual.key() {
            return Err(DerivedIndexParityDenied::KeySequenceMismatch);
        }
    }
    Ok(())
}

fn verify_value_identity(
    authoritative: &DerivedIndexParityBasis,
    rebuilt: &DerivedIndexParityBasis,
    counters: &mut DerivedIndexParityCounterSnapshot,
) -> Result<(), DerivedIndexParityDenied> {
    for (expected, actual) in authoritative
        .ordered_rows()
        .iter()
        .zip(rebuilt.ordered_rows())
    {
        counters.record_value(expected.value_fingerprint(), actual.value_fingerprint());
        if expected.value_fingerprint() != actual.value_fingerprint() {
            return Err(DerivedIndexParityDenied::ValueIdentityMismatch);
        }
    }
    Ok(())
}

fn verify_counter_shape(
    authoritative: &DerivedIndexParityBasis,
    rebuilt: &DerivedIndexParityBasis,
    counter_shape: DerivedIndexCounterShapeParity,
    counters: &mut DerivedIndexParityCounterSnapshot,
) -> Result<(), DerivedIndexParityDenied> {
    if matches!(
        counter_shape,
        DerivedIndexCounterShapeParity::ExactDeterministicPhysicalShape
    ) {
        counters.record_counter_shape(
            authoritative
                .counter_shape()
                .len()
                .min(rebuilt.counter_shape().len()),
        );
        if authoritative.counter_shape() != rebuilt.counter_shape() {
            return Err(DerivedIndexParityDenied::CounterShapeMismatch);
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedIndexParityDenied {
    CoverageMismatch,
    KeySequenceMismatch,
    ValueIdentityMismatch,
    CounterShapeMismatch,
}

#[derive(Debug, PartialEq, Eq)]
enum DerivedIndexParityCase {
    Verified(DerivedIndexParityWitness),
    Denied(DerivedIndexParityDenied),
}

#[derive(Debug, PartialEq, Eq)]
pub struct DerivedIndexParityOutcome {
    case: DerivedIndexParityCase,
    counters: DerivedIndexParityCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedIndexParityView<'a> {
    Verified(&'a DerivedIndexParityWitness),
    Denied(&'a DerivedIndexParityDenied),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DerivedIndexParityCaseId(&'static str);

impl DerivedIndexParityCaseId {
    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

pub fn derived_index_parity_cases() -> impl Iterator<Item = DerivedIndexParityCaseId> {
    [
        "verified",
        "denied.coverage",
        "denied.key_sequence",
        "denied.value_identity",
        "denied.counter_shape",
    ]
    .into_iter()
    .map(DerivedIndexParityCaseId)
}

impl DerivedIndexParityOutcome {
    fn from_result(
        result: Result<DerivedIndexParityWitness, DerivedIndexParityDenied>,
        counters: DerivedIndexParityCounterSnapshot,
    ) -> Self {
        let case = match result {
            Ok(witness) => DerivedIndexParityCase::Verified(witness),
            Err(denial) => DerivedIndexParityCase::Denied(denial),
        };
        Self { case, counters }
    }

    pub fn view(&self) -> DerivedIndexParityView<'_> {
        match &self.case {
            DerivedIndexParityCase::Verified(value) => DerivedIndexParityView::Verified(value),
            DerivedIndexParityCase::Denied(value) => DerivedIndexParityView::Denied(value),
        }
    }

    pub fn case_id(&self) -> DerivedIndexParityCaseId {
        match &self.case {
            DerivedIndexParityCase::Verified(_) => DerivedIndexParityCaseId("verified"),
            DerivedIndexParityCase::Denied(denial) => match denial {
                DerivedIndexParityDenied::CoverageMismatch => {
                    DerivedIndexParityCaseId("denied.coverage")
                }
                DerivedIndexParityDenied::KeySequenceMismatch => {
                    DerivedIndexParityCaseId("denied.key_sequence")
                }
                DerivedIndexParityDenied::CounterShapeMismatch => {
                    DerivedIndexParityCaseId("denied.counter_shape")
                }
                DerivedIndexParityDenied::ValueIdentityMismatch => {
                    DerivedIndexParityCaseId("denied.value_identity")
                }
            },
        }
    }

    pub fn into_verified(self) -> Result<DerivedIndexParityWitness, Self> {
        match self.case {
            DerivedIndexParityCase::Verified(value) => Ok(value),
            case => Err(Self {
                case,
                counters: self.counters,
            }),
        }
    }

    pub const fn counters(&self) -> DerivedIndexParityCounterSnapshot {
        self.counters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutParityVerification;

pub const fn layout_parity_verification() -> LayoutParityVerification {
    LayoutParityVerification
}

impl LayoutParityVerification {
    pub fn verify(
        self,
        receipt: DerivedIndexCandidateReadmissionReceipt,
    ) -> DerivedIndexParityOutcome {
        let (result, counters) = verify_parity(receipt);
        DerivedIndexParityOutcome::from_result(result, counters)
    }
}
