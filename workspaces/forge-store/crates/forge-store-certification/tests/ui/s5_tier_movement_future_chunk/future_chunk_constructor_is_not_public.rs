use forge_store_physical_format::PhysicalFutureChunkReference;
use forge_store_physical_isolation::{
    ChunkEpoch, MovablePhysicalRef, PhysicalReadReachabilityBarrier,
};

fn main() {
    let reference: PhysicalFutureChunkReference = todo!();
    let epoch: ChunkEpoch = todo!();
    let reachability: PhysicalReadReachabilityBarrier = todo!();

    let _ = MovablePhysicalRef::future_chunk(reference, epoch, reachability);
}
