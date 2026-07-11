use forge_store_operations::Phase28ImportLayoutExt;

struct FakeImport;

impl Phase28ImportLayoutExt for FakeImport {
    fn phase28_declared_import_chunks(&self) -> u64 {
        1
    }

    fn phase28_local_import_chunks(&self) -> u64 {
        1
    }
}

fn main() {}
