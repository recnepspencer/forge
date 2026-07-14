use worth_store_layout_indexes::evolution::migration::LayoutMigrationInterruptionOutcome;

fn worth() -> LayoutMigrationInterruptionOutcome {
    LayoutMigrationInterruptionOutcome { case: panic!() }
}

fn main() {
    let _ = worth();
}
