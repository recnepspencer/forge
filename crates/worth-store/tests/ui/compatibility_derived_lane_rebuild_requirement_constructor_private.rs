use worth_store::{DerivedCompatibilityLaneKind, DerivedLaneRebuildRequirement, DerivedRebuildRequirement};

fn main() {
    let _ = DerivedLaneRebuildRequirement::new(
        DerivedCompatibilityLaneKind::MaintenanceSummarySupport,
        requirement(),
    );
}

fn requirement() -> DerivedRebuildRequirement {
    panic!("compile-fail fixture")
}
