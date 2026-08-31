mod axis_demand;
#[allow(
    dead_code,
    reason = "Gate 0 exposes a read-only coherent owner snapshot before consumption"
)]
mod owner_snapshot;

pub(crate) use axis_demand::UiAppearanceStateAxisDemand;
pub use owner_snapshot::UiAppearanceOwnerSnapshot;
