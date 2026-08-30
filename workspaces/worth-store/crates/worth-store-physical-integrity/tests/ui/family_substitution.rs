use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPhysicalWorkObligation,
    IntegrityValidatedPreviousRootSelector,
};

fn requires_current(_: IntegrityValidatedCurrentRootSelector<'_>) {}
fn requires_physical_work(_: IntegrityValidatedPhysicalWorkObligation<'_>) {}

fn substitute(previous: IntegrityValidatedPreviousRootSelector<'_>) {
    requires_current(previous);
}

fn substitute_physical_work(physical_work: IntegrityValidatedPhysicalWorkObligation<'_>) {
    requires_current(physical_work);
}

fn substitute_current(current: IntegrityValidatedCurrentRootSelector<'_>) {
    requires_physical_work(current);
}

fn main() {}
