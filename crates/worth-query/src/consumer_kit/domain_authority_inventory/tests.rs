use super::*;

#[test]
fn current_domain_authority_inventory_is_source_complete() {
    let audit = current_domain_authority_inventory_audit();
    assert!(audit.is_complete(), "{:#?}", audit.findings());
    assert!(audit.observed_authority_count() > 70);
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
            "src/domain/materialization.rs",
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
                "src/domain/materialization.rs".to_string(),
                1,
                "materialize_shadow_domain_authority".to_string()
            ),
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
    assert_eq!(grammar.package_fields().len(), 10);
    assert_eq!(grammar.transcript_owners().len(), grammar.stages().len());
    assert_eq!(
        grammar.stages()[4],
        WorthQueryDomainInstallationGrammarStage::ObtainRuntimeAffineHandle
    );
}
