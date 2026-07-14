#![allow(unreachable_code)]

use worth_query::facade::certification::{EffectExecutionCertificationBundle, EffectExecutionCertificationOutputDigest, EffectExecutionCertificationRow};

fn main() {
    let _ = EffectExecutionCertificationBundle {
        rows: vec![todo!() as EffectExecutionCertificationRow],
        outputs: vec![todo!() as EffectExecutionCertificationOutputDigest],
        seeded_bundle_digest: String::new(),
        phase4_bundle_digest: String::new(),
        certification_bundle_digest: String::new(),
    };
}
