use worth_spatial::facade::planar_contracts::{
    PlanarAdmissionClass, PlanarAdmissionFamily, PlanarAdmissionReceipt, PlanarRuntimeConcern,
};

fn main() {
    let _forged = PlanarAdmissionReceipt {
        family: PlanarAdmissionFamily::DirtyPlanarInput,
        concern: PlanarRuntimeConcern::DiagnosticsLocalization,
        class: PlanarAdmissionClass::Admitted,
        row_digest: String::from("forged-row"),
        matrix_digest: String::from("forged-matrix"),
    };
}
