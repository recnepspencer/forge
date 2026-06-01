use std::collections::BTreeMap;

use forge_foundational::facade::{
    aspects, compatibility, AspectLocator, AspectValue, BoundarySourceLocator,
    ContractValidatedAspectValueView, LocatorAuthority,
};
use forge_proof::TransitionOutcome;
use forge_relational::facade::transactions::AspectFieldPatch;
use serde_json::Value;

use super::declarations::scalar_string_contract;
use super::keys::{aspect_key, field_key, planned_single_field_locator};

pub(crate) fn single_aspect_field_patch_from_external_json(
    aspect_label: &str,
    field_label: &str,
    value: Value,
) -> Result<AspectFieldPatch, String> {
    aspect_field_patch_from_external_json_values([(aspect_label, field_label, value)])
}

pub(crate) fn aspect_field_patch_from_external_json_values<'a>(
    values: impl IntoIterator<Item = (&'a str, &'a str, Value)>,
) -> Result<AspectFieldPatch, String> {
    let mut targets = BTreeMap::new();
    for (aspect_label, field_label, value) in values {
        targets.insert(
            planned_single_field_locator(aspect_key(aspect_label)?, field_key(field_label)?),
            lower_external_json_through_scalar_string_contract(aspect_label, &value)?,
        );
    }
    Ok(AspectFieldPatch::from(targets))
}

pub(crate) fn lower_external_json_through_scalar_string_contract(
    aspect_label: &str,
    value: &Value,
) -> Result<AspectValue, String> {
    let contract = scalar_string_contract(aspect_label)?;
    let source = BoundarySourceLocator::aspect(AspectLocator::new(
        LocatorAuthority::SupportOnly,
        aspects().vocabulary().key(aspect_label).map_err(|_| {
            format!("`{aspect_label}` is not a foundational compatibility source key")
        })?,
    ));
    let TransitionOutcome::Success(validated) =
        compatibility().json().lower_value(&contract, source, value)
    else {
        return Err(format!(
            "external JSON for aspect `{aspect_label}` did not lower through foundational compatibility"
        ));
    };
    match validated.payload().view() {
        ContractValidatedAspectValueView::Scalar(value) => Ok(value.clone()),
        ContractValidatedAspectValueView::Struct(_) => Err(format!(
            "external JSON for aspect `{aspect_label}` lowered to a struct where a scalar field patch was required"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn external_json_field_patch_ingress_uses_foundational_contract_validation() {
        let patch =
            single_aspect_field_patch_from_external_json("name", "name", json!("validated"))
                .expect("string aspect should lower through compatibility");
        let locator = planned_single_field_locator(
            aspect_key("name").expect("aspect key"),
            field_key("name").expect("field key"),
        );

        assert_eq!(
            patch.get(&locator),
            Some(&AspectValue::String("validated".into()))
        );
    }

    #[test]
    fn external_json_field_patch_ingress_rejects_values_outside_contract_shape() {
        let denial = single_aspect_field_patch_from_external_json("name", "name", json!(3))
            .expect_err("numeric JSON must not bypass the string aspect contract");

        assert!(denial.contains("foundational compatibility"));
    }
}
