use worth_query::facade::{
    foundation::{AdmittedBasisCapability, BasisOperationLane},
    installed::WorthQueryInstalledOperatingWorld,
    runtime::WorthQueryRuntime,
};

fn forge<'runtime, L: BasisOperationLane>(
    runtime: &'runtime WorthQueryRuntime,
    basis: AdmittedBasisCapability<L>,
) -> WorthQueryInstalledOperatingWorld<'runtime, L> {
    WorthQueryInstalledOperatingWorld { runtime, basis }
}

fn main() {}
