use worth_query::facade::foundation::WorthQueryEntityIdentity;

use crate::UiCollectionProjectionFactReceipt;

pub(super) fn assert_exact_update(
    fact: &UiCollectionProjectionFactReceipt,
    expected_entity: &WorthQueryEntityIdentity,
    expected_value: &str,
) {
    let expected_identity = expected_entity.evidence_identity().operational_key();
    assert!(matches!(
        fact.changes(),
        [crate::UiCollectionProjectionChange::Update { row }]
            if row.query_identity().operational_key() == expected_identity
    ));
    let crate::UiProjectionAvailability::Present(crate::UiPresentProjection::Current(value)) =
        fact.availability()
    else {
        panic!("valid post-denial delivery must produce one current projection");
    };
    assert!(matches!(
        value.rows(),
        [row] if row.selected_values().len() == 1
            && row.selected_values()[0].as_str() == expected_value
    ));
}
