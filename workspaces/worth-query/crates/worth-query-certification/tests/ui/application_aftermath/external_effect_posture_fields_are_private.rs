use worth_query_execution::facade::domain_computation::ExternalEffectPosture;

fn recycle(posture: ExternalEffectPosture) {
    let ExternalEffectPosture {
        kind,
        identity,
        predecessor,
    } = posture;
    let _ = (kind, identity, predecessor);
}

fn main() {}
