use crate::facade::runtime::RelationalRuntime;
use crate::tests::support::runtime_with_test_schema;

pub(crate) fn cad_runtime() -> RelationalRuntime {
    runtime_with_test_schema()
}
