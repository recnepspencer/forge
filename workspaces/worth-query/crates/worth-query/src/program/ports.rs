use crate::memory_workspace::WorthQueryEntity;
use crate::runtime::WorthQueryAspectTouch;

use super::values::WorthQueryProgramValue;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPortType {
    String,
    Integer,
    Boolean,
    ProgramValue,
    EntityIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryTypedPort {
    name: String,
    port_type: WorthQueryPortType,
    optional: bool,
    required_aspects: Vec<WorthQueryAspectTouch>,
    binding_slot: Option<String>,
    result_shape: Option<String>,
}

impl WorthQueryTypedPort {
    pub fn new(name: impl Into<String>, port_type: WorthQueryPortType) -> Self {
        Self {
            name: name.into(),
            port_type,
            optional: false,
            required_aspects: Vec::new(),
            binding_slot: None,
            result_shape: None,
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn with_required_aspect(mut self, aspect: WorthQueryAspectTouch) -> Self {
        self.required_aspects.push(aspect);
        self
    }

    pub fn with_binding_slot(mut self, slot: impl Into<String>) -> Self {
        self.binding_slot = Some(slot.into());
        self
    }

    pub fn with_result_shape(mut self, shape: impl Into<String>) -> Self {
        self.result_shape = Some(shape.into());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn port_type(&self) -> &WorthQueryPortType {
        &self.port_type
    }

    pub fn optionality(&self) -> bool {
        self.optional
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryOperationInput {
    name: String,
    value: WorthQueryProgramValue,
}

impl WorthQueryOperationInput {
    pub fn new(name: impl Into<String>, value: WorthQueryProgramValue) -> Self {
        Self::from_program_value(name, value)
    }

    pub fn from_program_value(name: impl Into<String>, value: WorthQueryProgramValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &WorthQueryProgramValue {
        &self.value
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryOperationOutput {
    name: String,
    value: WorthQueryProgramValue,
}

impl WorthQueryOperationOutput {
    pub fn new(name: impl Into<String>, value: WorthQueryProgramValue) -> Self {
        Self::from_program_value(name, value)
    }

    pub fn from_program_value(name: impl Into<String>, value: WorthQueryProgramValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }

    pub(crate) fn from_live_read_entities(
        name: impl Into<String>,
        rows: impl IntoIterator<Item = WorthQueryEntity>,
    ) -> Self {
        Self::from_program_value(name, WorthQueryProgramValue::from_live_read_entities(rows))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &WorthQueryProgramValue {
        &self.value
    }
}
