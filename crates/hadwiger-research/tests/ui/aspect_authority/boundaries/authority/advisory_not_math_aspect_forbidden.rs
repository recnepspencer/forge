use hadwiger_research::facade::{AdvisoryAspectRecord, UnitDistanceAspectRecord};

fn needs_unit_distance(_: UnitDistanceAspectRecord) {}

fn pass_advisory_as_math(advisory: AdvisoryAspectRecord) {
    needs_unit_distance(advisory);
}

fn main() {
    let _ = pass_advisory_as_math as fn(AdvisoryAspectRecord);
}
