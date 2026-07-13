pub(crate) fn admitted_materialization(
    family: crate::AdmittedPhysicalArtifactFamily,
    coverage: crate::LayoutCoverageWitness,
) -> crate::AdmittedLayoutMaterialization {
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    match coverage.upper_bound().basis_kind() {
        crate::materialization::CoverageBasisKind::RootEpoch => crate::access_planning()
            .admit_current_catalog_root_materialization(family, &catalog)
            .expect("rebuild fixture root materialization admission"),
        crate::materialization::CoverageBasisKind::WalLsn => {
            crate::strategy::tests_support::persisted_lsm_materialization(family, &catalog).0
        }
        other => panic!("rebuild fixture does not support {other:?}"),
    }
}
