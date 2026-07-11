use forge_store_physical_isolation::{
    CompactionCutoverDelta, CompactionRewritePublication, PhysicalPublicationReceipt,
};

fn main() {
    let delta: CompactionCutoverDelta = todo!();
    let publication: PhysicalPublicationReceipt = todo!();
    let _ = CompactionRewritePublication::publish(delta, publication);
}
