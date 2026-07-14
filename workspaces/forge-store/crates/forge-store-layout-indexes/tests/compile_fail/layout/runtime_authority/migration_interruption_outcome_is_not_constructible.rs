use forge_store_layout_indexes::evolution::migration::LayoutMigrationInterruptionOutcome;

fn forge() -> LayoutMigrationInterruptionOutcome {
    LayoutMigrationInterruptionOutcome { case: panic!() }
}

fn main() {
    let _ = forge();
}
