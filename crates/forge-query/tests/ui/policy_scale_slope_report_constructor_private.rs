use forge_query::facade::{PolicyScaleCounterSnapshot, PolicyScaleFixtureSize, PolicyScaleSlopeReport};

fn main() {
    let snapshot =
        PolicyScaleCounterSnapshot::new(PolicyScaleFixtureSize::Small, 1, 1, 1, 1, 1, 1, 1, 0);
    let _ = PolicyScaleSlopeReport::new(snapshot.clone(), snapshot.clone(), snapshot);
}
