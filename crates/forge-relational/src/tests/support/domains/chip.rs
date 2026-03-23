use crate::facade::runtime::RelationalRuntime;
use crate::tests::support::runtime_with_test_schema;

pub(crate) fn chip_runtime() -> RelationalRuntime {
    runtime_with_test_schema()
}
