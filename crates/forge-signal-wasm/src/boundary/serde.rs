use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsValue;

use super::errors::ForgeSignalJsError;

pub fn from_js<T>(value: JsValue) -> Result<T, ForgeSignalJsError>
where
    T: DeserializeOwned,
{
    serde_wasm_bindgen::from_value(value)
        .map_err(|err| ForgeSignalJsError::invalid_input(format!("invalid wasm payload: {err}")))
}

pub fn to_js<T>(value: &T) -> Result<JsValue, ForgeSignalJsError>
where
    T: Serialize,
{
    let serializer = serde_wasm_bindgen::Serializer::json_compatible();
    value.serialize(&serializer).map_err(|err| {
        ForgeSignalJsError::internal(format!("failed to serialize wasm value: {err}"))
    })
}
