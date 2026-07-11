use forge_store_layout_indexes::{
    layout_migration::LayoutBindingWitness, layout_rebuild::S8DerivedIndexParityWitness,
};

fn main() {
    let derived_projection: S8DerivedIndexParityWitness = todo!();
    let _ = LayoutBindingWitness::new(todo!(), todo!(), todo!(), derived_projection);
}
