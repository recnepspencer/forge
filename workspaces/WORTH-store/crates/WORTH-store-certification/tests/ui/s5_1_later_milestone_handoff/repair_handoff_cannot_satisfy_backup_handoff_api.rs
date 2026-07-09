use worth_store_operations::{
    BackupExportCustodyReadiness, S10RepairBlastRadiusHandoff,
};

fn main() {
    let repair_handoff: S10RepairBlastRadiusHandoff = todo!();
    let _backup = BackupExportCustodyReadiness::from_s10_handoff(repair_handoff);
}
