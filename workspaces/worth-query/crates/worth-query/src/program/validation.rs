use std::collections::BTreeMap;

use super::error::WorthQueryProgramError;
use super::operations::WorthQueryOperation;
use super::ports::{WorthQueryOperationInput, WorthQueryPortType};
use super::values::WorthQueryProgramValue;

pub(crate) fn validate_inputs(
    operation: &WorthQueryOperation,
    inputs: &[WorthQueryOperationInput],
) -> Result<BTreeMap<String, WorthQueryProgramValue>, WorthQueryProgramError> {
    let provided = inputs
        .iter()
        .map(|input| (input.name(), input.value()))
        .collect::<BTreeMap<_, _>>();
    let mut bound = BTreeMap::new();
    for port in operation.inputs() {
        let Some(value) = provided.get(port.name()) else {
            if port.optionality() {
                continue;
            }
            return Err(WorthQueryProgramError::new(format!(
                "missing required input `{}`",
                port.name()
            )));
        };
        if !value_matches_port(value, port.port_type()) {
            return Err(WorthQueryProgramError::new(format!(
                "input `{}` does not match required type {:?}",
                port.name(),
                port.port_type()
            )));
        }
        bound.insert(port.name().to_string(), (*value).clone());
    }
    Ok(bound)
}

pub(super) fn expect_string(
    value: WorthQueryProgramValue,
    label: &str,
) -> Result<String, WorthQueryProgramError> {
    value.string_value().map(ToOwned::to_owned).ok_or_else(|| {
        WorthQueryProgramError::new(format!("bound `{label}` must evaluate to a string"))
    })
}

fn value_matches_port(value: &WorthQueryProgramValue, port_type: &WorthQueryPortType) -> bool {
    match port_type {
        WorthQueryPortType::String | WorthQueryPortType::EntityIdentity => value.is_string(),
        WorthQueryPortType::Integer => value.is_integer(),
        WorthQueryPortType::Boolean => value.is_boolean(),
        WorthQueryPortType::ProgramValue => true,
    }
}
