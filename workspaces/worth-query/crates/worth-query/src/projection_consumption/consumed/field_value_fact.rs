use worth_foundational::facade::{AspectValue, CanonicalAspectValueIdentityBasis};

use super::native_value::{ConsumedNativeValue, ConsumedNativeValueView};
use crate::projection_consumption::contracts::MaterializedProjectionContract;
use crate::projection_consumption::facts::ProjectionFactFieldPath;
use crate::projection_consumption::source::ProjectionSourceFamily;

#[derive(Clone, Debug, PartialEq)]
pub struct ConsumedFieldValueFact {
    source_row_identity: String,
    source_family: ProjectionSourceFamily,
    source_identity: String,
    projection_authority: String,
    field_path: ProjectionFactFieldPath,
    value: ConsumedNativeValue,
}

impl ConsumedFieldValueFact {
    pub fn source_row_identity(&self) -> &str {
        &self.source_row_identity
    }

    pub fn field_path(&self) -> &ProjectionFactFieldPath {
        &self.field_path
    }

    pub fn source_family(&self) -> ProjectionSourceFamily {
        self.source_family
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn projection_authority(&self) -> &str {
        &self.projection_authority
    }

    pub fn native_value(&self) -> ConsumedNativeValueView<'_> {
        self.value.view()
    }

    pub(crate) fn value_canonical_identity_basis(&self) -> CanonicalAspectValueIdentityBasis {
        self.value.canonical_identity_basis()
    }

    pub(crate) fn new(
        contract: &MaterializedProjectionContract,
        source_row_identity: impl Into<String>,
        field_path: ProjectionFactFieldPath,
        value: AspectValue,
    ) -> Self {
        Self::new_native(
            contract,
            source_row_identity,
            field_path,
            ConsumedNativeValue::scalar(value),
        )
    }

    pub(crate) fn new_native(
        contract: &MaterializedProjectionContract,
        source_row_identity: impl Into<String>,
        field_path: ProjectionFactFieldPath,
        value: ConsumedNativeValue,
    ) -> Self {
        Self {
            source_row_identity: source_row_identity.into(),
            source_family: contract.source_family(),
            source_identity: contract.source_identity().to_string(),
            projection_authority: contract.contract_digest().to_string(),
            field_path,
            value,
        }
    }
}
