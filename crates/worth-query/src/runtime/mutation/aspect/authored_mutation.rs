use worth_foundational::facade::{AspectValue, ContractValidationInput, StructAspectValue};

use crate::memory_workspace::WorthQueryWorkspaceError;
use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAspectMutationOperationKind, WorthQueryAspectTouch,
};

use super::super::{
    WorthQueryDesiredAspectValue, WorthQueryParsedAspectTarget, WorthQueryParsedDesiredAspect,
};

/// Parsed authoring intent. Contract validation occurs only at runtime admission.
#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAuthoredAspectMutation {
    parsed: WorthQueryParsedDesiredAspect,
}

impl WorthQueryAuthoredAspectMutation {
    pub(crate) fn native_string_value(value: impl Into<String>) -> AspectValue {
        AspectValue::String(value.into().into())
    }

    #[cfg(test)]
    pub(crate) fn new(
        aspect_touch: WorthQueryAspectTouch,
        value: AspectValue,
    ) -> Result<Self, WorthQueryWorkspaceError> {
        Self::new_set(aspect_touch, value)
    }

    pub(crate) fn new_set(
        aspect_touch: WorthQueryAspectTouch,
        value: impl Into<ContractValidationInput>,
    ) -> Result<Self, WorthQueryWorkspaceError> {
        Ok(Self::from_touch_parts(
            aspect_touch,
            WorthQueryDesiredAspectValue::set_native(value.into()),
        ))
    }

    pub(crate) fn new_clear(
        aspect_touch: WorthQueryAspectTouch,
    ) -> Result<Self, WorthQueryWorkspaceError> {
        Ok(Self {
            parsed: WorthQueryParsedDesiredAspect::new(
                aspect_touch.into_parsed_target(),
                WorthQueryDesiredAspectValue::clear(),
            ),
        })
    }

    pub fn aspect_touch(&self) -> WorthQueryAspectTouch {
        WorthQueryAspectTouch::from_parsed_target(self.parsed_target().clone())
    }

    pub(crate) fn parsed_target(&self) -> &WorthQueryParsedAspectTarget {
        self.parsed.target()
    }

    pub fn foundational_value(&self) -> Option<&AspectValue> {
        self.parsed.desired().value()
    }

    pub fn foundational_struct_value(&self) -> Option<&StructAspectValue> {
        self.parsed.desired().struct_value()
    }

    pub(crate) fn validation_input(&self) -> Option<&ContractValidationInput> {
        self.parsed.desired().validation_input()
    }

    pub(crate) fn terminal_digest_material(&self) -> String {
        format!(
            "{}={}",
            WorthQueryAspectTouch::from_parsed_target(self.parsed_target().clone())
                .admitted_touch_digest_part(),
            self.parsed.desired().terminal_digest_material()
        )
    }

    pub fn clears_existing_value(&self) -> bool {
        self.parsed.desired().clears_existing_value()
    }

    pub fn declared_operation(&self) -> WorthQueryAspectMutationOperation {
        WorthQueryAspectMutationOperation::from_touch(
            WorthQueryAspectTouch::from_parsed_target(self.parsed_target().clone()),
            if self.clears_existing_value() {
                WorthQueryAspectMutationOperationKind::Clear
            } else {
                WorthQueryAspectMutationOperationKind::Set
            },
        )
    }

    fn from_touch_parts(
        aspect_touch: WorthQueryAspectTouch,
        desired: WorthQueryDesiredAspectValue,
    ) -> Self {
        Self {
            parsed: WorthQueryParsedDesiredAspect::new(aspect_touch.into_parsed_target(), desired),
        }
    }
}
