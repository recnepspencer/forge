mod support_inventory;

pub use support_inventory::{WorthUiRuntimeSupportInventory, RUNTIME_SUPPORT_INVENTORY};
#[deprecated(note = "use RUNTIME_SUPPORT_INVENTORY")]
pub use support_inventory::RUNTIME_SUPPORT_INVENTORY as PHASE3_RUNTIME_SUPPORT_INVENTORY;
