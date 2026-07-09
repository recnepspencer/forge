use worth_store_physical_format::PhysicalPageId;
use worth_store_physical_isolation::GenerationCountedPhysicalReference;

fn main() {
    let page_id = PhysicalPageId::from_raw(1).unwrap();
    let _: GenerationCountedPhysicalReference = page_id;
}
