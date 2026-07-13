use forge_store_layout_indexes::{
    evolution::migration::LayoutBindingWitness, maintenance::DerivedIndexParityWitness,
};

fn main() {
    let derived_projection: DerivedIndexParityWitness = todo!();
    let _ = LayoutBindingWitness::new(todo!(), todo!(), todo!(), derived_projection);
}
