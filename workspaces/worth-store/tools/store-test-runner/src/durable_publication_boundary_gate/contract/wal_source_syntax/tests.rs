use super::ParsedRustSource;

#[test]
fn macro_tokens_cannot_hide_a_denied_call() {
    let source = ParsedRustSource::parse(
        r#"
            fn inspect() {
                let _names = worth_dbg!(tree.list_file_names(&directory));
            }
        "#,
        "macro denial control",
    )
    .expect("parse macro denial control");
    assert!(
        source
            .function("inspect")
            .expect("find macro denial control")
            .deny("method:list_file_names")
            .is_err(),
        "MUTANT_PREDICATE:wal-source-macro-denial-bypass-accepted"
    );
}

#[test]
fn deferred_closure_cannot_supply_a_required_step() {
    let source = ParsedRustSource::parse(
        r#"
            fn inspect() {
                let _counterfeit = || tree.list_file_names_bounded(&directory, limit);
            }
        "#,
        "deferred closure control",
    )
    .expect("parse deferred closure control");
    assert!(
        source
            .function("inspect")
            .expect("find deferred closure control")
            .require_exact("method:list_file_names_bounded", 1)
            .is_err(),
        "MUTANT_PREDICATE:wal-source-dead-closure-requirement-accepted"
    );
}

#[test]
fn statically_dead_branch_cannot_supply_a_required_step() {
    let source = ParsedRustSource::parse(
        r#"
            fn inspect() {
                if false {
                    tree.list_file_names_bounded(&directory, limit);
                }
            }
        "#,
        "dead branch control",
    )
    .expect("parse dead branch control");
    assert!(
        source
            .function("inspect")
            .expect("find dead branch control")
            .require_exact("method:list_file_names_bounded", 1)
            .is_err(),
        "MUTANT_PREDICATE:wal-source-dead-branch-requirement-accepted"
    );
}

#[test]
fn trivially_unreachable_match_arm_cannot_supply_a_required_step() {
    let source = ParsedRustSource::parse(
        r#"
            fn inspect() {
                match true {
                    false => tree.list_file_names_bounded(&directory, limit),
                    true => Vec::new(),
                };
            }
        "#,
        "unreachable match control",
    )
    .expect("parse unreachable match control");
    assert!(
        source
            .function("inspect")
            .expect("find unreachable match control")
            .require_exact("method:list_file_names_bounded", 1)
            .is_err(),
        "MUTANT_PREDICATE:wal-source-unreachable-match-requirement-accepted"
    );
}

#[test]
fn nested_function_cannot_supply_a_required_step() {
    let source = ParsedRustSource::parse(
        r#"
            fn inspect() {
                fn counterfeit() {
                    tree.list_file_names_bounded(&directory, limit);
                }
            }
        "#,
        "nested function control",
    )
    .expect("parse nested function control");
    assert!(
        source
            .function("inspect")
            .expect("find nested function control")
            .require_exact("method:list_file_names_bounded", 1)
            .is_err(),
        "MUTANT_PREDICATE:wal-source-nested-function-requirement-accepted"
    );
}

#[test]
fn deferred_syntax_remains_visible_to_prohibitions() {
    let source = ParsedRustSource::parse(
        r#"
            fn inspect() {
                let _counterfeit = || tree.list_file_names(&directory);
            }
        "#,
        "deferred prohibition control",
    )
    .expect("parse deferred prohibition control");
    assert!(
        source
            .function("inspect")
            .expect("find deferred prohibition control")
            .deny("method:list_file_names")
            .is_err(),
        "MUTANT_PREDICATE:wal-source-deferred-prohibition-bypass-accepted"
    );
}

#[test]
fn cfg_test_function_does_not_compete_with_production_owner() {
    let source = ParsedRustSource::parse(
        r#"
            fn reopen_wal_inventory() {}

            #[cfg(test)]
            mod tests {
                fn reopen_wal_inventory() {}
            }
        "#,
        "cfg-test owner control",
    )
    .expect("parse cfg-test owner control");
    source
        .function("reopen_wal_inventory")
        .unwrap_or_else(|denial| {
            panic!("MUTANT_PREDICATE:wal-source-cfg-test-owner-collision: {denial}")
        });
}

#[test]
fn nested_calls_follow_rust_evaluation_order() {
    let source = ParsedRustSource::parse(
        r#"
            fn nested_order() {
                owner.release_group_before_effect();
                Reserved::from_members(pending).release_after_no_effect();
            }
        "#,
        "nested-call control",
    )
    .expect("parse nested-call control");
    source
        .function("nested_order")
        .expect("find nested-call control")
        .require_in_order(&[
            "method:release_group_before_effect",
            "call:from_members",
            "method:release_after_no_effect",
        ])
        .unwrap_or_else(|denial| {
            panic!("MUTANT_PREDICATE:wal-source-semantic-order-inverted-accepted: {denial}")
        });
}
