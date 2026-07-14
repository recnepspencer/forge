use worth_store::{
    CapsuleSupportManifest, ImportedSupportSemanticAccess, SupportImportAdmissionWitness,
};

fn manifest() -> CapsuleSupportManifest {
    panic!("compile-fail fixture never executes")
}

fn main() {
    let manifest = manifest();
    let witness = SupportImportAdmissionWitness::new(&manifest, "target-admission").unwrap();
    let _access =
        ImportedSupportSemanticAccess::from_import_admission(witness, "imported-semantic").unwrap();
}
