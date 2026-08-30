use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_integrity::{
    IntegrityValidatedCurrentRootSelector, IntegrityValidatedPhysicalWorkObligation,
    IntegrityValidatedPreviousRootSelector, IntegrityValidatedRootManifest, PhysicalArtifactScope,
    PhysicalByteRange, PhysicalIntegrityValidationRecord, UntrustedPhysicalArtifact,
};

fn main() {
    let _input = UntrustedPhysicalArtifact::from_bounded_bytes(b"untrusted");
}

fn selector_scope(store: StableStoreIdentity) -> PhysicalArtifactScope {
    PhysicalArtifactScope::current_root_selector(
        store,
        worth_store_physical_format::PhysicalRecordFormatDeclaration::builder()
            .admit()
            .unwrap(),
        PhysicalByteRange::new(0, 107).unwrap(),
    )
}

fn valid_current<'media>(
    validated: IntegrityValidatedCurrentRootSelector<'media>,
    input: UntrustedPhysicalArtifact<'media>,
) -> PhysicalIntegrityValidationRecord {
    let scope = validated.scope();
    let _same_incarnation = validated.matches_input(input);
    let record = validated.into_validation_record();
    let _same_scope = record.matches_scope(scope);
    record
}

fn valid_previous<'media>(
    validated: IntegrityValidatedPreviousRootSelector<'media>,
    input: UntrustedPhysicalArtifact<'media>,
) -> PhysicalIntegrityValidationRecord {
    let scope = validated.scope();
    let _same_incarnation = validated.matches_input(input);
    let record = validated.into_validation_record();
    let _same_scope = record.matches_scope(scope);
    record
}

fn valid_manifest<'media>(
    validated: IntegrityValidatedRootManifest<'media>,
    input: UntrustedPhysicalArtifact<'media>,
) -> PhysicalIntegrityValidationRecord {
    let scope = validated.scope();
    let _same_incarnation = validated.matches_input(input);
    let record = validated.into_validation_record();
    let _same_scope = record.matches_scope(scope);
    record
}

fn valid_physical_work<'media>(
    validated: IntegrityValidatedPhysicalWorkObligation<'media>,
    input: UntrustedPhysicalArtifact<'media>,
) -> PhysicalIntegrityValidationRecord {
    let scope = validated.scope();
    let _identity = validated.identity();
    let _operation = validated.operation_code();
    let _target = validated.target();
    let _payload_digest = validated.payload_digest();
    let _same_incarnation = validated.matches_input(input);
    let record = validated.into_validation_record();
    let _same_scope = record.matches_scope(scope);
    record
}
