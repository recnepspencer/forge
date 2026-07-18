use std::collections::BTreeSet;

use crate::discovery::{CaseKind, TestCaseSurface};

pub(super) fn registration_alias_violations(
    case: &TestCaseSurface,
    target_identities: &BTreeSet<&str>,
) -> Vec<String> {
    let mut violations = Vec::new();
    let mut aliases = BTreeSet::new();
    for alias in &case.registration_alias_targets {
        if case.kind != CaseKind::UiFixture {
            violations.push(format!(
                "non-UI proof declares a registration alias: {}",
                case.identity.stable_id
            ));
        }
        if case.target_identity.as_deref() == Some(alias.as_str()) {
            violations.push(format!(
                "proof aliases its canonical registration target: {}",
                case.identity.stable_id
            ));
        }
        if !aliases.insert(alias.as_str()) {
            violations.push(format!(
                "proof repeats registration alias {alias}: {}",
                case.identity.stable_id
            ));
        }
        if !target_identities.contains(alias.as_str()) {
            violations.push(format!(
                "proof aliases unknown registration target {alias}: {}",
                case.identity.stable_id
            ));
        }
    }
    violations
}
