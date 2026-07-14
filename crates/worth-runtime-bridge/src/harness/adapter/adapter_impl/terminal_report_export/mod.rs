mod json_projection;
mod writeback_counter_json_projection;

pub(in crate::harness::adapter::adapter_impl) use json_projection::{
    execution_extensions_json, execution_summary_json,
};
pub(in crate::harness::adapter::adapter_impl) use writeback_counter_json_projection::writeback_counter_snapshot_json;
