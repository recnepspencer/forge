use std::path::Path;

use super::{audit_forbidden_call, ForbiddenCall};

fn forbidden_call() -> ForbiddenCall<'static> {
    ForbiddenCall {
        type_name: "UiGraphCoreIndexes",
        method_name: "build",
        allowed_paths: &[],
        message: "rebuilds core indexes outside the graph mutation boundary",
    }
}

fn violates(source: &str) -> bool {
    audit_forbidden_call(Path::new("graph/fake.rs"), source, &forbidden_call()).is_some()
}

#[test]
fn detects_direct_qualified_calls() {
    assert!(violates("fn bad() { UiGraphCoreIndexes::build(plan); }"));
}

#[test]
fn detects_use_alias_calls() {
    assert!(violates(
        "use crate::graph::UiGraphCoreIndexes as Indexes; fn bad() { Indexes::build(plan); }"
    ));
}

#[test]
fn detects_type_alias_calls() {
    assert!(violates(
        "type Indexes = UiGraphCoreIndexes; fn bad() { Indexes::build(plan); }"
    ));
}

#[test]
fn detects_local_rebinding_calls() {
    assert!(violates(
        "fn bad() { let build = UiGraphCoreIndexes::build; build(plan); }"
    ));
}

#[test]
fn ignores_comments_and_strings() {
    assert!(!violates(
        r#"fn okay() { let _ = "UiGraphCoreIndexes::build(plan)"; // UiGraphCoreIndexes::build(plan)
        /* UiGraphCoreIndexes::build(plan) */ }"#
    ));
}
