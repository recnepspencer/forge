use forge_store_physical_isolation::CurrentPhysicalRoot;

fn main() {
    let root: CurrentPhysicalRoot = todo!();
    let _ = root.page_epoch_for_publication(7);
}
