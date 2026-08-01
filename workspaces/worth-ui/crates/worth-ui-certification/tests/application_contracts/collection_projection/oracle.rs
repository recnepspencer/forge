use std::collections::BTreeMap;

use worth_ui_query_binding::{
    UiCollectionProjectionFactReceipt, UiPresentProjection, UiProjectionAvailability,
};

#[derive(Default)]
pub(super) struct ExpectedKeyedRows {
    rows: BTreeMap<[u8; 32], String>,
}

impl ExpectedKeyedRows {
    pub(super) fn insert(&mut self, identity: [u8; 32], value: impl Into<String>) {
        assert!(
            self.rows.insert(identity, value.into()).is_none(),
            "expected row identity must be unique"
        );
    }

    pub(super) fn update(&mut self, identity: &[u8; 32], value: impl Into<String>) {
        *self
            .rows
            .get_mut(identity)
            .expect("updated expected identity must exist") = value.into();
    }

    pub(super) fn remove(&mut self, identity: &[u8; 32]) {
        assert!(
            self.rows.remove(identity).is_some(),
            "removed expected identity must exist"
        );
    }

    pub(super) fn selected(&self, identities: &[[u8; 32]]) -> BTreeMap<[u8; 32], String> {
        identities
            .iter()
            .map(|identity| {
                (
                    *identity,
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
        expected: &BTreeMap<[u8; 32], String>,
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
                    row.row().identity().host_correlation_digest(),
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
