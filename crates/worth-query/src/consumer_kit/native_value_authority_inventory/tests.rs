use super::*;

#[test]
fn phase_twenty_one_native_value_grammar_is_complete_and_single_authority() {
    let rows = worth_query_native_value_grammar();
    let audit = audit_native_value_grammar(rows);

    assert!(audit.is_closed(), "native grammar gaps: {audit:#?}");
    assert!(audit.missing_scalar_types().is_empty());
    assert!(audit.duplicate_families().is_empty());
    assert!(audit.missing_cell_families().is_empty());
    assert_eq!(audit.struct_row_count(), 1);
    assert_eq!(
        rows.len(),
        26,
        "24 scalar families, two string forms, and one struct row"
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.scalar_type()
                == Some(worth_foundational::facade::ScalarAspectType::String))
            .count(),
        2,
        "raw and symbol-backed interned strings require separate evidence rows"
    );
    for row in rows {
        let _ = row.family();
        assert!(!row.semantic_carrier().is_empty());
        assert!(!row.authoring_path().is_empty());
        assert!(!row.predicate_capabilities().is_empty());
        assert!(!row.projection_form().is_empty());
        assert!(!row.refinement_form().is_empty());
        assert!(!row.certification_owner().is_empty());
    }
}

#[test]
fn phase_twenty_one_inventory_names_every_closure_owner_and_consumer_surface() {
    let rows = worth_query_native_value_authority_rows();
    assert!(!rows.is_empty());
    for row in rows {
        assert!(
            !row.closure_owner().is_empty(),
            "missing owner for {}",
            row.symbol()
        );
        assert!(
            !row.consumer_surfaces().is_empty(),
            "missing consumer surface for {}",
            row.symbol()
        );
        let _ = row.class();
        let _ = row.disposition();
    }
}

#[test]
fn phase_twenty_one_current_query_source_inventory_is_closed() {
    let audit = current_native_value_authority_audit()
        .expect("Query source-backed native-value inventory should load");
    assert!(audit.observed_site_count() >= worth_query_native_value_authority_rows().len());
    assert!(
        audit.is_closed(),
        "native-value authority findings: {:#?}",
        audit.findings()
    );
}

#[test]
fn source_inventory_rejects_seeded_coarse_scalar_authority() {
    assert_seed_rejected(
        "src/seeded_scalar.rs",
        "pub enum SeededPredicateValue { String(String), Integer(i64) }",
        "SeededPredicateValue",
    );
}

#[test]
fn source_inventory_rejects_seeded_public_value_stringifier() {
    assert_seed_rejected(
        "src/seeded_encoder.rs",
        r#"
        use worth_foundational::facade::AspectValue;
        pub fn aspect_value_string(value: &AspectValue) -> String {
            match value { AspectValue::Null => "null".to_string(), _ => "other".to_string() }
        }
        "#,
        "aspect_value_string",
    );
}

#[test]
fn source_inventory_rejects_seeded_debug_identity_encoder() {
    assert_seed_rejected(
        "src/seeded_debug_encoder.rs",
        r#"
        use worth_foundational::facade::AspectValue;
        fn aspect_value_identity(value: &AspectValue) -> String {
            format!("{value:?}")
        }
        "#,
        "aspect_value_identity",
    );
}

#[test]
fn source_inventory_rejects_seeded_scalar_only_struct_bridge() {
    assert_seed_rejected(
        "src/seeded_bridge.rs",
        r#"
        use worth_foundational::facade::ContractValidatedAspectValueView;
        pub fn seeded_bridge(value: ContractValidatedAspectValueView<'_>) {
            match value {
                ContractValidatedAspectValueView::Scalar(_) => (),
                ContractValidatedAspectValueView::Struct(_) => panic!("unsupported"),
            }
        }
        "#,
        "seeded_bridge",
    );
}

#[test]
fn source_inventory_rejects_seeded_misleading_row_and_bypass_wrapper() {
    let audit = audit_native_value_authority_sources(&[
        WorthQueryNativeValueSource::new("src/seeded_row.rs", "pub struct SeededNativeRow;"),
        WorthQueryNativeValueSource::new(
            "src/seeded_wrapper.rs",
            r#"
            use worth_foundational::facade::AspectValue;
            pub struct SeededValidatedAspectValue(AspectValue);
            impl SeededValidatedAspectValue {
                pub fn from_native(value: AspectValue) -> Self { Self(value) }
            }
            "#,
        ),
    ]);
    let symbols = audit
        .findings()
        .iter()
        .filter(|finding| finding.kind() == WorthQueryNativeValueFindingKind::UnclassifiedAuthority)
        .map(|finding| finding.site().symbol())
        .collect::<Vec<_>>();
    assert!(symbols.contains(&"SeededNativeRow"));
    assert!(symbols.contains(&"SeededValidatedAspectValue"));
    assert!(symbols.contains(&"SeededValidatedAspectValue::from_native"));
}

fn assert_seed_rejected(path: &str, source: &str, expected_symbol: &str) {
    let audit =
        audit_native_value_authority_sources(&[WorthQueryNativeValueSource::new(path, source)]);
    assert!(
        audit.findings().iter().any(|finding| {
            finding.kind() == WorthQueryNativeValueFindingKind::UnclassifiedAuthority
                && finding.site().path() == path
                && finding.site().symbol() == expected_symbol
        }),
        "missing seeded finding for {expected_symbol}: {:#?}",
        audit.findings()
    );
    let finding = audit
        .findings()
        .iter()
        .find(|finding| finding.site().symbol() == expected_symbol)
        .expect("seeded finding should remain inspectable");
    assert!(finding.site().line() > 0);
}
