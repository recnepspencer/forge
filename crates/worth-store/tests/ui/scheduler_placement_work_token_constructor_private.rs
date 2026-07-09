use worth_store::{PlacementExecutionOrigin, SchedulerPlacementWorkToken};

fn main() {
    let _ = SchedulerPlacementWorkToken::new(
        "placement:derived:snapshot:42",
        PlacementExecutionOrigin::Background,
    );
}
