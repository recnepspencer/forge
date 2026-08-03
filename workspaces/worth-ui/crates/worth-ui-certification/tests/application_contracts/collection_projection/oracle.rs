use std::collections::BTreeMap;

use worth_ui_query_binding::{
    UiCollectionProjectionFactReceipt, UiPresentProjection, UiProjectionAvailability,
};

#[derive(Default)]
pub(super) struct ExpectedKeyedRows {
    rows: BTreeMap<String, String>,
}

impl ExpectedKeyedRows {
    pub(super) fn insert(&mut self, identity: impl Into<String>, value: impl Into<String>) {
        assert!(
            self.rows.insert(identity.into(), value.into()).is_none(),
            "expected row identity must be unique"
        );
    }

    pub(super) fn update(&mut self, identity: &str, value: impl Into<String>) {
        *self
            .rows
            .get_mut(identity)
            .expect("updated expected identity must exist") = value.into();
    }

    pub(super) fn remove(&mut self, identity: &str) {
        assert!(
            self.rows.remove(identity).is_some(),
            "removed expected identity must exist"
        );
    }

    pub(super) fn selected(&self, identities: &[String]) -> BTreeMap<String, String> {
        identities
            .iter()
            .map(|identity| {
                (
                    identity.clone(),
                    self.rows
                        .get(identity)
                        .expect("selected expected identity must exist")
                        .clone(),
                )
            })
            .collect()
    }

    pub(super) fn assert_fact_rows(
        &self,
        fact: &UiCollectionProjectionFactReceipt,
        expected: &BTreeMap<String, String>,
    ) {
        let actual = present(fact)
            .rows()
            .iter()
            .map(|row| {
                assert_eq!(
                    row.selected_values().len(),
                    1,
                    "QP04 declares one native text field"
                );
                (
                    row.row().reporting_projection().as_str().to_owned(),
                    row.selected_values()[0].as_str().to_owned(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        assert_eq!(&actual, expected);
    }
}

pub(super) fn present(
    fact: &UiCollectionProjectionFactReceipt,
) -> &worth_ui_query_binding::UiCollectionProjectionValue {
    match fact.availability() {
        UiProjectionAvailability::Present(UiPresentProjection::Current(value)) => value,
        other => panic!("expected current collection fact, got {other:?}"),
    }
}
