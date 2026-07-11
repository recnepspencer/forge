use forge_store_operations::Phase28ExportLayoutExt;

struct FakeExport;

impl Phase28ExportLayoutExt for FakeExport {
    fn phase28_declared_export_chunks(&self) -> u64 {
        1
    }
}

fn main() {}
