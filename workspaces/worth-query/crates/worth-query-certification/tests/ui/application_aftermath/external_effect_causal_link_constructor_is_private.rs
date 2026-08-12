use worth_query_execution::facade::domain_computation::{
    ExternalEffectCausalLink, ExternalEffectPosture,
};

fn forge_link(posture: &ExternalEffectPosture) {
    let _ = ExternalEffectCausalLink::to(posture.identity());
}

fn main() {}
