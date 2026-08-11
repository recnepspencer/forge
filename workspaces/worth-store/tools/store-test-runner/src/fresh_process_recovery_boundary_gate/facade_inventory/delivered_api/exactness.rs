use std::collections::BTreeSet;

use super::delivered_facades;

pub(crate) fn assert_exact_inventory(expected: BTreeSet<(String, String)>) -> Result<(), String> {
    let actual = delivered_facades()?;
    let omitted = actual.difference(&expected).collect::<Vec<_>>();
    let stale = expected.difference(&actual).collect::<Vec<_>>();
    if omitted.is_empty() && stale.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "delivered C.8 facade inventory omitted {omitted:?} or retained stale {stale:?}"
        ))
    }
}
