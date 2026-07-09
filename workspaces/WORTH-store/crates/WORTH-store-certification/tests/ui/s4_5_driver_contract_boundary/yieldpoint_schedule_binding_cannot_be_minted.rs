use worth_store_physical_certification::{PhysicalBoundaryYieldpoint, YieldpointScheduleBinding};

fn main() {
    let _WORTHd = YieldpointScheduleBinding {
        scheduled_yieldpoint: "memory-pressure-boundary".to_owned(),
        declared_yieldpoint: PhysicalBoundaryYieldpoint::memory_pressure_boundary(),
    };
}
