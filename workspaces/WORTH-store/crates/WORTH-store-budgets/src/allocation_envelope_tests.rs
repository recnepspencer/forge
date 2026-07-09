use crate::{
    AllocationBudgetDenial, AllocationByteBudget, AllocationEnvelopeDeclaration, AllocationScope,
    FixedMetadataReservation,
};

fn budget(bytes: u64) -> AllocationByteBudget {
    AllocationByteBudget::bytes(bytes).unwrap()
}

fn fixed(bytes: u64) -> FixedMetadataReservation {
    FixedMetadataReservation::constant_bytes(bytes).unwrap()
}

#[test]
fn allocation_envelope_declaration_requires_every_scope() {
    let denial = AllocationEnvelopeDeclaration::declare()
        .foreground(budget(1))
        .maintenance(budget(1))
        .recovery(budget(1))
        .scrub(budget(1))
        .import_export(budget(1))
        .fixed_metadata(fixed(1))
        .seal()
        .unwrap_err();

    assert_eq!(
        denial,
        AllocationBudgetDenial::MissingScopeBudget(AllocationScope::Streaming)
    );
}

#[test]
fn zero_allocation_budgets_and_fixed_metadata_are_denied() {
    assert_eq!(
        AllocationByteBudget::bytes(0).unwrap_err(),
        AllocationBudgetDenial::AllocationBudgetIsZero
    );
    assert_eq!(
        FixedMetadataReservation::constant_bytes(0).unwrap_err(),
        AllocationBudgetDenial::FixedMetadataReservationIsZero
    );
}
