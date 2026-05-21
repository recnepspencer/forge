use worth_kernel::facade::{
    analyze_primitive_intent_conflict, analyze_primitive_intent_conflict_with_capabilities,
    prepare_primitive_intent_clarification_request, resolve_primitive_intent_conflict_by_choice,
    resolve_primitive_intent_conflict_by_policy,
};

fn main() {
    let _ = analyze_primitive_intent_conflict;
    let _ = analyze_primitive_intent_conflict_with_capabilities;
    let _ = prepare_primitive_intent_clarification_request;
    let _ = resolve_primitive_intent_conflict_by_choice;
    let _ = resolve_primitive_intent_conflict_by_policy;
}
