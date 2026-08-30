use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPreviousRootSelector,
};

fn requires_current(_: IntegrityValidatedCurrentRootSelector<'_>) {}

fn substitute(previous: IntegrityValidatedPreviousRootSelector<'_>) {
    requires_current(previous);
}

fn main() {}
