use worth_foundational::facade::AspectValue;

use super::native_value::{
    ConsumedNativeValue, ConsumedNativeValueIdentityBasis, ConsumedNativeValueView,
};
use crate::projection_consumption::contracts::BoundProjectionFactFamily;
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
    native_selection_identity: Option<u64>,
    native_contract_context: Option<ConsumedNativeContractContext>,
    value: ConsumedNativeValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ConsumedNativeContractContext {
    key: worth_foundational::facade::AspectKey,
    identity: worth_foundational::facade::AspectIdentity,
    revision: worth_foundational::facade::AspectContractRevision,
}

impl ConsumedNativeContractContext {
    fn from_declaration(
        declaration: &crate::projection_consumption::DeclaredNativeFactContract,
    ) -> Self {
        Self {
            key: declaration.contract().key().clone(),
            identity: declaration.contract().identity(),
            revision: declaration.contract().revision(),
        }
    }

    pub(crate) fn key(&self) -> &worth_foundational::facade::AspectKey {
        &self.key
    }

    pub(crate) fn identity(&self) -> worth_foundational::facade::AspectIdentity {
        self.identity
    }

    pub(crate) fn revision(&self) -> worth_foundational::facade::AspectContractRevision {
        self.revision
    }
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

    pub(crate) fn native_selection_identity(&self) -> Option<u64> {
        self.native_selection_identity
    }

    pub(crate) fn native_contract_context(&self) -> Option<&ConsumedNativeContractContext> {
        self.native_contract_context.as_ref()
    }

    pub(crate) fn value_canonical_identity_basis(&self) -> ConsumedNativeValueIdentityBasis {
        self.value.canonical_identity_basis()
    }

    pub(crate) fn cloned_native_value(&self) -> ConsumedNativeValue {
        self.value.clone()
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
        Self::new_with_native_declaration(contract, source_row_identity, field_path, None, value)
    }

    pub(crate) fn new_declared_native(
        contract: &MaterializedProjectionContract,
        source_row_identity: impl Into<String>,
        declaration: &crate::projection_consumption::DeclaredNativeFactContract,
        value: ConsumedNativeValue,
    ) -> Self {
        Self::new_with_native_declaration(
            contract,
            source_row_identity,
            declaration.field_path().clone(),
            Some(declaration),
            value,
        )
    }

    pub(crate) fn new_from_bound_family(
        contract: &MaterializedProjectionContract,
        source_row_identity: impl Into<String>,
        family: &BoundProjectionFactFamily,
        value: ConsumedNativeValue,
    ) -> Self {
        match family.native_contract() {
            Some(declaration) => {
                Self::new_declared_native(contract, source_row_identity, declaration, value)
            }
            None => Self::new_native(
                contract,
                source_row_identity,
                family
                    .field_path()
                    .expect("field families retain their requested path")
                    .clone(),
                value,
            ),
        }
    }

    fn new_with_native_declaration(
        contract: &MaterializedProjectionContract,
        source_row_identity: impl Into<String>,
        field_path: ProjectionFactFieldPath,
        native_declaration: Option<&crate::projection_consumption::DeclaredNativeFactContract>,
        value: ConsumedNativeValue,
    ) -> Self {
        Self {
            source_row_identity: source_row_identity.into(),
            source_family: contract.source_family(),
            source_identity: contract.source_identity().to_string(),
            projection_authority: contract.contract_digest().to_string(),
            field_path,
            native_selection_identity: native_declaration
                .map(crate::projection_consumption::DeclaredNativeFactContract::selection_identity),
            native_contract_context: native_declaration
                .map(ConsumedNativeContractContext::from_declaration),
            value,
        }
    }
}
