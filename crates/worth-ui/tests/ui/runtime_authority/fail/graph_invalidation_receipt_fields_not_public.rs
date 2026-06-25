use worth_ui::facade::{
    WorthUiGraphDependencyEdge, WorthUiGraphInvalidationCounters,
    WorthUiGraphInvalidationReceipt, WorthUiRuntimeFactSet, WorthUiRuntimeFactSetDigest,
};

fn main() {
    let _receipt = WorthUiGraphInvalidationReceipt {
        authoritative_changed_facts: WorthUiRuntimeFactSet::empty(),
        affected_facts: WorthUiRuntimeFactSet::empty(),
        traversed_edges: Vec::<WorthUiGraphDependencyEdge>::new(),
        counters: counters(),
        receipt_digest: digest(),
    };

    panic!("compile-fail fixture only checks graph invalidation receipt field privacy");
}

fn counters() -> WorthUiGraphInvalidationCounters {
    panic!("fixture only checks graph invalidation receipt field privacy")
}

fn digest() -> WorthUiRuntimeFactSetDigest {
    WorthUiRuntimeFactSet::empty().digest()
}
