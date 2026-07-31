use super::*;

#[test]
fn built_in_resource_policy_registry_exposes_freeze_evidence() {
    let registry = FrozenResourcePolicyRegistry::built_in();
    let report = registry.freeze_report();
    let descriptor_count = registry.descriptor_count();

    assert_eq!(descriptor_count, built_in_policy_registrations().len());
    assert_eq!(report.descriptor_count(), descriptor_count);
    assert_eq!(report.id_index_width(), descriptor_count);
    assert_eq!(report.kind_name_index_width(), descriptor_count);
    assert_eq!(
        report.registry_digest().as_str(),
        registry.registry_digest().as_str()
    );
    assert!(report
        .registry_digest()
        .as_str()
        .starts_with("resource-policy-registry:"));
    assert!(report
        .registry_digest()
        .as_str()
        .contains("signal.resource.retry.disabled"));
}
