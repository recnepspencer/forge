use worth_ui::facade::WorthUiExecutionPlanEquivalence;

fn consume_plan_equivalence(_: WorthUiExecutionPlanEquivalence) {}

fn main() {
    let left: *const () = std::ptr::null();
    let right: *const () = std::ptr::null();
    consume_plan_equivalence(std::ptr::eq(left, right));
}
