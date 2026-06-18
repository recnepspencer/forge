use topology::facade::{
    PlanarBooleanLoopBlueprintCloseout, PlanarBooleanLoopBlueprintRegistry,
    PlanarBooleanLoopBlueprintRegistryIdentity, PlanarBooleanLoopOperatorRow,
    PlanarBooleanLoopValidatorRow,
};

fn main() {
    let _ = PlanarBooleanLoopBlueprintRegistry {
        operators: Vec::<PlanarBooleanLoopOperatorRow>::new(),
        validators: Vec::<PlanarBooleanLoopValidatorRow>::new(),
        closeout: loop {
            break Option::<PlanarBooleanLoopBlueprintCloseout>::None.unwrap();
        },
        identity: loop {
            break Option::<PlanarBooleanLoopBlueprintRegistryIdentity>::None.unwrap();
        },
    };
}
