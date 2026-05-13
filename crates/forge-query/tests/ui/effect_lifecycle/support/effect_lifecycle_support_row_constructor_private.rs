use forge_query::facade::{
    BasisFamily, EffectFamily, EffectLifecycleSupportRow, EffectSupportPosture,
};

fn main() {
    let _ = EffectLifecycleSupportRow {
        basis_family: BasisFamily::CurrentHead,
        effect_family: EffectFamily::Mutation,
        posture: EffectSupportPosture::Admitted,
        row_digest: String::new(),
    };
}
