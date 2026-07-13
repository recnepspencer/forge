use forge_store_layout_indexes::{
    access_planning::access_planning, maintenance::DerivedIndexParityWitness,
};

fn main() {
    let derived_projection: DerivedIndexParityWitness = todo!();
    let _ = access_planning().require_exact_point_access(derived_projection);
}
