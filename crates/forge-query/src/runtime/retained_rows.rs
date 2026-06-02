use serde::de::DeserializeOwned;
use serde_json::Value;

use super::ForgeQueryRuntimeError;

pub(in crate::runtime) fn decode_single_retained_row<T>(
    rows: &[Value],
    view_name: &str,
    stage: &'static str,
) -> Result<T, ForgeQueryRuntimeError>
where
    T: DeserializeOwned,
{
    match rows {
        [] => Err(ForgeQueryRuntimeError::RetainedRowDecode {
            view_name: view_name.to_string(),
            stage,
            message: "expected one retained row, found none".to_string(),
        }),
        [row] => serde_json::from_value(row.clone()).map_err(|error| {
            ForgeQueryRuntimeError::RetainedRowDecode {
                view_name: view_name.to_string(),
                stage,
                message: format!("row failed to decode: {error}"),
            }
        }),
        _ => Err(ForgeQueryRuntimeError::RetainedRowDecode {
            view_name: view_name.to_string(),
            stage,
            message: format!("expected one retained row, found {}", rows.len()),
        }),
    }
}
