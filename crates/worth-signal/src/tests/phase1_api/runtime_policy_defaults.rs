use crate::facade::*;

#[test]
fn runtime_policy_maps_into_s9_contract_and_strategy_defaults() {
    let operational = SignalRuntimePolicy::operational();
    let development = SignalRuntimePolicy::development();
    let forensic = SignalRuntimePolicy::forensic();

    assert_eq!(operational.default_path_class(), PathClass::Operational);
    assert_eq!(
        operational.default_artifact_policy_class(),
        ArtifactPolicyClass::OperationalMinimal
    );
    assert_eq!(
        operational.default_execution_strategy(),
        ResolvedExecutionStrategy::SparseIncremental
    );
    assert_eq!(
        operational.default_maintenance_strategy(),
        ResolvedMaintenanceStrategy::DensityAdaptive
    );
    assert_eq!(
        operational.default_authority_policy(),
        AuthorityPolicy::SpeculativeThenReconcile
    );

    assert_eq!(development.default_path_class(), PathClass::Rich);
    assert_eq!(
        development.default_artifact_policy_class(),
        ArtifactPolicyClass::DevelopmentRetained
    );
    assert_eq!(
        development.default_execution_strategy(),
        ResolvedExecutionStrategy::DenseStageBatched
    );
    assert_eq!(
        development.default_maintenance_strategy(),
        ResolvedMaintenanceStrategy::Incremental
    );
    assert_eq!(
        development.default_authority_policy(),
        AuthorityPolicy::SpeculativeThenReconcile
    );

    assert_eq!(forensic.default_path_class(), PathClass::Rich);
    assert_eq!(
        forensic.default_artifact_policy_class(),
        ArtifactPolicyClass::ForensicReconstructable
    );
    assert_eq!(
        forensic.default_execution_strategy(),
        ResolvedExecutionStrategy::SparseIncremental
    );
    assert_eq!(
        forensic.default_maintenance_strategy(),
        ResolvedMaintenanceStrategy::Incremental
    );
    assert_eq!(
        forensic.default_authority_policy(),
        AuthorityPolicy::SpeculativeThenReconcile
    );
}
