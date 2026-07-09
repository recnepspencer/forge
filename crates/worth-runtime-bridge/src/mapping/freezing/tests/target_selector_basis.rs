use crate::mapping::TruthPatchTargetSelector;
use worth_foundational::facade::FieldKey;

#[test]
fn target_selector_canonical_basis_uses_native_target_kinds_not_prefix_coordinates() {
    let cases = [
        (
            field_target("name"),
            "target-selector|kind=entity-field|field-path=name",
        ),
        (
            TruthPatchTargetSelector::relation_endpoint(),
            "target-selector|kind=entity-relation-endpoint",
        ),
        (
            TruthPatchTargetSelector::region(),
            "target-selector|kind=entity-region",
        ),
        (
            TruthPatchTargetSelector::partition(),
            "target-selector|kind=entity-partition",
        ),
        (
            TruthPatchTargetSelector::facet(),
            "target-selector|kind=entity-facet",
        ),
        (TruthPatchTargetSelector::any(), "target-selector|kind=any"),
    ];

    for (selector, expected_basis) in cases {
        assert_eq!(selector.canonical_basis().as_ref(), expected_basis);
        assert!(
            !selector.canonical_basis().starts_with("field:"),
            "target selectors must not preserve route-prefix coordinates"
        );
    }
}

fn field_target(value: &str) -> TruthPatchTargetSelector {
    TruthPatchTargetSelector::entity_field(
        FieldKey::new(value.to_owned()).expect("valid native field key"),
    )
}
