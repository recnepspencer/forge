use worth_store_physical_isolation::PhysicalByteGuard;

fn main() {
    let guard: PhysicalByteGuard<'_> = todo!();
    let _bytes = guard.as_bytes();
}
