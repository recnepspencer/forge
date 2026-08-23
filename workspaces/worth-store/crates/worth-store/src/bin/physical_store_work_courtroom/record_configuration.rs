use worth_store::physical_runtime::{
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, AdmittedRecordPlacementPolicy,
    ManifestEntryCapacity, PhysicalRecordAccessPolicy, PhysicalRecordFormatDeclaration,
    PhysicalRecordPlacementPolicy, RecordByteLimit,
};

pub(crate) fn record_configuration() -> (
    AdmittedPhysicalRecordFormat,
    AdmittedRecordPlacementPolicy,
    AdmittedRecordAccessPolicy,
) {
    let format = AdmittedPhysicalRecordFormat::admit(
        PhysicalRecordFormatDeclaration::builder()
            .admit()
            .expect("canonical v1 format declaration"),
    );
    let placement = PhysicalRecordPlacementPolicy::builder()
        .manifest_capacity(ManifestEntryCapacity::new(64).expect("nonzero capacity"))
        .extent_threshold(RecordByteLimit::new(8_192).expect("nonzero threshold"))
        .admit(format)
        .expect("courtroom placement is compatible");
    let access = PhysicalRecordAccessPolicy::builder()
        .admit(format)
        .expect("courtroom access is compatible");
    (format, placement, access)
}
