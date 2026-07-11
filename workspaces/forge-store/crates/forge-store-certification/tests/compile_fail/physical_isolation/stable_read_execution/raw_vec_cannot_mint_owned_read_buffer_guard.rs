use forge_store_physical_isolation::{PhysicalByteGuard, PhysicalByteGuardAdmission};

fn main() {
    let admission: PhysicalByteGuardAdmission = todo!();
    let bytes = vec![1, 2, 3, 4];
    let receipt = todo!();
    let _guard = PhysicalByteGuard::from_owned_read_buffer(admission, receipt, bytes);
}
