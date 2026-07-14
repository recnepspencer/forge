use worth_query::facade::foundation::{EffectAuthorityLane, EffectEnvelopePrimaryResult, EffectFamily, SelfDescribingEffectEnvelope};

fn main() {
    let _ = SelfDescribingEffectEnvelope {
        declared_effect_family: EffectFamily::Mutation,
        authority_lane: EffectAuthorityLane::Relational,
        primary_result: EffectEnvelopePrimaryResult::MutationCommitted,
        warnings: Vec::new(),
        trace_digest: String::new(),
        structural_deltas: Vec::new(),
        integrity_digest: String::new(),
        performance_digest: String::new(),
        boundary_digest: String::new(),
        envelope_digest: String::new(),
    };
}
