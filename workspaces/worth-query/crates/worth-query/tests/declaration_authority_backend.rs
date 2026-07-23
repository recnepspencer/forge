use worth_query::facade::runtime::{
    WorthQueryRuntime, WorthQueryRuntimeFacadeFamily, WorthQueryRuntimeFamilySupport,
    WorthQueryRuntimeFamilySupportStatus,
};

const DECLARATION_AUTHORITY_DENIAL: &str =
    "declaration-authority runtimes do not own query execution";

#[test]
fn declaration_authority_backend_builds_with_every_execution_family_fail_closed() {
    let runtime = WorthQueryRuntime::builder()
        .declaration_authority_backend()
        .build()
        .expect("declaration authority should be a complete explicit backend posture");
    let profile = runtime.support_profile();

    for family in WorthQueryRuntimeFacadeFamily::ALL {
        let support = profile
            .support_for(family)
            .expect("every public runtime family must declare its posture");
        assert_eq!(
            support.status(),
            WorthQueryRuntimeFamilySupportStatus::Unsupported
        );
        assert_eq!(support.denial_reason(), Some(DECLARATION_AUTHORITY_DENIAL));
        assert!(!support.ordinary_downstream_dx());
    }
}

#[test]
fn declaration_authority_backend_rejects_execution_backend_parts() {
    let result = WorthQueryRuntime::builder()
        .declaration_authority_backend()
        .support_profile(
            worth_query::facade::runtime::WorthQueryRuntimeSupportProfile::new(std::iter::empty::<
                WorthQueryRuntimeFamilySupport,
            >()),
        )
        .build();
    let error = match result {
        Ok(_) => panic!("declaration authority cannot be combined with execution backend parts"),
        Err(error) => error,
    };

    assert!(format!("{error:?}").contains("runtime_backend_authority_selection"));
}
