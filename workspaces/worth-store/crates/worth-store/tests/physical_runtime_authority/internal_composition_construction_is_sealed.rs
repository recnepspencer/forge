use worth_store::physical_runtime::{
    AdmittedPhysicalRuntime, InstalledCapabilityStatus, PhysicalRuntimeAdmission,
};

fn construct_admission() {
    let _admission = PhysicalRuntimeAdmission {
        declared_root: unavailable(),
    };
}

fn construct_runtime() {
    let _runtime = AdmittedPhysicalRuntime {
        runtime_identity: unavailable(),
        resource_lifecycle: unavailable(),
        diagnostics: unavailable(),
        shutdown: unavailable(),
    };
}

fn construct_capability_status() {
    let _status = InstalledCapabilityStatus { _private: () };
}

fn unavailable<T>() -> T {
    panic!("compile-fail specimen")
}

fn main() {}
