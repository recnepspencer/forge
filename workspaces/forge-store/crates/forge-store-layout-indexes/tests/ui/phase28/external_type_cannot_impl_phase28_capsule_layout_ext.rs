use forge_store_operations::Phase28CapsuleLayoutExt;

struct FakeCapsule;

impl Phase28CapsuleLayoutExt for FakeCapsule {
    fn phase28_declared_capsule_bytes(&self) -> u64 {
        1
    }
}

fn main() {}
