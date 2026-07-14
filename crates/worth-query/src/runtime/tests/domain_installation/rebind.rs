use super::*;

#[test]
fn rebind_reissues_current_authority_only_for_equivalent_package_meaning() {
    let prior_runtime = installed_runtime();
    let current_runtime = installed_runtime();
    let prior = prior_runtime.domain(InstalledDomain).unwrap();
    let rebound = current_runtime
        .rebind_domain(prior.rebind_request())
        .unwrap();
    assert_eq!(
        rebound.handle().package_identity(),
        prior.package_identity()
    );
    assert_ne!(
        rebound.receipt().prior_witness_identity(),
        rebound.receipt().current_witness_identity()
    );
    current_runtime
        .validate_installed_domain_handle(rebound.handle())
        .unwrap();
    let changed = changed_package_runtime();
    let denial = changed.rebind_domain(prior.rebind_request()).unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryDomainRebindDenialKind::PackageMeaningChanged
    );
    assert_eq!(
        denial.next_action(),
        WorthQueryDomainRebindNextAction::ReconcilePackageMeaning
    );
    assert_ne!(
        denial.prior_package_identity(),
        denial.current_package_identity().unwrap()
    );
}

#[test]
fn rebind_to_runtime_without_the_domain_prescribes_installation() {
    let prior_runtime = installed_runtime();
    let prior = prior_runtime.domain(InstalledDomain).unwrap();
    let empty_runtime = complete_backend_from_parts_builder()
        .build_backend_from_parts()
        .build()
        .unwrap();
    let denial = empty_runtime
        .rebind_domain(prior.rebind_request())
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryDomainRebindDenialKind::DomainNotInstalled
    );
    assert_eq!(
        denial.next_action(),
        WorthQueryDomainRebindNextAction::InstallDomainPackage
    );
}
