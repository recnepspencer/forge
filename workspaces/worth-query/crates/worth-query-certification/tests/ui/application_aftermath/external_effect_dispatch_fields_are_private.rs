use worth_query_execution::facade::domain_computation::WorthQueryExternalEffectDispatch;

fn cannot_recombine_dispatch(dispatch: WorthQueryExternalEffectDispatch) {
    let WorthQueryExternalEffectDispatch {
        correlation,
        posture,
        causal_ladder,
        canonical_work,
    } = dispatch;
    let _ = (correlation, posture, causal_ladder, canonical_work);
}

fn main() {}
