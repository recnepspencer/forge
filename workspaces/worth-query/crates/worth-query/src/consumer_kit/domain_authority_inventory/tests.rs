use super::*;

#[test]
fn current_domain_authority_inventory_is_source_complete() {
    let audit = current_domain_authority_inventory_audit()
        .expect("the complete Query production source tree must be auditable");
    assert!(audit.is_complete(), "{:#?}", audit.findings());
    assert!(
        audit.observed_authority_count() >= worth_query_domain_authority_inventory_rows().len()
    );
}

#[test]
fn runtime_core_package_inputs_are_classified_across_both_facades() {
    let rows = worth_query_domain_authority_inventory_rows();
    let expected_paths = [
        "src/facade/exports_domain.rs",
        "src/facade/exports_runtime_core.rs",
    ];
    for symbol in [
        "WorthQueryDomainIdentityNamespace",
        "WorthQueryDomainIdentityName",
        "WorthQueryDomainSemanticVersion",
        "WorthQueryDomainIdentityDeclaration",
        "WorthQueryDomainPackage",
    ] {
        let row = rows
            .iter()
            .find(|row| row.symbol() == symbol)
            .expect("every runtime-core package input needs one semantic authority row");
        assert_eq!(
            row.current_class(),
            WorthQueryDomainAuthorityClass::PackageInput
        );
        assert_eq!(row.exporting_paths(), expected_paths);
    }
}

#[test]
fn authority_in_an_unlisted_production_file_cannot_evade_the_workspace_audit() {
    let root = std::env::temp_dir().join(format!(
        "worth-query-domain-authority-audit-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should follow the Unix epoch")
            .as_nanos()
    ));
    let crate_root = root.join("crates/worth-query");
    std::fs::create_dir_all(crate_root.join("src/unlisted"))
        .expect("hostile source tree should be creatable");
    std::fs::write(
        crate_root.join("src/unlisted/shadow_authority.rs"),
        "pub struct ShadowOperationRegistry;\n",
    )
    .expect("hostile authority source should be writable");

    let audit = audit_workspace_domain_authority_inventory(&root)
        .expect("hostile source tree should be auditable");
    let finding = audit
        .findings()
        .iter()
        .find(|finding| {
            finding.kind() == WorthQueryDomainAuthorityFindingKind::UnclassifiedSemanticAuthority
                && finding.site().symbol() == "ShadowOperationRegistry"
        })
        .expect("an unlisted production authority must be reported");
    assert_eq!(finding.site().path(), "src/unlisted/shadow_authority.rs");
    assert_eq!(finding.site().line(), 1);

    std::fs::remove_dir_all(root).expect("hostile source tree should be removable");
}

#[test]
fn canonical_authority_paths_remain_and_competing_paths_are_absent() {
    let rows = worth_query_domain_authority_inventory_rows();
    let class_for = |symbol| {
        rows.iter()
            .find(|row| row.symbol() == symbol)
            .map(|row| (row.current_class(), row.target_class()))
    };

    assert_eq!(
        class_for("WorthQueryDomainPackage"),
        Some((
            WorthQueryDomainAuthorityClass::PackageInput,
            WorthQueryDomainAuthorityClass::PackageInput,
        ))
    );
    assert_eq!(
        class_for("WorthQueryRuntimeBuilder::domain_package"),
        Some((
            WorthQueryDomainAuthorityClass::CanonicalInstallation,
            WorthQueryDomainAuthorityClass::CanonicalInstallation,
        ))
    );
    assert_eq!(
        class_for("WorthQueryInstalledDomainHandle"),
        Some((
            WorthQueryDomainAuthorityClass::InstalledHandleCapability,
            WorthQueryDomainAuthorityClass::InstalledHandleCapability,
        ))
    );
    assert_eq!(
        class_for("WorthQueryGraphReadOperationRegistry"),
        Some((
            WorthQueryDomainAuthorityClass::DerivedIndex,
            WorthQueryDomainAuthorityClass::DerivedIndex,
        ))
    );
    for removed in [
        "WorthQueryRuntimeBuilder::invariant_catalog",
        "WorthQueryRuntimeBuilder::invariant_registration_artifact",
        "WorthQueryRuntimeBuilder::graph_obligation",
        "WorthQueryRuntimeBuilder::graph_scoped_custom_invariant",
        "WorthQueryRuntimeBuilder::custom_invariant",
        "WorthQueryRuntimeBuilder::register_invariant",
        "worth_query_domain",
    ] {
        assert!(
            class_for(removed).is_none(),
            "legacy authority survived: {removed}"
        );
    }
    assert!(rows.iter().all(|row| {
        row.current_class() != WorthQueryDomainAuthorityClass::CompatibilityPath
            && row.target_class() != WorthQueryDomainAuthorityClass::ProhibitedCompetingAuthority
    }));
    assert!(rows
        .iter()
        .all(|row| !row.symbol().starts_with("materialize_")));
}

#[test]
fn seeded_competing_authorities_report_exact_definition_sites() {
    let sources = [
        WorthQueryDomainAuthoritySource::new(
            "src/domain/raw.rs",
            "pub fn register_domain(raw: impl Into<String>) -> WorthQueryDomainContributionSurface { todo!() }",
        ),
        WorthQueryDomainAuthoritySource::new(
            "src/domain/registry.rs",
            "pub struct ShadowOperationRegistry;",
        ),
        WorthQueryDomainAuthoritySource::new(
            "src/facade/domain_materialization.rs",
            "pub fn materialize_shadow_domain_authority() {}",
        ),
        WorthQueryDomainAuthoritySource::new(
            "src/domain/semantic_adapter.rs",
            "pub struct ShadowDomainAdapter;",
        ),
    ];

    let audit = audit_domain_authority_sources(&sources);
    let unclassified = audit
        .findings()
        .iter()
        .filter(|finding| {
            finding.kind() == WorthQueryDomainAuthorityFindingKind::UnclassifiedSemanticAuthority
        })
        .map(|finding| {
            (
                finding.site().path().to_string(),
                finding.site().line(),
                finding.site().symbol().to_string(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        unclassified,
        vec![
            (
                "src/domain/raw.rs".to_string(),
                1,
                "register_domain".to_string()
            ),
            (
                "src/domain/registry.rs".to_string(),
                1,
                "ShadowOperationRegistry".to_string()
            ),
            (
                "src/domain/semantic_adapter.rs".to_string(),
                1,
                "ShadowDomainAdapter".to_string()
            ),
            (
                "src/facade/domain_materialization.rs".to_string(),
                1,
                "materialize_shadow_domain_authority".to_string()
            ),
        ]
    );
}

#[test]
fn physical_adapter_is_not_semantic_domain_authority() {
    let audit = audit_domain_authority_sources(&[WorthQueryDomainAuthoritySource::new(
        "src/runtime/physical_boundary/source.rs",
        "pub struct WorthQueryRuntimeSourceDomainAdapter;",
    )]);
    assert!(audit.findings().iter().all(|finding| {
        finding.kind() != WorthQueryDomainAuthorityFindingKind::UnclassifiedSemanticAuthority
    }));
}

#[test]
fn installation_grammar_freezes_order_fields_and_owners() {
    let grammar = worth_query_domain_installation_grammar();
    assert_eq!(grammar.stages().len(), 7);
    assert_eq!(grammar.package_fields().len(), 9);
    assert!(!grammar.package_fields().contains(&"graph-obligations"));
    assert_eq!(grammar.transcript_owners().len(), grammar.stages().len());
    assert_eq!(
        grammar.stages()[4],
        WorthQueryDomainInstallationGrammarStage::ObtainRuntimeAffineHandle
    );
}

#[test]
fn installed_domain_facade_has_one_executable_authority_root() {
    let facade = include_str!("../../facade/exports_domain.rs");
    for required in [
        "WorthQueryDomainPackage",
        "WorthQueryDomainEntryMarker",
        "WorthQueryDomainOperatingContext",
        "WorthQueryInstalledDomainHandle",
        "WorthQueryInstalledDomainDeclarationContext",
        "WorthQueryInstalledDomainExecutionReceipt",
    ] {
        assert!(facade.contains(required), "domain facade lost {required}");
    }
    for forbidden in [
        "WorthQueryApplicationFacade",
        "WorthQueryConfiguredDomainHandle",
        "WorthQueryGraphReadOperationRegistry",
        "materialize_",
    ] {
        assert!(
            !facade.contains(forbidden),
            "domain facade reintroduced competing authority {forbidden}"
        );
    }
}
