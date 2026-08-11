//! One concrete binding declared through the public axis-authoring surface.

worth_proof::binding_axes! {
    pub(crate) struct WitnessAxes {
        pub(crate) runtime: u8 => Runtime,
    }
    drift pub(crate) enum WitnessDrift;
}

pub(crate) fn binding() -> worth_proof::Binding<WitnessAxes> {
    worth_proof::Binding::new(WitnessAxes { runtime: 3 })
}
