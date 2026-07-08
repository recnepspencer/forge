use forge_store_layout_indexes::{
    S8AccessShapeDeclaration, S8PointAccessShape, S8PrefixAccessShape, S8RangeAccessShape,
};

fn main() {
    let _ = core::any::TypeId::of::<S8AccessShapeDeclaration>();
    let _ = core::any::TypeId::of::<S8PointAccessShape>();
    let _ = core::any::TypeId::of::<S8RangeAccessShape>();
    let _ = core::any::TypeId::of::<S8PrefixAccessShape>();
}
