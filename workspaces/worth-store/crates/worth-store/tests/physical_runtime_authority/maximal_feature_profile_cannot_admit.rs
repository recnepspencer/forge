use worth_store::{
    certification::StoreCertificationProgram,
    physical_runtime::{AdmittedPhysicalRuntime, PhysicalStore},
};

fn admit_certification_value(certification: StoreCertificationProgram) {
    let _runtime = PhysicalStore::admit(certification);
}

fn construct_test_runtime() {
    let _runtime = AdmittedPhysicalRuntime::new_for_test();
}

fn main() {}
