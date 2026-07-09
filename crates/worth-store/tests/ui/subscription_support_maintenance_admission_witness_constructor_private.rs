use worth_store::{SupportMaintenanceAdmissionWitness, SupportMaintenanceDescriptor};

fn descriptor() -> SupportMaintenanceDescriptor {
    panic!("compile-fail fixture never executes")
}

fn main() {
    let descriptor = descriptor();
    let _witness = SupportMaintenanceAdmissionWitness::new(&descriptor);
}
