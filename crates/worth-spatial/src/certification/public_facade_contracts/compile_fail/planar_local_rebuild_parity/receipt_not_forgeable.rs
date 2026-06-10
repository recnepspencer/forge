use worth_spatial::facade::planar_local_rebuild_parity::{
    PlanarLocalRebuildParityCounters, PlanarLocalRebuildParityReceipt,
};

fn main() {
    let _receipt = PlanarLocalRebuildParityReceipt::new(
        panic!("basis constructor is not public"),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        PlanarLocalRebuildParityCounters::certified(0, 0, 0, 0),
    );
}
