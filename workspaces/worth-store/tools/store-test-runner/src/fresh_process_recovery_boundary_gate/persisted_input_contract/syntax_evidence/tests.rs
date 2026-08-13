use super::parse_document;

#[test]
fn comments_and_string_literals_cannot_supply_persisted_syntax_evidence() {
    let index = parse_document("// struct PersistedFact;\nconst NOTE: &str = \"decode_fact\";")
        .expect("parse controlled syntax mutant");
    assert!(!index.declarations.contains("PersistedFact"));
    assert!(index.function_calls.is_empty());
}

#[test]
fn configured_shadowed_wrong_callee_and_lazy_callback_decoys_are_rejected() {
    let index = parse_document(
        "#[cfg(any())] fn disabled(){ Right::publish(); }\n\
         fn causal(){\n\
             let publish = || (); publish();\n\
             Wrong::publish();\n\
             wrong.encode_binding_record();\n\
             #[cfg(any())] Right::disabled();\n\
             #[cfg(any())] let _discarded = Right::configured_local();\n\
             if false { Right::false_branch(); }\n\
             false && Right::short_circuit();\n\
             match false { true => Right::false_arm(), false => Right::selected_arm() };\n\
             match false { _bound @ true => Right::bound_false_arm(), false => () };\n\
             match false { false if false && true => Right::false_guard(), _ => () };\n\
             store(|| Right::lazy());\n\
             binding_compaction.for_each_record(|| Right::publish());\n\
         }",
    )
    .expect("parse controlled dead-syntax mutants");
    let calls = &index.function_calls["causal"];
    assert!(!index.function_calls.contains_key("disabled"));
    assert!(calls.contains("callback:publish"));
    assert!(calls.contains("path:Wrong::publish"));
    assert!(!calls.contains("path:Right::disabled"));
    assert!(!calls.contains("path:Right::configured_local"));
    assert!(!calls.contains("path:Right::false_branch"));
    assert!(!calls.contains("path:Right::short_circuit"));
    assert!(!calls.contains("path:Right::false_arm"));
    assert!(!calls.contains("path:Right::bound_false_arm"));
    assert!(!calls.contains("path:Right::false_guard"));
    assert!(calls.contains("path:Right::selected_arm"));
    assert!(!calls.contains("path:Right::lazy"));
    assert!(calls.contains("path:Right::publish"));
    assert!(calls.contains("method:wrong.encode_binding_record"));
    assert!(!calls.contains("method:right.encode_binding_record"));
}
