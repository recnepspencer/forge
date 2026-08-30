use worth_foundational::facade::{AspectValue, ScalarAspectType};

use crate::application_schema::TypedApplicationValue;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryPrincipalMappingStatus {
    Enabled,
    Disabled,
}
crate::worth_query_portable_type!(
    WorthQueryPrincipalMappingStatus => "worth.query.principal_mapping_status.v1"
);

impl TypedApplicationValue for WorthQueryPrincipalMappingStatus {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Bool;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Bool(matches!(self, Self::Enabled))
    }
}
