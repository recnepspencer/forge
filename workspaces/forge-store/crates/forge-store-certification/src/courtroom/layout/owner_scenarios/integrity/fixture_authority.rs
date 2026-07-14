use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_test_support::harness::layout::{
    layout_integrity_authority, unresolved_layout_authority_record, LayoutIntegrityAuthorityFixture,
};

use super::super::fixture_admission::admit_family;

pub(super) fn authority(
    seed: &str,
) -> (
    LayoutIntegrityAuthorityFixture,
    forge_store_layout_indexes::AdmittedPhysicalArtifactFamily,
) {
    let fixture = layout_integrity_authority(seed);
    let family = admit_family(
        DurableArtifactFamilyId::PhysicalRootManifest,
        fixture.security_scope(),
    );
    (fixture, family)
}

pub(super) fn import_witness(
    family: forge_store_layout_indexes::AdmittedPhysicalArtifactFamily,
    fixture: &LayoutIntegrityAuthorityFixture,
    seed: &str,
) -> forge_store_recovery_physics::RecoveryLayoutReadmissionWitness {
    let record = unresolved_layout_authority_record(seed);
    forge_store_recovery_physics::layout_readmission()
        .admit_import(
            family.family_id(),
            &record,
            fixture.current_authority(),
            fixture.security_scope().witnesses(),
        )
        .expect("unresolved physical authority must cross explicit import readmission")
}
