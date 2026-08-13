use worth_query_execution::facade::domain_computation::{
    ExternalEffectPosture, ExternalEffectPostureIdentity, ExternalEffectPostureKind,
};

fn forge_posture(identity: ExternalEffectPostureIdentity) {
    let _ = ExternalEffectPosture::root(ExternalEffectPostureKind::ProviderCommit, identity);
}

fn main() {}
