pub mod frame;
pub mod frame_snapshot;
pub mod navigation;
pub mod run_summary;
pub mod stable_surface_ids;
pub mod surface_renderer;

pub use frame::ValidationShellFrame;
pub use frame_snapshot::ShellFrameSnapshot;
pub use navigation::{NavigationSelection, ValidationPageId};
pub use run_summary::ValidationRunSummary;
pub use stable_surface_ids::{
    StableShellSurface, StableShellSurfaceId, StableShellSurfaceManifest,
    StableShellSurfacePlacement,
};
pub use surface_renderer::ShellSurfaceRenderer;
