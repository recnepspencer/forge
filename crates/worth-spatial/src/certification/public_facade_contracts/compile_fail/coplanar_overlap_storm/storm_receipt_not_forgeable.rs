use worth_spatial::facade::coplanar_overlap_storm::CoplanarOverlapStormReceipt;

fn main() {
    let _ = CoplanarOverlapStormReceipt {
        storm_digest: String::from("fake storm"),
        workload_identity: String::from("fake workload"),
        operator_identity: String::from("fake operator"),
        counters: unconstructible(),
    };
}

fn unconstructible<T>() -> T {
    unreachable!()
}
