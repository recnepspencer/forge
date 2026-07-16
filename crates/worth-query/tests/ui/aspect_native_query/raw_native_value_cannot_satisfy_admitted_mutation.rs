use worth_foundational::facade::AspectValue;
use worth_query::facade::runtime::WorthQueryBackendAdmissibleMutation;

fn require_admitted(_: WorthQueryBackendAdmissibleMutation) {}

fn main() {
    require_admitted(AspectValue::Null);
}
