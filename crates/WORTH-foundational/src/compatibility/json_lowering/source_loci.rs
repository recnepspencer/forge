use crate::aspects::{CanonicalFieldPath, ContractValidationDenial, FieldKey};
use crate::locators::{AspectFieldLocator, BoundarySourceLocator};

pub(super) fn contract_validation_source(
    source: &BoundarySourceLocator,
    denial: &ContractValidationDenial,
) -> BoundarySourceLocator {
    match denial {
        ContractValidationDenial::MissingRequiredField(field)
        | ContractValidationDenial::UnknownField(field)
        | ContractValidationDenial::FieldTypeMismatch { field, .. } => field_source(source, field),
        ContractValidationDenial::ScalarTypeMismatch { .. }
        | ContractValidationDenial::StructValueRequired
        | ContractValidationDenial::ScalarValueRequired => source.clone(),
    }
}

pub(super) fn field_source(
    source: &BoundarySourceLocator,
    field_key: &FieldKey,
) -> BoundarySourceLocator {
    match source {
        BoundarySourceLocator::Aspect(aspect) => {
            BoundarySourceLocator::aspect_field(AspectFieldLocator::new(
                aspect.authority(),
                aspect.aspect_key().clone(),
                CanonicalFieldPath::single(field_key.clone()),
            ))
        }
        _ => source.clone(),
    }
}
