use worth_store_aspect_native::StoreTerminalProjectionText;
use worth_store_operations::S10RepairBlastRadiusHandoff;

fn main() {
    let projection =
        StoreTerminalProjectionText::new_terminal_projection_text("terminal output");
    let _ = S10RepairBlastRadiusHandoff::from_repair_blast_radius_readiness(projection);
}
