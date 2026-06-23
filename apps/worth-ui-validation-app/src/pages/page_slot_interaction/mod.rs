mod projection;
mod render_plan;
mod renderer;

pub use projection::ValidationPageSlotInteractionProjection;
pub use render_plan::{
    ValidationPageSlotAppearanceDependencyProof, ValidationPageSlotDensityDependencyProof,
    ValidationPageSlotInteractionRenderPlan, ValidationPageSlotInteractionSlotRow,
};
pub use renderer::render_page_slot_interaction;
