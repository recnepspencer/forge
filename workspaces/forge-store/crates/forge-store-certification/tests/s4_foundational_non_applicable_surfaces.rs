use forge_store_recovery_physics::{
    deny_non_applicable_surface, RecoveryEvidenceDenial, NON_APPLICABLE_FOUNDATIONAL_SURFACES,
    RECOVERY_ADMISSION_MECHANISMS,
};

#[test]
fn every_non_applicable_foundational_surface_is_denied_for_recovery_admission() {
    for surface in NON_APPLICABLE_FOUNDATIONAL_SURFACES {
        for mechanism in RECOVERY_ADMISSION_MECHANISMS {
            assert_eq!(
                deny_non_applicable_surface(surface, mechanism),
                RecoveryEvidenceDenial::NonApplicableFoundationalSurface
            );
        }
    }
}
