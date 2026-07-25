use std::marker::PhantomData;
use worth_query::facade::{
    foundation::BasisOperationLane,
    installed::{WorthQueryInstalledOperatingWorld, WorthQueryOperationFamilyView},
};

fn forge<'view, 'runtime, F, L: BasisOperationLane>(
    world: &'view WorthQueryInstalledOperatingWorld<'runtime, L>,
) -> WorthQueryOperationFamilyView<'view, 'runtime, F, L> {
    WorthQueryOperationFamilyView {
        world,
        _family: PhantomData,
    }
}

fn main() {}
