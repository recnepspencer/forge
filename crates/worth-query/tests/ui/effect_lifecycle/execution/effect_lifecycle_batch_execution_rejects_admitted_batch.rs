use worth_query::facade::foundation::{AdmittedEffectBatch, EffectExecutionAuthority};
use worth_relational::facade::runtime::RelationalRuntimeApi;

fn admitted_batch() -> AdmittedEffectBatch {
    unimplemented!()
}

fn main() {
    let mut runtime = RelationalRuntimeApi::builder().build();
    let _ = admitted_batch().execute_with(EffectExecutionAuthority::relational(&mut runtime));
}
